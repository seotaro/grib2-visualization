//! GRIB2 decode utility

use std::mem;

use super::section::section5_template::Template3 as Section5Template3;
use super::type_utils_impl::i16_be;
use super::type_utils_impl::u16_be;
use super::type_utils_impl::u32_be;

pub(crate) fn unpack(buf: &[u8], bits: usize) -> Vec<u16> {
    assert!(bits < mem::size_of::<u16>() * 8);

    const TEMP_BITS: usize = mem::size_of::<u32>() * 8; // 一時変数のビット数
    const TEMP_FULL_BIT: u32 = !0; // 一時変数で全ビットを立てた状態

    let totol_bits = buf.len() * 8;

    let mut values: Vec<u16> = Vec::new();
    let mut bit_pos = 0; // バッファでのビット位置
    while (bit_pos + bits) <= totol_bits {
        let byte_pos = bit_pos / 8; // バッファでのバイト位置

        let value = if (byte_pos + 4) < buf.len() {
            u32_be(&buf[byte_pos..byte_pos + 4])
        } else {
            // 残りバッファが足りない分はゼロ埋めする。
            let mut t = 0u32;
            let type_len = mem::size_of::<u32>();
            for i in 0..type_len {
                let k = if (byte_pos + i) < buf.len() {
                    buf[byte_pos + i] as u32
                } else {
                    0u32
                };
                t |= k << ((type_len - i - 1) * 8);
            }
            t
        };

        let temp_start_bit_pos = bit_pos % 8; // 一時変数でのスタート
        let temp_end_bit_pos = temp_start_bit_pos + bits; // 一時変数でのエンド
        assert!(temp_end_bit_pos < TEMP_BITS); // 一時変数に収まること

        let mask = TEMP_FULL_BIT
            & (TEMP_FULL_BIT >> temp_start_bit_pos)
            & (TEMP_FULL_BIT << (TEMP_BITS - temp_end_bit_pos));

        values.push(((value & mask) >> (TEMP_BITS - temp_end_bit_pos)) as u16);

        bit_pos += bits;
    }
    assert!(bit_pos <= totol_bits);

    return values;
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct RunLength {
    value: usize,
    length: usize,
}

// レベル値（8bpp）を返す。
pub(crate) fn unpack_run_length_packing(buf: &[u8], bits: usize, max_level: usize) -> Vec<u8> {
    let values = unpack(buf, bits);

    // ランレングスの値と長さの組を取得する。
    let mut runlengths: Vec<RunLength> = Vec::new();
    {
        // 最初の値セット
        let mut runlength = RunLength {
            value: values[0] as usize,
            length: 1,
        };

        // レングスは任意の数値を底とするn進数で表す。
        let base = 2usize.pow(bits as u32) - 1 - max_level; // n進数の底。
        let mut digit = 0; // 桁数

        for i in 1..values.len() {
            let t = values[i] as usize;

            if max_level < t {
                // ランレングス値の場合
                runlength.length += base.pow(digit) * (t - (max_level + 1));
                digit += 1;
            } else {
                // 格子点値の場合

                // 直前の値セットを追加する。
                runlengths.push(runlength);

                // 新しい値セットを始める。
                runlength = RunLength {
                    value: t,
                    length: 1,
                };

                digit = 0;
            }
        }

        // 最後の値セット
        if 0 < runlength.length {
            runlengths.push(runlength);
        }
    }

    // ランレングスを展開する。
    let mut dest = Vec::<u8>::new();
    for runlength in runlengths.iter() {
        dest.resize(dest.len() + runlength.length, runlength.value as u8);
    }
    return dest;
}

struct Group {
    group_ref: usize,
    values: Vec<u16>,
}

// 16bpp の simgle packing data を返す。
pub(crate) fn unpack_complex_packing_and_spatial_differencing(
    buf: &[u8],
    section5_template3: Section5Template3,
) -> Vec<u16> {
    let bits = section5_template3.bits();
    assert!(bits < mem::size_of::<u16>() * 8);

    assert!(section5_template3.order() == 2); // 2階空間差分 決め打ち

    let octets = section5_template3.octets();
    assert!(octets == 2); // 2オクテット 決め打ち
    let mut i = 0;
    let h1 = u16_be(&buf[i..(i + octets)]);
    i += octets;
    let h2 = u16_be(&buf[i..(i + octets)]);
    i += octets;
    let hmin = i16_be(&buf[i..(i + octets)]);
    i += octets;

    let ng = section5_template3.ng();
    let gr_width_ref = section5_template3.group_width_reference();
    let gr_width_bits = section5_template3.group_width_bits();
    let gr_len_ref = section5_template3.group_length_reference();
    let gr_len_inc = section5_template3.group_length_increment();
    let gr_len_bits = section5_template3.group_length_bits();
    let last_gr_len = section5_template3.last_group_length();

    let gr_ref_len = ((bits * ng) as f32 / 8.0).ceil() as usize;
    let gr_ref = unpack(&buf[i..(i + gr_ref_len)], bits);
    i += gr_ref_len;

    let gr_width_len = ((gr_width_bits * ng) as f32 / 8.0).ceil() as usize;
    let gr_width = unpack(&buf[i..(i + gr_width_len)], gr_width_bits);
    i += gr_width_len;

    let gr_len_len = ((gr_len_bits * ng) as f32 / 8.0).ceil() as usize;
    let gr_len = unpack(&buf[i..(i + gr_len_len)], gr_len_bits);
    i += gr_len_len;

    let mut groups: Vec<Group> = Vec::new();
    for m in 1..=ng {
        let group_length = if m == ng {
            last_gr_len
        } else {
            gr_len_ref + gr_len_inc * gr_len[m - 1] as usize
        };
        let group_width = gr_width_ref + gr_width[m - 1] as usize;

        let values;
        if 0 < group_width {
            let length = ((group_width * group_length) as f32 / 8.0).ceil() as usize;
            values = unpack(&buf[i..(i + length)], group_width);
            i += length;
        } else {
            values = vec![0; group_length];
        }

        groups.push(Group {
            values: values,
            group_ref: gr_ref[m - 1] as usize,
        })
    }

    let mut h: Vec<i16> = Vec::new();
    for n in 0..groups.len() {
        let group = &groups[n];
        for value in group.values.iter() {
            h.push(*value as i16 + group.group_ref as i16 + hmin);
        }
    }
    h[0] = h1 as i16;
    h[1] = h2 as i16;

    let mut g: Vec<u16> = Vec::new();
    g.push(h[0] as u16);
    g.push(h[1] as u16);
    for n in 2..h.len() {
        g.push((h[n] + 2 * g[n - 1] as i16 - g[n - 2] as i16) as u16);
    }

    return g;
}

pub(crate) fn unpack_simple_packing(src: &Vec<usize>, r: f32, e: i32, d: i32) -> Vec<f32> {
    let mut dest: Vec<f32> = vec![f32::NAN; src.len()];
    for (i, value) in src.into_iter().enumerate() {
        dest[i] = (r + (*value as f32) * 2.0f32.powi(e)) / 10.0f32.powi(d);
    }
    return dest;
}

pub(crate) fn unpack_level(src: &Vec<u8>, factor: isize, levels: &Vec<u16>) -> Vec<f32> {
    let mut dest: Vec<f32> = vec![f32::NAN; src.len()];
    for (i, value) in src.into_iter().enumerate() {
        if 0 < *value {
            dest[i] = (levels[*value as usize] as f32) / 10.0f32.powi(factor as i32);
        }
    }
    return dest;
}

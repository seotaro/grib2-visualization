//! GRIB2 decode utility

use std::mem;

use super::type_utils_impl::u32_be;

// デコードして 16bpp データ を返す。
pub(crate) fn unpack_simple_packing(buf: &[u8], bits: usize) -> Vec<u16> {
    assert!(bits < mem::size_of::<u16>() * 8);

    const TEMP_BITS: usize = mem::size_of::<u32>() * 8; // 一時変数のビット数
    const TEMP_FULL_BIT: u32 = !0; // 一時変数で全ビットを立てた状態

    let totol_bits = buf.len() * 8;

    let mut values: Vec<u16> = Vec::new();
    let mut bit_pos = 0; // バッファでのビット位置
    while (bit_pos + bits) <= totol_bits {
        let byte_pos = bit_pos / 8; // バッファでのバイト位置
                                    // println!("{}: {}", bit_pos, byte_pos);
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

    values
}

pub(crate) fn decode_packed_scale_value(src: &Vec<usize>, r: f32, e: i32, d: i32) -> Vec<f32> {
    let mut dest: Vec<f32> = vec![0.0; src.len()];
    for (i, value) in src.into_iter().enumerate() {
        dest[i] = (r + (*value as f32) * 2.0f32.powi(e)) / 10.0f32.powi(d);
    }
    return dest;
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct RunLength {
    value: usize,
    length: usize,
}

// デコードしてレベル値（8bpp）を返す。
pub(crate) fn unpack_run_length_packing(buf: &[u8], bits: usize, max_level: usize) -> Vec<u8> {
    let values = unpack_simple_packing(buf, bits);

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

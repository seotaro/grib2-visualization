//! GRIB2 bit-map utility

use num_traits::{Bounded, Num, NumCast};

use super::section::IsSection7DataType;

// ビットマップを適用する
pub(crate) fn apply_bit_map<T>(src: &[T], bit_map: &[u8], width: usize, height: usize) -> Vec<T>
where
    T: Num + NumCast + Bounded + Copy + IsSection7DataType,
{
    let mut dest: Vec<T> = Vec::new();

    let mut src_pos = 0;
    for j in 0..height {
        for i in 0..width {
            let index = width * j + i;

            let bit_pos = index % 8;
            let mask = match bit_pos {
                0 => 0x80,
                1 => 0x40,
                2 => 0x20,
                3 => 0x10,
                4 => 0x08,
                5 => 0x04,
                6 => 0x02,
                7 => 0x01,
                _ => 0x00,
            };

            let byte = bit_map[index / 8];
            if 0 < (byte & mask) {
                // presence
                dest.push(src[src_pos]);
                src_pos += 1;
            } else {
                // absence
                dest.push(T::max_value());
            }
        }
    }

    return dest;
}

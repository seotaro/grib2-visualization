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
            let mask = 0x80 >> bit_pos;

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

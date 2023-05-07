//! GRIB2 utility

use chrono::{DateTime, TimeZone, Utc};
use std::{convert::TryInto, mem};

pub(crate) fn u8_be(src: &[u8]) -> u8 {
    assert!(src.len() == 1);
    u8::from_be_bytes(src.try_into().unwrap())
}

pub(crate) fn i8_be(src: &[u8]) -> i8 {
    const FULL_BIT: u8 = !0; // 全ビットを立てた状態
    const SIGN_BIT: u8 = 0x80;

    let u = u8_be(src);

    // 全て1ならmissingのため変換しない。
    if u == FULL_BIT {
        return u as i8;
    }
    // MSBが1なら負数表現
    if 0 < (u & SIGN_BIT) {
        return (u & !SIGN_BIT) as i8 * -1; // MSBを除いた数値が負の数。
    }

    return u as i8; // 正数なので変換しない。
}

pub(crate) fn u16_be(src: &[u8]) -> u16 {
    assert!(src.len() == 2);
    u16::from_be_bytes(src.try_into().unwrap())
}

pub(crate) fn i16_be(src: &[u8]) -> i16 {
    const FULL_BIT: u16 = !0; // 全ビットを立てた状態
    const SIGN_BIT: u16 = 0x8000;

    let u = u16_be(src);

    // 全て1ならmissingのため変換しない。
    if u == FULL_BIT {
        return u as i16;
    }

    // MSBが1なら負数表現
    if 0 < (u & SIGN_BIT) {
        return (u & !SIGN_BIT) as i16 * -1; // MSBを除いた数値が負の数。
    }
    return u as i16; // 正数なので変換しない。
}

pub(crate) fn usize_be(src: &[u8]) -> usize {
    let type_len = mem::size_of::<usize>();
    let mut value: usize = 0;
    for i in 0..type_len {
        value |= (src[i] as usize) << ((type_len - i - 1) * 8);
    }

    return value;
}

pub(crate) fn u32_be(src: &[u8]) -> u32 {
    assert!(src.len() == 4);
    u32::from_be_bytes(src.try_into().unwrap())
}

pub(crate) fn i32_be(src: &[u8]) -> i32 {
    const FULL_BIT: u32 = !0; // 全ビットを立てた状態
    const SIGN_BIT: u32 = 0x8000_0000;

    let u = u32_be(src);

    // 全て1ならmissingのため変換しない。
    if u == FULL_BIT {
        return u as i32;
    }

    // MSBが1なら負数表現
    if 0 < (u & SIGN_BIT) {
        return (u & !SIGN_BIT) as i32 * -1; // MSBを除いた数値が負の数。
    }
    return u as i32; // 正数なので変換しない。
}

pub(crate) fn u64_be(src: &[u8]) -> u64 {
    assert!(src.len() == 8);
    u64::from_be_bytes(src.try_into().unwrap())
}

pub(crate) fn i64_be(src: &[u8]) -> i64 {
    const FULL_BIT: u64 = !0; // 全ビットを立てた状態
    const SIGN_BIT: u64 = 0x8000_0000_0000_0000;

    let u = u64_be(src);

    // 全て1ならmissingのため変換しない。
    if u == FULL_BIT {
        return u as i64;
    }

    // MSBが1なら負数表現
    if 0 < (u & SIGN_BIT) {
        return (u & !SIGN_BIT) as i64 * -1; // MSBを除いた数値が負の数。
    }
    return u as i64; // 正数なので変換しない。
}

pub(crate) fn float_be(src: &[u8]) -> f32 {
    assert!(src.len() == 4);
    f32::from_be_bytes(src.try_into().unwrap())
}

pub(crate) fn datetime_be(src: &[u8]) -> DateTime<Utc> {
    assert!(src.len() == 7);
    let year = u16_be(&src[0..2]) as i32;
    let month = src[2] as u32;
    let day = src[3] as u32;
    let hour = src[4] as u32;
    let minute = src[5] as u32;
    let second = src[6] as u32;
    Utc.ymd(year, month, day).and_hms(hour, minute, second)
}

// [sec]
pub(crate) fn time_span_be(src: &[u8]) -> usize {
    assert!(src.len() == 5);
    let unit = u8_be(&src[0..1]);
    let value = u32_be(&src[1..5]) as usize;

    let span = match unit {
        0 => Some(value * 60),           // Minute
        1 => Some(value * 60 * 60),      // Hour
        2 => Some(value * 24 * 60 * 60), // Day
        // 3=> // Month
        // 4=> // Year
        // 5=> // Decade (10 years)
        // 6=> // Normal (30 years)
        // 7=> // Century (100 years)
        // 8..=9=>   // Reserved
        10 => Some(value * 3 * 60 * 60),  // 3 hours
        11 => Some(value * 6 * 60 * 60),  // 6 hours
        12 => Some(value * 12 * 60 * 60), // 12 hours
        13 => Some(value),                // Second
        // 14..=191=>    // Reserved
        // 192..=254=>   // Reserved for local use
        // 255=>   // Missing
        _ => None,
    };

    assert!(span != None);
    match span {
        Some(x) => x,
        None => 0,
    }
}

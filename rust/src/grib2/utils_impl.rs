//! GRIB2 utility

use super::section::Section1;
use super::section::Section2;
use super::section::Section3;
use super::section::Section4;
use super::section::Section5;
use super::section::Section6;
use super::section::Section7;
use super::section::SectionSet;
use super::section::SectionSets;
use super::type_utils_impl::u32_be;
use super::type_utils_impl::u64_be;

// バイト列が "GRIB" なら true を返す。
pub(crate) fn is_start_indicator(buf: &[u8]) -> bool {
    assert_eq!(buf.len(), 4);

    if (buf[0] == b'G') && (buf[1] == b'R') && (buf[2] == b'I') && (buf[3] == b'B') {
        return true;
    }
    return false;
}

// バイト列が "7777" なら true を返す。
pub(crate) fn is_end_indicator(buf: &[u8]) -> bool {
    assert_eq!(buf.len(), 4);

    if (buf[0] == b'7') && (buf[1] == b'7') && (buf[2] == b'7') && (buf[3] == b'7') {
        return true;
    }
    return false;
}

pub(crate) fn parse(buf: &[u8]) -> SectionSets {
    // セクション1～セクション7までの入れ物を作っておく。
    let mut sectionset = SectionSet {
        section1: None,
        section2: None,
        section3: None,
        section4: None,
        section5: None,
        section6: None,
        section7: None,
    };
    let mut sectionsets = SectionSets::new();

    // 先頭から順番にセクションを切り分けていく。
    let mut grib2_length = 0;
    let mut pos = 0;
    while pos < buf.len() {
        let number_of_section = if is_start_indicator(&buf[pos + 0..pos + 4]) {
            0
        } else if is_end_indicator(&buf[pos + 0..pos + 4]) {
            8
        } else {
            buf[pos + 4]
        };

        let length_of_section = match number_of_section {
            0 => 16,
            8 => 4,
            _ => u32_be(&buf[pos + 0..pos + 4]) as usize,
        };

        let section_buf = &buf[pos..pos + length_of_section];
        match number_of_section {
            0 => grib2_length = u64_be(&buf[8..16]) as usize,
            1 => sectionset.section1 = Some(Section1::create(section_buf)),
            2 => sectionset.section2 = Some(Section2::create(section_buf)),
            3 => sectionset.section3 = Some(Section3::create(section_buf)),
            4 => sectionset.section4 = Some(Section4::create(section_buf)),
            5 => sectionset.section5 = Some(Section5::create(section_buf)),
            6 => {
                let section6 = Section6::create(section_buf);
                match section6.bit_map_indicator() {
                    0 | 255 => sectionset.section6 = Some(section6),
                    254 => (), // 直前のビットマップを使う
                    _ => (),
                }
            }
            7 => {
                sectionset.section7 = (|| -> Option<Section7> {
                    Some(Section7::create(section_buf, sectionset.section5?))
                })()
            }
            _ => (),
        }

        if number_of_section == 7 {
            sectionsets.push(sectionset.clone());
        }

        pos = pos + length_of_section;
    }

    sectionsets
}

pub(crate) fn parameter_name(category: usize, number: usize) -> Option<String> {
    match category {
        // Temperature
        0 => match number {
            0 => Some(String::from("Temperature [K]")),
            _ => None,
        },

        // Moisture
        1 => match number {
            1 => Some(String::from("Relative Humidity [%]")),
            8 => Some(String::from("Total Precipitation [kg m-2]")),
            52 => Some(String::from("Total precipitation rate [kg m-2 s-1]")),
            57 => Some(String::from("Total snowfall rate [m s-1]")),
            200 => Some(String::from("1時間降水量レベル値")),
            201 => Some(String::from("10分間降水強度（1時間換算値）レベル値")),
            203 => Some(String::from("降水強度レベル値(解析、予報）")),
            206 => Some(String::from("土壌雨量タンクレベル値")),
            214 => Some(String::from("降水強度の誤差の要因")),
            215 => Some(String::from("表面雨量指数値")),
            216 => Some(String::from("浸水危険度判定値")),
            217 => Some(String::from("洪水危険度判定値")),
            218 => Some(String::from("浸水・洪水危険度判定値")),
            232 => Some(String::from("積雪の深さのレベル値")),
            233 => Some(String::from("降雪の深さの合計のレベル値")),
            _ => None,
        },

        // Momentum
        2 => match number {
            2 => Some(String::from("U-Component of Wind [m s-1]")),
            3 => Some(String::from("V-Component of Wind [m s-1]")),
            8 => Some(String::from("Vertical Velocity (Pressure) [Pa s-1]")),
            _ => None,
        },

        // Mass
        3 => match number {
            0 => Some(String::from("Pressure [Pa]")),
            1 => Some(String::from("Pressure Reduced to MSL [Pa]")),
            5 => Some(String::from("Geopotential Height [gpm]")),
            _ => None,
        },

        // Short-wave radiation
        4 => match number {
            7 => Some(String::from("Downward short-wave radiation flux [W m-2]")),
            _ => None,
        },

        // Cloud
        6 => match number {
            1 => Some(String::from("Total Cloud Cover [%]")),
            3 => Some(String::from("Low Cloud Cover [%]")),
            4 => Some(String::from("Medium Cloud Cover [%]")),
            5 => Some(String::from("High Cloud Cover [%]")),
            8 => Some(String::from("Cloud type")),
            12 => Some(String::from("Cloud top")),
            200 => Some(String::from("品質情報")),
            201 => Some(String::from("雲・ダストの有無")),
            202 => Some(String::from("雪氷の有無")),
            _ => None,
        },

        // Physical atmospheric properties
        19 => match number {
            0 => Some(String::from("Visibility [m]")),
            2 => Some(String::from("Thunderstorm probability [%]")),
            _ => None,
        },

        // Miscellaneous
        191 => match number {
            192 => Some(String::from("天気")),
            _ => None,
        },

        // ナウキャスト
        193 => match number {
            0 => Some(String::from("竜巻発生確度")),
            1 => Some(String::from("雷活動度")),
            _ => None,
        },

        _ => None,
    }
}

pub(crate) fn first_plane_name(
    plane_type: usize,
    plane_factor: isize,
    plane_value: isize,
) -> Option<String> {
    let v = (plane_value as f32) / 10.0f32.powi(plane_factor as i32);
    match plane_type {
        0 => Some(String::from("Reserved")),
        1 => Some(String::from("Ground or Water Surface")),
        2 => Some(String::from("Cloud Base Level")),
        3 => Some(String::from("Level of Cloud Tops")),
        4 => Some(String::from("Level of 0o C Isotherm")),
        5 => Some(String::from(
            "Level of Adiabatic Condensation Lifted from the Surface",
        )),
        6 => Some(String::from("Maximum Wind Level")),
        7 => Some(String::from("Tropopause")),
        8 => Some(String::from("Nominal Top of the Atmosphere")),
        9 => Some(String::from("Sea Bottom")),
        10 => Some(String::from("Entire Atmosphere")),
        11 => Some(format!("Cumulonimbus Base (CB): {}[m]", v)),
        12 => Some(format!("Cumulonimbus Top (CT): {}[m]", v)),
        13 => Some(format!("Lowest level where vertically integrated cloud cover exceeds the specified percentage (cloud base for a given percentage cloud cover): {}[%]", v)),
        14 => Some(String::from("Level of free convection (LFC)")),
        15 => Some(String::from("Convection condensation level (CCL)")),
        16 => Some(String::from("Level of neutral buoyancy or equilibrium (LNB)")),
        17..=19 => Some(String::from("Reserved")),
        20 => Some(format!("Isothermal Level: {}[K]", v)),
        21 => Some(format!("Lowest level where mass density exceeds the specified value(base for a given threshold of mass density): {}[kg m-3]", v)),
        22 => Some(format!("Highest level where mass density exceeds the specified value (top for a given threshold of mass density): {}[kg m-3]", v)),
        23 => Some(format!("Lowest level where air concentration exceeds the specified value (base for a given threshold of air concentration: {}[Bq m-3]", v)),
        24 => Some(format!("Highest level where air concentration exceeds the specified value (top for a given threshold of air concentration): {}[Bq m-3]", v)),
        25 => Some(format!("Highest level where radar reflectivity exceeds the specified value (echo top for a given threshold of reflectivity): {}[dBZ]", v)),
        26 => Some(format!("Convective cloud layer base: {}[m]", v)),
        27 => Some(format!("Convective cloud layer top: {}[m]", v)),
        28..=29 => Some(String::from("Reserved")),
        30 => Some(format!("Specified radius from the centre of the Sun: {}[m]", v)),
        31 => Some(String::from("Ionospheric D-region level")),
        32 => Some(String::from("Ionospheric E-region level")),
        33 => Some(String::from("Ionospheric F1-region level")),
        34 => Some(String::from("Ionospheric F1-region level")),
        35 => Some(String::from("Ionospheric F2-region level")),	
        36..=99 => Some(String::from("Reserved")),
        100 => Some(format!("Isobaric Surface: {}[Pa]", v)),
        101 => Some(format!("Mean Sea Level: {}[m]", v)),
        103 => Some(format!("Specified Height Level Above Ground: {}[m]", v)),
        _ => None,
    }
}

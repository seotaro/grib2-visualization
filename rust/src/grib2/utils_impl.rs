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
use super::type_utils_impl::u8_be;

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
        genre: None,
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
            0 => {
                grib2_length = u64_be(&buf[8..16]) as usize;
                sectionset.genre = Some(u8_be(&buf[6..7]) as usize);
            }
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

pub(crate) fn parameter_name(genre: usize, category: usize, number: usize) -> Option<String> {
    match genre {
        // 0 Meteorological products
        0 => match category {
            // Temperature
            0 => match number {
                0 => Some(String::from("Temperature [K]")),
                1 => Some(String::from("Virtual temperature [K]")),
                2 => Some(String::from("Potential temperature [K]")),
                3 => Some(String::from("Pseudo-adiabatic potential temperature or equivalent potential temperature [K]")),
                4 => Some(String::from("Maximum temperature [K]")),
                5 => Some(String::from("Minimum temperature [K]")),
                6 => Some(String::from("Dewpoint temperature [K]")),
                7 => Some(String::from("Dewpoint depression (or deficit) [K]")),
                8 => Some(String::from("Lapse rate [K m-1]")),
                9 => Some(String::from("Temperature anomaly [K]")),
                10 => Some(String::from("Latent heat net flux [W m-2]")),
                11 => Some(String::from("Sensible heat net flux [W m-2]")),
                12 => Some(String::from("Heat index [K]")),
                13 => Some(String::from("Wind chill factor [K]")),
                14 => Some(String::from("Minimum dewpoint depression [K]")),
                15 => Some(String::from("Virtual potential temperature [K]")),
                16 => Some(String::from("Snow phase change heat flux [W m-2]")),
                17 => Some(String::from("Skin temperature [K]")),
                18 => Some(String::from("Snow temperature (top of snow) [K]")),
                19 => Some(String::from("Turbulent transfer coefficient for heat")),
                20 => Some(String::from("Turbulent diffusion coefficient for heat [m2 s-1]")),
                21 => Some(String::from("Apparent temperature [K]")),
                22 => Some(String::from("Temperature tendency due to short-wave radiation [K s-1]")),
                23 => Some(String::from("Temperature tendency due to long-wave radiation [K s-1]")),
                24 => Some(String::from("Temperature tendency due to short-wave radiation, clear sky [K s-1]")),
                25 => Some(String::from("Temperature tendency due to long-wave radiation, clear sky [K s-1]")),
                26 => Some(String::from("Temperature tendency due to parameterization [K s-1]")),
                27 => Some(String::from("Wet-bulb temperature [K]")),
                28 => Some(String::from("Unbalanced component of temperature [K]")),
                29 => Some(String::from("Temperature advection [K s-1]")),
                30 => Some(String::from("Latent heat net flux due to evaporation [W m-2]")),
                31 => Some(String::from("Latent heat net flux due to sublimation [W m-2]")),
                32 => Some(String::from("Wet-bulb potential temperature [K]")),
                33..=191 => Some(String::from("Reserved")),
                192..=254 => Some(String::from("Reserved for local use")),
                _ => None,
            },

            // Moisture
            1 => match number {
                0 => Some(String::from("Specific humidity [kg kg-1]")),
                1 => Some(String::from("Relative Humidity [%]")),
                2 => Some(String::from("Humidity mixing ratio [kg kg-1]")),
                3 => Some(String::from("Precipitable water [kg m-2]")),
                4 => Some(String::from("Vapour pressure [Pa]")),
                5 => Some(String::from("Saturation deficit [Pa]")),
                6 => Some(String::from("Evaporation [kg m-2]")),
                7 => Some(String::from("Precipitation rate [kg m-2 s-1]")),
                8 => Some(String::from("Total Precipitation [kg m-2]")),
                9 => Some(String::from("Large-scale precipitation (non-convective) [kg m-2]")),
                10 => Some(String::from("Convective precipitation [kg m-2]")),
                11 => Some(String::from("Snow depth [m]")),
                12 => Some(String::from("Snowfall rate water equivalent [kg m-2 s-1]")),
                13 => Some(String::from("Water equivalent of accumulated snow depth [kg m-2]")),
                14 => Some(String::from("Convective snow [kg m-2]")),
                15 => Some(String::from("Large-scale snow [kg m-2]")),
                16 => Some(String::from("Snow melt [kg my]")),
                17 => Some(String::from("Snow age [d]")),
                18 => Some(String::from("Absolute humidity [kg m-3]")),
                19 => Some(String::from("Precipitation type")),
                20 => Some(String::from("Integrated liquid water [kg m-2]")),
                21 => Some(String::from("Condensate [kg kg-1]")),
                22 => Some(String::from("Cloud mixing ratio [kg kg-1]")),
                23 => Some(String::from("Ice water mixing ratio [kg kg-1]")),
                24 => Some(String::from("Rain mixing ratio [kg kg-1]")),
                25 => Some(String::from("Snow mixing ratio [kg kg-1]")),
                26 => Some(String::from("Horizontal moisture convergence [kg kg-1 s-1]")),
                27 => Some(String::from("Maximum relative humidity [%]")),
                28 => Some(String::from("Maximum absolute humidity [kg m-3]")),
                29 => Some(String::from("Total snowfall [m]")),
                30 => Some(String::from("Precipitable water category")),
                31 => Some(String::from("Hail [m]")),
                32 => Some(String::from("Graupel (snow pellets) [kg kg-1]")),
                33 => Some(String::from("Categorical rain")),
                34 => Some(String::from("Categorical freezing rain")),
                35 => Some(String::from("Categorical ice pellets")),
                36 => Some(String::from("Categorical snow")),
                37 => Some(String::from("Convective precipitation rate [kg m-2 s-1]")),
                38 => Some(String::from("Horizontal moisture divergence [kg kg-1 s-1]")),
                39 => Some(String::from("Per cent frozen precipitation [%]")),
                40 => Some(String::from("Potential evaporation [kg m-2]")),
                41 => Some(String::from("Potential evaporation rate [W m-2]")),
                42 => Some(String::from("Snow cover [%]")),
                43 => Some(String::from("Rain fraction of total cloud water Proportion")),
                44 => Some(String::from("Rime factor")),
                45 => Some(String::from("Total column integrated rain [kg m-2]")),
                46 => Some(String::from("Total column integrated snow [kg m-2]")),
                47 => Some(String::from("Large scale water precipitation (non-convective) [kg m-2]")),
                48 => Some(String::from("Convective water precipitation [kg m-2]")),
                49 => Some(String::from("Total water precipitation [kg m-2]")),
                50 => Some(String::from("Total snow precipitation [kg my]")),
                51 => Some(String::from("Total column water (Vertically integrated total water (vapour + cloud water/ice)) [kg my]")),
                52 => Some(String::from("Total precipitation rate [kg m-2 s-1]")),
                53 => Some(String::from("Total snowfall rate water equivalent [kg m-2 s-1]")),
                54 => Some(String::from("Large scale precipitation rate [kg m-2 s-1]")),
                55 => Some(String::from("Convective snowfall rate water equivalent [kg m-2 s-1]")),
                56 => Some(String::from("Large scale snowfall rate water equivalent [kg m-2 s-1]")),
                57 => Some(String::from("Total snowfall rate [m s-1]")),
                58 => Some(String::from("Convective snowfall rate [m s-1]")),
                59 => Some(String::from("Large scale snowfall rate [m s-1]")),
                60 => Some(String::from("Snow depth water equivalent [kg m-2]")),
                61 => Some(String::from("Snow density [kg m-3]")),
                62 => Some(String::from("Snow evaporation [kg m-2]")),
                63 => Some(String::from("Reserved")),
                64 => Some(String::from("Total column integrated water vapour [kg m-2]")),
                65 => Some(String::from("Rain precipitation rate [kg m-2 s-1]")),
                66 => Some(String::from("Snow precipitation rate [kg m-2 s-1]")),
                67 => Some(String::from("Freezing rain precipitation rate [kg m-2 s-1]")),
                68 => Some(String::from("Ice pellets precipitation rate [kg m-2 s-1]")),
                69 => Some(String::from("Total column integrated cloud water [kg m-2]")),
                70 => Some(String::from("Total column integrated cloud ice [kg m-2]")),
                71 => Some(String::from("Hail mixing ratio [kg kg-1]")),
                72 => Some(String::from("Total column integrated hail [kg m-2]")),
                73 => Some(String::from("Hail precipitation rate [kg m-2 s-1]")),
                74 => Some(String::from("Total column integrated graupel [kg m-2]")),
                75 => Some(String::from("Graupel (snow pellets) precipitation rate [kg m-2 s-1]")),
                76 => Some(String::from("Convective rain rate [kg m-2 s-1]")),
                77 => Some(String::from("Large scale rain rate [kg m-2 s-1]")),
                78 => Some(String::from("Total column integrated water (all components including precipitation) [kg m-2]")),
                79 => Some(String::from("Evaporation rate [kg m-2 s-1]")),
                80 => Some(String::from("Total condensate [kg kg-1]")),
                81 => Some(String::from("Total column-integrated condensate [kg m-2]")),
                82 => Some(String::from("Cloud ice mixing-ratio [kg kg-1]")),
                83 => Some(String::from("Specific cloud liquid water content [kg kg-1]")),
                84 => Some(String::from("Specific cloud ice water content [kg kg-1]")),
                85 => Some(String::from("Specific rainwater content [kg kg-1]")),
                86 => Some(String::from("Specific snow water content [kg kg-1]")),
                87 => Some(String::from("Stratiform precipitation rate [kg m-2 s-1]")),
                88 => Some(String::from("Categorical convective precipitation")),
                89 => Some(String::from("Reserved")),
                90 => Some(String::from("Total kinematic moisture flux [kg kg-1 m s-1]")),
                91 => Some(String::from("u-component (zonal) kinematic moisture flux [kg kg-1 m s-1]")),
                92 => Some(String::from("v-component (meridional) kinematic moisture flux [kg kg-1 m s-1]")),
                93 => Some(String::from("Relative humidity with respect to water [%]")),
                94 => Some(String::from("Relative humidity with respect to ice [%]")),
                95 => Some(String::from("Freezing or frozen precipitation rate [kg m-2 s-1]")),
                96 => Some(String::from("Mass density of rain [kg m-3]")),
                97 => Some(String::from("Mass density of snow [kg m-3]")),
                98 => Some(String::from("Mass density of graupel [kg m-3]")),
                99 => Some(String::from("Mass density of hail [kg m-3]")),
                100 => Some(String::from("Specific number concentration of rain [kg-1]")),
                101 => Some(String::from("Specific number concentration of snow [kg-1]")),
                102 => Some(String::from("Specific number concentration of graupel [kg-1]")),
                103 => Some(String::from("Specific number concentration of hail [kg-1]")),
                104 => Some(String::from("Number density of rain [m-3]")),
                105 => Some(String::from("Number density of snow [m-3]")),
                106 => Some(String::from("Number density of graupel [m-3]")),
                107 => Some(String::from("Number density of hail [m-3]")),
                108 => Some(String::from("Specific humidity tendency due to parameterization [kg kg-1 s-1]")),
                109 => Some(String::from("Mass density of liquid water coating on hail expressed as mass of liquid water per unit volume of air [kg m-3]")),
                110 => Some(String::from("Specific mass of liquid water coating on hail expressed as mass of liquid water per unit mass of moist air [kg kg-1]")),
                111 => Some(String::from("Mass mixing ratio of liquid water coating on hail expressed as mass of liquid water per unit mass of dry air [kg kg-1]")),
                112 => Some(String::from("Mass density of liquid water coating on graupel expressed as mass of liquid water per unit volume of air [kg m-3]")),
                113 => Some(String::from("Specific mass of liquid water coating on graupel expressed as mass of liquid water per unit mass of moist air [kg kg-1]")),
                114 => Some(String::from("Mass mixing ratio of liquid water coating on graupel expressed as mass of liquid water per unit mass of dry air [kg kg-1]")),
                115 => Some(String::from("Mass density of liquid water coating on snow expressed as mass of liquid water per unit volume of air [kg m-3]")),
                116 => Some(String::from("Specific mass of liquid water coating on snow expressed as mass of liquid water per unit mass of moist air [kg kg-1]")),
                117 => Some(String::from("Mass mixing ratio of liquid water coating on snow expressed as mass of liquid water per unit mass of dry air [kg kg-1]")),
                118 => Some(String::from("Unbalanced component of specific humidity [kg kg-1]")),
                119 => Some(String::from("Unbalanced component of specific cloud liquid water content [kg kg-1]")),
                120 => Some(String::from("Unbalanced component of specific cloud ice water content [kg kg-1]")),
                121 => Some(String::from("Fraction of snow cover Proportion")),
                122 => Some(String::from("Precipitation intensity index ")),
                123 => Some(String::from("Dominant precipitation type")),
                124 => Some(String::from("Presence of showers")),
                125 => Some(String::from("Presence of blowing snow")),
                126 => Some(String::from("Presence of blizzard")),
                127 => Some(String::from("Ice pellets (non-water equivalent) precipitation rate [m/s]")),
                128 => Some(String::from("Total solid precipitation rate [kg m-2 s-1]")),
                129 => Some(String::from("Effective radius of cloud water [m]")),
                130 => Some(String::from("Effective radius of rain [m]")),
                131 => Some(String::from("Effective radius of cloud ice [m]")),
                132 => Some(String::from("Effective radius of snow [m]")),
                133 => Some(String::from("Effective radius of graupel [m]")),
                134 => Some(String::from("Effective radius of hail [m]")),
                135 => Some(String::from("Effective radius of subgrid liquid clouds [m]")),
                136 => Some(String::from("Effective radius of subgrid ice clouds [m]")),
                137 => Some(String::from("Effective aspect ratio of rain")),
                138 => Some(String::from("Effective aspect ratio of cloud ice")),
                139 => Some(String::from("Effective aspect ratio of snow")),
                140 => Some(String::from("Effective aspect ratio of graupel")),
                141 => Some(String::from("Effective aspect ratio of hail")),
                142 => Some(String::from("Effective aspect ratio of subgrid ice clouds")),
                143 => Some(String::from("Potential evaporation rate [kg m-2 s-1]")),
                144 => Some(String::from("Specific rain water content (convective) [kg kg-1]")),
                145 => Some(String::from("Specific snow water content (convective) [kg kg-1]")),
                146 => Some(String::from("Cloud ice precipitation rate [kg m-2 s-1]")),
                147 => Some(String::from("Character of precipitation")),
                148 => Some(String::from("Snow evaporation rate [kg m-2 s-1]")),
                149 => Some(String::from("Cloud water mixing ratio [kg kg-1]")),
                150..=191 => Some(String::from("Reserved")),
                192..=199 => Some(String::from("Reserved for local use")),
                200 => Some(String::from("1時間降水量レベル値")),
                201 => Some(String::from("10分間降水強度（1時間換算値）レベル値")),
                202 => Some(String::from("Reserved for local use")),
                203 => Some(String::from("降水強度レベル値(解析、予報）")),
                204..=205 => Some(String::from("Reserved for local use")),
                206 => Some(String::from("土壌雨量タンクレベル値")),
                207 => Some(String::from("Reserved for local use")),
                208 => Some(String::from("土砂災害警戒判定値")),
                209..=213 => Some(String::from("Reserved for local use")),
                214 => Some(String::from("降水強度の誤差の要因")),
                215 => Some(String::from("表面雨量指数値")),
                216 => Some(String::from("浸水危険度判定値")),
                217 => Some(String::from("洪水危険度判定値")),
                218 => Some(String::from("浸水・洪水危険度判定値")),
                219..=231 => Some(String::from("Reserved for local use")),
                232 => Some(String::from("積雪の深さのレベル値")),
                233 => Some(String::from("降雪の深さの合計のレベル値")),
                234..=254 => Some(String::from("Reserved for local use")),
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
                0 => Some(String::from(
                    "Net short-wave radiation flux (surface) [W m-2]",
                )),
                1 => Some(String::from(
                    "Net short-wave radiation flux (top of atmosphere) [W m-2]",
                )),
                2 => Some(String::from("Short-wave radiation flux [W m-2]")),
                3 => Some(String::from("Global radiation flux [W m-2]")),
                4 => Some(String::from("Brightness temperature [K]")),
                5 => Some(String::from(
                    "Radiance (with respect to wave number) [W m-1 sr-1]",
                )),
                6 => Some(String::from(
                    "Radiance (with respect to wavelength) [W m-3 sr-1]",
                )),
                7 => Some(String::from("Downward short-wave radiation flux [W m-2]")),
                8 => Some(String::from("Upward short-wave radiation flux [W m-2]")),
                9 => Some(String::from("Net short wave radiation flux [W m-2]")),
                10 => Some(String::from("Photosynthetically active radiation [W m-2]")),
                11 => Some(String::from(
                    "Net short-wave radiation flux, clear sky [W m-2]",
                )),
                12 => Some(String::from("Downward UV radiation [W m-2]")),
                13 => Some(String::from("Direct short-wave radiation flux [W m-2]")),
                14 => Some(String::from("Diffuse short-wave radiation flux [W m-2]")),
                15 => Some(String::from(
                    "Upward UV radiation emitted/reflected from the Earth's surface [W m-2]",
                )),
                16..=49 => Some(String::from("Reserved")),
                50 => Some(String::from(
                    "UV index (under clear sky)",
                )),
                51 => Some(String::from("UV index")),
                52 => Some(String::from(
                    "Downward short-wave radiation flux, clear sky [W m-2]",
                )),
                53 => Some(String::from(
                    "Upward short-wave radiation flux, clear sky [W m-2]",
                )),
                54 => Some(String::from(
                    "Direct normal short-wave radiation flux (see Note 3) [W m-2]",
                )),
                55..=191 => Some(String::from("Reserved")),
                192..=254 => Some(String::from("Reserved for local use")),
                _ => None,
            },

            // Cloud
            6 => match number {
                0 => Some(String::from("Cloud ice [kg m-2]")),
                1 => Some(String::from("Total Cloud Cover [%]")),
                3 => Some(String::from("Low Cloud Cover [%]")),
                4 => Some(String::from("Medium Cloud Cover [%]")),
                5 => Some(String::from("High Cloud Cover [%]")),
                8 => Some(String::from("Cloud type")),
                12 => Some(String::from("Cloud top")),
                33 => Some(String::from("Sunshine duration [s]")),
                194 => Some(String::from("品質情報")),
                200 => Some(String::from("品質情報")),
                201 => Some(String::from("雲・ダストの有無")),
                202 => Some(String::from("雪氷の有無")),
                _ => None,
            },

            // aerosols
            13 => match number {
                0 => Some(String::from("Aerosol type")),
                1..=191 => Some(String::from("Reserved")),
                192 => Some(String::from("ダスト下層濃度 [kgm-3]")),
                193 => Some(String::from("ダスト気柱積算量 [kgm-2]")),
                194..=254 => Some(String::from("Reserved for local use")),
                _ => None,
            },

            // trace gases
            14 => match number {
                0 => Some(String::from("Total ozone [DU]")),
                1 => Some(String::from("Ozone mixing ratio [kg kg-1]")),
                2 => Some(String::from("Total column integrated ozone [DU]")),
                3..=191 => Some(String::from("Reserved")),
                192..=254 => Some(String::from("Reserved for local use")),
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
        },

        // Hydrological products
        1 => None,

        // Land surface products
        2 => None,

        // Satellite remote sensing products (formerly "Space products")
        3 => None,

        // Space weather products
        4 => None,

        // Reserved
        5..=9 => None,

        // Oceanographic products
        10 => match category {
            // waves
            0 => match number {
                0 => Some(String::from("Temperature [K]")),
                3 => Some(String::from(
                    "Significant height of combined wind waves and swell [m]",
                )),
                4 => Some(String::from("Direction of wind waves [degree true]")),
                5 => Some(String::from("Significant height of wind waves [m]")),
                6 => Some(String::from("Mean period of wind waves [s]")),
                47 => Some(String::from(
                    "Significant wave height of first swell partition [m]",
                )),
                48 => Some(String::from(
                    "Significant wave height of second swell partition [m]",
                )),
                49 => Some(String::from(
                    "Significant wave height of third swell partition [m]",
                )),
                50 => Some(String::from(
                    "Mean wave period of first swell partition [s]",
                )),
                51 => Some(String::from(
                    "Mean wave period of second swell partition [s]",
                )),
                52 => Some(String::from(
                    "Mean wave period of third swell partition [s]",
                )),
                53 => Some(String::from(
                    "Mean wave direction of first swell partition [°]",
                )),
                54 => Some(String::from(
                    "Mean wave direction of second swell partition [°]",
                )),
                55 => Some(String::from(
                    "Mean wave direction of third swell partition [°]",
                )),
                10 => Some(String::from("Primary wave direction [degree true]")),
                11 => Some(String::from("Primary wave mean period [s]")),
                _ => None,
            },
            _ => None,
        },

        // Reserved
        11..=191 => None,

        // Reserved for local use
        192..=254 => None,

        // Missing
        255 => None,

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

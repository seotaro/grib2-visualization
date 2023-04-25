//! GRIB2 SectionSet implementation

use chrono::{DateTime, Utc};
use std::fmt;

use super::super::bit_map_utils_impl::apply_bit_map;
use super::super::decode_utils_impl::unpack_run_length_packing;
use super::super::decode_utils_impl::unpack_simple_packing;
use super::section5_template::TemplateNumber as Section5TemplateNumber;
use super::Bounds;
use super::PackingType;
use super::RawRunLengthPackingImage;
use super::RawSimplePackingImage;
use super::SectionSet;
use super::SectionSets;
use super::SectionSetsIter;

impl<'a> SectionSet<'a> {
    // 有効な1ピクセルあたりのビット数
    pub fn bpp(&self) -> Option<usize> {
        self.section5?.bpp()
    }

    pub fn packing_type(&self) -> Option<PackingType> {
        self.section5?.packing_type()
    }

    pub fn point_count(&self) -> Option<usize> {
        Some(self.section3?.point_count())
    }

    // データセットの参照時刻を返す。
    pub fn reference_datetime(&self) -> Option<DateTime<Utc>> {
        Some(self.section1?.reference_time())
    }

    // データセットの時刻を返す。
    pub fn datetime(&self) -> Option<DateTime<Utc>> {
        let reference_time = self.section1?.reference_time();
        Some(self.section4?.template()?.datetime(reference_time))
    }

    // パラメーターカテゴリー
    pub fn parameter_category(&self) -> Option<usize> {
        Some(self.section4?.template()?.parameter_category())
    }

    // パラメーター番号
    pub fn parameter_number(&self) -> Option<usize> {
        Some(self.section4?.template()?.parameter_number())
    }

    // パラメーター番号
    pub fn first_plane_type(&self) -> Option<usize> {
        self.section4?.template()?.first_plane_type()
    }

    // パラメーター番号
    pub fn first_plane_factor(&self) -> Option<isize> {
        self.section4?.template()?.first_plane_factor()
    }

    // パラメーター番号
    pub fn first_plane_value(&self) -> Option<isize> {
        self.section4?.template()?.first_plane_value()
    }

    pub fn bounds(&self) -> Option<Bounds> {
        Some(Bounds {
            left: self.section3?.lo1(),
            bottom: self.section3?.la2(),
            right: self.section3?.lo2(),
            top: self.section3?.la1(),
        })
    }

    pub fn unpack(&self) -> Result<RawSimplePackingImage, String> {
        let mut width = 0;
        let mut height = 0;
        match self.section3 {
            Some(sec3) => {
                width = sec3.ni();
                height = sec3.nj();
            }
            _ => {}
        };

        return match self.section5 {
            Some(sec5) => match self.section6 {
                Some(sec6) => match self.section7 {
                    Some(sec7) => match sec5.template() {
                        Some(Section5TemplateNumber::T0(t)) => {
                            let pixels = unpack_simple_packing(&sec7.buf[5..], t.bits());
                            match sec6.bit_map_indicator() {
                                0 => Ok(RawSimplePackingImage {
                                    width,
                                    height,
                                    r: t.r(),
                                    e: t.e(),
                                    d: t.d(),
                                    bits: t.bits(),
                                    pixels: apply_bit_map(&pixels, sec6.bit_map(), width, height),
                                }),
                                _ => Ok(RawSimplePackingImage {
                                    width,
                                    height,
                                    r: t.r(),
                                    e: t.e(),
                                    d: t.d(),
                                    bits: t.bits(),
                                    pixels,
                                }),
                            }
                        }
                        Some(Section5TemplateNumber::T3(t)) => {
                            Err(format!("Template 5.3 は未実装"))
                        }
                        Some(Section5TemplateNumber::T200(t)) => {
                            Err(format!("Template 5.200 は未実装"))
                        }
                        _ => Err(format!("Invalid template 5")),
                    },
                    _ => Err(format!("Invalid section 7")),
                },
                _ => Err(format!("Invalid section 6")),
            },
            _ => Err(format!("Invalid section 5")),
        };
    }

    pub fn unpack_run_length(&self) -> Result<RawRunLengthPackingImage, String> {
        let mut width = 0;
        let mut height = 0;
        match self.section3 {
            Some(sec3) => {
                width = sec3.ni();
                height = sec3.nj();
            }
            _ => {}
        };

        return match self.section5 {
            Some(sec5) => match self.section6 {
                Some(sec6) => match self.section7 {
                    Some(sec7) => match sec5.template() {
                        Some(Section5TemplateNumber::T0(t)) => {
                            Err(format!("Template 5.0 は未実装"))
                        }
                        Some(Section5TemplateNumber::T3(t)) => {
                            Err(format!("Template 5.3 は未実装"))
                        }
                        Some(Section5TemplateNumber::T200(t)) => {
                            let pixels = unpack_run_length_packing(&sec7.buf[5..], t.bits(), t.v());
                            match sec6.bit_map_indicator() {
                                0 => Ok(RawRunLengthPackingImage {
                                    width,
                                    height,
                                    bits: t.bits(),
                                    factor: t.factor(),
                                    levels: t.scaled_representative_values(),
                                    pixels: apply_bit_map(&pixels, sec6.bit_map(), width, height),
                                }),
                                _ => Ok(RawRunLengthPackingImage {
                                    width,
                                    height,
                                    bits: t.bits(),
                                    factor: t.factor(),
                                    levels: t.scaled_representative_values(),
                                    pixels,
                                }),
                            }
                        }
                        _ => Err(format!("Invalid template 5")),
                    },
                    _ => Err(format!("Invalid section 7")),
                },
                _ => Err(format!("Invalid section 6")),
            },
            _ => Err(format!("Invalid section 5")),
        };
    }

    // pub(crate)  fn decode(&self) -> Result<Vec<f32>, String> {
    //     match self.unpack() {
    //         Ok(values) => match self.sections[5] {
    //             Some(buf) => {
    //                 let sec5 = section5::Section { buf: buf };
    //                 match sec5.template() {
    //                     Ok(template) => match template {
    //                         section5::TemplateNumber::T0(t) => {
    //                             Ok(decode_packed_scale_value(
    //                                 &values,
    //                                 t.r() as f32,
    //                                 t.e() as i32,
    //                                 t.d() as i32,
    //                             ))
    //                         }
    //                     },
    //                     Err(msg) => Err(msg),
    //                 }
    //             }
    //             _ => Err(format!("Invalid section 5")),
    //         },
    //         Err(msg) => Err(msg),
    //     }
    // }
}

impl fmt::Display for SectionSet<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}",
            match self.section1 {
                Some(sec) => format!("{}", sec),
                _ => format!("--Section1\n"),
            },
            match self.section2 {
                Some(sec) => format!("{}", sec),
                _ => format!("--Section2\n"),
            },
            match self.section3 {
                Some(sec) => format!("{}", sec),
                _ => format!("--Section3\n"),
            },
            match self.section4 {
                Some(sec) => format!("{}", sec),
                _ => format!("--Section4\n"),
            },
            match self.section5 {
                Some(sec) => format!("{}", sec),
                _ => format!("--Section5\n"),
            },
            match self.section6 {
                Some(sec) => format!("{}", sec),
                _ => format!("--Section6\n"),
            },
            match self.section7 {
                Some(sec) => format!("{}", sec),
                _ => format!("--Section7\n"),
            },
        )
    }
}

impl<'a> SectionSets<'a> {
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub(crate) fn push(&mut self, sectionset: SectionSet<'a>) {
        self.items.push(sectionset);
    }

    pub(crate) fn extend(&mut self, sectionsets: SectionSets<'a>) {
        self.items.extend(sectionsets.items);
    }

    pub fn get(&self, index: usize) -> SectionSet<'a> {
        return self.items[index];
    }

    pub fn len(&self) -> usize {
        return self.items.len();
    }

    pub fn iter(&self) -> SectionSetsIter {
        SectionSetsIter {
            items: &self.items,
            index: 0,
        }
    }
}

impl<'a> Iterator for SectionSetsIter<'a> {
    type Item = SectionSet<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.items.len() {
            let result = &self.items[self.index];
            self.index += 1;
            Some(*result)
        } else {
            None
        }
    }
}

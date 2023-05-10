//! GRIB2 SectionSet implementation

use chrono::{DateTime, Utc};
use std::fmt;

use super::super::bit_map_utils_impl::apply_bit_map;
use super::super::decode_utils_impl::unpack;
use super::super::decode_utils_impl::unpack_complex_packing_and_spatial_differencing;
use super::super::decode_utils_impl::unpack_run_length_packing;
use super::section5_template::TemplateNumber as Section5TemplateNumber;
use super::Bounds;
use super::PackingType;
use super::RawRunLengthPackingImage;
use super::RawSimplePackingImage;
use super::SectionSet;
use super::SectionSets;
use super::SectionSetsIter;

impl<'a> SectionSet<'a> {
    // Discipline – GRIB Master table number
    pub(crate) fn master_table_number(&self) -> Option<usize> {
        Some(self.section0?.master_table_number())
    }

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

    // Di – i direction increment (see Notes 1 and 5)
    pub fn di(&self) -> Option<usize> {
        Some(self.section3?.di())
    }

    // Dj – j direction increment (see Notes 1 and 5)
    pub fn dj(&self) -> Option<usize> {
        Some(self.section3?.dj())
    }

    pub fn unpack_simple(&self) -> Result<RawSimplePackingImage, String> {
        let sec3 = self.section3.ok_or("Invalid section 3");
        let width = sec3?.ni();
        let height = sec3?.nj();

        let sec5 = self.section5.ok_or("Invalid section 5");
        let sec6 = self.section6.ok_or("Invalid section 6");
        let sec7 = self.section7.ok_or("Invalid section 7");

        return match sec5?.template() {
            Some(Section5TemplateNumber::T0(t)) => {
                assert!(t.bits() * sec5?.point_count() <= sec7?.buf[5..].len() * 8);
                let pixels = unpack(&sec7?.buf[5..], t.bits(), t.bits() * sec5?.point_count());
                assert!(pixels.len() == sec5?.point_count());

                match sec6?.bit_map_indicator() {
                    0 => Ok(RawSimplePackingImage {
                        width,
                        height,
                        r: t.r(),
                        e: t.e(),
                        d: t.d(),
                        bits: t.bits(),
                        pixels: apply_bit_map(&pixels, sec6?.bit_map(), width, height),
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
            _ => Err(format!("Invalid template 5")),
        };
    }

    pub fn unpack_run_length(&self) -> Result<RawRunLengthPackingImage, String> {
        let sec3 = self.section3.ok_or("Invalid section 3");
        let width = sec3?.ni();
        let height = sec3?.nj();

        let sec5 = self.section5.ok_or("Invalid section 5");
        let sec6 = self.section6.ok_or("Invalid section 6");
        let sec7 = self.section7.ok_or("Invalid section 7");

        return match sec5?.template() {
            Some(Section5TemplateNumber::T200(t)) => {
                let pixels = unpack_run_length_packing(&sec7?.buf[5..], t.bits(), t.v());
                assert!(pixels.len() == sec5?.point_count());

                match sec6?.bit_map_indicator() {
                    0 => Ok(RawRunLengthPackingImage {
                        width,
                        height,
                        bits: t.bits(),
                        factor: t.factor(),
                        levels: t.levels(),
                        pixels: apply_bit_map(&pixels, sec6?.bit_map(), width, height),
                    }),
                    _ => Ok(RawRunLengthPackingImage {
                        width,
                        height,
                        bits: t.bits(),
                        factor: t.factor(),
                        levels: t.levels(),
                        pixels,
                    }),
                }
            }
            _ => Err(format!("Invalid template 5")),
        };
    }

    pub fn unpack_complex_packing_and_spatial_differencing(
        &self,
    ) -> Result<RawSimplePackingImage, String> {
        let sec3 = self.section3.ok_or("Invalid section 3");
        let width = sec3?.ni();
        let height = sec3?.nj();

        let sec5 = self.section5.ok_or("Invalid section 5");
        let sec6 = self.section6.ok_or("Invalid section 6");
        let sec7 = self.section7.ok_or("Invalid section 7");

        return match sec5?.template() {
            Some(Section5TemplateNumber::T3(t)) => {
                let pixels = unpack_complex_packing_and_spatial_differencing(&sec7?.buf[5..], t);
                assert!(pixels.len() == sec5?.point_count());

                match sec6?.bit_map_indicator() {
                    0 => Ok(RawSimplePackingImage {
                        width,
                        height,
                        r: t.r(),
                        e: t.e(),
                        d: t.d(),
                        bits: t.bits(),
                        pixels: apply_bit_map(&pixels, sec6?.bit_map(), width, height),
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
            _ => Err(format!("Invalid template 5")),
        };
    }
}

impl fmt::Display for SectionSet<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            match self.section4 {
                Some(sec) => format!("{}", sec),
                _ => format!("template4.?"),
            },
            match self.section5 {
                Some(sec) => format!("{}", sec),
                _ => format!("template5.?"),
            },
            match self.section6 {
                Some(sec) => format!("{}", sec),
                _ => format!("template6.?"),
            },
            match self.section7 {
                Some(sec) => format!("{}", sec),
                _ => format!("template7.?"),
            },
        )
    }
}

impl fmt::Debug for SectionSet<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}",
            match self.section0 {
                Some(sec) => format!("{:?}", sec),
                _ => String::from(""),
            },
            match self.section1 {
                Some(sec) => format!("{:?}", sec),
                _ => String::from(""),
            },
            match self.section2 {
                Some(sec) => format!("{:?}", sec),
                _ => String::from(""),
            },
            match self.section3 {
                Some(sec) => format!("{:?}", sec),
                _ => String::from(""),
            },
            match self.section4 {
                Some(sec) => format!("{:?}", sec),
                _ => String::from(""),
            },
            match self.section5 {
                Some(sec) => format!("{:?}", sec),
                _ => String::from(""),
            },
            match self.section6 {
                Some(sec) => format!("{:?}", sec),
                _ => String::from(""),
            },
            match self.section7 {
                Some(sec) => format!("{:?}", sec),
                _ => String::from(""),
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

// impl fmt::Display for PackingType {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             PackingType::Simple => write!(f, "simple: "),
//             PackingType::RunLength => write!(f, "run length: "),
//             PackingType::ComplexPackingAndSpatialDifferencing => {
//                 write!(f, "complex packing and spatial...")
//             }
//         }
//     }
// }

// impl fmt::Debug for PackingType {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "bit_map: ")
//     }
// }

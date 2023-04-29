//! GRIB2 declaration

pub mod bit_map_utils_impl;
pub mod decode_utils_impl;
pub mod section;
pub mod type_utils_impl;
pub mod utils_impl;

use self::section::SectionSets;
use self::utils_impl::parse;

#[derive(Clone)]
pub struct Grib2<'a> {
    sectionsets: SectionSets<'a>,
}

impl<'a> Grib2<'a> {
    pub fn new() -> Self {
        Self {
            sectionsets: SectionSets::new(),
        }
    }

    pub fn parse(&mut self, buf: &'a [u8]) {
        self.sectionsets.extend(parse(&buf));
    }

    pub fn sectionsets(&self) -> &SectionSets<'a> {
        return &self.sectionsets;
    }

    pub fn dump(&self) {
        for (i, sectionset) in self.sectionsets.iter().enumerate() {
            println!("{:05}\n{} ", i, sectionset);
        }
    }
}

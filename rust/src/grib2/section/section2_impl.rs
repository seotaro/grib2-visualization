//! GRIB2 Section2 implementation

use std::fmt;

use super::super::type_utils_impl::u32_be;
use super::super::type_utils_impl::u8_be;
use super::Section;
use super::Section2;

impl<'a> Section2<'a> {
    pub(crate) fn create(buf: &'a [u8]) -> Self {
        Self { buf: buf }
    }
}

impl<'a> Section for Section2<'a> {
    // Length of section in octets
    fn length(&self) -> usize {
        u32_be(&self.buf[0..4]) as usize
    }

    // Number of section
    fn section_number(&self) -> usize {
        u8_be(&self.buf[4..5]) as usize
    }
}

impl fmt::Display for Section2<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "--Section2\nlength: {}\n", self.length(),)
    }
}

//! GRIB2 Section7 implementation

use std::fmt;

use super::super::type_utils_impl::u32_be;
use super::super::type_utils_impl::u8_be;
use super::Section;
use super::Section7;

impl<'a> Section7<'a> {
    pub(crate) fn create(buf: &'a [u8]) -> Self {
        Self { buf: buf }
    }
}

impl<'a> Section for Section7<'a> {
    // Length of section in octets
    fn length(&self) -> usize {
        u32_be(&self.buf[0..4]) as usize
    }

    // Number of section
    fn section_number(&self) -> usize {
        u8_be(&self.buf[4..5]) as usize
    }
}

impl fmt::Display for Section7<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "--Section7\nlength: {}\n", self.length(),)
    }
}

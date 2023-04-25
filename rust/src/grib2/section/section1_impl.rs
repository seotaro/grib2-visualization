//! GRIB2 Section1 implementation

use chrono::{DateTime, Utc};
use std::fmt;

use super::super::type_utils_impl::datetime_be;
use super::super::type_utils_impl::u32_be;
use super::super::type_utils_impl::u8_be;
use super::Section;
use super::Section1;

impl<'a> Section1<'a> {
    pub(crate) fn create(buf: &'a [u8]) -> Self {
        Self { buf: buf }
    }

    // Reference time of data
    pub(crate) fn reference_time(&self) -> DateTime<Utc> {
        datetime_be(&self.buf[12..19])
    }
}

impl fmt::Debug for Section1<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.reference_time())
    }
}

impl<'a> Section for Section1<'a> {
    // Length of section in octets
    fn length(&self) -> usize {
        u32_be(&self.buf[0..4]) as usize
    }

    // Number of section
    fn section_number(&self) -> usize {
        u8_be(&self.buf[4..5]) as usize
    }
}

impl fmt::Display for Section1<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "--Section1\nlength: {}\n", self.length(),)
    }
}

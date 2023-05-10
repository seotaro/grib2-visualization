//! GRIB2 Section0 implementation

use std::fmt;

use super::super::type_utils_impl::u64_be;
use super::super::type_utils_impl::u8_be;
use super::Section;
use super::Section0;

impl<'a> Section0<'a> {
    pub(crate) fn create(buf: &'a [u8]) -> Self {
        Self { buf: buf }
    }

    // Discipline – GRIB Master table number
    pub(crate) fn master_table_number(&self) -> usize {
        u8_be(&self.buf[6..7]) as usize
    }

    // Total length of GRIB message in octets
    fn total_length(&self) -> usize {
        u64_be(&self.buf[8..16]) as usize
    }
}

impl<'a> Section for Section0<'a> {
    // Length of section in octets
    fn length(&self) -> usize {
        16
    }

    // Number of section
    fn section_number(&self) -> usize {
        0
    }
}

impl fmt::Display for Section0<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "--Section0\nmaster table number: {}\n",
            self.master_table_number(),
        )
    }
}

impl fmt::Debug for Section0<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
Section0\n\
\tmaster table number: {}\n\
\ttotal length: {}\n\
",
            self.master_table_number(),
            self.total_length()
        )
    }
}

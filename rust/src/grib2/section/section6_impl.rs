//! GRIB2 Section6 implementation

use std::fmt;

use super::super::type_utils_impl::u32_be;
use super::super::type_utils_impl::u8_be;
use super::Section;
use super::Section6;

impl<'a> Section6<'a> {
    pub(crate) fn create(buf: &'a [u8]) -> Self {
        Self { buf: buf }
    }

    // Bit-map indicator
    // 0 = A bit map applies to this product and is specified in this Section
    // 254 = A bit map defined previously in the same “GRIB” message applies to this product
    // 255 = A bit map does not apply to this product
    pub(crate) fn bit_map_indicator(&self) -> usize {
        u8_be(&self.buf[5..6]) as usize
    }

    // Bit-map
    pub(crate) fn bit_map(&self) -> &[u8] {
        &self.buf[6..]
    }
}

impl<'a> Section for Section6<'a> {
    // Length of section in octets
    fn length(&self) -> usize {
        u32_be(&self.buf[0..4]) as usize
    }

    // Number of section
    fn section_number(&self) -> usize {
        u8_be(&self.buf[4..5]) as usize
    }
}

impl fmt::Display for Section6<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bit_map: {}", self.bit_map_indicator())
    }
}

impl fmt::Debug for Section6<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bit_map indicator: {}", self.bit_map_indicator())
    }
}

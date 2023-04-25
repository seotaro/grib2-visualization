//! GRIB2 Section5 implementation

use std::fmt;

use super::super::type_utils_impl::u16_be;
use super::super::type_utils_impl::u32_be;
use super::super::type_utils_impl::u8_be;
use super::section5_template::Template0;
use super::section5_template::Template200;
use super::section5_template::Template3;
use super::section5_template::TemplateNumber;
use super::PackingType;
use super::Section;
use super::Section5;

impl<'a> Section5<'a> {
    pub(crate) fn create(buf: &'a [u8]) -> Self {
        Self { buf: buf }
    }

    pub fn bpp(&self) -> Option<usize> {
        Some(self.template()?.bpp())
    }

    pub fn packing_type(&self) -> Option<PackingType> {
        Some(self.template()?.packing_type())
    }

    // Number of data points where one or more values are specified in Section 7
    // when a bit map is present, total number of data points when a bit map is absent 10-11
    fn point_count(&self) -> usize {
        u32_be(&self.buf[5..9]) as usize
    }

    // Data Representation Template Number (see code Table 5.0)
    pub(crate) fn template_number(&self) -> usize {
        u16_be(&self.buf[9..11]) as usize
    }

    // return template
    pub(crate) fn template(&self) -> Option<TemplateNumber> {
        match self.template_number() {
            0 => Some(TemplateNumber::T0(Template0 { buf: self.buf })),
            3 => Some(TemplateNumber::T3(Template3 { buf: self.buf })),
            200 => Some(TemplateNumber::T200(Template200 { buf: self.buf })),
            _ => None,
        }
    }
}

impl<'a> Section for Section5<'a> {
    // Length of section in octets
    fn length(&self) -> usize {
        u32_be(&self.buf[0..4]) as usize
    }

    // Number of section
    fn section_number(&self) -> usize {
        u8_be(&self.buf[4..5]) as usize
    }
}

impl fmt::Display for Section5<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.template() {
            Some(template) => write!(
                f,
                "\
--Section5\n\
length: {}\n\
point count: {}\n\
----template5.{}\n{}\
                ",
                self.length(),
                self.point_count(),
                self.template_number(),
                template
            ),
            None => write!(f, "none",),
        }
    }
}

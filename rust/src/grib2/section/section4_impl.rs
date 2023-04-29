//! GRIB2 Section4 implementation

use chrono::{DateTime, Utc};
use std::fmt;

use super::super::type_utils_impl::u16_be;
use super::super::type_utils_impl::u32_be;
use super::super::type_utils_impl::u8_be;
use super::section4_template::Template0;
use super::section4_template::Template50008;
use super::section4_template::Template50011;
use super::section4_template::Template8;
use super::section4_template::TemplateNumber;
use super::Section;
use super::Section4;

impl<'a> Section4<'a> {
    pub(crate) fn create(buf: &'a [u8]) -> Self {
        Self { buf: buf }
    }

    // Product Definition Template Number (see Code Table 4.0)
    fn template_number(&self) -> usize {
        u16_be(&self.buf[7..9]) as usize
    }

    // return template
    pub(crate) fn template(&self) -> Option<TemplateNumber> {
        match self.template_number() {
            0 => Some(TemplateNumber::T0(Template0 { buf: self.buf })),
            8 => Some(TemplateNumber::T8(Template8 { buf: self.buf })),
            50011 => Some(TemplateNumber::T50011(Template50011 { buf: self.buf })),
            50008 => Some(TemplateNumber::T50008(Template50008 { buf: self.buf })),
            _ => None,
        }
    }
}

impl<'a> Section for Section4<'a> {
    // Length of section in octets
    fn length(&self) -> usize {
        u32_be(&self.buf[0..4]) as usize
    }

    // Number of section
    fn section_number(&self) -> usize {
        u8_be(&self.buf[4..5]) as usize
    }
}

impl fmt::Display for Section4<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "template4.{}", self.template_number())
    }
}

impl fmt::Debug for Section4<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.template() {
            Some(template) => write!(f, "template4.{} {:?}", self.template_number(), template),
            None => write!(f, "None"),
        }
    }
}

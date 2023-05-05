//! GRIB2 Section7 implementation

use std::fmt;

use super::super::type_utils_impl::u32_be;
use super::super::type_utils_impl::u8_be;
use super::section5_template::TemplateNumber as Section5TemplateNumber;
use super::section7_template::Template0;
use super::section7_template::Template200;
use super::section7_template::Template3;
use super::section7_template::TemplateNumber;
use super::Section;
use super::Section5;
use super::Section7;

impl<'a> Section7<'a> {
    pub(crate) fn create(buf: &'a [u8], section5: Section5<'a>) -> Self {
        Self { buf, section5 }
    }

    pub(crate) fn template_number(&self) -> usize {
        self.section5.template_number()
    }

    // return template
    pub(crate) fn template(&self) -> Option<TemplateNumber> {
        match self.section5.template_number() {
            0 => Some(TemplateNumber::T0(Template0 { buf: self.buf })),
            3 => {
                if let Section5TemplateNumber::T3(t) = self.section5.template()? {
                    Some(TemplateNumber::T3(Template3 {
                        buf: self.buf,
                        section5_template3: t,
                    }))
                } else {
                    None
                }
            }
            200 => Some(TemplateNumber::T200(Template200 { buf: self.buf })),
            _ => None,
        }
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
        write!(f, "template7.{}", self.template_number())
    }
}

impl fmt::Debug for Section7<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.template() {
            Some(template) => write!(
                f,
                "Section7\n\
\ttemplate: 7.{}\n{:?}",
                self.template_number(),
                template
            ),
            None => write!(
                f,
                "Section7\n\
\tunsported template7.{}\n",
                self.template_number()
            ),
        }
    }
}

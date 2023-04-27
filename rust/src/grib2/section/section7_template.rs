//! GRIB2 Section7 template declaration

pub(crate) mod section7_template_impl;

use super::section5_template::Template3 as Section5Template3;

pub(crate) enum TemplateNumber<'a> {
    T0(Template0<'a>),     // 7.0
    T3(Template3<'a>),     // 7.3
    T200(Template200<'a>), // 7.200
}

pub(crate) trait Template {}

// template 7.0
// Grid point data – simple packing
#[derive(Copy, Clone)]
pub(crate) struct Template0<'a> {
    pub(crate) buf: &'a [u8],
}

// template 7.3
// Grid point data – complex packing and spatial differencing
#[derive(Copy, Clone)]
pub(crate) struct Template3<'a> {
    pub(crate) buf: &'a [u8],
    pub(crate) section5_template3: Section5Template3<'a>,
}

// template 7.200
// Grid point data – run length packing with level values
#[derive(Copy, Clone)]
pub(crate) struct Template200<'a> {
    pub(crate) buf: &'a [u8],
}

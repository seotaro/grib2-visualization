//! GRIB2 Section5 template declaration

pub(crate) mod section5_template_impl;

use super::super::section::PackingType;

pub(crate) enum TemplateNumber<'a> {
    T0(Template0<'a>),     // 5.0
    T3(Template3<'a>),     // 5.3
    T200(Template200<'a>), // 5.200
}

pub(crate) trait Template {
    fn bpp(&self) -> usize;
    fn packing_type(&self) -> PackingType;
}

// template 5.0
// Grid point data – simple packing
#[derive(Copy, Clone)]
pub(crate) struct Template0<'a> {
    pub(crate) buf: &'a [u8],
}

// template 5.3
// Grid point data – complex packing and spatial differencing
#[derive(Copy, Clone)]
pub(crate) struct Template3<'a> {
    pub(crate) buf: &'a [u8],
}
// template 5.200
// Grid point data – run length packing with level values
#[derive(Copy, Clone)]
pub(crate) struct Template200<'a> {
    pub(crate) buf: &'a [u8],
}

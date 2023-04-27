//! GRIB2 Section7 template implementation

use std::fmt;

use super::Template;
use super::Template0;
use super::Template200;
use super::Template3;
use super::TemplateNumber;

impl<'a> TemplateNumber<'a> {}

impl fmt::Display for TemplateNumber<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TemplateNumber::T0(t) => write!(f, "{}", t),
            TemplateNumber::T3(t) => write!(f, "{}", t),
            TemplateNumber::T200(t) => write!(f, "{}", t),
        }
    }
}
impl fmt::Debug for TemplateNumber<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TemplateNumber::T0(t) => write!(f, "{:?}", t),
            TemplateNumber::T3(t) => write!(f, "{:?}", t),
            TemplateNumber::T200(t) => write!(f, "{:?}", t),
        }
    }
}

// template 7.0
// Grid point data – simple packing
impl<'a> Template0<'a> {}

impl<'a> Template for Template0<'a> {}

impl fmt::Display for Template0<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "",)
    }
}

impl fmt::Debug for Template0<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "",)
    }
}

// template 7.3
// Grid point data – complex packing and spatial differencing
impl<'a> Template3<'a> {}

impl<'a> Template for Template3<'a> {}

impl fmt::Display for Template3<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl fmt::Debug for Template3<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "",)
    }
}

// template 7.200
// Grid point data – run length packing with level values
impl<'a> Template200<'a> {}

impl<'a> Template for Template200<'a> {}

impl fmt::Display for Template200<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl fmt::Debug for Template200<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

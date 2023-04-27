//! GRIB2 Section5 template implementation

use std::fmt;

use super::super::super::section::PackingType;
use super::super::super::type_utils_impl::float_be;
use super::super::super::type_utils_impl::i16_be;
use super::super::super::type_utils_impl::i8_be;
use super::super::super::type_utils_impl::u16_be;
use super::super::super::type_utils_impl::u32_be;
use super::super::super::type_utils_impl::u8_be;
use super::Template;
use super::Template0;
use super::Template200;
use super::Template3;
use super::TemplateNumber;

impl<'a> TemplateNumber<'a> {
    pub fn bpp(&self) -> usize {
        match self {
            TemplateNumber::T0(t) => t.bpp(),
            TemplateNumber::T3(t) => t.bpp(),
            TemplateNumber::T200(t) => t.bpp(),
        }
    }

    pub fn packing_type(&self) -> PackingType {
        match self {
            TemplateNumber::T0(t) => t.packing_type(),
            TemplateNumber::T3(t) => t.packing_type(),
            TemplateNumber::T200(t) => t.packing_type(),
        }
    }
}

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

// template 5.0
// Grid point data – simple packing
impl<'a> Template0<'a> {
    // Reference value (R) (IEEE 32-bit floating-point value)
    pub(crate) fn r(&self) -> f32 {
        float_be(&self.buf[11..15])
    }
    // Binary Scale Factor (E)
    pub(crate) fn e(&self) -> isize {
        i16_be(&self.buf[15..17]) as isize
    }

    // Decimal Scale Factor (D)
    pub(crate) fn d(&self) -> isize {
        i16_be(&self.buf[17..19]) as isize
    }
    // Number of bits used for each packed value for simple packing, or for each group reference value for complex packing or spatial differencing
    pub(crate) fn bits(&self) -> usize {
        u8_be(&self.buf[19..20]) as usize
    }

    pub fn decode_value(r: f32, e: isize, d: isize, value: isize) -> f32 {
        (r + (value as f32) * 2.0f32.powi(e as i32)) / 10.0f32.powi(d as i32)
    }
}

impl<'a> Template for Template0<'a> {
    fn bpp(&self) -> usize {
        self.bits()
    }

    fn packing_type(&self) -> PackingType {
        PackingType::Simple
    }
}

impl fmt::Display for Template0<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
r: {}\n\
e: {}\n\
d: {}\n\
bit length: {}\n\
            ",
            self.r(),
            self.e(),
            self.d(),
            self.bits(),
        )
    }
}

impl fmt::Debug for Template0<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
r: {}, \
e: {}, \
d: {}, \
bit length: {}, \
            ",
            self.r(),
            self.e(),
            self.d(),
            self.bits(),
        )
    }
}

// template 5.3
// Grid point data – complex packing and spatial differencing
impl<'a> Template3<'a> {
    // Reference value (R) (IEEE 32-bit floating-point value)
    pub(crate) fn r(&self) -> f32 {
        float_be(&self.buf[11..15])
    }
    // Binary Scale Factor (E)
    pub(crate) fn e(&self) -> isize {
        i16_be(&self.buf[15..17]) as isize
    }

    // Decimal Scale Factor (D)
    pub(crate) fn d(&self) -> isize {
        i16_be(&self.buf[17..19]) as isize
    }
    // Number of bits used for each packed value for simple packing, or for each group reference value for complex packing or spatial differencing
    pub(crate) fn bits(&self) -> usize {
        u8_be(&self.buf[19..20]) as usize
    }

    // NG – number of groups of data values into which field is split
    pub(crate) fn ng(&self) -> usize {
        u32_be(&self.buf[31..35]) as usize
    }

    // Reference for group widths
    pub(crate) fn group_width_reference(&self) -> usize {
        u8_be(&self.buf[35..36]) as usize
    }

    // Number of bits used for the group widths (after the reference value in octet 36 has been removed
    pub(crate) fn group_width_bits(&self) -> usize {
        u8_be(&self.buf[36..37]) as usize
    }

    // Reference for group lengths
    pub(crate) fn group_length_reference(&self) -> usize {
        u32_be(&self.buf[37..41]) as usize
    }

    // Length increment for the group lengths
    pub(crate) fn group_length_increment(&self) -> usize {
        u8_be(&self.buf[41..42]) as usize
    }

    // True length of last group
    pub(crate) fn last_group_length(&self) -> usize {
        u32_be(&self.buf[42..46]) as usize
    }

    // Number of bits used for the scaled group lengths (after subtraction of the reference value given in octets 38–41 and division by the length increment given in octet 42)
    pub(crate) fn group_length_bits(&self) -> usize {
        u8_be(&self.buf[46..47]) as usize
    }

    // Order of spatial differencing
    pub(crate) fn order(&self) -> usize {
        u8_be(&self.buf[47..48]) as usize
    }

    // Number of octets required in the data section to specify extra descriptors needed for spatial differencing (octets 6–ww in data template 7.3
    pub(crate) fn octets(&self) -> usize {
        u8_be(&self.buf[48..49]) as usize
    }
}

impl<'a> Template for Template3<'a> {
    fn bpp(&self) -> usize {
        self.bits()
    }

    fn packing_type(&self) -> PackingType {
        PackingType::ComplexPackingAndSpatialDifferencing
    }
}

impl fmt::Display for Template3<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
r: {}\n\
e: {}\n\
d: {}\n\
bit length: {}\n\
            ",
            self.r(),
            self.e(),
            self.d(),
            self.bits(),
        )
    }
}

impl fmt::Debug for Template3<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
r: {}, \
e: {}, \
d: {}, \
bits: {}, \
ng: {}, \
group_width_reference: {}, \
group_width_bits: {}, \
group_length_reference: {}, \
group_length_increment: {}, \
last_group_length: {}, \
group_length_bits: {}, \
order: {}, \
octets: {}, \
            ",
            self.r(),
            self.e(),
            self.d(),
            self.bits(),
            self.ng(),
            self.group_width_reference(),
            self.group_width_bits(),
            self.group_length_reference(),
            self.group_length_increment(),
            self.last_group_length(),
            self.group_length_bits(),
            self.order(),
            self.octets(),
        )
    }
}

// template 5.200
// Grid point data – run length packing with level values
impl<'a> Template200<'a> {
    // Number of bits used for each packed value in the run length packing with level value
    pub(crate) fn bits(&self) -> usize {
        u8_be(&self.buf[11..12]) as usize
    }

    // MV – maximum value within the levels that are used in the packing
    pub(crate) fn v(&self) -> usize {
        u16_be(&self.buf[12..14]) as usize
    }
    // MVL – maximum value of level (predefined)
    pub(crate) fn m(&self) -> usize {
        u16_be(&self.buf[14..16]) as usize
    }

    // Decimal scale factor of 1)) representative value of each level
    pub(crate) fn factor(&self) -> isize {
        i8_be(&self.buf[16..17]) as isize
    }

    // List of MVL scaled representative values of each level from lv = 1 to MVL
    pub(crate) fn levels(&self) -> Vec<u16> {
        let m = self.m();

        let mut dest: Vec<u16> = vec![0; m];
        for i in 0..m {
            let index = 17 + i * 2;
            dest[i] = u16_be(&self.buf[index..index + 2]);
        }

        return dest;
    }
}

impl<'a> Template for Template200<'a> {
    fn bpp(&self) -> usize {
        self.bits()
    }

    fn packing_type(&self) -> PackingType {
        PackingType::RunLength
    }
}

impl fmt::Display for Template200<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
bits: {}\n\
v: {}\n\
m: {}\n\
factor: {}\n\
levels.len(): {}\n\
            ",
            self.bits(),
            self.v(),
            self.m(),
            self.factor(),
            self.levels().len(),
        )
    }
}

impl fmt::Debug for Template200<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
bits: {}, \
v: {}, \
m: {}, \
factor: {}, \
levels.len(): {}, \
            ",
            self.bits(),
            self.v(),
            self.m(),
            self.factor(),
            self.levels().len(),
        )
    }
}

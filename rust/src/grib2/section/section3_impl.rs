//! GRIB2 Section3 implementation

use std::fmt;

use super::super::type_utils_impl::i32_be;
use super::super::type_utils_impl::u32_be;
use super::super::type_utils_impl::u8_be;
use super::Section;
use super::Section3;

impl<'a> Section3<'a> {
    pub(crate) fn create(buf: &'a [u8]) -> Self {
        Self { buf: buf }
    }

    // Number of data points
    pub(crate) fn point_count(&self) -> usize {
        u32_be(&self.buf[6..10]) as usize
    }

    // Ni – number of points along a parallel
    pub(crate) fn ni(&self) -> usize {
        u32_be(&self.buf[30..34]) as usize
    }

    // Nj – number of points along a meridian
    pub(crate) fn nj(&self) -> usize {
        u32_be(&self.buf[34..38]) as usize
    }

    // La1 – latitude of first grid point (see Note 1)
    pub(crate) fn la1(&self) -> isize {
        i32_be(&self.buf[46..50]) as isize
    }

    // Lo1 – longitude of first grid point (see Note 1)
    pub(crate) fn lo1(&self) -> isize {
        i32_be(&self.buf[50..54]) as isize
    }

    // La2 – latitude of last grid point (see Note 1)
    pub(crate) fn la2(&self) -> isize {
        i32_be(&self.buf[55..59]) as isize
    }

    // Lo2 – longitude of last grid point (see Note 1)
    pub(crate) fn lo2(&self) -> isize {
        i32_be(&self.buf[59..63]) as isize
    }

    // Di – i direction increment (see Notes 1 and 5)
    fn di(&self) -> usize {
        u32_be(&self.buf[63..67]) as usize
    }

    // Dj – j direction increment (see Notes 1 and 5)
    fn dj(&self) -> usize {
        u32_be(&self.buf[67..71]) as usize
    }
}

impl<'a> Section for Section3<'a> {
    // Length of section in octets
    fn length(&self) -> usize {
        u32_be(&self.buf[0..4]) as usize
    }

    // Number of section
    fn section_number(&self) -> usize {
        u8_be(&self.buf[4..5]) as usize
    }
}

impl fmt::Display for Section3<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
--Section3\n\
length: {}\n\
point count: {}\n\
Ni: {}\n\
Nj: {}\n\
La1: {}\n\
Lo1: {}\n\
La2: {}\n\
Lo2: {}\n\
Di: {}\n\
Dj: {}\n\
        ",
            self.length(),
            self.point_count(),
            self.ni(),
            self.nj(),
            self.la1(),
            self.lo1(),
            self.la2(),
            self.lo2(),
            self.di(),
            self.dj(),
        )
    }
}

impl fmt::Debug for Section3<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
point count: {}, \
Ni: {}, \
Nj: {}, \
La1: {}, \
Lo1: {}, \
La2: {}, \
Lo2: {}, \
Di: {}, \
Dj: {}\
        ",
            self.point_count(),
            self.ni(),
            self.nj(),
            self.la1(),
            self.lo1(),
            self.la2(),
            self.lo2(),
            self.di(),
            self.dj(),
        )
    }
}

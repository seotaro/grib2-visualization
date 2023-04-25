//! GRIB2 Section declaration

pub mod section1_impl;
pub mod section2_impl;
pub mod section3_impl;
pub mod section4_impl;
pub mod section4_template;
pub mod section5_impl;
pub mod section5_template;
pub mod section6_impl;
pub mod section7_impl;
pub mod section_impl;
pub mod sectionset_impl;

use serde::Serialize;

// GRIB2 Section7 data type
pub trait IsSection7DataType {}

pub struct RawSimplePackingImage {
    pub width: usize,
    pub height: usize,
    pub r: f32,
    pub e: isize,
    pub d: isize,
    pub bits: usize,
    pub pixels: Vec<u16>,
}

pub struct RawRunLengthPackingImage {
    pub width: usize,
    pub height: usize,
    pub bits: usize,
    pub factor: isize,
    pub levels: Vec<u16>,
    pub pixels: Vec<u8>,
}

#[derive(Serialize, Clone)]
pub struct Bounds {
    pub left: isize,
    pub bottom: isize,
    pub right: isize,
    pub top: isize,
}

#[derive(Copy, Clone)]
pub(crate) struct Section1<'a> {
    buf: &'a [u8],
}

#[derive(Copy, Clone)]
pub(crate) struct Section2<'a> {
    buf: &'a [u8],
}

#[derive(Copy, Clone)]
pub(crate) struct Section3<'a> {
    buf: &'a [u8],
}

#[derive(Copy, Clone)]
pub(crate) struct Section4<'a> {
    buf: &'a [u8],
}

#[derive(Copy, Clone)]
pub(crate) struct Section5<'a> {
    buf: &'a [u8],
}

#[derive(Copy, Clone)]
pub(crate) struct Section6<'a> {
    buf: &'a [u8],
}

#[derive(Copy, Clone)]
pub(crate) struct Section7<'a> {
    buf: &'a [u8],
}

pub(crate) trait Section {
    // Length of section in octets
    fn length(&self) -> usize;

    // Number of section
    fn section_number(&self) -> usize;
}

#[derive(Copy, Clone)]
pub struct SectionSet<'a> {
    pub(crate) section1: Option<Section1<'a>>,
    pub(crate) section2: Option<Section2<'a>>,
    pub(crate) section3: Option<Section3<'a>>,
    pub(crate) section4: Option<Section4<'a>>,
    pub(crate) section5: Option<Section5<'a>>,
    pub(crate) section6: Option<Section6<'a>>,
    pub(crate) section7: Option<Section7<'a>>,
}

#[derive(Clone)]
pub struct SectionSets<'a> {
    pub(crate) items: Vec<SectionSet<'a>>,
}

pub struct SectionSetsIter<'a> {
    items: &'a Vec<SectionSet<'a>>,
    index: usize,
}

#[derive(Serialize)]
pub enum PackingType {
    Simple,
    RunLength,
}

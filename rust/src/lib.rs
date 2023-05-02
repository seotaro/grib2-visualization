//! JS との境界を定義したファイル

mod grib2;

use chrono::{DateTime, Utc};
use grib2::section::Bounds;
use grib2::section::PackingType;
use grib2::utils_impl::first_plane_name;
use grib2::Grib2;
use serde::Serialize;
use std::cmp;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct SimplePackingAttributes {
    pub width: usize,
    pub height: usize,
    pub di: usize,
    pub dj: usize,
    bounds: Bounds,
    pub r: f32,
    pub e: isize,
    pub d: isize,
    pub bits: usize,
    pixels: Vec<u16>,
    pub min: u16,
    pub max: u16,
}
#[wasm_bindgen]
impl SimplePackingAttributes {
    pub fn bounds(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.bounds).unwrap()
    }
    pub fn pixels(&self) -> Vec<u16> {
        self.pixels.clone()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct RunLengthPackingAttributes {
    pub width: usize,
    pub height: usize,
    pub di: usize,
    pub dj: usize,
    bounds: Bounds,
    pub bits: usize,
    pub factor: isize,
    levels: Vec<u16>,
    pixels: Vec<u8>,
    pub min: u8,
    pub max: u8,
}

#[wasm_bindgen]
impl RunLengthPackingAttributes {
    pub fn bounds(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.bounds).unwrap()
    }
    pub fn pixels(&self) -> Vec<u8> {
        self.pixels.clone()
    }
    pub fn levels(&self) -> Vec<u16> {
        self.levels.clone()
    }
}

#[wasm_bindgen]
pub struct PackingImage {
    packing_type: PackingType,
    simple_packing_attributes: Option<SimplePackingAttributes>,
    run_length_packing_attributes: Option<RunLengthPackingAttributes>,
}
#[wasm_bindgen]
impl PackingImage {
    pub fn packing_type(&self) -> String {
        match self.packing_type {
            PackingType::Simple => String::from("simple"),
            PackingType::RunLength => String::from("run-length"),
            PackingType::ComplexPackingAndSpatialDifferencing => String::from("simple"),
        }
    }

    pub fn simple_packing_attributes(&self) -> Option<SimplePackingAttributes> {
        self.simple_packing_attributes.clone()
    }
    pub fn run_length_packing_attributes(&self) -> Option<RunLengthPackingAttributes> {
        self.run_length_packing_attributes.clone()
    }
}

// JSValue は u32, i32 まで
#[wasm_bindgen]
#[derive(Serialize)]
pub struct Item {
    reference_datetime: Option<DateTime<Utc>>,
    packing_type: Option<PackingType>,
    point_count: Option<u32>,
    genre: Option<u32>,
    parameter_description: Option<String>,
    parameter_category: Option<u32>,
    parameter_number: Option<u32>,
    datetime: Option<DateTime<Utc>>,
    first_plane_name: Option<String>,
    first_plane_type: Option<u32>,
    first_plane_factor: Option<i32>,
    first_plane_value: Option<i32>,
}

#[wasm_bindgen]
pub struct Grib2Wrapper {
    buf: Vec<u8>, // grib2 から参照するデータ
    grib2: Grib2<'static>,
    items: Vec<Item>,
}

#[wasm_bindgen]
impl Grib2Wrapper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            buf: Vec::new(),
            grib2: Grib2::new(),
            items: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.buf.clear();
        self.items.clear();
    }

    pub fn load(&mut self, src: &[u8]) {
        self.buf.extend_from_slice(src);
        let slice =
            unsafe { std::slice::from_raw_parts_mut(self.buf.as_mut_ptr(), self.buf.len()) };

        self.grib2 = Grib2::new();
        self.grib2.parse(slice);

        let mut items = Vec::new();
        for sectionset in self.grib2.sectionsets().iter() {
            items.push(Item {
                reference_datetime: sectionset.reference_datetime(),
                packing_type: sectionset.packing_type(),
                point_count: Self::to_u32(sectionset.point_count()),
                parameter_description: self.parameter_description(
                    sectionset.genre,
                    sectionset.parameter_category(),
                    sectionset.parameter_number(),
                ),
                genre: Self::to_u32(sectionset.genre),
                parameter_category: Self::to_u32(sectionset.parameter_category()),
                parameter_number: Self::to_u32(sectionset.parameter_number()),
                datetime: sectionset.datetime(),
                first_plane_name: Self::first_plane_name(
                    sectionset.first_plane_type(),
                    sectionset.first_plane_factor(),
                    sectionset.first_plane_value(),
                ),
                first_plane_type: Self::to_u32(sectionset.first_plane_type()),
                first_plane_factor: Self::to_i32(sectionset.first_plane_factor()),
                first_plane_value: Self::to_i32(sectionset.first_plane_value()),
            })
        }
        self.items = items;
    }

    pub fn dump(&self) {
        let sectionsets = self.grib2.sectionsets();
        for (i, sectionset) in sectionsets.iter().enumerate() {
            log(&format!(
                "No.{:03} {:?} {:?} {} ",
                i,
                self.parameter_description(
                    sectionset.genre,
                    sectionset.parameter_category(),
                    sectionset.parameter_number(),
                ),
                Self::first_plane_name(
                    sectionset.first_plane_type(),
                    sectionset.first_plane_factor(),
                    sectionset.first_plane_value(),
                ),
                sectionset
            ));
        }
    }

    pub fn parameter_description(
        &self,
        genre: Option<usize>,
        parameter_category: Option<usize>,
        parameter_number: Option<usize>,
    ) -> Option<String> {
        let description =
            self.grib2
                .parameter_description(genre?, parameter_category?, parameter_number?);

        return Some(format!("{} [{}]", description?.name, description?.unit));
    }

    pub fn first_plane_name(
        plane_type: Option<usize>,
        plane_factor: Option<isize>,
        plane_value: Option<isize>,
    ) -> Option<String> {
        first_plane_name(plane_type?, plane_factor?, plane_value?)
    }

    pub fn to_u32(src: Option<usize>) -> Option<u32> {
        Some(src? as u32)
    }

    pub fn to_i32(src: Option<isize>) -> Option<i32> {
        Some(src? as i32)
    }

    // JSON 形式で返す
    pub fn items(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.items).unwrap()
    }

    pub fn unpack_image(&self, index: usize) -> Option<PackingImage> {
        let sectionsets = self.grib2.sectionsets();
        let sectionset = sectionsets.get(index);
        let bounds = sectionset.bounds()?;
        let di = sectionset.di()?;
        let dj = sectionset.dj()?;
        let packing_type = sectionset.packing_type()?;

        match packing_type {
            PackingType::Simple => {
                let image = sectionset.unpack_simple().ok()?;

                // u16::MAX はビットマップ適用して範囲外のピクセル
                let mut min = u16::MAX - 1;
                let mut max = 0;
                for pixel in image.pixels.iter() {
                    if *pixel == u16::MAX {
                        continue;
                    }

                    min = cmp::min(min, *pixel);
                    max = cmp::max(max, *pixel);
                }

                Some(PackingImage {
                    packing_type,
                    simple_packing_attributes: Some(SimplePackingAttributes {
                        width: image.width,
                        height: image.height,
                        di,
                        dj,
                        bounds,
                        r: image.r,
                        e: image.e,
                        d: image.d,
                        bits: image.bits,
                        pixels: image.pixels,
                        min,
                        max,
                    }),
                    run_length_packing_attributes: None,
                })
            }
            PackingType::RunLength => {
                let image = sectionset.unpack_run_length().ok()?;

                let mut min = image.pixels[0];
                let mut max = image.pixels[0];
                for pixel in image.pixels.iter() {
                    min = cmp::min(min, *pixel);
                    max = cmp::max(max, *pixel);
                }

                Some(PackingImage {
                    packing_type,
                    simple_packing_attributes: None,
                    run_length_packing_attributes: Some(RunLengthPackingAttributes {
                        width: image.width,
                        height: image.height,
                        di,
                        dj,
                        bounds,
                        bits: image.bits,
                        factor: image.factor,
                        pixels: image.pixels,
                        min,
                        max,
                        levels: image.levels,
                    }),
                })
            }
            PackingType::ComplexPackingAndSpatialDifferencing => {
                let image = sectionset
                    .unpack_complex_packing_and_spatial_differencing()
                    .ok()?;

                // u16::MAX はビットマップ適用して範囲外のピクセル
                let mut min = u16::MAX - 1;
                let mut max = 0;
                for pixel in image.pixels.iter() {
                    if *pixel == u16::MAX {
                        continue;
                    }

                    min = cmp::min(min, *pixel);
                    max = cmp::max(max, *pixel);
                }

                Some(PackingImage {
                    packing_type,
                    simple_packing_attributes: Some(SimplePackingAttributes {
                        width: image.width,
                        height: image.height,
                        di,
                        dj,
                        bounds,
                        r: image.r,
                        e: image.e,
                        d: image.d,
                        bits: image.bits,
                        pixels: image.pixels,
                        min,
                        max,
                    }),
                    run_length_packing_attributes: None,
                })
            }
        }
    }
}

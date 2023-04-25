//! JS との境界を定義したファイル

mod grib2;

use chrono::{DateTime, Utc};
use grib2::section::Bounds;
use grib2::section::PackingType;
use grib2::utils_impl::{first_plane_name, parameter_name};
use grib2::Grib2;
use serde::Serialize;
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
pub struct RawSimplePackingImage {
    pub width: usize,
    pub height: usize,
    pub r: f32,
    pub e: isize,
    pub d: isize,
    pub bits: usize,
    bounds: Bounds,
    pixels: Vec<u16>,
}

#[wasm_bindgen]
impl RawSimplePackingImage {
    pub fn packing_type(&self) -> String {
        "simple".to_string()
    }

    pub fn pixels(&self) -> Vec<u16> {
        self.pixels.clone()
    }

    // pub fn bounds(&self) -> Bounds {
    //     self.bounds.clone()
    // }
}

// JSValue は u32, i32 まで
#[wasm_bindgen]
#[derive(Serialize)]
pub struct Item {
    reference_datetime: Option<DateTime<Utc>>,
    packing_type: Option<PackingType>,
    point_count: Option<u32>,
    parameter_name: Option<String>,
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
                parameter_name: Self::parameter_name(
                    sectionset.parameter_category(),
                    sectionset.parameter_number(),
                ),
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

    pub fn parameter_name(
        parameter_category: Option<usize>,
        parameter_number: Option<usize>,
    ) -> Option<String> {
        parameter_name(parameter_category?, parameter_number?)
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

    pub fn parse_simple_packing_image(&self, index: usize) -> RawSimplePackingImage {
        let sectionsets = self.grib2.sectionsets();

        log(&format!("parse_simple_packing_image {}", sectionsets.len()));

        let sectionset = sectionsets.get(index);
        let packing_type = sectionset.packing_type();

        let bounds = sectionset.bounds().unwrap();
        let image = sectionset.unpack().unwrap();
        RawSimplePackingImage {
            width: image.width,
            height: image.height,
            r: image.r,
            e: image.e,
            d: image.d,
            bits: image.bits,
            bounds: bounds,
            pixels: image.pixels,
        }
    }
}

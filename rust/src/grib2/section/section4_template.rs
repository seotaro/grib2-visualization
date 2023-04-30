//! GRIB2 Section4 template declaration

use chrono::{DateTime, Utc};

pub(crate) mod section4_template_impl;

pub(crate) enum TemplateNumber<'a> {
    T0(Template0<'a>),   // 4.0
    T1(Template1<'a>),   // 4.1
    T8(Template8<'a>),   // 4.8
    T9(Template9<'a>),   // 4.9
    T11(Template11<'a>), // 4.11
    // 4.50000
    T50008(Template50008<'a>), // 4.50008
    T50009(Template50009<'a>), // 4.50009
    T50011(Template50011<'a>), // 4.50011
    T50012(Template50012<'a>), // 4.50012
                               // 4.50030
}

pub(crate) trait Template {
    // パラメーターカテゴリー
    fn parameter_category(&self) -> usize;

    // パラメーター番号
    fn parameter_number(&self) -> usize;

    // データセットの時刻
    fn datetime(&self, reference_time: DateTime<Utc>) -> DateTime<Utc>;
}

// template 4.0
#[derive(Copy, Clone)]
pub(crate) struct Template0<'a> {
    pub(crate) buf: &'a [u8],
}

// template 4.1
#[derive(Copy, Clone)]
pub(crate) struct Template1<'a> {
    pub(crate) buf: &'a [u8],
}

// template 4.8
#[derive(Copy, Clone)]
pub(crate) struct Template8<'a> {
    pub(crate) buf: &'a [u8],
}

// template 4.9
#[derive(Copy, Clone)]
pub(crate) struct Template9<'a> {
    pub(crate) buf: &'a [u8],
}

// template 4.11
#[derive(Copy, Clone)]
pub(crate) struct Template11<'a> {
    pub(crate) buf: &'a [u8],
}

// template 4.50008
#[derive(Copy, Clone)]
pub(crate) struct Template50008<'a> {
    pub(crate) buf: &'a [u8],
}

// template 4.50009
#[derive(Copy, Clone)]
pub(crate) struct Template50009<'a> {
    pub(crate) buf: &'a [u8],
}

// template 4.50011
#[derive(Copy, Clone)]
pub(crate) struct Template50011<'a> {
    pub(crate) buf: &'a [u8],
}

// template 4.50012
#[derive(Copy, Clone)]
pub(crate) struct Template50012<'a> {
    pub(crate) buf: &'a [u8],
}

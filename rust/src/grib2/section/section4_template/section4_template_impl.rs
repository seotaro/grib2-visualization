//! GRIB2 Section4 template implementation

use chrono::{DateTime, Duration, Utc};
use std::fmt;

use super::super::super::type_utils_impl::datetime_be;
use super::super::super::type_utils_impl::i32_be;
use super::super::super::type_utils_impl::i8_be;
use super::super::super::type_utils_impl::time_span_be;
use super::super::super::type_utils_impl::u16_be;
use super::super::super::type_utils_impl::u8_be;
use super::Template;
use super::Template0;
use super::Template1;
use super::Template11;
use super::Template50000;
use super::Template50008;
use super::Template50009;
use super::Template50011;
use super::Template50012;
use super::Template8;
use super::Template9;
use super::TemplateNumber;

impl<'a> TemplateNumber<'a> {
    pub(crate) fn parameter_category(&self) -> usize {
        match self {
            TemplateNumber::T0(t) => t.parameter_category(),
            TemplateNumber::T1(t) => t.parameter_category(),
            TemplateNumber::T8(t) => t.parameter_category(),
            TemplateNumber::T9(t) => t.parameter_category(),
            TemplateNumber::T11(t) => t.parameter_category(),
            TemplateNumber::T50000(t) => t.parameter_category(),
            TemplateNumber::T50008(t) => t.parameter_category(),
            TemplateNumber::T50009(t) => t.parameter_category(),
            TemplateNumber::T50011(t) => t.parameter_category(),
            TemplateNumber::T50012(t) => t.parameter_category(),
        }
    }

    pub(crate) fn parameter_number(&self) -> usize {
        match self {
            TemplateNumber::T0(t) => t.parameter_number(),
            TemplateNumber::T1(t) => t.parameter_number(),
            TemplateNumber::T8(t) => t.parameter_number(),
            TemplateNumber::T9(t) => t.parameter_number(),
            TemplateNumber::T11(t) => t.parameter_number(),
            TemplateNumber::T50000(t) => t.parameter_number(),
            TemplateNumber::T50008(t) => t.parameter_number(),
            TemplateNumber::T50009(t) => t.parameter_number(),
            TemplateNumber::T50011(t) => t.parameter_number(),
            TemplateNumber::T50012(t) => t.parameter_number(),
        }
    }

    pub(crate) fn datetime(&self, reference_time: DateTime<Utc>) -> DateTime<Utc> {
        match self {
            TemplateNumber::T0(t) => t.datetime(reference_time),
            TemplateNumber::T1(t) => t.datetime(reference_time),
            TemplateNumber::T8(t) => t.datetime(reference_time),
            TemplateNumber::T9(t) => t.datetime(reference_time),
            TemplateNumber::T11(t) => t.datetime(reference_time),
            TemplateNumber::T50000(t) => t.datetime(reference_time),
            TemplateNumber::T50008(t) => t.datetime(reference_time),
            TemplateNumber::T50009(t) => t.datetime(reference_time),
            TemplateNumber::T50011(t) => t.datetime(reference_time),
            TemplateNumber::T50012(t) => t.datetime(reference_time),
        }
    }

    pub(crate) fn first_plane_type(&self) -> Option<usize> {
        match self {
            TemplateNumber::T0(t) => Some(t.first_plane_type()),
            TemplateNumber::T1(t) => Some(t.first_plane_type()),
            TemplateNumber::T8(t) => Some(t.first_plane_type()),
            TemplateNumber::T9(t) => Some(t.first_plane_type()),
            TemplateNumber::T11(t) => Some(t.first_plane_type()),
            TemplateNumber::T50000(t) => Some(t.first_plane_type()),
            TemplateNumber::T50008(t) => Some(t.first_plane_type()),
            TemplateNumber::T50009(t) => Some(t.first_plane_type()),
            TemplateNumber::T50011(t) => Some(t.first_plane_type()),
            TemplateNumber::T50012(t) => Some(t.first_plane_type()),
        }
    }

    pub(crate) fn first_plane_factor(&self) -> Option<isize> {
        match self {
            TemplateNumber::T0(t) => Some(t.first_plane_factor()),
            TemplateNumber::T1(t) => Some(t.first_plane_factor()),
            TemplateNumber::T8(t) => Some(t.first_plane_factor()),
            TemplateNumber::T9(t) => Some(t.first_plane_factor()),
            TemplateNumber::T11(t) => Some(t.first_plane_factor()),
            TemplateNumber::T50000(t) => Some(t.first_plane_factor()),
            TemplateNumber::T50008(t) => Some(t.first_plane_factor()),
            TemplateNumber::T50009(t) => Some(t.first_plane_factor()),
            TemplateNumber::T50011(t) => Some(t.first_plane_factor()),
            TemplateNumber::T50012(t) => Some(t.first_plane_factor()),
        }
    }

    pub(crate) fn first_plane_value(&self) -> Option<isize> {
        match self {
            TemplateNumber::T0(t) => Some(t.first_plane_value()),
            TemplateNumber::T1(t) => Some(t.first_plane_value()),
            TemplateNumber::T8(t) => Some(t.first_plane_value()),
            TemplateNumber::T9(t) => Some(t.first_plane_value()),
            TemplateNumber::T11(t) => Some(t.first_plane_value()),
            TemplateNumber::T50000(t) => Some(t.first_plane_value()),
            TemplateNumber::T50008(t) => Some(t.first_plane_value()),
            TemplateNumber::T50009(t) => Some(t.first_plane_value()),
            TemplateNumber::T50011(t) => Some(t.first_plane_value()),
            TemplateNumber::T50012(t) => Some(t.first_plane_value()),
        }
    }
}

impl fmt::Display for TemplateNumber<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TemplateNumber::T0(t) => write!(f, "{}", t),
            TemplateNumber::T1(t) => write!(f, "{}", t),
            TemplateNumber::T8(t) => write!(f, "{}", t),
            TemplateNumber::T9(t) => write!(f, "{}", t),
            TemplateNumber::T11(t) => write!(f, "{}", t),
            TemplateNumber::T50000(t) => write!(f, "{}", t),
            TemplateNumber::T50008(t) => write!(f, "{}", t),
            TemplateNumber::T50009(t) => write!(f, "{}", t),
            TemplateNumber::T50011(t) => write!(f, "{}", t),
            TemplateNumber::T50012(t) => write!(f, "{}", t),
        }
    }
}
impl fmt::Debug for TemplateNumber<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TemplateNumber::T0(t) => write!(f, "{:?}", t),
            TemplateNumber::T1(t) => write!(f, "{:?}", t),
            TemplateNumber::T8(t) => write!(f, "{:?}", t),
            TemplateNumber::T9(t) => write!(f, "{:?}", t),
            TemplateNumber::T11(t) => write!(f, "{:?}", t),
            TemplateNumber::T50000(t) => write!(f, "{:?}", t),
            TemplateNumber::T50008(t) => write!(f, "{:?}", t),
            TemplateNumber::T50009(t) => write!(f, "{:?}", t),
            TemplateNumber::T50011(t) => write!(f, "{:?}", t),
            TemplateNumber::T50012(t) => write!(f, "{:?}", t),
        }
    }
}

// template 4.0
impl<'a> Template0<'a> {
    // 作画処理の種類
    fn generating_type(&self) -> usize {
        u8_be(&self.buf[11..12]) as usize
    }

    // Background generating process identifier
    fn background_generating_identifier(&self) -> usize {
        u8_be(&self.buf[12..13]) as usize
    }

    // Hours after reference time of data cut–off (see Note)
    fn cut_off_hours(&self) -> usize {
        u16_be(&self.buf[14..16]) as usize
    }

    // Minutes after reference time of data cut–off
    fn cut_off_minutes(&self) -> usize {
        u8_be(&self.buf[16..17]) as usize
    }

    // 予報時間
    fn forecast_time_span(&self) -> usize {
        time_span_be(&self.buf[17..22])
    }

    // 第一固定面の種類
    fn first_plane_type(&self) -> usize {
        u8_be(&self.buf[22..23]) as usize
    }

    // 第一固定面の尺度因子
    fn first_plane_factor(&self) -> isize {
        i8_be(&self.buf[23..24]) as isize
    }

    // 第一固定面の尺度付きの値
    fn first_plane_value(&self) -> isize {
        i32_be(&self.buf[24..28]) as isize
    }

    // 第二固定面の種類
    fn second_plane_type(&self) -> usize {
        u8_be(&self.buf[28..29]) as usize
    }

    // 第二固定面の尺度因子
    fn second_plane_factor(&self) -> isize {
        i8_be(&self.buf[29..30]) as isize
    }

    // 第二固定面の尺度付きの値
    fn second_plane_value(&self) -> isize {
        i32_be(&self.buf[30..34]) as isize
    }
}

impl<'a> Template for Template0<'a> {
    // パラメーターカテゴリー
    fn parameter_category(&self) -> usize {
        u8_be(&self.buf[9..10]) as usize
    }
    // パラメーター番号
    fn parameter_number(&self) -> usize {
        u8_be(&self.buf[10..11]) as usize
    }

    // データセットの時刻を返す。
    fn datetime(&self, reference_time: DateTime<Utc>) -> DateTime<Utc> {
        reference_time + Duration::seconds(self.forecast_time_span() as i64)
    }
}

impl fmt::Display for Template0<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
parameter-category: {}\n\
parameter-number: {}\n\
generating type: {}\n\
background generating identifier: {}\n\
cut-off hours: {}\n\
cut-off minutes: {}\n\
forecast_time_span: {}\n\
first-plane-type: {}\n\
first-plane-factor: {}\n\
first-plane-value: {}\n\
second-plane-type: {}\n\
second-plane-factor: {}\n\
second-plane-value: {}\n\
",
            self.parameter_category(),
            self.parameter_number(),
            self.generating_type(),
            self.background_generating_identifier(),
            self.cut_off_hours(),
            self.cut_off_minutes(),
            self.forecast_time_span(),
            self.first_plane_type(),
            self.first_plane_factor(),
            self.first_plane_value(),
            self.second_plane_type(),
            self.second_plane_factor(),
            self.second_plane_value(),
        )
    }
}
impl fmt::Debug for Template0<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
parameter-category: {}, \
parameter-number: {}, \
first-plane-type: {}, \
first-plane-factor: {}, \
first-plane-value: {}, \
second-plane-type: {}, \
second-plane-factor: {}, \
second-plane-value: {}, \
",
            self.parameter_category(),
            self.parameter_number(),
            self.first_plane_type(),
            self.first_plane_factor(),
            self.first_plane_value(),
            self.second_plane_type(),
            self.second_plane_factor(),
            self.second_plane_value()
        )
    }
}

// template 4.1
impl<'a> Template1<'a> {
    // 作画処理の種類
    fn generating_type(&self) -> usize {
        u8_be(&self.buf[11..12]) as usize
    }

    // Background generating process identifier
    fn background_generating_identifier(&self) -> usize {
        u8_be(&self.buf[12..13]) as usize
    }

    // Hours after reference time of data cut–off (see Note)
    fn cut_off_hours(&self) -> usize {
        u16_be(&self.buf[14..16]) as usize
    }

    // Minutes after reference time of data cut–off
    fn cut_off_minutes(&self) -> usize {
        u8_be(&self.buf[16..17]) as usize
    }

    // 予報時間
    pub(crate) fn forecast_time_span(&self) -> usize {
        time_span_be(&self.buf[17..22])
    }

    // 第一固定面の種類
    fn first_plane_type(&self) -> usize {
        u8_be(&self.buf[22..23]) as usize
    }

    // 第一固定面の尺度因子
    fn first_plane_factor(&self) -> isize {
        i8_be(&self.buf[23..24]) as isize
    }

    // 第一固定面の尺度付きの値
    fn first_plane_value(&self) -> isize {
        i32_be(&self.buf[24..28]) as isize
    }

    // 第二固定面の種類
    fn second_plane_type(&self) -> usize {
        u8_be(&self.buf[28..29]) as usize
    }

    // 第二固定面の尺度因子
    fn second_plane_factor(&self) -> isize {
        i8_be(&self.buf[29..30]) as isize
    }

    // 第二固定面の尺度付きの値
    fn second_plane_value(&self) -> isize {
        i32_be(&self.buf[30..34]) as isize
    }
}

impl<'a> Template for Template1<'a> {
    // パラメーターカテゴリー
    fn parameter_category(&self) -> usize {
        u8_be(&self.buf[9..10]) as usize
    }
    // パラメーター番号
    fn parameter_number(&self) -> usize {
        u8_be(&self.buf[10..11]) as usize
    }

    // データセットの時刻を返す。
    fn datetime(&self, reference_time: DateTime<Utc>) -> DateTime<Utc> {
        reference_time + Duration::seconds(self.forecast_time_span() as i64)
    }
}

impl fmt::Display for Template1<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
parameter-category: {}\n\
parameter-number: {}\n\
generating type: {}\n\
background generating identifier: {}\n\
cut-off hours: {}\n\
cut-off minutes: {}\n\
forecast_time_span: {}\n\
first-plane-type: {}\n\
first-plane-factor: {}\n\
first-plane-value: {}\n\
second-plane-type: {}\n\
second-plane-factor: {}\n\
second-plane-value: {}\n\
",
            self.parameter_category(),
            self.parameter_number(),
            self.generating_type(),
            self.background_generating_identifier(),
            self.cut_off_hours(),
            self.cut_off_minutes(),
            self.forecast_time_span(),
            self.first_plane_type(),
            self.first_plane_factor(),
            self.first_plane_value(),
            self.second_plane_type(),
            self.second_plane_factor(),
            self.second_plane_value(),
        )
    }
}
impl fmt::Debug for Template1<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
parameter-category: {}, \
parameter-number: {}, \
first-plane-type: {}, \
first-plane-factor: {}, \
first-plane-value: {}, \
second-plane-type: {}, \
second-plane-factor: {}, \
second-plane-value: {}, \
",
            self.parameter_category(),
            self.parameter_number(),
            self.first_plane_type(),
            self.first_plane_factor(),
            self.first_plane_value(),
            self.second_plane_type(),
            self.second_plane_factor(),
            self.second_plane_value(),
        )
    }
}

// template 4.8
impl<'a> Template8<'a> {
    // 作画処理の種類
    fn generating_type(&self) -> usize {
        u8_be(&self.buf[11..12]) as usize
    }

    // Background generating process identifier
    fn background_generating_identifier(&self) -> usize {
        u8_be(&self.buf[12..13]) as usize
    }

    // Hours after reference time of data cut–off (see Note)
    fn cut_off_hours(&self) -> usize {
        u16_be(&self.buf[14..16]) as usize
    }

    // Minutes after reference time of data cut–off
    fn cut_off_minutes(&self) -> usize {
        u8_be(&self.buf[16..17]) as usize
    }

    // 予報時間
    pub(crate) fn forecast_time_span(&self) -> usize {
        time_span_be(&self.buf[17..22])
    }

    // 第一固定面の種類
    fn first_plane_type(&self) -> usize {
        u8_be(&self.buf[22..23]) as usize
    }

    // 第一固定面の尺度因子
    fn first_plane_factor(&self) -> isize {
        i8_be(&self.buf[23..24]) as isize
    }

    // 第一固定面の尺度付きの値
    fn first_plane_value(&self) -> isize {
        i32_be(&self.buf[24..28]) as isize
    }

    // 第二固定面の種類
    fn second_plane_type(&self) -> usize {
        u8_be(&self.buf[28..29]) as usize
    }

    // 第二固定面の尺度因子
    fn second_plane_factor(&self) -> isize {
        i8_be(&self.buf[29..30]) as isize
    }

    // 第二固定面の尺度付きの値
    fn second_plane_value(&self) -> isize {
        i32_be(&self.buf[30..34]) as isize
    }

    // 全時間間隔の終了時刻
    fn end_time(&self) -> DateTime<Utc> {
        datetime_be(&self.buf[34..41])
    }

    // 統計処理の種類
    fn statistics_type(&self) -> usize {
        u8_be(&self.buf[46..47]) as usize
    }

    // 統計処理した期間の長さ
    fn statistics_time_span(&self) -> usize {
        time_span_be(&self.buf[48..53])
    }
}

impl<'a> Template for Template8<'a> {
    // パラメーターカテゴリー
    fn parameter_category(&self) -> usize {
        u8_be(&self.buf[9..10]) as usize
    }
    // パラメーター番号
    fn parameter_number(&self) -> usize {
        u8_be(&self.buf[10..11]) as usize
    }

    // データセットの時刻を返す。
    fn datetime(&self, reference_time: DateTime<Utc>) -> DateTime<Utc> {
        self.end_time()
    }
}

impl fmt::Display for Template8<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
parameter-category: {}\n\
parameter-number: {}\n\
generating type: {}\n\
background generating identifier: {}\n\
cut-off hours: {}\n\
cut-off minutes: {}\n\
forecast_time_span: {}\n\
first-plane-type: {}\n\
first-plane-factor: {}\n\
first-plane-value: {}\n\
second-plane-type: {}\n\
second-plane-factor: {}\n\
second-plane-value: {}\n\
end time: {}\n\
statistics type: {}\n\
statistics time span: {}\n\
",
            self.parameter_category(),
            self.parameter_number(),
            self.generating_type(),
            self.background_generating_identifier(),
            self.cut_off_hours(),
            self.cut_off_minutes(),
            self.forecast_time_span(),
            self.first_plane_type(),
            self.first_plane_factor(),
            self.first_plane_value(),
            self.second_plane_type(),
            self.second_plane_factor(),
            self.second_plane_value(),
            self.end_time(),
            self.statistics_type(),
            self.statistics_time_span(),
        )
    }
}
impl fmt::Debug for Template8<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
parameter-category: {}, \
parameter-number: {}, \
first-plane-type: {}, \
first-plane-factor: {}, \
first-plane-value: {}, \
second-plane-type: {}, \
second-plane-factor: {}, \
second-plane-value: {}, \
",
            self.parameter_category(),
            self.parameter_number(),
            self.first_plane_type(),
            self.first_plane_factor(),
            self.first_plane_value(),
            self.second_plane_type(),
            self.second_plane_factor(),
            self.second_plane_value(),
        )
    }
}

// template 4.9
impl<'a> Template9<'a> {
    // 作画処理の種類
    fn generating_type(&self) -> usize {
        u8_be(&self.buf[11..12]) as usize
    }

    // Background generating process identifier
    fn background_generating_identifier(&self) -> usize {
        u8_be(&self.buf[12..13]) as usize
    }

    // Hours after reference time of data cut–off (see Note)
    fn cut_off_hours(&self) -> usize {
        u16_be(&self.buf[14..16]) as usize
    }

    // Minutes after reference time of data cut–off
    fn cut_off_minutes(&self) -> usize {
        u8_be(&self.buf[16..17]) as usize
    }

    // 予報時間
    pub(crate) fn forecast_time_span(&self) -> usize {
        time_span_be(&self.buf[17..22])
    }

    // 第一固定面の種類
    fn first_plane_type(&self) -> usize {
        u8_be(&self.buf[22..23]) as usize
    }

    // 第一固定面の尺度因子
    fn first_plane_factor(&self) -> isize {
        i8_be(&self.buf[23..24]) as isize
    }

    // 第一固定面の尺度付きの値
    fn first_plane_value(&self) -> isize {
        i32_be(&self.buf[24..28]) as isize
    }

    // 第二固定面の種類
    fn second_plane_type(&self) -> usize {
        u8_be(&self.buf[28..29]) as usize
    }

    // 第二固定面の尺度因子
    fn second_plane_factor(&self) -> isize {
        i8_be(&self.buf[29..30]) as isize
    }

    // 第二固定面の尺度付きの値
    fn second_plane_value(&self) -> isize {
        i32_be(&self.buf[30..34]) as isize
    }

    // 全時間間隔の終了時刻
    fn end_time(&self) -> DateTime<Utc> {
        datetime_be(&self.buf[47..54])
    }

    // 統計処理の種類
    fn statistics_type(&self) -> usize {
        u8_be(&self.buf[59..60]) as usize
    }

    // 統計処理した期間の長さ
    fn statistics_time_span(&self) -> usize {
        time_span_be(&self.buf[61..66])
    }
}

impl<'a> Template for Template9<'a> {
    // パラメーターカテゴリー
    fn parameter_category(&self) -> usize {
        u8_be(&self.buf[9..10]) as usize
    }
    // パラメーター番号
    fn parameter_number(&self) -> usize {
        u8_be(&self.buf[10..11]) as usize
    }

    // データセットの時刻を返す。
    fn datetime(&self, reference_time: DateTime<Utc>) -> DateTime<Utc> {
        self.end_time()
    }
}

impl fmt::Display for Template9<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
parameter-category: {}\n\
parameter-number: {}\n\
generating type: {}\n\
background generating identifier: {}\n\
cut-off hours: {}\n\
cut-off minutes: {}\n\
forecast_time_span: {}\n\
first-plane-type: {}\n\
first-plane-factor: {}\n\
first-plane-value: {}\n\
second-plane-type: {}\n\
second-plane-factor: {}\n\
second-plane-value: {}\n\
end time: {}\n\
statistics type: {}\n\
statistics time span: {}\n\
",
            self.parameter_category(),
            self.parameter_number(),
            self.generating_type(),
            self.background_generating_identifier(),
            self.cut_off_hours(),
            self.cut_off_minutes(),
            self.forecast_time_span(),
            self.first_plane_type(),
            self.first_plane_factor(),
            self.first_plane_value(),
            self.second_plane_type(),
            self.second_plane_factor(),
            self.second_plane_value(),
            self.end_time(),
            self.statistics_type(),
            self.statistics_time_span(),
        )
    }
}
impl fmt::Debug for Template9<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
parameter-category: {}, \
parameter-number: {}, \
first-plane-type: {}, \
first-plane-factor: {}, \
first-plane-value: {}, \
second-plane-type: {}, \
second-plane-factor: {}, \
second-plane-value: {}, \
",
            self.parameter_category(),
            self.parameter_number(),
            self.first_plane_type(),
            self.first_plane_factor(),
            self.first_plane_value(),
            self.second_plane_type(),
            self.second_plane_factor(),
            self.second_plane_value(),
        )
    }
}

// template 4.11
impl<'a> Template11<'a> {
    // 作画処理の種類
    fn generating_type(&self) -> usize {
        u8_be(&self.buf[11..12]) as usize
    }

    // Background generating process identifier
    fn background_generating_identifier(&self) -> usize {
        u8_be(&self.buf[12..13]) as usize
    }

    // Hours after reference time of data cut–off (see Note)
    fn cut_off_hours(&self) -> usize {
        u16_be(&self.buf[14..16]) as usize
    }

    // Minutes after reference time of data cut–off
    fn cut_off_minutes(&self) -> usize {
        u8_be(&self.buf[16..17]) as usize
    }

    // 予報時間
    pub(crate) fn forecast_time_span(&self) -> usize {
        time_span_be(&self.buf[17..22])
    }

    // 第一固定面の種類
    fn first_plane_type(&self) -> usize {
        u8_be(&self.buf[22..23]) as usize
    }

    // 第一固定面の尺度因子
    fn first_plane_factor(&self) -> isize {
        i8_be(&self.buf[23..24]) as isize
    }

    // 第一固定面の尺度付きの値
    fn first_plane_value(&self) -> isize {
        i32_be(&self.buf[24..28]) as isize
    }

    // 第二固定面の種類
    fn second_plane_type(&self) -> usize {
        u8_be(&self.buf[28..29]) as usize
    }

    // 第二固定面の尺度因子
    fn second_plane_factor(&self) -> isize {
        i8_be(&self.buf[29..30]) as isize
    }

    // 第二固定面の尺度付きの値
    fn second_plane_value(&self) -> isize {
        i32_be(&self.buf[30..34]) as isize
    }

    // 全時間間隔の終了時刻
    fn end_time(&self) -> DateTime<Utc> {
        datetime_be(&self.buf[37..44])
    }

    // 統計処理の種類
    fn statistics_type(&self) -> usize {
        u8_be(&self.buf[49..50]) as usize
    }

    // 統計処理した期間の長さ
    fn statistics_time_span(&self) -> usize {
        time_span_be(&self.buf[51..56])
    }
}

impl<'a> Template for Template11<'a> {
    // パラメーターカテゴリー
    fn parameter_category(&self) -> usize {
        u8_be(&self.buf[9..10]) as usize
    }
    // パラメーター番号
    fn parameter_number(&self) -> usize {
        u8_be(&self.buf[10..11]) as usize
    }

    // データセットの時刻を返す。
    fn datetime(&self, reference_time: DateTime<Utc>) -> DateTime<Utc> {
        self.end_time()
    }
}

impl fmt::Display for Template11<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
parameter-category: {}\n\
parameter-number: {}\n\
generating type: {}\n\
background generating identifier: {}\n\
cut-off hours: {}\n\
cut-off minutes: {}\n\
forecast_time_span: {}\n\
first-plane-type: {}\n\
first-plane-factor: {}\n\
first-plane-value: {}\n\
second-plane-type: {}\n\
second-plane-factor: {}\n\
second-plane-value: {}\n\
end time: {}\n\
statistics type: {}\n\
statistics time span: {}\n\
",
            self.parameter_category(),
            self.parameter_number(),
            self.generating_type(),
            self.background_generating_identifier(),
            self.cut_off_hours(),
            self.cut_off_minutes(),
            self.forecast_time_span(),
            self.first_plane_type(),
            self.first_plane_factor(),
            self.first_plane_value(),
            self.second_plane_type(),
            self.second_plane_factor(),
            self.second_plane_value(),
            self.end_time(),
            self.statistics_type(),
            self.statistics_time_span(),
        )
    }
}
impl fmt::Debug for Template11<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
parameter-category: {}, \
parameter-number: {}, \
first-plane-type: {}, \
first-plane-factor: {}, \
first-plane-value: {}, \
second-plane-type: {}, \
second-plane-factor: {}, \
second-plane-value: {}, \
",
            self.parameter_category(),
            self.parameter_number(),
            self.first_plane_type(),
            self.first_plane_factor(),
            self.first_plane_value(),
            self.second_plane_type(),
            self.second_plane_factor(),
            self.second_plane_value(),
        )
    }
}

// template 4.50000
impl<'a> Template50000<'a> {
    // 作画処理の種類
    fn generating_type(&self) -> usize {
        u8_be(&self.buf[11..12]) as usize
    }

    // Background generating process identifier
    fn background_generating_identifier(&self) -> usize {
        u8_be(&self.buf[12..13]) as usize
    }

    // Hours after reference time of data cut–off (see Note)
    fn cut_off_hours(&self) -> usize {
        u16_be(&self.buf[14..16]) as usize
    }

    // Minutes after reference time of data cut–off
    fn cut_off_minutes(&self) -> usize {
        u8_be(&self.buf[16..17]) as usize
    }

    // 予報時間
    fn forecast_time_span(&self) -> usize {
        time_span_be(&self.buf[17..22])
    }

    // 第一固定面の種類
    fn first_plane_type(&self) -> usize {
        u8_be(&self.buf[22..23]) as usize
    }

    // 第一固定面の尺度因子
    fn first_plane_factor(&self) -> isize {
        i8_be(&self.buf[23..24]) as isize
    }

    // 第一固定面の尺度付きの値
    fn first_plane_value(&self) -> isize {
        i32_be(&self.buf[24..28]) as isize
    }

    // 第二固定面の種類
    fn second_plane_type(&self) -> usize {
        u8_be(&self.buf[28..29]) as usize
    }

    // 第二固定面の尺度因子
    fn second_plane_factor(&self) -> isize {
        i8_be(&self.buf[29..30]) as isize
    }

    // 第二固定面の尺度付きの値
    fn second_plane_value(&self) -> isize {
        i32_be(&self.buf[30..34]) as isize
    }
}

impl<'a> Template for Template50000<'a> {
    // パラメーターカテゴリー
    fn parameter_category(&self) -> usize {
        u8_be(&self.buf[9..10]) as usize
    }
    // パラメーター番号
    fn parameter_number(&self) -> usize {
        u8_be(&self.buf[10..11]) as usize
    }

    // データセットの時刻を返す。
    fn datetime(&self, reference_time: DateTime<Utc>) -> DateTime<Utc> {
        reference_time + Duration::seconds(self.forecast_time_span() as i64)
    }
}

impl fmt::Display for Template50000<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
parameter-category: {}\n\
parameter-number: {}\n\
generating type: {}\n\
background generating identifier: {}\n\
cut-off hours: {}\n\
cut-off minutes: {}\n\
forecast time span: {}\n\
first-plane-type: {}\n\
first-plane-factor: {}\n\
first-plane-value: {}\n\
second-plane-type: {}\n\
second-plane-factor: {}\n\
second-plane-value: {}\n\
",
            self.parameter_category(),
            self.parameter_number(),
            self.generating_type(),
            self.background_generating_identifier(),
            self.cut_off_hours(),
            self.cut_off_minutes(),
            self.forecast_time_span(),
            self.first_plane_type(),
            self.first_plane_factor(),
            self.first_plane_value(),
            self.second_plane_type(),
            self.second_plane_factor(),
            self.second_plane_value(),
        )
    }
}

impl fmt::Debug for Template50000<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
parameter-category: {}, \
parameter-number: {}, \
first-plane-type: {}, \
first-plane-factor: {}, \
first-plane-value: {}, \
second-plane-type: {}, \
second-plane-factor: {}, \
second-plane-value: {}, \
",
            self.parameter_category(),
            self.parameter_number(),
            self.first_plane_type(),
            self.first_plane_factor(),
            self.first_plane_value(),
            self.second_plane_type(),
            self.second_plane_factor(),
            self.second_plane_value(),
        )
    }
}

// template 4.50008
impl<'a> Template50008<'a> {
    // 作画処理の種類
    fn generating_type(&self) -> usize {
        u8_be(&self.buf[11..12]) as usize
    }

    // Background generating process identifier
    fn background_generating_identifier(&self) -> usize {
        u8_be(&self.buf[12..13]) as usize
    }

    // Hours after reference time of data cut–off (see Note)
    fn cut_off_hours(&self) -> usize {
        u16_be(&self.buf[14..16]) as usize
    }

    // Minutes after reference time of data cut–off
    fn cut_off_minutes(&self) -> usize {
        u8_be(&self.buf[16..17]) as usize
    }

    // 予報時間
    fn forecast_time_span(&self) -> usize {
        time_span_be(&self.buf[17..22])
    }

    // 第一固定面の種類
    fn first_plane_type(&self) -> usize {
        u8_be(&self.buf[22..23]) as usize
    }

    // 第一固定面の尺度因子
    fn first_plane_factor(&self) -> isize {
        i8_be(&self.buf[23..24]) as isize
    }

    // 第一固定面の尺度付きの値
    fn first_plane_value(&self) -> isize {
        i32_be(&self.buf[24..28]) as isize
    }

    // 第二固定面の種類
    fn second_plane_type(&self) -> usize {
        u8_be(&self.buf[28..29]) as usize
    }

    // 第二固定面の尺度因子
    fn second_plane_factor(&self) -> isize {
        i8_be(&self.buf[29..30]) as isize
    }

    // 第二固定面の尺度付きの値
    fn second_plane_value(&self) -> isize {
        i32_be(&self.buf[30..34]) as isize
    }

    // 全時間間隔の終了時刻
    fn end_time(&self) -> DateTime<Utc> {
        datetime_be(&self.buf[34..41])
    }

    // 統計処理の種類
    fn statistics_type(&self) -> usize {
        u8_be(&self.buf[46..47]) as usize
    }

    // 統計処理した期間の長さ
    fn statistics_time_span(&self) -> usize {
        time_span_be(&self.buf[48..53])
    }
}

impl<'a> Template for Template50008<'a> {
    // パラメーターカテゴリー
    fn parameter_category(&self) -> usize {
        u8_be(&self.buf[9..10]) as usize
    }
    // パラメーター番号
    fn parameter_number(&self) -> usize {
        u8_be(&self.buf[10..11]) as usize
    }

    // データセットの時刻を返す。
    fn datetime(&self, reference_time: DateTime<Utc>) -> DateTime<Utc> {
        self.end_time()
    }
}

impl fmt::Display for Template50008<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
parameter-category: {}\n\
parameter-number: {}\n\
generating type: {}\n\
background generating identifier: {}\n\
cut-off hours: {}\n\
cut-off minutes: {}\n\
forecast time span: {}\n\
first-plane-type: {}\n\
first-plane-factor: {}\n\
first-plane-value: {}\n\
second-plane-type: {}\n\
second-plane-factor: {}\n\
second-plane-value: {}\n\
end time: {}\n\
statistics type: {}\n\
statistics time span: {}\n\
",
            self.parameter_category(),
            self.parameter_number(),
            self.generating_type(),
            self.background_generating_identifier(),
            self.cut_off_hours(),
            self.cut_off_minutes(),
            self.forecast_time_span(),
            self.first_plane_type(),
            self.first_plane_factor(),
            self.first_plane_value(),
            self.second_plane_type(),
            self.second_plane_factor(),
            self.second_plane_value(),
            self.end_time(),
            self.statistics_type(),
            self.statistics_time_span(),
        )
    }
}

impl fmt::Debug for Template50008<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
parameter-category: {}, \
parameter-number: {}, \
first-plane-type: {}, \
first-plane-factor: {}, \
first-plane-value: {}, \
second-plane-type: {}, \
second-plane-factor: {}, \
second-plane-value: {}, \
",
            self.parameter_category(),
            self.parameter_number(),
            self.first_plane_type(),
            self.first_plane_factor(),
            self.first_plane_value(),
            self.second_plane_type(),
            self.second_plane_factor(),
            self.second_plane_value(),
        )
    }
}

// template 4.50009
impl<'a> Template50009<'a> {
    // 作画処理の種類
    fn generating_type(&self) -> usize {
        u8_be(&self.buf[11..12]) as usize
    }

    // Background generating process identifier
    fn background_generating_identifier(&self) -> usize {
        u8_be(&self.buf[12..13]) as usize
    }

    // Hours after reference time of data cut–off (see Note)
    fn cut_off_hours(&self) -> usize {
        u16_be(&self.buf[14..16]) as usize
    }

    // Minutes after reference time of data cut–off
    fn cut_off_minutes(&self) -> usize {
        u8_be(&self.buf[16..17]) as usize
    }

    // 予報時間
    fn forecast_time_span(&self) -> usize {
        time_span_be(&self.buf[17..22])
    }

    // 第一固定面の種類
    fn first_plane_type(&self) -> usize {
        u8_be(&self.buf[22..23]) as usize
    }

    // 第一固定面の尺度因子
    fn first_plane_factor(&self) -> isize {
        i8_be(&self.buf[23..24]) as isize
    }

    // 第一固定面の尺度付きの値
    fn first_plane_value(&self) -> isize {
        i32_be(&self.buf[24..28]) as isize
    }

    // 第二固定面の種類
    fn second_plane_type(&self) -> usize {
        u8_be(&self.buf[28..29]) as usize
    }

    // 第二固定面の尺度因子
    fn second_plane_factor(&self) -> isize {
        i8_be(&self.buf[29..30]) as isize
    }

    // 第二固定面の尺度付きの値
    fn second_plane_value(&self) -> isize {
        i32_be(&self.buf[30..34]) as isize
    }

    // 全時間間隔の終了時刻
    fn end_time(&self) -> DateTime<Utc> {
        datetime_be(&self.buf[34..41])
    }

    // 統計処理の種類
    fn statistics_type(&self) -> usize {
        u8_be(&self.buf[46..47]) as usize
    }

    // 統計処理した期間の長さ
    fn statistics_time_span(&self) -> usize {
        time_span_be(&self.buf[48..53])
    }
}

impl<'a> Template for Template50009<'a> {
    // パラメーターカテゴリー
    fn parameter_category(&self) -> usize {
        u8_be(&self.buf[9..10]) as usize
    }
    // パラメーター番号
    fn parameter_number(&self) -> usize {
        u8_be(&self.buf[10..11]) as usize
    }

    // データセットの時刻を返す。
    fn datetime(&self, reference_time: DateTime<Utc>) -> DateTime<Utc> {
        self.end_time()
    }
}

impl fmt::Display for Template50009<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
parameter-category: {}\n\
parameter-number: {}\n\
generating type: {}\n\
background generating identifier: {}\n\
cut-off hours: {}\n\
cut-off minutes: {}\n\
forecast time span: {}\n\
first-plane-type: {}\n\
first-plane-factor: {}\n\
first-plane-value: {}\n\
second-plane-type: {}\n\
second-plane-factor: {}\n\
second-plane-value: {}\n\
end time: {}\n\
statistics type: {}\n\
statistics time span: {}\n\
",
            self.parameter_category(),
            self.parameter_number(),
            self.generating_type(),
            self.background_generating_identifier(),
            self.cut_off_hours(),
            self.cut_off_minutes(),
            self.forecast_time_span(),
            self.first_plane_type(),
            self.first_plane_factor(),
            self.first_plane_value(),
            self.second_plane_type(),
            self.second_plane_factor(),
            self.second_plane_value(),
            self.end_time(),
            self.statistics_type(),
            self.statistics_time_span(),
        )
    }
}

impl fmt::Debug for Template50009<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
parameter-category: {}, \
parameter-number: {}, \
first-plane-type: {}, \
first-plane-factor: {}, \
first-plane-value: {}, \
second-plane-type: {}, \
second-plane-factor: {}, \
second-plane-value: {}, \
",
            self.parameter_category(),
            self.parameter_number(),
            self.first_plane_type(),
            self.first_plane_factor(),
            self.first_plane_value(),
            self.second_plane_type(),
            self.second_plane_factor(),
            self.second_plane_value(),
        )
    }
}

// template 4.50011
impl<'a> Template50011<'a> {
    // 作画処理の種類
    fn generating_type(&self) -> usize {
        u8_be(&self.buf[11..12]) as usize
    }

    // Background generating process identifier
    fn background_generating_identifier(&self) -> usize {
        u8_be(&self.buf[12..13]) as usize
    }

    // Hours after reference time of data cut–off (see Note)
    fn cut_off_hours(&self) -> usize {
        u16_be(&self.buf[14..16]) as usize
    }

    // Minutes after reference time of data cut–off
    fn cut_off_minutes(&self) -> usize {
        u8_be(&self.buf[16..17]) as usize
    }

    // 予報時間
    pub(crate) fn forecast_time_span(&self) -> usize {
        time_span_be(&self.buf[17..22])
    }

    // 第一固定面の種類
    fn first_plane_type(&self) -> usize {
        u8_be(&self.buf[22..23]) as usize
    }

    // 第一固定面の尺度因子
    fn first_plane_factor(&self) -> isize {
        i8_be(&self.buf[23..24]) as isize
    }

    // 第一固定面の尺度付きの値
    fn first_plane_value(&self) -> isize {
        i32_be(&self.buf[24..28]) as isize
    }

    // 第二固定面の種類
    fn second_plane_type(&self) -> usize {
        u8_be(&self.buf[28..29]) as usize
    }

    // 第二固定面の尺度因子
    fn second_plane_factor(&self) -> isize {
        i8_be(&self.buf[29..30]) as isize
    }

    // 第二固定面の尺度付きの値
    fn second_plane_value(&self) -> isize {
        i32_be(&self.buf[30..34]) as isize
    }

    // 全時間間隔の終了時刻
    fn end_time(&self) -> DateTime<Utc> {
        datetime_be(&self.buf[34..41])
    }

    // 統計処理の種類
    fn statistics_type(&self) -> usize {
        u8_be(&self.buf[46..47]) as usize
    }

    // 統計処理した期間の長さ
    fn statistics_time_span(&self) -> usize {
        time_span_be(&self.buf[48..53])
    }
}

impl<'a> Template for Template50011<'a> {
    // パラメーターカテゴリー
    fn parameter_category(&self) -> usize {
        u8_be(&self.buf[9..10]) as usize
    }
    // パラメーター番号
    fn parameter_number(&self) -> usize {
        u8_be(&self.buf[10..11]) as usize
    }
    // データセットの時刻を返す。
    fn datetime(&self, reference_time: DateTime<Utc>) -> DateTime<Utc> {
        self.end_time()
    }
}

impl fmt::Display for Template50011<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
parameter-category: {}\n\
parameter-number: {}\n\
generating type: {}\n\
background generating identifier: {}\n\
cut-off hours: {}\n\
cut-off minutes: {}\n\
forecast time span: {}\n\
first-plane-type: {}\n\
first-plane-factor: {}\n\
first-plane-value: {}\n\
second-plane-type: {}\n\
second-plane-factor: {}\n\
second-plane-value: {}\n\
end time: {}\n\
statistics type: {}\n\
statistics time span: {}\n\
",
            self.parameter_category(),
            self.parameter_number(),
            self.generating_type(),
            self.background_generating_identifier(),
            self.cut_off_hours(),
            self.cut_off_minutes(),
            self.forecast_time_span(),
            self.first_plane_type(),
            self.first_plane_factor(),
            self.first_plane_value(),
            self.second_plane_type(),
            self.second_plane_factor(),
            self.second_plane_value(),
            self.end_time(),
            self.statistics_type(),
            self.statistics_time_span(),
        )
    }
}
impl fmt::Debug for Template50011<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
parameter-category: {}, \
parameter-number: {}, \
first-plane-type: {}, \
first-plane-factor: {}, \
first-plane-value: {}, \
second-plane-type: {}, \
second-plane-factor: {}, \
second-plane-value: {}, \
",
            self.parameter_category(),
            self.parameter_number(),
            self.first_plane_type(),
            self.first_plane_factor(),
            self.first_plane_value(),
            self.second_plane_type(),
            self.second_plane_factor(),
            self.second_plane_value(),
        )
    }
}

// template 4.50012
impl<'a> Template50012<'a> {
    // 作画処理の種類
    fn generating_type(&self) -> usize {
        u8_be(&self.buf[11..12]) as usize
    }

    // Background generating process identifier
    fn background_generating_identifier(&self) -> usize {
        u8_be(&self.buf[12..13]) as usize
    }

    // Hours after reference time of data cut–off (see Note)
    fn cut_off_hours(&self) -> usize {
        u16_be(&self.buf[14..16]) as usize
    }

    // Minutes after reference time of data cut–off
    fn cut_off_minutes(&self) -> usize {
        u8_be(&self.buf[16..17]) as usize
    }

    // 予報時間
    pub(crate) fn forecast_time_span(&self) -> usize {
        time_span_be(&self.buf[17..22])
    }

    // 第一固定面の種類
    fn first_plane_type(&self) -> usize {
        u8_be(&self.buf[22..23]) as usize
    }

    // 第一固定面の尺度因子
    fn first_plane_factor(&self) -> isize {
        i8_be(&self.buf[23..24]) as isize
    }

    // 第一固定面の尺度付きの値
    fn first_plane_value(&self) -> isize {
        i32_be(&self.buf[24..28]) as isize
    }

    // 第二固定面の種類
    fn second_plane_type(&self) -> usize {
        u8_be(&self.buf[28..29]) as usize
    }

    // 第二固定面の尺度因子
    fn second_plane_factor(&self) -> isize {
        i8_be(&self.buf[29..30]) as isize
    }

    // 第二固定面の尺度付きの値
    fn second_plane_value(&self) -> isize {
        i32_be(&self.buf[30..34]) as isize
    }

    // 全時間間隔の終了時刻
    fn end_time(&self) -> DateTime<Utc> {
        datetime_be(&self.buf[34..41])
    }

    // 統計処理の種類
    fn statistics_type(&self) -> usize {
        u8_be(&self.buf[46..47]) as usize
    }

    // 統計処理した期間の長さ
    fn statistics_time_span(&self) -> usize {
        time_span_be(&self.buf[48..53])
    }
}

impl<'a> Template for Template50012<'a> {
    // パラメーターカテゴリー
    fn parameter_category(&self) -> usize {
        u8_be(&self.buf[9..10]) as usize
    }
    // パラメーター番号
    fn parameter_number(&self) -> usize {
        u8_be(&self.buf[10..11]) as usize
    }
    // データセットの時刻を返す。
    fn datetime(&self, reference_time: DateTime<Utc>) -> DateTime<Utc> {
        self.end_time()
    }
}

impl fmt::Display for Template50012<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
parameter-category: {}\n\
parameter-number: {}\n\
generating type: {}\n\
background generating identifier: {}\n\
cut-off hours: {}\n\
cut-off minutes: {}\n\
forecast time span: {}\n\
first-plane-type: {}\n\
first-plane-factor: {}\n\
first-plane-value: {}\n\
second-plane-type: {}\n\
second-plane-factor: {}\n\
second-plane-value: {}\n\
end time: {}\n\
statistics type: {}\n\
statistics time span: {}\n\
",
            self.parameter_category(),
            self.parameter_number(),
            self.generating_type(),
            self.background_generating_identifier(),
            self.cut_off_hours(),
            self.cut_off_minutes(),
            self.forecast_time_span(),
            self.first_plane_type(),
            self.first_plane_factor(),
            self.first_plane_value(),
            self.second_plane_type(),
            self.second_plane_factor(),
            self.second_plane_value(),
            self.end_time(),
            self.statistics_type(),
            self.statistics_time_span(),
        )
    }
}
impl fmt::Debug for Template50012<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
parameter-category: {}, \
parameter-number: {}, \
first-plane-type: {}, \
first-plane-factor: {}, \
first-plane-value: {}, \
second-plane-type: {}, \
second-plane-factor: {}, \
second-plane-value: {}, \
",
            self.parameter_category(),
            self.parameter_number(),
            self.first_plane_type(),
            self.first_plane_factor(),
            self.first_plane_value(),
            self.second_plane_type(),
            self.second_plane_factor(),
            self.second_plane_value(),
        )
    }
}

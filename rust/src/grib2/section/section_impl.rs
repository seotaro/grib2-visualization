//! GRIB2 Section implementation

use super::IsSection7DataType;

impl IsSection7DataType for u8 {}
impl IsSection7DataType for u16 {}
impl IsSection7DataType for u32 {}

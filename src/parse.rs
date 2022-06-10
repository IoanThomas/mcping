use crate::{result, Error};

pub fn i32_to_usize(value: i32) -> result::Result<usize> {
    usize::try_from(value).map_err(|_| Error::IntConversion)
}

pub fn usize_to_i32(value: usize) -> result::Result<i32> {
    i32::try_from(value).map_err(|_| Error::IntConversion)
}

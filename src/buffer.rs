use crate::{result, Error};
use mc_varint::{VarInt, VarIntWrite};
use std::io::Write;

pub fn write_u16(buffer: &mut Vec<u8>, data: u16) -> result::Result<()> {
    buffer
        .write_all(&data.to_be_bytes())
        .map_err(|_| Error::BufferWrite)
}

pub fn write_var_int(buffer: &mut Vec<u8>, data: impl Into<VarInt>) -> result::Result<()> {
    buffer
        .write_var_int(data.into())
        .map_err(|_| Error::BufferWrite)
}

pub fn write_string(buffer: &mut Vec<u8>, data: impl AsRef<str>) -> result::Result<()> {
    let data = data.as_ref();
    let length = i32::try_from(data.len()).map_err(|_| Error::IntConversion)?;

    write_var_int(buffer, length)?;

    buffer
        .write_all(data.as_bytes())
        .map_err(|_| Error::BufferWrite)
}

pub fn write_bytes(buffer: &mut Vec<u8>, data: &[u8]) -> result::Result<()> {
    buffer.write_all(data).map_err(|_| Error::BufferWrite)
}

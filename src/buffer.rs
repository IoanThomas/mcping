use crate::{parse, result, Error};
use mc_varint::{VarInt, VarIntWrite};
use std::io::Write;

pub fn write_u16(buffer: &mut Vec<u8>, data: u16) -> result::Result<()> {
    buffer
        .write_all(&data.to_be_bytes())
        .map_err(Error::BufferWrite)
}

pub fn write_var_int(buffer: &mut Vec<u8>, data: impl Into<VarInt>) -> result::Result<()> {
    buffer
        .write_var_int(data.into())
        .map_err(Error::BufferWrite)
}

pub fn write_string(buffer: &mut Vec<u8>, data: impl AsRef<str>) -> result::Result<()> {
    let data = data.as_ref();
    let length = parse::usize_to_i32(data.len())?;

    write_var_int(buffer, length)?;

    buffer
        .write_all(data.as_bytes())
        .map_err(Error::BufferWrite)
}

pub fn write_bytes(buffer: &mut Vec<u8>, data: &[u8]) -> result::Result<()> {
    buffer.write_all(data).map_err(Error::BufferWrite)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_u16_one_byte() {
        let mut buffer = vec![];
        write_u16(&mut buffer, 0x002d).expect("failed to write u16");

        assert_eq!(buffer[0], 0x00);
        assert_eq!(buffer[1], 0x2d);
    }

    #[test]
    fn write_u16_two_bytes() {
        let mut buffer = vec![];
        write_u16(&mut buffer, 0x010e).expect("failed to write u16");

        assert_eq!(buffer[0], 0x01);
        assert_eq!(buffer[1], 0x0e);
    }

    #[test]
    fn write_var_int_small() {
        let mut buffer = vec![];
        write_var_int(&mut buffer, 0x8a).expect("failed to write var int");

        assert_eq!(buffer[0], 0x8a);
    }

    #[test]
    fn write_var_int_large() {
        let mut buffer = vec![];
        write_var_int(&mut buffer, 0x38a).expect("failed to write var int");

        assert_eq!(buffer[0], 0x8a);
        assert_eq!(buffer[1], 0x07);
    }

    #[test]
    fn write_string_empty() {
        let mut buffer = vec![];
        write_string(&mut buffer, "").expect("failed to write string");

        assert_eq!(buffer[0], 0);
        assert_eq!(buffer.get(1), None);
    }

    #[test]
    fn write_string_short() {
        let mut buffer = vec![];
        write_string(&mut buffer, "hello").expect("failed to write string");

        assert_eq!(buffer[0], 5);
        assert_eq!(buffer[1], 104);
        assert_eq!(buffer[5], 111);
    }

    #[test]
    fn write_byte_array() {
        let mut buffer = vec![];
        write_bytes(&mut buffer, &[0x12, 0x34, 0x56, 0x78]).expect("failed to write string");

        assert_eq!(buffer[0], 0x12);
        assert_eq!(buffer[1], 0x34);
        assert_eq!(buffer[2], 0x56);
        assert_eq!(buffer[3], 0x78);
    }
}

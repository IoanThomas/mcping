use crate::error::Error;
use crate::result::Result;
use mc_varint::{VarInt, VarIntRead};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::num::NonZeroU16;

pub fn create(address: impl AsRef<str>, port: NonZeroU16) -> Result<TcpStream> {
    TcpStream::connect((address.as_ref(), port.get())).map_err(Error::Connect)
}

pub fn send_data(connection: &mut TcpStream, data: &[u8]) -> Result<()> {
    connection.write_all(data).map_err(Error::ConnectionWrite)
}

pub fn receive_var_int(connection: &mut TcpStream) -> Result<VarInt> {
    connection.read_var_int().map_err(Error::ConnectionRead)
}

pub fn receive_bytes(connection: &mut TcpStream, count: usize) -> Result<Vec<u8>> {
    let mut json_bytes = vec![0; count];
    connection
        .read_exact(&mut json_bytes)
        .map_err(Error::ConnectionRead)?;

    Ok(json_bytes)
}

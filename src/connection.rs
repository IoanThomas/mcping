use crate::{result, Error};
use mc_varint::{VarInt, VarIntRead};
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn create(address: impl AsRef<str>, port: u16) -> result::Result<TcpStream> {
    TcpStream::connect((address.as_ref(), port)).map_err(|_| Error::Connect)
}

pub fn send_data(connection: &mut TcpStream, data: &[u8]) -> result::Result<()> {
    connection
        .write_all(data)
        .map_err(|_| Error::ConnectionWrite)
}

pub fn receive_var_int(connection: &mut TcpStream) -> result::Result<VarInt> {
    connection.read_var_int().map_err(|_| Error::ConnectionRead)
}

pub fn receive_bytes(connection: &mut TcpStream, count: usize) -> result::Result<Vec<u8>> {
    let mut json_bytes = vec![0; count];
    connection
        .read(&mut json_bytes)
        .map_err(|_| Error::ConnectionRead)?;

    Ok(json_bytes)
}

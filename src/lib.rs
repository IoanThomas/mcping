pub use crate::error::Error;
pub use crate::result::Result;

use crate::response::Response;
use mc_varint::VarInt;
use std::num::NonZeroU16;

pub mod response;

mod buffer;
mod connection;
mod error;
mod parse;
mod result;

pub fn get_server_response(address: impl AsRef<str>, port: NonZeroU16) -> Result<Response> {
    let json_bytes = get_server_response_raw(address, port)?;

    create_response_from_json_bytes(&json_bytes)
}

pub fn get_server_response_json(address: impl AsRef<str>, port: NonZeroU16) -> Result<String> {
    let json_bytes = get_server_response_raw(address, port)?;

    String::from_utf8(json_bytes).map_err(Error::Utf8Parse)
}

fn get_server_response_raw(address: impl AsRef<str>, port: NonZeroU16) -> Result<Vec<u8>> {
    let address = address.as_ref();

    let handshake_packet = create_handshake_packet(address, port)?;
    let request_packet = create_request_packet()?;

    let mut connection = connection::create(address, port)?;

    connection::send_data(&mut connection, &handshake_packet)?;
    connection::send_data(&mut connection, &request_packet)?;

    let _packet_length = connection::receive_var_int(&mut connection)?;
    let _packet_id = connection::receive_var_int(&mut connection)?;
    let response_length = connection::receive_var_int(&mut connection)?;

    let response_length = parse::i32_to_usize(response_length.into())?;
    connection::receive_bytes(&mut connection, response_length)
}

fn create_handshake_packet(address: impl AsRef<str>, port: NonZeroU16) -> Result<Vec<u8>> {
    const HANDSHAKE_PACKET_ID: i32 = 0;
    const UNKNOWN_PROTOCOL_VERSION: i32 = -1;
    const STATUS_STATE: i32 = 1;

    let data = create_handshake_data(UNKNOWN_PROTOCOL_VERSION, address, port, STATUS_STATE)?;
    create_packet(HANDSHAKE_PACKET_ID, &data)
}

fn create_request_packet() -> result::Result<Vec<u8>> {
    const REQUEST_PACKET_ID: i32 = 0;

    create_packet(REQUEST_PACKET_ID, &[])
}

fn create_handshake_data(
    protocol_version: impl Into<VarInt>,
    server_address: impl AsRef<str>,
    server_port: NonZeroU16,
    next_state: impl Into<VarInt>,
) -> Result<Vec<u8>> {
    let mut bytes = vec![];

    buffer::write_var_int(&mut bytes, protocol_version)?;
    buffer::write_string(&mut bytes, server_address)?;
    buffer::write_u16(&mut bytes, server_port.get())?;
    buffer::write_var_int(&mut bytes, next_state)?;

    Ok(bytes)
}

fn create_packet(id: impl Into<VarInt>, data: &[u8]) -> Result<Vec<u8>> {
    let mut id_bytes = vec![];
    buffer::write_var_int(&mut id_bytes, id)?;

    let packet_length = parse::usize_to_i32(id_bytes.len() + data.len())?;
    let mut bytes = vec![];

    buffer::write_var_int(&mut bytes, packet_length)?;
    buffer::write_bytes(&mut bytes, &id_bytes)?;
    buffer::write_bytes(&mut bytes, data)?;

    Ok(bytes)
}

fn create_response_from_json_bytes(json_bytes: &[u8]) -> Result<Response> {
    serde_json::from_slice(json_bytes).map_err(Error::JsonParse)
}

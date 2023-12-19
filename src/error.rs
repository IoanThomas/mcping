use std::fmt::{Display, Formatter};
use std::num::TryFromIntError;
use std::{error, fmt, io};

#[derive(Debug)]
pub enum Error {
    BufferWrite(io::Error),
    Connect(io::Error),
    ConnectionWrite(io::Error),
    ConnectionRead(io::Error),
    JsonParse(serde_json::Error),
    IntConversion(TryFromIntError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::BufferWrite(error) => write!(f, "failed to write to buffer: {error}"),
            Self::Connect(error) => write!(f, "failed to connect to server: {error}"),
            Self::ConnectionWrite(error) => write!(f, "failed to send data to server: {error}"),
            Self::ConnectionRead(error) => write!(f, "failed to receive data from server: {error}"),
            Self::JsonParse(error) => write!(f, "failed to parse JSON response: {error}"),
            Self::IntConversion(error) => write!(f, "failed to convert integer: {error}"),
        }
    }
}

impl error::Error for Error {}

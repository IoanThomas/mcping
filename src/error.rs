use std::fmt::{Display, Formatter};
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    BufferWrite,
    Connect,
    ConnectionWrite,
    ConnectionRead,
    JsonParse,
    IntConversion,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::BufferWrite => write!(f, "failed to write to buffer"),
            Self::Connect => write!(f, "failed to connect to server"),
            Self::ConnectionWrite => write!(f, "failed to send data to server"),
            Self::ConnectionRead => write!(f, "failed to receive data from server"),
            Self::JsonParse => write!(f, "failed to parse json response"),
            Self::IntConversion => write!(f, "failed to convert between integer types"),
        }
    }
}

impl error::Error for Error {}

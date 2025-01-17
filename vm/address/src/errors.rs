// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use super::{BLS_PUB_LEN, PAYLOAD_HASH_LEN};
use data_encoding::DecodeError;
use std::{fmt, io, num};

#[derive(Debug, PartialEq)]
pub enum Error {
    UnknownNetwork,
    UnknownProtocol,
    InvalidPayload,
    InvalidLength,
    InvalidPayloadLength(usize),
    InvalidBLSLength(usize),
    InvalidChecksum,
    Base32Decoding(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnknownNetwork => write!(f, "Unknown address network"),
            Error::UnknownProtocol => write!(f, "Unknown address protocol"),
            Error::InvalidPayload => write!(f, "Invalid address payload"),
            Error::InvalidLength => write!(f, "Invalid address length"),
            Error::InvalidPayloadLength(len) => write!(
                f,
                "Invalid payload length, wanted: {} got: {}",
                PAYLOAD_HASH_LEN, len
            ),
            Error::InvalidBLSLength(len) => write!(
                f,
                "Invalid BLS pub key length, wanted: {} got: {}",
                BLS_PUB_LEN, len
            ),
            Error::InvalidChecksum => write!(f, "Invalid address checksum"),
            Error::Base32Decoding(err) => write!(f, "Decoding error: {}", err),
        }
    }
}

impl From<DecodeError> for Error {
    fn from(e: DecodeError) -> Error {
        Error::Base32Decoding(e.to_string())
    }
}

impl From<num::ParseIntError> for Error {
    fn from(_: num::ParseIntError) -> Error {
        Error::InvalidPayload
    }
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Error {
        Error::InvalidPayload
    }
}

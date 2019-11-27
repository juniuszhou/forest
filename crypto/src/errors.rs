use address::Error as AddressError;
use secp256k1::Error as SecpError;
use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    SigningError(String),
    InvalidRecovery(String),
    InvalidPubKey(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::SigningError(ref s) => write!(f, "Could not sign data: {}", s),
            Error::InvalidRecovery(ref s) => {
                write!(f, "Could not recover public key from signature: {}", s)
            }
            Error::InvalidPubKey(ref s) => {
                write!(f, "Invalid generated pub key to create address: {}", s)
            }
        }
    }
}

impl From<Box<dyn error::Error>> for Error {
    fn from(err: Box<dyn error::Error>) -> Error {
        // Pass error encountered in signer trait as module error type
        Error::SigningError(err.description().to_string())
    }
}

impl From<AddressError> for Error {
    fn from(err: AddressError) -> Error {
        // convert error from generating address
        Error::InvalidPubKey(err.to_string())
    }
}

impl From<SecpError> for Error {
    fn from(err: SecpError) -> Error {
        match err {
            SecpError::InvalidRecoveryId => Error::InvalidRecovery(format!("{:?}", err)),
            _ => Error::SigningError(format!("{:?}", err)),
        }
    }
}

// TODO: Remove once cbor marshalling and unmarshalling implemented
impl From<String> for Error {
    fn from(err: String) -> Error {
        // Pass error encountered in signer trait as module error type
        Error::SigningError(err)
    }
}
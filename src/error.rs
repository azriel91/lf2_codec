use std::fmt::{self, Display};

use crate::{DecodeError, EncodeError};

/// Error that occurs when reading or writing to a stream.
#[derive(Debug)]
pub enum Error {
    /// Errors that happen during decoding.
    DecodeError(DecodeError),
    /// Errors that happen during encoding.
    EncodeError(EncodeError),
}

impl From<DecodeError> for Error {
    fn from(error: DecodeError) -> Self {
        Self::DecodeError(error)
    }
}

impl From<EncodeError> for Error {
    fn from(error: EncodeError) -> Self {
        Self::EncodeError(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DecodeError(e) => write!(f, "{}", e),
            Self::EncodeError(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {}

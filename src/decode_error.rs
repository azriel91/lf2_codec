use std::{
    fmt::{self, Display},
    io,
};

/// Error occurred reading from the stream to decode.
#[derive(Debug)]
pub struct DecodeError {
    /// Underlying IO error.
    pub error: io::Error,
}

impl From<io::Error> for DecodeError {
    fn from(error: io::Error) -> Self {
        Self { error }
    }
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Failed to decode object data. Underlying error: {}",
            self.error
        )
    }
}

impl std::error::Error for DecodeError {}

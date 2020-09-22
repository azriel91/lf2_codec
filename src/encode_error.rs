use std::{
    fmt::{self, Display},
    io,
};

/// Error occurred reading from the stream to encode.
#[derive(Debug)]
pub struct EncodeError {
    /// Underlying IO error.
    pub error: io::Error,
}

impl From<io::Error> for EncodeError {
    fn from(error: io::Error) -> Self {
        Self { error }
    }
}

impl Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Failed to encode object data. Underlying error: {}",
            self.error
        )
    }
}

impl std::error::Error for EncodeError {}

#![deny(missing_docs, missing_debug_implementations)]

//! Encodes and decodes Little Fighter 2 (LF2) data files.
//!
//! # Examples
//!
//! ## Encode
//!
//! ```rust,edition2018
//! use lf2_codec::{DataEncoder, EncodeError};
//!
//! const CHARACTER_DAT_ENCODED: &[u8] = b"\
//!   This is sample data as bytes. \
//!   The first 123 bytes are ignored during decoding, the rest are decoded using a caesar cipher. \
//!   \xab\xc6\xaf\xd5\xc0\xd4\xa7\xcc\xcc\xcf\xb3\
//!   \xe1\xc6\xb5\xca\x83\x93\x97\xdf\xe4\xe2\xac\
//!   \xdb\xab\xc6\xc0\xd9\xd4\xad\xe3\xd2\xa5";
//!
//! # fn main() -> Result<(), EncodeError> {
//! let data = "<bmp_begin>name: Azriel<bmp_end>";
//!
//! let encoded = DataEncoder::encode(data.as_bytes())?;
//!
//! assert_eq!(CHARACTER_DAT_ENCODED, encoded);
//! #
//! # Ok(())
//! # }
//! ```
//!
//! ## Decode
//!
//! ```rust,edition2018
//! use lf2_codec::{DataDecoder, DecodeError};
//!
//! const CHARACTER_DAT_ENCODED: &[u8] = b"\
//!   This is sample data as bytes. \
//!   The first 123 bytes are ignored during decoding, the rest are decoded using a caesar cipher. \
//!   \xab\xc6\xaf\xd5\xc0\xd4\xa7\xcc\xcc\xcf\xb3\
//!   \xe1\xc6\xb5\xca\x83\x93\x97\xdf\xe4\xe2\xac\
//!   \xdb\xab\xc6\xc0\xd9\xd4\xad\xe3\xd2\xa5";
//!
//! # fn main() -> Result<(), DecodeError> {
//! let decoded = DataDecoder::decode(CHARACTER_DAT_ENCODED)?;
//!
//! let expected = "<bmp_begin>name: Azriel<bmp_end>";
//!
//! assert_eq!(expected, String::from_utf8_lossy(&decoded));
//! #
//! # Ok(())
//! # }
//! ```

use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

pub use crate::{decode_error::DecodeError, encode_error::EncodeError, error::Error};

mod decode_error;
mod encode_error;
mod error;

/// Key used to shift the ascii code of each object.
pub const CAESAR_CIPHER: &[u8] = b"odBearBecauseHeIsVeryGoodSiuHungIsAGo";

/// Data used to fill the first 123 bytes of the data file. Strictly 123 bytes long.
pub const DATA_HEADER: &[u8; 123] = b"This is sample data as bytes. \
    The first 123 bytes are ignored during decoding, the rest are decoded using a caesar cipher. ";

/// Default number of bytes to allocate for the final result.
const DATA_CAPACITY_DEFAULT: usize = 1024;

/// Encodes data that the LF2 application may read.
#[derive(Debug)]
pub struct DataEncoder;

impl DataEncoder {
    /// Encodes object data for the LF2 application.
    ///
    /// # Parameters
    ///
    /// * `stream`: The stream of object data to encode.
    pub fn encode<R>(stream: R) -> Result<Vec<u8>, EncodeError>
    where
        R: Read,
    {
        let mut encoded = Vec::with_capacity(DATA_CAPACITY_DEFAULT);
        encoded.extend(DATA_HEADER.iter());

        let bytes = stream.bytes();
        bytes.zip(CAESAR_CIPHER.iter().copied().cycle()).try_fold(
            encoded,
            |mut encoded, (byte_result, cipher_byte)| match byte_result {
                Ok(byte) => {
                    let encoded_byte = byte.wrapping_add(cipher_byte);
                    encoded.push(encoded_byte);
                    Ok(encoded)
                }
                Err(error) => Err(EncodeError { error }),
            },
        )
    }
}

/// Decodes object data from LF2.
#[derive(Debug)]
pub struct DataDecoder;

impl DataDecoder {
    /// Decodes LF2 object data.
    ///
    /// # Parameters
    ///
    /// * `stream`: The stream of encoded object data.
    pub fn decode<R>(stream: R) -> Result<Vec<u8>, DecodeError>
    where
        R: Read,
    {
        let bytes = stream.bytes();

        bytes
            .skip(123)
            .zip(CAESAR_CIPHER.iter().copied().cycle())
            .try_fold(
                Vec::with_capacity(DATA_CAPACITY_DEFAULT),
                |mut decoded, (byte_result, cipher_byte)| match byte_result {
                    Ok(byte) => {
                        let decoded_byte = byte.wrapping_sub(cipher_byte);
                        decoded.push(decoded_byte);
                        Ok(decoded)
                    }
                    Err(error) => Err(DecodeError { error }),
                },
            )
    }

    /// Decodes LF2 object data from a file path.
    ///
    /// # Parameters
    ///
    /// * `path`: Path to the file to decode.
    pub fn decode_path<P>(path: P) -> Result<Vec<u8>, DecodeError>
    where
        P: AsRef<Path>,
    {
        let path = AsRef::<Path>::as_ref(&path);
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);

        Self::decode(buf_reader)
    }
}

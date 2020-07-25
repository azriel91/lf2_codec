use std::io::{Error, Read};

/// Key used to shift the ascii code of each character.
const CAESAR_CIPHER: &[u8] = b"odBearBecauseHeIsVeryGoodSiuHungIsAGo";

/// Default number of bytes to allocate for the final result.
const DECODED_CAPACITY_DEFAULT: usize = 1024;

#[derive(Debug)]
pub struct DataDecoder;

impl DataDecoder {
    pub fn decode<R>(stream: R) -> Result<Vec<u8>, Error>
    where
        R: Read,
    {
        let bytes = stream.bytes();

        bytes
            .skip(123)
            .zip(CAESAR_CIPHER.iter().copied().cycle())
            .try_fold(
                Vec::with_capacity(DECODED_CAPACITY_DEFAULT),
                |mut decoded, (byte_result, cipher_byte)| match byte_result {
                    Ok(byte) => {
                        let decoded_byte = byte.wrapping_sub(cipher_byte);
                        decoded.push(decoded_byte);
                        Ok(decoded)
                    }
                    Err(e) => Err(e),
                },
            )
    }
}

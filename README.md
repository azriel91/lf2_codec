[![docs](https://docs.rs/lf2_codec/badge.svg)](https://docs.rs/lf2_codec)
[![crates.io](https://img.shields.io/crates/v/lf2_codec.svg)](https://crates.io/crates/lf2_codec)

# LF2 Codec

Encodes and decodes Little Fighter 2 (LF2) data files.

## Usage

LF2 codec can be used as an application or a library.

### Application

```sh
# Installation
cargo install lf2_codec

# Running
lf2_codec decode character.dat > character.txt
lf2_codec encode character.txt > character.dat
```

### Library

```rust
use lf2_codec::DataDecoder;

let decoded_bytes = DataDecoder::decode_path("character.dat")?;
// or
// let character_dat_reader = BufReader::new(File::open("character.dat")?);
// let decoded_bytes = DataDecoder::decode(character_dat_reader)?;

let decoded = String::from_utf8(decoded_bytes)?;

println!("{}", decoded);
```

## License

Licensed the [Zlib license](LICENSE-ZLIB.md).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be licensed as above, without any additional terms or conditions.

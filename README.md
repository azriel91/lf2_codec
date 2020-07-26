# LF2 Decode

Decodes Little Fighter 2 (LF2) data files.

## Usage

### Library

```rust
use lf2_decode::DataDecoder;

let character_dat_reader = BufReader::new(File::open("character.dat")?);

let decoded_bytes = DataDecoder::decode(character_dat_reader)?;
let decoded = String::from_utf8(decoded_bytes)?;

println!("{}", decoded);
```

### Binary

```
cargo run --release -- character.dat > character.txt
```

## Development

```bash
git clone git@github.com:azriel91/lf2_decode.git
git switch -c feature/nnn/short-description # e.g. feature/123/implement-something

# open a PR
```

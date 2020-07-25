use std::{
    env,
    fs::File,
    io::{BufReader, Error},
};

use lf2_decode::DataDecoder;

fn print_help() {
    eprintln!("Usage: {} <character.dat ..>", env!("CARGO_PKG_NAME"));
}

fn main() -> Result<(), Error> {
    let args_os = env::args_os();
    if args_os.len() <= 1 {
        print_help();
    } else {
        args_os
            // TODO: First argument may be application name, or not.
            .skip(1)
            .try_for_each(|arg_os| {
                let file = File::open(arg_os)?;
                let buf_reader = BufReader::new(file);

                let decoded = DataDecoder::decode(buf_reader)?;
                let decoded =
                    String::from_utf8(decoded).expect("Failed to convert bytes to UTF8 string.");

                println!("{}", decoded);

                Result::<(), Error>::Ok(())
            })?;
    }
    Ok(())
}

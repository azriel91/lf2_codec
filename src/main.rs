use std::{
    env,
    fs::File,
    io::{self, BufReader, Write},
};

use lf2_codec::{DataDecoder, DataEncoder};

/// Dynamic error type for simpler code.
type Error = Box<dyn std::error::Error>;

// Type alias for function to run.
type Operation = fn(BufReader<File>) -> Result<Vec<u8>, Error>;

fn print_help() {
    eprintln!(
        "Usage: {} <encode|decode> <character.dat ..>",
        env!("CARGO_PKG_NAME")
    );
}

fn main() -> Result<(), Error> {
    let mut args_os = env::args_os();

    // Skip first argument -- usually application name.
    args_os.next();

    let operation: Option<Operation> = match args_os.next() {
        Some(arg_os) if arg_os == "encode" => {
            Some(|reader| DataEncoder::encode(reader).map_err(Error::from))
        }
        Some(arg_os) if arg_os == "decode" => {
            Some(|reader| DataDecoder::decode(reader).map_err(Error::from))
        }
        _ => None,
    };

    if let Some(operation) = operation {
        args_os.try_for_each(|arg_os| {
            let file = File::open(arg_os)?;
            let buf_reader = BufReader::new(file);

            let ende_coded = operation(buf_reader)?;
            io::stdout().write_all(&ende_coded)?;

            Result::<(), Error>::Ok(())
        })?;
    } else {
        print_help();
    }

    Ok(())
}

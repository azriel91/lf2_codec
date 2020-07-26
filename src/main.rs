use std::{
    env,
    fs::File,
    io::{self, BufReader, Error, Write},
};

use lf2_decode::{DataDecoder, DataEncoder};

// Type alias for function to run.
type Operation = fn(BufReader<File>) -> Result<Vec<u8>, Error>;

fn print_help() {
    eprintln!("Usage: {} <character.dat ..>", env!("CARGO_PKG_NAME"));
}

fn main() -> Result<(), Error> {
    let mut args_os = env::args_os();

    // TODO: First argument may be application name, or not.
    args_os.next();

    let operation: Option<Operation> = match args_os.next() {
        Some(arg_os) if arg_os == "encode" => Some(DataEncoder::encode),
        Some(arg_os) if arg_os == "decode" => Some(DataDecoder::decode),
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

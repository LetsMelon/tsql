use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::exit;

use tsql::{parse_file, TransformSQL};

const HELP: &str = "\
tsql

USAGE:
    tsql [INPUT] [OUTPUT]

FLAGS:
  -h, --help            Prints help information
";

#[derive(Debug)]
struct AppArgs {
    tsql_path: PathBuf,
    out_path: PathBuf,
}

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            exit(1);
        }
    };

    let tables = parse_file(&args.tsql_path).unwrap();

    let mut file = BufWriter::new(File::create(&args.out_path).unwrap());
    for (_, table) in tables {
        table.transform(&mut file).unwrap();
    }
    file.flush().unwrap();
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let args = AppArgs {
        // Parses a required free-standing/positional argument.
        tsql_path: pargs.free_from_str()?,
        out_path: pargs.free_from_str()?,
    };

    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}

use std::env;
use std::path::Path;
use std::process::exit;

use tsql::parse_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage:");
        println!("\ttsql PATH_TO_TSQL_FILE");
        exit(1);
    }

    match parse_file(Path::new(&args[1])) {
        Ok(parsed) => {
            for (_, table) in parsed {
                println!("{table:?}");
            }
        }
        Err(err) => {
            println!("encountered an error");
            println!("{err:?}");
            exit(1);
        }
    }
}

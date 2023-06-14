use std::env;
use std::path::Path;
use std::process::exit;

use tsql::parse_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("usage:");
        println!("\ttsql PATH_TO_TSQL_FILE OUT_FILE");
        exit(1);
    }

    let tables = parse_file(Path::new(&args[1])).unwrap();
}

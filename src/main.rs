use std::env;
use std::fs::File;
use std::path::Path;
use std::process::exit;

use tsql::{parse_file, TransformSQL};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("usage:");
        println!("\ttsql PATH_TO_TSQL_FILE OUT_FILE");
        exit(1);
    }

    let tables = parse_file(Path::new(&args[1])).unwrap();

    let mut file = File::create(Path::new(&args[2])).unwrap();
    for (_, table) in tables {
        table.transform(&mut file).unwrap();
    }
}

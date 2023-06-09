use std::env;
use std::path::Path;

use tsql::parse_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    let tsql_path = Path::new(args.last().unwrap());

    let parsed = parse_file(tsql_path).unwrap();
    println!("{:?}", parsed);
}

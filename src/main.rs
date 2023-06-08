use std::env;
use std::fs::read_to_string;
use std::path::Path;

mod parse;
mod types;

use crate::parse::parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    let tsql_path = Path::new(args.last().unwrap());
    let mut content = read_to_string(tsql_path).unwrap().replace("\n", "");

    let mut tables = vec![];

    while content.len() != 0 {
        let parser_result = parse(&content);

        if let Ok((raw, table)) = parser_result {
            content = raw.to_string();

            println!("{table:?}");

            tables.push(table);
        } else {
            println!("error: {:?}", parser_result);
            break;
        }
    }
}

use std::fs::read_to_string;
use std::path::Path;

use anyhow::{bail, Result};
use types::Table;

use crate::parse::parse;

mod parse;
mod types;

pub fn parse_str(mut content: String) -> Result<Vec<Table>> {
    let mut tables = vec![];

    while content.len() != 0 {
        let out = parse(&content);

        match out {
            Ok(sth) => {
                let c = sth.0.to_string().clone();
                tables.push(sth.1);
                content = c;
            }
            Err(err) => bail!(format!("{:?}", err)),
        }
    }

    Ok(tables)
}

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Vec<Table>> {
    let content = read_to_string(path).unwrap().replace("\n", "");

    parse_str(content)
}

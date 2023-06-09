use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

use anyhow::{bail, Result};
use types::Table;

use crate::parse::parse;

mod parse;
pub mod types;

pub type TableCollection = HashMap<String, Table>;

pub fn parse_str(mut content: String) -> Result<TableCollection> {
    let mut tables = HashMap::new();

    while content.len() != 0 {
        let out = parse(&content);

        match out {
            Ok((c, table)) => {
                let c = c.to_string().clone();

                let name = table.name.clone();
                tables.insert(name, table);

                content = c;
            }
            Err(err) => bail!(format!("{:?}", err)),
        }
    }

    Ok(tables)
}

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<TableCollection> {
    let content = read_to_string(path)?.replace("\n", "");

    parse_str(content)
}

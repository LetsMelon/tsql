#![feature(variant_count)]

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::path::Path;
use std::rc::Rc;

use anyhow::{bail, Result};
use types::{Table, TableCollection};

use crate::parser::parse;

mod parser;
pub mod types;

pub fn parse_str(mut content: String) -> Result<TableCollection> {
    let mut raw_tables = BTreeMap::new();

    while content.len() != 0 {
        let out = parse(&content);

        match out {
            Ok((c, table)) => {
                let c = c.to_string().clone();

                let name = table.name.clone();
                raw_tables.insert(name, Rc::new(RefCell::new(table)));

                content = c;
            }
            Err(err) => bail!(format!("{:?}", err)),
        }
    }

    Table::parse_raw_tables(raw_tables)
}

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<TableCollection> {
    let content = read_to_string(path)?.replace("\n", "");

    parse_str(content)
}

#![feature(variant_count)]

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;

use anyhow::{bail, Result};
use types::{Table, TableCollection};

use crate::parser::parse;

mod parser;
pub mod types;

pub fn parse_str(mut content: &str) -> Result<TableCollection> {
    let mut raw_tables = BTreeMap::new();

    while !content.is_empty() {
        let out = parse(content);

        match out {
            Ok((c, table)) => {
                let name = table.name.clone();
                raw_tables.insert(name, Rc::new(RefCell::new(table)));

                content = c;
            }
            Err(err) => bail!("An error occurred while parsing: {:?}", err),
        }
    }

    Table::parse_raw_tables(raw_tables)
}

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<TableCollection> {
    // TODO check if the `.replace(...)` is necessary
    let content = read_to_string(path)?.replace('\n', "");

    parse_str(&content)
}

// TODO add docs
pub trait TransformSQL {
    fn transform_into_sql<W: Write>(&self, buffer: &mut W) -> Result<()>;
}

// TODO add docs
pub trait TransformTSQL {
    fn transform_into_tsql<W: Write>(&self, buffer: &mut W) -> Result<()>;
}

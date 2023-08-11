#![feature(variant_count)]

use std::cell::RefCell;
use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;

use anyhow::{bail, Result};
use types::{RawTableCollection, Table, TableCollection};

use crate::parser::parse;

mod parser;
pub mod types;

/// Starts the parsing process with the given `&str`.
pub fn parse_str(mut content: &str) -> Result<TableCollection> {
    let mut raw_tables = RawTableCollection::new();

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

/// Starts the parsing process with a path as argument by reading the whole file into memory.
///
/// For more info see: [`parse_str`]
pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<TableCollection> {
    // TODO check if the `.replace(...)` is necessary
    let content = read_to_string(path)?.replace('\n', "");

    parse_str(&content)
}

/// Trait with methods to transform a struct into `sql`-code.
pub trait TransformSQL {
    /// Transforms the struct into `sql`-code.
    ///
    /// Writes the output to a generic buffer, which implements [`Write`].
    fn transform_into_sql<W: Write>(&self, buffer: &mut W) -> Result<()>;
}

/// Trait with methods to transform a struct into `tsql`-code.
pub trait TransformTSQL {
    /// Transforms the struct into `tsql`-code.
    ///
    /// Writes the output to a generic buffer, which implements [`Write`].
    fn transform_into_tsql<W: Write>(&self, buffer: &mut W) -> Result<()>;
}

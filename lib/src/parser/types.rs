use std::collections::HashMap;
use std::io::Write;

use anyhow::Result;

use crate::TransformTSQL;

#[derive(Debug)]
pub struct RawTable {
    pub extra: TableExtra,

    pub name: String,

    pub fields: HashMap<String, FieldType>,
}

impl RawTable {
    pub fn has_fk(&self) -> bool {
        !self.fk_tables().is_empty()
    }

    pub fn fk_tables(&self) -> Vec<String> {
        let mut table_names = Vec::with_capacity(self.fields.len());

        for field_type in self.fields.values() {
            match field_type {
                FieldType::Real(_) => (),
                FieldType::Virtual((field, FieldExtra::ForeignKey)) => match &field.datatype {
                    RawDataType::ForeignKeyTable(table_name) => {
                        table_names.push(table_name.clone())
                    }
                    _ => (),
                },
            };
        }

        table_names
    }
}

#[derive(Debug)]
pub enum FieldType {
    Real(RawField),
    Virtual((RawField, FieldExtra)),
}

#[derive(Debug)]
pub struct RawField {
    pub name: String,
    pub datatype: RawDataType,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RawDataType {
    Unknown,
    Int,
    Bool,
    BigInt,
    Date,
    DateTime,
    Time,
    Double,
    Float,
    Uuid,

    VarChar(u16),
    Char(u8),
    Text(u16),

    Decimal(u8, u8),

    ForeignKeyTable(String),
}

impl RawDataType {
    pub fn parse(input: &str, argument: Vec<&str>) -> Option<Self> {
        match (input, argument.len()) {
            ("int", 0) => Some(RawDataType::Int),
            ("bool", 0) => Some(RawDataType::Bool),
            ("bigint", 0) => Some(RawDataType::BigInt),
            ("date", 0) => Some(RawDataType::Date),
            ("datetime", 0) => Some(RawDataType::DateTime),
            ("time", 0) => Some(RawDataType::Time),
            ("double", 0) => Some(RawDataType::Double),
            ("float", 0) => Some(RawDataType::Float),
            ("uuid", 0) => Some(RawDataType::Uuid),
            ("_", 0) => Some(RawDataType::Unknown),

            ("varchar", 1) => match argument[0].parse() {
                Ok(l) => Some(RawDataType::VarChar(l)),
                _ => None,
            },
            ("char", 1) => match argument[0].parse() {
                Ok(l) => Some(RawDataType::Char(l)),
                _ => None,
            },
            ("text", 1) => match argument[0].parse() {
                Ok(l) => Some(RawDataType::Text(l)),
                _ => None,
            },

            ("decimal", 2) => match (argument[0].parse(), argument[1].parse()) {
                (Ok(precision), Ok(scale)) => Some(RawDataType::Decimal(precision, scale)),
                _ => None,
            },

            (item, 0) => Some(RawDataType::ForeignKeyTable(item.to_string())),

            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub struct TableExtra {
    pub(crate) primary_key: Vec<String>,
}

impl TableExtra {
    pub fn new_with_pk(primary_key: Vec<String>) -> Self {
        TableExtra {
            primary_key,
            ..Self::default()
        }
    }
}

impl TransformTSQL for TableExtra {
    fn transform_into_tsql<W: Write>(&self, buffer: &mut W) -> Result<()> {
        if !self.primary_key.is_empty() {
            writeln!(buffer, "@primary_key({})", self.primary_key.join(", "))?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldExtra {
    ForeignKey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagHelper {
    PrimaryKey,
}

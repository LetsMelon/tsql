use std::collections::HashMap;

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

    ForeignKeyTable(String),
}

impl RawDataType {
    pub fn parse(input: &str, argument: Option<&str>) -> Option<Self> {
        match (input, argument) {
            ("int", None) => Some(RawDataType::Int),
            ("bool", None) => Some(RawDataType::Bool),
            ("bigint", None) => Some(RawDataType::BigInt),
            ("date", None) => Some(RawDataType::Date),
            ("datetime", None) => Some(RawDataType::DateTime),
            ("time", None) => Some(RawDataType::Time),
            ("double", None) => Some(RawDataType::Double),
            ("float", None) => Some(RawDataType::Float),
            ("uuid", None) => Some(RawDataType::Uuid),
            ("_", None) => Some(RawDataType::Unknown),

            ("varchar", Some(length)) => match length.parse() {
                Ok(l) => Some(RawDataType::VarChar(l)),
                _ => None,
            },
            ("char", Some(length)) => match length.parse() {
                Ok(l) => Some(RawDataType::Char(l)),
                _ => None,
            },
            ("text", Some(length)) => match length.parse() {
                Ok(l) => Some(RawDataType::Text(l)),
                _ => None,
            },

            (item, None) => Some(RawDataType::ForeignKeyTable(item.to_string())),

            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub struct TableExtra {
    pub primary_key: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum FieldExtra {
    ForeignKey,
}

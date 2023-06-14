use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::rc::Rc;

use anyhow::{bail, Result};
use static_assertions::const_assert_eq;

use crate::parser::types::{FieldExtra, FieldType, RawDataType, RawField, RawTable, TableExtra};

pub type GenericCollection<T> = BTreeMap<String, T>;
pub type TableCollection = GenericCollection<Table>;
pub(crate) type RawTableCollection = GenericCollection<Rc<RefCell<RawTable>>>;

fn get_first_element<'a, K: Ord, V>(collection: &'a BTreeMap<K, V>) -> Option<(&'a K, &'a V)> {
    let key = collection.keys().next();

    if key.is_none() {
        return None;
    }

    let key = key.unwrap();

    Some((key, collection.get(key).unwrap()))
}

#[derive(Debug, Default)]
pub struct Table {
    pub(crate) extra: TableExtra,

    pub(crate) name: String,

    pub(crate) fields: HashMap<String, Field>,
}

impl Table {
    pub(crate) fn parse_raw_tables(mut raw_tables: RawTableCollection) -> Result<TableCollection> {
        let mut parsed = TableCollection::new();

        let mut raw_tables_order: VecDeque<Rc<RefCell<RawTable>>> =
            VecDeque::with_capacity(raw_tables.len());

        // TODO use while let
        while raw_tables.len() != 0 {
            let (name, table) = get_first_element(&raw_tables).unwrap();

            let name = name.clone();
            let table = table.clone();

            let has_fk = table.borrow().has_fk();

            if has_fk {
                let fk_tables = table.borrow().fk_tables();

                let ordered_tables = raw_tables_order
                    .iter()
                    .map(|item| item.borrow().name.clone())
                    .collect::<Vec<_>>();

                let mut all_fk_are_ordered = true;
                for fk_table in fk_tables {
                    if !ordered_tables.contains(&fk_table) {
                        all_fk_are_ordered = false;
                        break;
                    }
                }

                if !all_fk_are_ordered {
                    // TODO check for loop inside raw_tables with table
                    raw_tables.remove(&name);
                    raw_tables.insert(name, table);

                    continue;
                }
            }

            raw_tables.remove(&name);
            raw_tables_order.push_back(table);
        }

        while let Some(raw_table) = raw_tables_order.pop_front() {
            let parsed_table = Table::parse(raw_table, &parsed)?;

            parsed.insert(parsed_table.name.clone(), parsed_table);
        }

        Ok(parsed)
    }

    pub(crate) fn parse(
        raw: Rc<RefCell<RawTable>>,
        parsed_tables: &TableCollection,
    ) -> Result<Self> {
        let mut parsed_table = Table::default();

        let raw = raw.borrow();

        parsed_table.name = raw.name.clone();

        for (_, field_type) in &raw.fields {
            match field_type {
                FieldType::Real(raw_field) => {
                    parsed_table
                        .fields
                        .insert(raw_field.name.clone(), Field::parse(raw_field)?);
                }
                FieldType::Virtual((raw_field, FieldExtra::ForeignKey)) => {
                    let fk_table_name = match &raw_field.datatype {
                        RawDataType::ForeignKeyTable(fk_table_name) => fk_table_name,
                        _ => todo!(),
                    };

                    let fk_table = parsed_tables.get(fk_table_name).unwrap();
                    let fk_table_primary_key = &fk_table.extra.primary_key;

                    let prefix = &raw_field.name;
                    for fk_table_primary_key_name in fk_table_primary_key {
                        let fk_field = fk_table.fields.get(fk_table_primary_key_name).unwrap();

                        let field_name = format!("{}_{}", prefix, fk_field.name);

                        let field = Field {
                            name: field_name.clone(),
                            datatype: fk_field.datatype,
                            foreign_key_reference: Some((
                                fk_table_name.clone(),
                                Rc::new(fk_field.clone()),
                            )),
                        };

                        parsed_table.fields.insert(field_name, field);
                    }
                }
            };
        }

        for primary_key_field in &raw.extra.primary_key {
            if !parsed_table.fields.contains_key(primary_key_field) {
                bail!(
                    "Error: Table {:?} doesn't have a field with the name {:?}, encountered error while checking for primary key",
                    parsed_table.name,
                    primary_key_field
                );
            }

            parsed_table
                .extra
                .primary_key
                .push(primary_key_field.clone());
        }

        Ok(parsed_table)
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub(crate) name: String,
    pub(crate) datatype: DataType,
    // TODO change Rc<Field> to Box<Field>
    pub(crate) foreign_key_reference: Option<(String, Rc<Field>)>,
}

impl Field {
    fn parse(raw: &RawField) -> Result<Self> {
        Ok(Field {
            name: raw.name.to_string(),
            datatype: DataType::parse(&raw.datatype)?,
            foreign_key_reference: None,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DataType {
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
}
const_assert_eq!(
    std::mem::variant_count::<RawDataType>() - 2,
    std::mem::variant_count::<DataType>()
);

impl DataType {
    fn parse(raw: &RawDataType) -> Result<Self> {
        match raw {
            RawDataType::Int => Ok(DataType::Int),
            RawDataType::Bool => Ok(DataType::Bool),
            RawDataType::BigInt => Ok(DataType::BigInt),
            RawDataType::Date => Ok(DataType::Date),
            RawDataType::DateTime => Ok(DataType::DateTime),
            RawDataType::Time => Ok(DataType::Time),
            RawDataType::Double => Ok(DataType::Double),
            RawDataType::Float => Ok(DataType::Float),
            RawDataType::Uuid => Ok(DataType::Uuid),

            RawDataType::VarChar(args) => Ok(DataType::VarChar(*args)),
            RawDataType::Char(args) => Ok(DataType::Char(*args)),
            RawDataType::Text(args) => Ok(DataType::Text(*args)),

            RawDataType::Unknown => bail!("Error: encountered type unknown. raw: {:?}", raw),
            RawDataType::ForeignKeyTable(_) => {
                bail!("Error: encountered a foreign key. raw: {:?}", raw)
            }
        }
    }
}

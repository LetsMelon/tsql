use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::io::Write;
use std::rc::Rc;

use anyhow::{bail, Result};
use static_assertions::const_assert_eq;

use crate::parser::types::{FieldExtra, FieldType, RawDataType, RawField, RawTable, TableExtra};
use crate::TransformSQL;

pub type GenericCollection<T> = BTreeMap<String, T>;
pub type TableCollection = GenericCollection<Table>;
pub(crate) type RawTableCollection = GenericCollection<Rc<RefCell<RawTable>>>;

fn get_first_element<K: Ord, V>(collection: &BTreeMap<K, V>) -> Option<(&K, &V)> {
    let key = collection.keys().next()?;
    let item = collection.get(key)?;

    Some((key, item))
}

#[derive(Debug, Default)]
pub struct Table {
    pub(crate) extra: TableExtra,

    pub(crate) name: String,

    pub(crate) fields: HashMap<String, Field>,
}

impl Table {
    pub fn get_field(&self, key: &str) -> Option<&Field> {
        self.fields.get(key)
    }

    pub fn primary_keys(&self) -> &Vec<String> {
        &self.extra.primary_key
    }

    pub(crate) fn parse_raw_tables(mut raw_tables: RawTableCollection) -> Result<TableCollection> {
        let mut parsed = TableCollection::new();

        let mut raw_tables_order: VecDeque<Rc<RefCell<RawTable>>> =
            VecDeque::with_capacity(raw_tables.len());

        while let Some((name, table)) = get_first_element(&raw_tables) {
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

        for field_type in raw.fields.values() {
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

impl TransformSQL for Table {
    fn transform<W: Write>(&self, buffer: &mut W) -> Result<()> {
        writeln!(buffer, "CREATE TABLE {} (", self.name)?;

        let mut foreign_keys_table_fields: HashMap<String, Vec<&Field>> = HashMap::new();

        for field in self.fields.values() {
            field.transform(buffer)?;

            if field.foreign_key_reference.is_some() {
                let table = &field.foreign_key_reference.as_ref().unwrap().0;

                match foreign_keys_table_fields.get_mut(table) {
                    Some(fields_list) => {
                        fields_list.push(field);
                    }
                    None => {
                        foreign_keys_table_fields.insert(table.to_string(), vec![field]);
                    }
                };
            }
        }

        for (table_name, fields) in foreign_keys_table_fields {
            let field_names = fields
                .iter()
                .map(|item| item.name.as_str())
                .collect::<Vec<_>>()
                .join(",");

            let other_field_names = fields
                .iter()
                .map(|item| item.foreign_key_reference.as_ref().unwrap().1.name.clone())
                .collect::<Vec<_>>()
                .join(",");

            writeln!(
                buffer,
                "FOREIGN KEY ({}) REFERENCES {}({}),",
                field_names, table_name, other_field_names
            )?;
        }

        let primary_key_formatted = self.extra.primary_key.join(",");
        writeln!(buffer, "PRIMARY KEY ({})", primary_key_formatted)?;

        writeln!(buffer, ");")?;

        Ok(())
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

    pub fn datatype(&self) -> &DataType {
        &self.datatype
    }
}

impl TransformSQL for Field {
    fn transform<W: Write>(&self, buffer: &mut W) -> Result<()> {
        write!(buffer, "{} ", self.name)?;
        self.datatype.transform(buffer)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

    /// values: `(precision, scale)`
    Decimal(u8, u8),
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

            RawDataType::Decimal(precision, scale) => Ok(DataType::Decimal(*precision, *scale)),

            RawDataType::Unknown => bail!("Error: encountered type unknown. raw: {:?}", raw),
            RawDataType::ForeignKeyTable(_) => {
                bail!("Error: encountered a foreign key. raw: {:?}", raw)
            }
        }
    }
}

impl TransformSQL for DataType {
    fn transform<W: Write>(&self, buffer: &mut W) -> Result<()> {
        let formatted = match self {
            DataType::Int => "int".to_string(),
            DataType::Bool => "boolean".to_string(),
            DataType::BigInt => "bigint".to_string(),
            DataType::Date => "date".to_string(),
            DataType::DateTime => "datetime".to_string(),
            DataType::Time => "time".to_string(),
            DataType::Double => "double".to_string(),
            DataType::Float => "float".to_string(),
            DataType::Uuid => "uuid".to_string(),

            DataType::VarChar(args) => format!("varchar({})", args),
            DataType::Char(args) => format!("char({})", args),
            DataType::Text(args) => format!("text({})", args),

            DataType::Decimal(precision, scale) => format!("decimal({}, {})", precision, scale),
        };

        writeln!(buffer, "{},", formatted)?;

        Ok(())
    }
}

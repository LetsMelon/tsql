use std::collections::HashMap;

use crate::types::TableExtra;

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
                    _ => panic!("Error with field '{:?}'.\nThe field type is `FieldType::Virtual` but the datatype is not `RawDataType::ForeignKeyTable`.", field),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldExtra {
    ForeignKey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagHelper {
    PrimaryKey,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::RawTable;
    use crate::parser::types::{FieldExtra, FieldType, RawDataType, RawField};
    use crate::types::TableExtra;

    #[test]
    fn raw_table() {
        let table = RawTable {
            extra: TableExtra::default(),
            name: "".to_string(),
            fields: HashMap::new(),
        };

        assert_eq!(table.has_fk(), false);
        assert_eq!(table.fk_tables(), Vec::<String>::new());

        let table = RawTable {
            extra: TableExtra::default(),
            name: "".to_string(),
            fields: {
                let mut map = HashMap::new();

                // TODO I think this fk value is bullshit, so I have to check this with the real parsing program
                map.insert(
                    "human".to_string(),
                    FieldType::Virtual((
                        RawField {
                            name: "name".to_string(),
                            datatype: RawDataType::ForeignKeyTable("human".to_string()),
                        },
                        FieldExtra::ForeignKey,
                    )),
                );

                map.insert(
                    "bday".to_string(),
                    FieldType::Real(RawField {
                        name: "bday".to_string(),
                        datatype: RawDataType::DateTime,
                    }),
                );

                map
            },
        };

        assert_eq!(table.has_fk(), true);
        assert_eq!(table.fk_tables(), vec!["human".to_string()]);
    }

    mod raw_data_type {
        use itertools::Itertools;

        use crate::parser::types::RawDataType;

        #[test]
        fn arguments_len_0() {
            let fields = [
                ("int", RawDataType::Int),
                ("bool", RawDataType::Bool),
                ("bigint", RawDataType::BigInt),
                ("date", RawDataType::Date),
                ("datetime", RawDataType::DateTime),
                ("time", RawDataType::Time),
                ("double", RawDataType::Double),
                ("float", RawDataType::Float),
                ("uuid", RawDataType::Uuid),
                ("_", RawDataType::Unknown),
                (
                    "custom_data_type",
                    RawDataType::ForeignKeyTable("custom_data_type".to_string()),
                ),
            ];

            for (raw, result) in fields {
                assert_eq!(RawDataType::parse(raw, Vec::new()), Some(result));

                assert_eq!(RawDataType::parse(raw, vec![Default::default(); 4]), None);
            }
        }

        #[test]
        fn arguments_number_parsing() {
            enum DT {
                U8,
                U16,
            }

            impl DT {
                fn max_value(&self) -> String {
                    match self {
                        DT::U8 => u8::MAX.to_string(),
                        DT::U16 => u16::MAX.to_string(),
                    }
                }

                fn min_value(&self) -> String {
                    match self {
                        DT::U8 => u8::MIN.to_string(),
                        DT::U16 => u16::MIN.to_string(),
                    }
                }

                fn wrong_values(&self) -> Vec<String> {
                    let mut specific_values = match self {
                        DT::U8 => vec![u32::MAX.to_string()],
                        DT::U16 => vec![u32::MAX.to_string()],
                    };

                    let mut values = vec!["a".to_string(), "abc".to_string()];
                    values.append(&mut specific_values);
                    values
                }
            }

            // TODO that's some really weird witch magic and I _think_ it would be good if we can refactor this (a lot)
            // TODO a good first think to todo would be to remove the `unwrap()`s and return a `Result<RawDataType, _>` or an `Option<RawDataType>`
            // The `Box<dyn Fn(&str) -> RawDataType>` is so that we can create the type with the value in the test
            let fields: &[(&str, Box<dyn Fn(&str) -> RawDataType>, DT)] = &[
                (
                    "varchar",
                    Box::new(|value: &str| RawDataType::VarChar(value.parse().unwrap())) as _,
                    DT::U16,
                ),
                (
                    "char",
                    Box::new(|value: &str| RawDataType::Char(value.parse().unwrap())) as _,
                    DT::U8,
                ),
                (
                    "text",
                    Box::new(|value: &str| RawDataType::Text(value.parse().unwrap())) as _,
                    DT::U16,
                ),
            ];

            for (raw, result_type, arguments) in fields {
                let out = RawDataType::parse(raw, vec![&arguments.min_value()]);
                assert!(out.is_some());
                assert_eq!(out, Some(result_type(&arguments.min_value())));

                let out = RawDataType::parse(raw, vec![&arguments.max_value()]);
                assert!(out.is_some());
                assert_eq!(out, Some(result_type(&arguments.max_value())));

                for wrong_value in arguments.wrong_values() {
                    assert_eq!(RawDataType::parse(raw, vec![&wrong_value]), None);
                }
            }

            // ! same as in the comment before
            let fields: &[(&str, Box<dyn Fn((&str, &str)) -> RawDataType>, (DT, DT))] = &[(
                "decimal",
                Box::new(|(v1, v2): (&str, &str)| {
                    RawDataType::Decimal(v1.parse().unwrap(), v2.parse().unwrap())
                }) as _,
                (DT::U8, DT::U8),
            )];

            for (raw, result_type, arguments) in fields {
                let out = RawDataType::parse(
                    raw,
                    vec![&arguments.0.min_value(), &arguments.1.min_value()],
                );
                assert!(out.is_some());
                assert_eq!(
                    out,
                    Some(result_type((
                        &arguments.0.min_value(),
                        &arguments.1.min_value()
                    )))
                );

                let out = RawDataType::parse(
                    raw,
                    vec![&arguments.0.max_value(), &arguments.1.max_value()],
                );
                assert!(out.is_some());
                assert_eq!(
                    out,
                    Some(result_type((
                        &arguments.0.max_value(),
                        &arguments.1.max_value()
                    )))
                );

                for wrong_value in arguments
                    .0
                    .wrong_values()
                    .iter()
                    .cartesian_product(arguments.1.wrong_values())
                {
                    assert_eq!(
                        RawDataType::parse(raw, vec![wrong_value.0, &wrong_value.1]),
                        None
                    );
                }
            }
        }
    }
}

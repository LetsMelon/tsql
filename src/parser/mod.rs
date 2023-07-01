use std::collections::HashMap;

use nom::bytes::complete::tag;
use nom::IResult;

mod helper;
mod parser;
pub mod types;

use crate::parser::helper::preceded_space_get_word;
use crate::parser::parser::{parse_table_body, parse_table_extra, parse_table_fields};
use crate::parser::types::*;

pub fn parse(input: &str) -> IResult<&str, RawTable> {
    let (input, extra) = table_extra(input)?;

    // start of table
    let (input, _) = tag("table")(input)?;

    // parse name
    let (input, name) = preceded_space_get_word(input)?;

    // parse fields
    let (input, fields) = table_body(input)?;

    // end of table
    let (input, _) = tag(";")(input)?;

    Ok((
        input,
        RawTable {
            extra,
            name: name.to_string(),
            fields,
        },
    ))
}

fn table_extra(input: &str) -> IResult<&str, TableExtra> {
    let (input, item) = parse_table_extra(input)?;

    let mut table_extra = TableExtra::default();

    match item {
        Some((TagHelper::PrimaryKey, values)) => table_extra.primary_key.append(
            &mut values
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<_>>(),
        ),
        None => (),
    }

    Ok((input, table_extra))
}

fn parse_fields(input: &str) -> IResult<&str, HashMap<String, FieldType>> {
    let (input, raw_list) = parse_table_fields(input)?;

    let mut fields = HashMap::new();

    for raw_item in raw_list {
        let parsed_type =
            RawDataType::parse(raw_item.field_type, raw_item.field_type_arguments).unwrap();

        if let Some(field_extra) = raw_item.field_extra {
            fields.insert(
                raw_item.field_name.to_string(),
                FieldType::Virtual((
                    RawField {
                        name: raw_item.field_name.to_string(),
                        datatype: parsed_type,
                    },
                    field_extra,
                )),
            );
        } else {
            fields.insert(
                raw_item.field_name.to_string(),
                FieldType::Real(RawField {
                    name: raw_item.field_name.to_string(),
                    datatype: parsed_type,
                }),
            );
        }
    }

    Ok((input, fields))
}

fn table_body(input: &str) -> IResult<&str, HashMap<String, FieldType>> {
    let (input, raw_body) = parse_table_body(input)?;

    let (_, fields) = parse_fields(raw_body)?;

    Ok((input, fields))
}

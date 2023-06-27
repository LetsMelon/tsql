use std::collections::HashMap;

use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::{opt, value};
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair, preceded, tuple};
use nom::IResult;

mod helper;
mod parser;
pub mod types;

use self::parser::parse_table_body;
use crate::parser::helper::{get_word, preceded_space_get_word};
use crate::parser::parser::parse_table_fields;
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
    #[derive(Debug, Clone, Copy)]
    enum TagHelper {
        PrimaryKey,
    }

    let (input, item) = opt(preceded(
        tag("@"),
        pair(
            value(TagHelper::PrimaryKey, tag("primary_key")),
            delimited(
                tag("("),
                separated_list0(tuple((multispace0, tag(","), multispace0)), get_word),
                tag(")"),
            ),
        ),
    ))(input)?;

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

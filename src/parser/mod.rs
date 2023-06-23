use std::collections::HashMap;

use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{digit1, multispace0, space1};
use nom::combinator::{opt, value};
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
use nom::IResult;

mod helper;
pub mod types;

use crate::parser::helper::get_word;
use crate::parser::types::*;

pub fn parse(input: &str) -> IResult<&str, RawTable> {
    let (input, extra) = table_extra(input)?;

    // start of table
    let (input, _) = tag("table")(input)?;

    // parse name
    let (input, name) = table_name(input)?;

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

fn table_name(input: &str) -> IResult<&str, &str> {
    let (input, name) = preceded(space1, get_word)(input)?;

    Ok((input, name))
}

fn parse_fields(input: &str) -> IResult<&str, HashMap<String, FieldType>> {
    // TODO remove types, but rust-analyzer can't figure out the type of `raw_list`
    let (input, raw_list): (
        &str,
        Vec<((Option<FieldExtra>, Option<()>, &str, Option<&str>), &str)>,
    ) = terminated(
        separated_list0(
            tag(","),
            preceded(
                space1,
                separated_pair(
                    tuple((
                        opt(preceded(
                            opt(space1),
                            value(FieldExtra::ForeignKey, tag("@foreign_key()")),
                        )),
                        opt(value((), space1)),
                        // type
                        get_word,
                        // arguments
                        opt(delimited(tag("("), digit1, tag(")"))),
                    )),
                    space1,
                    // field name
                    get_word,
                ),
            ),
        ),
        tag(","),
    )(input)?;

    let mut fields = HashMap::new();

    for ((field_extra, _, field_type, field_type_arguments), field_name) in raw_list {
        let parsed_type = RawDataType::parse(field_type, field_type_arguments).unwrap();

        if let Some(field_extra) = field_extra {
            fields.insert(
                field_name.to_string(),
                FieldType::Virtual((
                    RawField {
                        name: field_name.to_string(),
                        datatype: parsed_type,
                    },
                    field_extra,
                )),
            );
        } else {
            fields.insert(
                field_name.to_string(),
                FieldType::Real(RawField {
                    name: field_name.to_string(),
                    datatype: parsed_type,
                }),
            );
        }
    }

    Ok((input, fields))
}

fn table_body(input: &str) -> IResult<&str, HashMap<String, FieldType>> {
    let (input, raw_body) = preceded(
        space1,
        delimited(tag("{"), take_while1(|c| c != '}'), tag("}")),
    )(input)?;

    let (_, fields) = parse_fields(raw_body)?;

    Ok((input, fields))
}

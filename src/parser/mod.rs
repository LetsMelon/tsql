use std::collections::HashMap;

use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{digit1, multispace0, space1};
use nom::combinator::{opt, value};
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
use nom::IResult;

use self::types::*;

pub mod types;

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
    let (input, item) = opt(preceded(
        tag("@"),
        pair(
            tag("primary_key"),
            delimited(
                tag("("),
                separated_list0(
                    tuple((multispace0, tag(","), multispace0)),
                    take_while1(get_is_word()),
                ),
                tag(")"),
            ),
        ),
    ))(input)?;

    let mut table_extra = TableExtra::default();

    match item {
        Some((tag, values)) => match tag {
            "primary_key" => table_extra.primary_key.append(
                &mut values
                    .iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<_>>(),
            ),
            _ => (),
        },
        None => (),
    }

    Ok((input, table_extra))
}

fn table_name(input: &str) -> IResult<&str, &str> {
    let (input, name) = preceded(space1, take_while1(get_is_word()))(input)?;

    Ok((input, name))
}

fn get_is_word() -> impl Fn(char) -> bool {
    |c| char::is_alphabetic(c) || c == '_'
}

fn parse_fields(input: &str) -> IResult<&str, HashMap<String, FieldType>> {
    let (input, raw_list) = terminated(
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
                        take_while1(get_is_word()),
                        // arguments
                        opt(delimited(tag("("), digit1, tag(")"))),
                    )),
                    space1,
                    // field name
                    take_while1(get_is_word()),
                ),
            ),
        ),
        tag(","),
    )(input)?;

    let mut fields = HashMap::new();

    for ((field_extra, _, field_type, field_type_arguments), field_name) in raw_list {
        let parsed_type = RawDataType::parse(field_type, field_type_arguments).unwrap();

        if field_extra.is_none() {
            fields.insert(
                field_name.to_string(),
                FieldType::Real(RawField {
                    name: field_name.to_string(),
                    datatype: parsed_type,
                }),
            );
        } else {
            fields.insert(
                field_name.to_string(),
                FieldType::Virtual((
                    RawField {
                        name: field_name.to_string(),
                        datatype: parsed_type,
                    },
                    field_extra.unwrap(),
                )),
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
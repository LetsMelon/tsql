use std::collections::HashMap;

use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{digit1, space1};
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
use nom::IResult;

use crate::types::{DataType, Field, RawDataType, Table, TableExtra};

pub fn parse(input: &str) -> IResult<&str, Table> {
    let (input, extra) = table_extra(input)?;

    // start of table
    let (input, _) = tag("table")(input)?;
    let (input, _) = space1(input)?;

    // parse name
    let (input, name) = table_name(input)?;

    // parse fields
    let (input, fields) = table_body(input)?;

    // end of table
    let (input, _) = tag(";")(input)?;

    Ok((
        input,
        Table {
            extra,
            raw_name: name.to_string(),
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
                    tuple((opt(space1), tag(","), opt(space1))),
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
    let (input, name) = take_while1(get_is_word())(input)?;

    Ok((input, name))
}

fn get_is_word() -> impl Fn(char) -> bool {
    |c| char::is_alphabetic(c) || c == '_'
}

fn table_body(input: &str) -> IResult<&str, HashMap<String, Field>> {
    let (input, _) = space1(input)?;

    let (input, raw_fields) = delimited(
        tag("("),
        terminated(
            separated_list0(
                tag(","),
                preceded(
                    space1,
                    separated_pair(
                        // field type + arguments
                        pair(
                            take_while1(get_is_word()),
                            opt(delimited(tag("("), digit1, tag(")"))),
                        ),
                        space1,
                        // field name
                        take_while1(get_is_word()),
                    ),
                ),
            ),
            tag(","),
        ),
        tag(")"),
    )(input)?;

    let mut fields = HashMap::new();

    for ((raw_datatype, argument), raw_name) in raw_fields {
        let raw_dt = RawDataType::parse(raw_datatype, argument).unwrap();
        let dt = DataType::Raw(RawDataType::Unknown);

        fields.insert(
            raw_name.to_string(),
            Field {
                raw_name: raw_name.to_string(),
                name: raw_name.to_string(),
                raw_datatype: raw_dt,
                datatype: dt,
            },
        );
    }

    Ok((input, fields))
}

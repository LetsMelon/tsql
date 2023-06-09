use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{digit1, space1};
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated};
use nom::IResult;

use crate::types::{DataType, Field, RawDataType, Table};

pub fn parse(input: &str) -> IResult<&str, Table> {
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
            raw_name: name.to_string(),
            name: name.to_string(),
            fields,
        },
    ))
}

fn table_name(input: &str) -> IResult<&str, &str> {
    let (input, name) = take_while1(get_is_word())(input)?;

    Ok((input, name))
}

fn get_is_word() -> impl Fn(char) -> bool {
    |c| char::is_alphabetic(c) || c == '_'
}

fn table_body(input: &str) -> IResult<&str, Vec<Field>> {
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

    let mut fields = vec![];

    for ((raw_datatype, argument), raw_name) in raw_fields {
        let raw_dt = RawDataType::parse(raw_datatype, argument).unwrap();
        let dt = DataType::Raw(RawDataType::Unknown);

        fields.push(Field {
            raw_name: raw_name.to_string(),
            name: raw_name.to_string(),
            raw_datatype: raw_dt,
            datatype: dt,
        });
    }

    Ok((input, fields))
}

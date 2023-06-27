use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{digit1, multispace0, space1};
use nom::combinator::{opt, value};
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};
use nom::IResult;

use crate::parser::helper::get_word;
use crate::parser::types::FieldExtra;

#[derive(Debug)]
pub struct RawParsedField<'a> {
    pub field_extra: Option<FieldExtra>,
    pub field_type: &'a str,
    pub field_type_arguments: Vec<&'a str>,
    pub field_name: &'a str,
}

// TODO add tests
fn parse_single_table_field(input: &str) -> IResult<&str, RawParsedField> {
    let (input, out): (
        &str,
        (
            (Option<FieldExtra>, Option<()>, &str, Option<Vec<&str>>),
            &str,
        ),
    ) = preceded(
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
                opt(delimited(
                    tag("("),
                    separated_list0(tuple((multispace0, tag(","), multispace0)), digit1),
                    tag(")"),
                )),
            )),
            space1,
            // field name
            get_word,
        ),
    )(input)?;

    Ok((
        input,
        RawParsedField {
            field_extra: out.0 .0,
            field_type: out.0 .2,
            field_type_arguments: out.0 .3.unwrap_or_default(),
            field_name: out.1,
        },
    ))
}

// TODO add tests
pub fn parse_table_fields(input: &str) -> IResult<&str, Vec<RawParsedField>> {
    terminated(
        separated_list0(tag(","), parse_single_table_field),
        tag(","),
    )(input)
}

// TODO add tests
pub fn parse_table_body(input: &str) -> IResult<&str, &str> {
    const OPENING_BRACKET: char = '{';
    const CLOSING_BRACKET: char = '}';

    preceded(
        space1,
        delimited(
            tag(OPENING_BRACKET.to_string().as_str()),
            take_while1(|c| c != CLOSING_BRACKET),
            tag(CLOSING_BRACKET.to_string().as_str()),
        ),
    )(input)
}

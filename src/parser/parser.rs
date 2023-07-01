use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{digit1, space1};
use nom::combinator::{opt, value};
use nom::multi::separated_list0;
use nom::sequence::{pair, preceded, separated_pair, terminated, tuple};
use nom::IResult;

use crate::parser::helper::{build_generic_delimited, build_separated_tuple_list, get_word};
use crate::parser::types::{FieldExtra, TagHelper};

#[derive(Debug, PartialEq, Eq)]
pub struct RawParsedField<'a> {
    pub field_extra: Option<FieldExtra>,
    pub field_type: &'a str,
    pub field_type_arguments: Vec<&'a str>,
    pub field_name: &'a str,
}

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
                opt(build_separated_tuple_list(digit1)),
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
        build_generic_delimited(
            take_while1(|c| c != CLOSING_BRACKET),
            OPENING_BRACKET,
            CLOSING_BRACKET,
        ),
    )(input)
}

// TODO add tests
pub fn parse_table_extra(input: &str) -> IResult<&str, Option<(TagHelper, Vec<&str>)>> {
    opt(preceded(
        tag("@"),
        pair(
            value(TagHelper::PrimaryKey, tag("primary_key")),
            build_separated_tuple_list(get_word),
        ),
    ))(input)
}

#[cfg(test)]
mod tests {
    mod parse_single_table_field {
        use crate::parser::parser::{parse_single_table_field, RawParsedField};
        use crate::parser::types::FieldExtra;

        #[test]
        fn just_works() {
            let out = parse_single_table_field("  int number");
            assert!(out.is_ok());
            let out = out.unwrap();
            assert_eq!(out.0, "");
            assert_eq!(
                out.1,
                RawParsedField {
                    field_extra: None,
                    field_type: "int",
                    field_type_arguments: Vec::new(),
                    field_name: "number"
                }
            );

            let out = parse_single_table_field("  varchar(512) text");
            assert!(out.is_ok());
            let out = out.unwrap();
            assert_eq!(out.0, "");
            assert_eq!(
                out.1,
                RawParsedField {
                    field_extra: None,
                    field_type: "varchar",
                    field_type_arguments: vec!["512"],
                    field_name: "text"
                }
            );

            let out = parse_single_table_field("  decimal(12, 3) number");
            assert!(out.is_ok());
            let out = out.unwrap();
            assert_eq!(out.0, "");
            assert_eq!(
                out.1,
                RawParsedField {
                    field_extra: None,
                    field_type: "decimal",
                    field_type_arguments: vec!["12", "3"],
                    field_name: "number"
                }
            );

            let out = parse_single_table_field("  @foreign_key()  int number");
            assert!(out.is_ok());
            let out = out.unwrap();
            assert_eq!(out.0, "");
            assert_eq!(
                out.1,
                RawParsedField {
                    field_extra: Some(FieldExtra::ForeignKey),
                    field_type: "int",
                    field_type_arguments: Vec::new(),
                    field_name: "number"
                }
            );
        }
    }
}

use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{multispace0, space1};
use nom::error::context;
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded, tuple};
use nom::{IResult, Parser};

/// This function takes an input string and extracts the first word from it.
///
/// A word is defined as a sequence of characters separated by underscores.
///
/// ## Examples
/// ```txt
/// use crate::parser::helper::get_word;
///
/// assert_eq!(get_word("hello world"), Ok((" world", "hello")));
/// assert_eq!(get_word("hello_world"), Ok(("", "hello_world")));
/// ```
pub fn get_word(input: &str) -> IResult<&str, &str> {
    context(
        "get_word",
        take_while1(|c| char::is_alphabetic(c) || c == '_'),
    )(input)
}

/// This function takes an input string and extracts the first word with a preceded whitespace from it.
///
/// For more see [`get_word`].
pub fn preceded_space_get_word(input: &str) -> IResult<&str, &str> {
    context("preceded_space_get_word", preceded(space1, get_word))(input)
}

// TODO add tests
/// Captures `(...VALUES,)` and parses the elements with `fct` of the list
pub fn separated_tuple_list<'a, F: Parser<&'a str, &'a str, nom::error::Error<&'a str>>>(
    input: &'a str,
    fct: F,
) -> IResult<&str, Vec<&str>> {
    context(
        "separated_tuple_list",
        delimited(
            tag("("),
            separated_list0(tuple((multispace0, tag(","), multispace0)), fct),
            tag(")"),
        ),
    )(input)
}

// TODO add tests
#[inline]
/// Returns a closure of type `Fn(&str) -> IResult<&str, Vec<&str>>` by moving the given `fct` into a closure which calls [`separated_tuple_list`].
///
/// For more see [`separated_tuple_list`].
pub fn build_separated_tuple_list<
    'a,
    F: Parser<&'a str, &'a str, nom::error::Error<&'a str>> + Copy,
>(
    fct: F,
) -> impl Fn(&'a str) -> Result<(&'a str, Vec<&'a str>), nom::Err<nom::error::Error<&'a str>>> {
    move |input| separated_tuple_list(input, fct)
}

#[cfg(test)]
mod tests {
    mod get_word {
        use nom::error::{Error, ErrorKind};
        use nom::Err;

        use crate::parser::helper::get_word;

        #[test]
        fn parses_alphabetic_chars() {
            assert_eq!(get_word("Hello"), Ok(("", "Hello")));
            assert_eq!(get_word("Hello123"), Ok(("123", "Hello")));
            assert_eq!(get_word("Hello@World"), Ok(("@World", "Hello")));
            assert_eq!(get_word("Hello World"), Ok((" World", "Hello")));
        }

        #[test]
        fn parses_alphabetic_with_underscore() {
            assert_eq!(get_word("Hello_World"), Ok(("", "Hello_World")));
            assert_eq!(get_word("Hello_World123"), Ok(("123", "Hello_World")));
            assert_eq!(
                get_word("Hello_World_World World"),
                Ok((" World", "Hello_World_World"))
            );
        }

        #[test]
        fn errors() {
            assert_eq!(
                get_word(""),
                Err(Err::Error(Error::new("", ErrorKind::TakeWhile1)))
            );
        }
    }
}

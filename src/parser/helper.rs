use nom::bytes::complete::take_while1;
use nom::character::complete::space1;
use nom::error::context;
use nom::sequence::preceded;
use nom::IResult;

// TODO add tests
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

use std::io::Write;

pub const COMMENT_LINE: &'static str = "================================";

pub fn writeln_sql_comment<W: Write, S: Into<String>>(
    writer: &mut W,
    content: S,
) -> std::io::Result<()> {
    writeln!(writer, "-- {}", content.into())
}

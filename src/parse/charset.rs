use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::eof,
    sequence::{preceded, terminated, tuple},
    IResult,
};
use shell_parser_common_rs::{
    charset::{parse_charset, Charset},
    ShellParseError,
};

use super::parts::newline_body;

pub(super) fn charset<'a>(input: &'a str) -> IResult<&'a str, Charset, ShellParseError> {
    preceded(
        space0,
        terminated(charset_body, tuple((space0, alt((newline_body, eof))))),
    )(input)
}

fn charset_body<'a>(input: &'a str) -> IResult<&'a str, Charset, ShellParseError> {
    preceded(tag("charset,"), parse_charset)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod charset {
        use super::*;

        #[test]
        fn sucess_when_valid_str() {
            let case = r#"charset,Shift_JIS"#;
            let (remain, result) = charset(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, Charset::ShiftJIS);

            let case = "charset,UTF-8\r\n\r\n";
            let (remain, result) = charset(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, Charset::UTF8);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = r#"// あいうえお
// かきくけこ

"#;
            assert!(charset(case).is_err());

            let case = r#"surface0
{
}
"#;
            assert!(charset(case).is_err());
        }
    }

    mod charset_body {
        use super::*;

        #[test]
        fn sucess_when_valid_str_ascii() {
            let case = "charset,ASCII\r\n";
            let (remain, result) = charset_body(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, Charset::ASCII);
        }

        #[test]
        fn sucess_when_valid_str_shift_jis() {
            let case = "charset,Shift_JIS\r\n";
            let (remain, result) = charset_body(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, Charset::ShiftJIS);
        }

        #[test]
        fn sucess_when_valid_str_utf_8() {
            let case = "charset,UTF-8\r\n";
            let (remain, result) = charset_body(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, Charset::UTF8);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "charset,x76";
            assert!(charset_body(case).is_err());
        }
    }
}

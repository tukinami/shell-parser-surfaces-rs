//! Parses a [`Seriko`] from `&str`.
//!
//! [`Seriko`]: crate::ast::Seriko
use std::borrow::Cow;

use nom::{
    branch::alt,
    combinator::{eof, map},
    multi::many0,
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::{BraceContainer, Seriko};

use self::{
    charset::charset,
    cursor::brace_seriko_cursor,
    descript::brace_descript,
    parts::{header_comments_func, whole_line_as_comment_line},
    surface::{brace_surface, brace_surface_append},
    surface_alias::brace_surface_alias,
    tooltip::brace_tooltip,
};

mod charset;
mod cursor;
mod descript;
mod parts;
mod surface;
mod surface_alias;
mod tooltip;

/// Decodes bytes to `Cow<'a, str>` from specified charset.
///
/// # Examples
///
/// ```
/// use encoding_rs::SHIFT_JIS;
///
/// use shell_parser_surfaces_rs::decode_bytes;
///
/// let case_raw = r#"
/// charset,Shift_JIS
///
/// descript
/// {
/// version,1
/// }
///
/// surface0
/// {
/// animation0.interval,sometimes
/// animation0.pattern0,overlay,101,100,168,67
/// animation0.pattern1,overlay,100,100,168,67
/// animation0.pattern2,overlay,101,100,168,67
/// animation0.pattern3,overlay,-1,100,168,67
/// }
/// "#;
///
/// let (case, _, _) = SHIFT_JIS.encode(case_raw);
/// let result = match decode_bytes(&case) {
///     Ok(v) => v,
///     Err(e) => {
///         eprintln!("{:?}", e);
///         return;
///     }
/// };
/// assert_eq!(result, case_raw);
/// ```
pub fn decode_bytes<'a>(input: &'a [u8]) -> Result<Cow<'a, str>, String> {
    let temp_str = String::from_utf8_lossy(input);
    let (_remain, charset) = match preceded(header_comments_func(charset), charset)(&temp_str) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    match charset.decode(input) {
        Ok(v) => Ok(v),
        Err(_) => Err(format!("Encoding failed: to {:?}", charset)),
    }
}

/// Parses a [`ShellSurfaces`] from `&str`.
///
/// [`ShellSurfaces`]: crate::ast::Seriko
///
/// # Examples
///
/// ```
/// use shell_parser_common_rs::charset::Charset;
/// use shell_parser_surfaces_rs::parse;
///
/// let case = r#"charset,Shift_JIS
///
/// descript
/// {
/// version,1
/// }
///
/// surface0
/// {
/// animation0.interval,sometimes
/// animation0.pattern0,overlay,101,100,168,67
/// animation0.pattern1,overlay,100,100,168,67
/// animation0.pattern2,overlay,101,100,168,67
/// animation0.pattern3,overlay,-1,100,168,67
/// }
/// "#;
///
/// let seriko = match parse(case) {
///      Ok(v) => v,
///      Err(e) => {
///          eprintln!("{:?}", e);
///          return;
///      }
///  };
///
///  assert_eq!(seriko.charset(), &Charset::ShiftJIS);
///  assert_eq!(seriko.braces().len(), 2);
/// ```
pub fn parse<'a>(input: &'a str) -> Result<Seriko, nom::Err<SerikoParseError>> {
    seriko(input).map(|(_, v)| v)
}

fn seriko<'a>(input: &'a str) -> IResult<&'a str, Seriko, SerikoParseError> {
    map(
        tuple((
            header_comments_func(charset),
            charset,
            braces,
            terminated(many0(whole_line_as_comment_line), eof),
        )),
        |(header_comments, c, b, footer_comments)| {
            Seriko::new(header_comments, c, b, footer_comments)
        },
    )(input)
}

fn braces<'a>(input: &'a str) -> IResult<&'a str, Vec<BraceContainer>, SerikoParseError> {
    many0(brace)(input)
}

fn brace<'a>(input: &'a str) -> IResult<&'a str, BraceContainer, SerikoParseError> {
    alt((
        brace_descript,
        brace_surface,
        brace_surface_append,
        brace_surface_alias,
        brace_seriko_cursor,
        brace_tooltip,
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    mod decode_bytes {
        use encoding_rs::SHIFT_JIS;

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case_raw = r#"
charset,Shift_JIS

descript
{
version,1
}

surface0
{
animation0.interval,sometimes
animation0.pattern0,overlay,101,100,168,67
animation0.pattern1,overlay,100,100,168,67
animation0.pattern2,overlay,101,100,168,67
animation0.pattern3,overlay,-1,100,168,67
}
"#;
            let (case, _, _) = SHIFT_JIS.encode(case_raw);
            let result = decode_bytes(&case).unwrap();
            assert_eq!(result, case_raw);
        }
    }

    mod parse {

        use crate::Brace;
        use shell_parser_common_rs::charset::Charset;

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = r#"
charset,Shift_JIS

descript
{
version,1
}

surface0
{
animation0.interval,sometimes
animation0.pattern0,overlay,101,100,168,67
animation0.pattern1,overlay,100,100,168,67
animation0.pattern2,overlay,101,100,168,67
animation0.pattern3,overlay,-1,100,168,67
}

surface1
{
element0,overlay,body0.png,0,0
element1,overlay,face1.png,0,0
}

surface10
{
collision0,40,56,95,90,Head

sakura.balloon.offsetx,80
sakura.balloon.offsety,-100
kero.balloon.offsetx,-30
kero.balloon.offsety,20
}

// SSPのみで有効な記法 ←このようにコメント行は//から始める
surface.append0-9
{
collision0,188,25,252,63,Head
collision1,190,92,236,118,Face
collision2,180,191,220,222,Bust
collision3,154,311,248,362,Skirt
}
// aaa
"#;
            let result = parse(case).unwrap();
            assert_eq!(result.charset(), &Charset::ShiftJIS);
            assert!(matches!(
                result.braces().get(0).unwrap().body(),
                Brace::Descript(_)
            ));
            assert!(matches!(
                result.braces().get(1).unwrap().body(),
                Brace::Surface(_)
            ));
            assert!(matches!(
                result.braces().get(2).unwrap().body(),
                Brace::Surface(_)
            ));
            assert!(matches!(
                result.braces().get(3).unwrap().body(),
                Brace::Surface(_)
            ));
            assert!(matches!(
                result.braces().get(4).unwrap().body(),
                Brace::SurfaceAppend(_)
            ));

            let case = "charset,UTF-8";
            let result = parse(case).unwrap();
            assert_eq!(result.charset(), &Charset::UTF8);
            assert!(result.braces().is_empty());
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = r#""#;
            assert!(parse(case).is_err());

            let case = r#"descript
{
}"#;
            assert!(parse(case).is_err());

            let case = r#"
charset,Shift_JIS
descript
{
"#;
            assert!(parse(case).is_err());
        }
    }

    mod seriko {
        use crate::Brace;
        use shell_parser_common_rs::charset::Charset;

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = r#"
charset,Shift_JIS

descript
{
version,1
}

surface0
{
animation0.interval,sometimes
animation0.pattern0,overlay,101,100,168,67
animation0.pattern1,overlay,100,100,168,67
animation0.pattern2,overlay,101,100,168,67
animation0.pattern3,overlay,-1,100,168,67
}

surface1
{
element0,overlay,body0.png,0,0
element1,overlay,face1.png,0,0
}

surface10
{
collision0,40,56,95,90,Head

sakura.balloon.offsetx,80
sakura.balloon.offsety,-100
kero.balloon.offsetx,-30
kero.balloon.offsety,20
}

// SSPのみで有効な記法 ←このようにコメント行は//から始める
surface.append0-9
{
collision0,188,25,252,63,Head
collision1,190,92,236,118,Face
collision2,180,191,220,222,Bust
collision3,154,311,248,362,Skirt
}
// aaa
"#;
            let (remain, result) = seriko(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result.charset(), &Charset::ShiftJIS);
            assert!(matches!(
                result.braces().get(0).unwrap().body(),
                Brace::Descript(_)
            ));
            assert!(matches!(
                result.braces().get(1).unwrap().body(),
                Brace::Surface(_)
            ));
            assert!(matches!(
                result.braces().get(2).unwrap().body(),
                Brace::Surface(_)
            ));
            assert!(matches!(
                result.braces().get(3).unwrap().body(),
                Brace::Surface(_)
            ));
            assert!(matches!(
                result.braces().get(4).unwrap().body(),
                Brace::SurfaceAppend(_)
            ));

            let case = "charset,UTF-8";
            let (remain, result) = seriko(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result.charset(), &Charset::UTF8);
            assert!(result.braces().is_empty());
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = r#""#;
            assert!(seriko(case).is_err());

            let case = r#"descript
{
}"#;
            assert!(seriko(case).is_err());

            let case = r#"
charset,Shift_JIS
descript
{
"#;
            assert!(seriko(case).is_err());
        }
    }

    mod braces {
        use crate::Brace;

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = r#"

descript
{
}

surface0
{
}"#;
            let (remain, result) = braces(case).unwrap();
            assert_eq!(remain, "");
            assert!(matches!(result.get(0).unwrap().body(), Brace::Descript(_)));
            assert!(matches!(result.get(1).unwrap().body(), Brace::Surface(_)));
        }
    }

    mod brace {
        use crate::Brace;

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "descript\r\n{\r\n}\r\n";
            let (remain, result) = brace(case).unwrap();
            assert_eq!(remain, "");
            assert!(matches!(result.body(), Brace::Descript(_)));

            let case = "surface0\r\n{\r\n}\r\n";
            let (remain, result) = brace(case).unwrap();
            assert_eq!(remain, "");
            assert!(matches!(result.body(), Brace::Surface(_)));

            let case = "surface.append0\r\n{\r\n}\r\n";
            let (remain, result) = brace(case).unwrap();
            assert_eq!(remain, "");
            assert!(matches!(result.body(), Brace::SurfaceAppend(_)));

            let case = "sakura.surface.alias\r\n{\r\n}\r\n";
            let (remain, result) = brace(case).unwrap();
            assert_eq!(remain, "");
            assert!(matches!(result.body(), Brace::SurfaceAlias(_)));

            let case = "sakura.cursor\r\n{\r\n}\r\n";
            let (remain, result) = brace(case).unwrap();
            assert_eq!(remain, "");
            assert!(matches!(result.body(), Brace::Cursor(_)));

            let case = "sakura.tooltips\r\n{\r\n}\r\n";
            let (remain, result) = brace(case).unwrap();
            assert_eq!(remain, "");
            assert!(matches!(result.body(), Brace::Tooltip(_)));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "descript{}";
            assert!(brace(case).is_err());
        }
    }
}

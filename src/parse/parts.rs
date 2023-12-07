use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{digit1, space0},
    combinator::{eof, map, map_res, not, opt},
    multi::many0,
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::{CommentLine, LineContainer, SerikoParseError, SurfaceTargetCharacterId};

pub(super) fn newline_body<'a>(input: &'a str) -> IResult<&'a str, &'a str, SerikoParseError> {
    alt((tag("\r\n"), tag("\r"), tag("\n")))(input)
}

pub(super) fn whole_line_without_newline<'a>(
    input: &'a str,
) -> IResult<&'a str, String, SerikoParseError> {
    map(
        alt((
            map(newline_body, |_v| ""),
            map(
                tuple((not(alt((tag("{"), tag("}")))), is_not("\r\n"), newline_body)),
                |(_, v, _)| v,
            ),
        )),
        |s| s.to_string(),
    )(input)
}

pub(super) fn whole_line_as_comment_line<'a>(
    input: &'a str,
) -> IResult<&'a str, CommentLine, SerikoParseError> {
    map(whole_line_without_newline, |v| CommentLine::new(v))(input)
}

pub(super) fn parse_inner_line_func<'a, T, F>(
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, LineContainer<T>, SerikoParseError>
where
    F: FnMut(&'a str) -> IResult<&'a str, T, SerikoParseError>,
{
    alt((
        map(tuple((space0, f, space0, newline_body)), |(_, v, _, _)| {
            LineContainer::Body(v)
        }),
        map(tuple((space0, whole_line_as_comment_line)), |(_, v)| {
            LineContainer::Comment(v)
        }),
    ))
}

pub(super) fn digit<'a, T>(input: &'a str) -> IResult<&'a str, T, SerikoParseError>
where
    T: FromStr,
{
    map_res(digit1, |v: &'a str| v.parse())(input)
}

pub(super) fn digit_neg<'a, T>(input: &'a str) -> IResult<&'a str, T, SerikoParseError>
where
    T: FromStr + std::ops::Neg<Output = T>,
{
    map(tuple((opt(tag("-")), digit::<T>)), |(sign, v)| {
        if sign.is_some() {
            -v
        } else {
            v
        }
    })(input)
}

pub(super) fn boolean<'a>(input: &'a str) -> IResult<&'a str, bool, SerikoParseError> {
    alt((map(tag("true"), |_| true), map(tag("false"), |_| false)))(input)
}

pub(super) fn surface_target_character_id<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceTargetCharacterId, SerikoParseError> {
    alt((
        surface_target_character_id_sakura,
        surface_target_character_id_kero,
        surface_target_character_id_char,
    ))(input)
}

fn surface_target_character_id_sakura<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceTargetCharacterId, SerikoParseError> {
    map(tag("sakura"), |_| SurfaceTargetCharacterId::Sakura)(input)
}

fn surface_target_character_id_kero<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceTargetCharacterId, SerikoParseError> {
    map(tag("kero"), |_| SurfaceTargetCharacterId::Kero)(input)
}

fn surface_target_character_id_char<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceTargetCharacterId, SerikoParseError> {
    map(tuple((tag("char"), digit)), |(_, v)| {
        SurfaceTargetCharacterId::Char(v)
    })(input)
}

pub(super) fn header_comments_func<'a, T, F>(
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<CommentLine>, SerikoParseError>
where
    F: FnMut(&'a str) -> IResult<&'a str, T, SerikoParseError>,
{
    many0(map(tuple((not(f), whole_line_as_comment_line)), |(_, v)| v))
}

pub(super) fn brace_name_func<'a, T, F>(
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, T, SerikoParseError>
where
    F: FnMut(&'a str) -> IResult<&'a str, T, SerikoParseError>,
{
    preceded(space0, terminated(f, tuple((space0, newline_body))))
}

pub(super) fn inner_brace_func<'a, T, F>(
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<LineContainer<T>>, SerikoParseError>
where
    F: FnMut(&'a str) -> IResult<&'a str, T, SerikoParseError>,
{
    map(
        tuple((
            tuple((space0, tag("{"), space0, newline_body)),
            many0(parse_inner_line_func(f)),
            tuple((space0, tag("}"), space0, alt((newline_body, eof)))),
        )),
        |(_, v, _)| v,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    mod newline_body {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "\r\na";
            let (remain, result) = newline_body(case).unwrap();
            assert_eq!(remain, "a");
            assert_eq!(result, "\r\n");

            let case = "\n\ra";
            let (remain, result) = newline_body(case).unwrap();
            assert_eq!(remain, "\ra");
            assert_eq!(result, "\n");

            let case = "\ra";
            let (remain, result) = newline_body(case).unwrap();
            assert_eq!(remain, "a");
            assert_eq!(result, "\r");
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "a";
            assert!(newline_body(case).is_err());
        }
    }

    mod whole_line_without_newline {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "\r\n";
            let (remain, result) = whole_line_without_newline(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, "".to_string());

            let case = "aaa\r\na";
            let (remain, result) = whole_line_without_newline(case).unwrap();
            assert_eq!(remain, "a");
            assert_eq!(result, "aaa".to_string());
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "aaa";
            assert!(whole_line_without_newline(case).is_err());

            let case = "{    \r\n";
            assert!(whole_line_without_newline(case).is_err());

            let case = "}\r\n";
            assert!(whole_line_without_newline(case).is_err());
        }
    }

    mod whole_line_as_comment_line {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "\r\n";
            let (remain, result) = whole_line_as_comment_line(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, CommentLine::new("".to_string()));

            let case = "aaa\r\na";
            let (remain, result) = whole_line_as_comment_line(case).unwrap();
            assert_eq!(remain, "a");
            assert_eq!(result, CommentLine::new("aaa".to_string()));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "aaa";
            assert!(whole_line_as_comment_line(case).is_err());
        }
    }

    mod parse_inner_func {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case_t_func = tag("abc");
            let mut case_func = parse_inner_line_func(case_t_func);

            let case = "    abc    \r\naaa";
            let (remain, result) = case_func(case).unwrap();
            assert_eq!(remain, "aaa");
            assert_eq!(result, LineContainer::Body("abc"));

            let case = "abc\r\n";
            let (remain, result) = case_func(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, LineContainer::Body("abc"));

            let case = "    aaa    \r\nbbb";
            let (remain, result) = case_func(case).unwrap();
            assert_eq!(remain, "bbb");
            assert_eq!(
                result,
                LineContainer::Comment(CommentLine::new("aaa    ".to_string()))
            );

            let case = "aaa\r\n";
            let (remain, result) = case_func(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                LineContainer::Comment(CommentLine::new("aaa".to_string()))
            );

            let case = "\r\n";
            let (remain, result) = case_func(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                LineContainer::Comment(CommentLine::new("".to_string()))
            );

            let case = "\r\n\r\n";
            let (remain, result) = case_func(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(
                result,
                LineContainer::Comment(CommentLine::new("".to_string()))
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case_t_func = tag("abc");
            let mut case_func = parse_inner_line_func(case_t_func);

            let case = "abc";
            assert!(case_func(case).is_err());

            let case = "aaa";
            assert!(case_func(case).is_err());

            let case = "";
            assert!(case_func(case).is_err());
        }
    }

    mod digit {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "1234\r\n";
            let (remain, result) = digit::<u16>(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, 1234);

            let case = "0";
            let (remain, result) = digit::<u8>(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, 0);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "-1";
            assert!(digit::<u8>(case).is_err());

            let case = "abc";
            assert!(digit::<u8>(case).is_err());
        }
    }

    mod digit_neg {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "1234\r\n";
            let (remain, result) = digit_neg::<i16>(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, 1234);

            let case = "-1234\r\n";
            let (remain, result) = digit_neg::<i16>(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, -1234);

            let case = "0";
            let (remain, result) = digit_neg::<i8>(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, 0);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "abc";
            assert!(digit_neg::<i8>(case).is_err());
        }
    }

    mod boolean {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "true";
            let (remain, result) = boolean(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, true);

            let case = "false";
            let (remain, result) = boolean(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, false);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "True";
            assert!(boolean(case).is_err());
        }
    }

    mod surface_target_character_id {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.tooltips";
            let (remain, result) = surface_target_character_id(case).unwrap();
            assert_eq!(remain, ".tooltips");
            assert_eq!(result, SurfaceTargetCharacterId::Sakura);

            let case = "kero.tooltips";
            let (remain, result) = surface_target_character_id(case).unwrap();
            assert_eq!(remain, ".tooltips");
            assert_eq!(result, SurfaceTargetCharacterId::Kero);

            let case = "char502.tooltips";
            let (remain, result) = surface_target_character_id(case).unwrap();
            assert_eq!(remain, ".tooltips");
            assert_eq!(result, SurfaceTargetCharacterId::Char(502));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "kk.tooltips";
            assert!(surface_target_character_id(case).is_err());
        }
    }

    mod header_comments_func {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case_t_func = tuple((space0, tag("abc"), space0, newline_body));
            let mut case_func = header_comments_func(case_t_func);

            let case = "    abc    \n";
            let (remain, result) = case_func(case).unwrap();
            assert_eq!(remain, "    abc    \n");
            assert_eq!(result, vec![]);

            let case = "    \n    \n    abc    \n";
            let (remain, result) = case_func(case).unwrap();
            assert_eq!(remain, "    abc    \n");
            assert_eq!(
                result,
                vec![
                    CommentLine::new("    ".to_string()),
                    CommentLine::new("    ".to_string())
                ]
            );

            let case = "\n\n    abc    \n";
            let (remain, result) = case_func(case).unwrap();
            assert_eq!(remain, "    abc    \n");
            assert_eq!(
                result,
                vec![
                    CommentLine::new("".to_string()),
                    CommentLine::new("".to_string())
                ]
            );
        }
    }

    mod brace_name_func {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case_t_func = tag("descript");
            let mut case_func = brace_name_func(case_t_func);

            let case = "    descript\r\n";
            let (remain, result) = case_func(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, "descript");
        }

        #[test]
        fn failed_when_invalid_str() {
            let case_t_func = tag("descript");
            let mut case_func = brace_name_func(case_t_func);

            let case = "    descript";
            assert!(case_func(case).is_err());
        }
    }

    mod inner_brace_func {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case_t_func = tag("abc");
            let mut case_func = inner_brace_func(case_t_func);

            let case = r#"    {    
    bbb
    abc
abc    
bbb    
    }    

{
}"#;
            let (remain, result) = case_func(case).unwrap();
            assert_eq!(
                remain,
                r#"
{
}"#
            );
            assert_eq!(
                result,
                vec![
                    LineContainer::Comment(CommentLine::new("bbb".to_string())),
                    LineContainer::Body("abc"),
                    LineContainer::Body("abc"),
                    LineContainer::Comment(CommentLine::new("bbb    ".to_string())),
                ]
            );

            let case = r#"{
bbb
}"#;
            let (remain, result) = case_func(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                vec![LineContainer::Comment(CommentLine::new("bbb".to_string()))]
            );
        }
    }

    #[test]
    fn failed_when_invalid_str() {
        let case_t_func = tag("abc");
        let mut case_func = inner_brace_func(case_t_func);

        let case = r#"{
abc"#;
        assert!(case_func(case).is_err());

        let case = r#"
abc
}"#;
        assert!(case_func(case).is_err());
    }
}

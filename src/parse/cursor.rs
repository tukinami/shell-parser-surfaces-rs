use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    combinator::map,
    sequence::{preceded, terminated, tuple},
    IResult,
};
use shell_parser_common_rs::ShellParseError;

use crate::{
    ast::{GestureKind, SerikoCursor, SerikoCursorGesture},
    Brace, BraceContainer, SurfaceTargetCharacterId,
};

use super::parts::{
    brace_name_func, digit, header_comments_func, inner_brace_func, surface_target_character_id,
};

pub(super) fn brace_seriko_cursor<'a>(
    input: &'a str,
) -> IResult<&'a str, BraceContainer, ShellParseError> {
    map(
        tuple((header_comments_func(seriko_cursor_name), seriko_cursor)),
        |(header_comments, body)| BraceContainer::new(header_comments, Brace::Cursor(body)),
    )(input)
}

fn seriko_cursor<'a>(input: &'a str) -> IResult<&'a str, SerikoCursor, ShellParseError> {
    map(
        tuple((seriko_cursor_name, inner_brace_func(seriko_cursor_define))),
        |(id, lines)| SerikoCursor::new(id, lines),
    )(input)
}

fn seriko_cursor_name<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceTargetCharacterId, ShellParseError> {
    brace_name_func(terminated(surface_target_character_id, tag(".cursor")))(input)
}

fn seriko_cursor_define<'a>(
    input: &'a str,
) -> IResult<&'a str, SerikoCursorGesture, ShellParseError> {
    map(
        tuple((
            gesture_kind,
            digit,
            preceded(tag(","), take_until(",")),
            preceded(tag(","), is_not("\r\n")),
        )),
        |(kind, id, collision, filename)| {
            SerikoCursorGesture::new(kind, id, collision.to_string(), filename.to_string())
        },
    )(input)
}

fn gesture_kind<'a>(input: &'a str) -> IResult<&'a str, GestureKind, ShellParseError> {
    alt((
        map(tag("mouseup"), |_| GestureKind::MouseUp),
        map(tag("mousedown"), |_| GestureKind::MouseDown),
        map(tag("mouserightdown"), |_| GestureKind::MouseRightDown),
        map(tag("mousewheel"), |_| GestureKind::MouseWheel),
        map(tag("mousehover"), |_| GestureKind::MouseHover),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod brace_seriko_cursor {
        use crate::{CommentLine, LineContainer};

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = r#"

sakura.cursor
{
mouseup0,Head,system:hand
mousedown0,Head,system:finger

mouseup1,Bust,system:hand
mousedown1,Bust,system:grip
}
"#;
            let (remain, result) = brace_seriko_cursor(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result.header_comments(),
                &vec![
                    CommentLine::new("".to_string()),
                    CommentLine::new("".to_string()),
                ]
            );
            assert_eq!(
                result.body(),
                &Brace::Cursor(SerikoCursor::new(
                    crate::ast::SurfaceTargetCharacterId::Sakura,
                    vec![
                        LineContainer::Body(SerikoCursorGesture::new(
                            GestureKind::MouseUp,
                            0,
                            "Head".to_string(),
                            "system:hand".to_string()
                        )),
                        LineContainer::Body(SerikoCursorGesture::new(
                            GestureKind::MouseDown,
                            0,
                            "Head".to_string(),
                            "system:finger".to_string()
                        )),
                        LineContainer::Comment(CommentLine::new("".to_string())),
                        LineContainer::Body(SerikoCursorGesture::new(
                            GestureKind::MouseUp,
                            1,
                            "Bust".to_string(),
                            "system:hand".to_string()
                        )),
                        LineContainer::Body(SerikoCursorGesture::new(
                            GestureKind::MouseDown,
                            1,
                            "Bust".to_string(),
                            "system:grip".to_string()
                        )),
                    ]
                ))
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = r#"

sakura.cursor {
mouseup0,Head,system:hand
mousedown0,Head,system:finger

mouseup1,Bust,system:hand
mousedown1,Bust,system:grip
}
"#;
            assert!(brace_seriko_cursor(case).is_err());
        }
    }

    mod seriko_cursor {
        use crate::{CommentLine, LineContainer};

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = r#"sakura.cursor
{
mouseup0,Head,system:hand
mousedown0,Head,system:finger

mouseup1,Bust,system:hand
mousedown1,Bust,system:grip
}
"#;
            let (remain, result) = seriko_cursor(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                SerikoCursor::new(
                    crate::ast::SurfaceTargetCharacterId::Sakura,
                    vec![
                        LineContainer::Body(SerikoCursorGesture::new(
                            GestureKind::MouseUp,
                            0,
                            "Head".to_string(),
                            "system:hand".to_string()
                        )),
                        LineContainer::Body(SerikoCursorGesture::new(
                            GestureKind::MouseDown,
                            0,
                            "Head".to_string(),
                            "system:finger".to_string()
                        )),
                        LineContainer::Comment(CommentLine::new("".to_string())),
                        LineContainer::Body(SerikoCursorGesture::new(
                            GestureKind::MouseUp,
                            1,
                            "Bust".to_string(),
                            "system:hand".to_string()
                        )),
                        LineContainer::Body(SerikoCursorGesture::new(
                            GestureKind::MouseDown,
                            1,
                            "Bust".to_string(),
                            "system:grip".to_string()
                        )),
                    ]
                )
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = r#"sakuracursor{}"#;
            assert!(seriko_cursor(case).is_err());
        }
    }

    mod seriko_cursor_name {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.cursor\r\n{";
            let (remain, result) = seriko_cursor_name(case).unwrap();
            assert_eq!(remain, "{");
            assert_eq!(result, SurfaceTargetCharacterId::Sakura);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "hoge.cursor\r\n";
            assert!(seriko_cursor_name(case).is_err());
        }
    }

    mod seriko_cursor_define {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "mouseup0,Head,system:hand";
            let (remain, result) = seriko_cursor_define(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                SerikoCursorGesture::new(
                    GestureKind::MouseUp,
                    0,
                    "Head".to_string(),
                    "system:hand".to_string()
                )
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "mouseup0,Head,\r\n";
            assert!(seriko_cursor_define(case).is_err());
        }
    }

    mod gesture_kind {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "mouseup";
            let (remain, result) = gesture_kind(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, GestureKind::MouseUp);

            let case = "mousedown";
            let (remain, result) = gesture_kind(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, GestureKind::MouseDown);

            let case = "mouserightdown";
            let (remain, result) = gesture_kind(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, GestureKind::MouseRightDown);

            let case = "mousewheel";
            let (remain, result) = gesture_kind(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, GestureKind::MouseWheel);

            let case = "mousehover";
            let (remain, result) = gesture_kind(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, GestureKind::MouseHover);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "Mousehover";
            assert!(gesture_kind(case).is_err());
        }
    }
}

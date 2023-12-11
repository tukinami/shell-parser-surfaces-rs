use nom::{
    bytes::complete::{is_not, tag},
    combinator::map,
    sequence::{terminated, tuple},
    IResult,
};
use shell_parser_common_rs::ShellParseError;

use crate::{Brace, BraceContainer, SurfaceTargetCharacterId, Tooltip, TooltipInner};

use super::parts::{
    brace_name_func, header_comments_func, inner_brace_func, surface_target_character_id,
};

pub(super) fn brace_tooltip<'a>(
    input: &'a str,
) -> IResult<&'a str, BraceContainer, ShellParseError> {
    map(
        tuple((header_comments_func(tooltip_name), tooltip)),
        |(header_comments, body)| BraceContainer::new(header_comments, Brace::Tooltip(body)),
    )(input)
}

fn tooltip<'a>(input: &'a str) -> IResult<&'a str, Tooltip, ShellParseError> {
    map(
        tuple((tooltip_name, inner_brace_func(tooltip_inner))),
        |(id, lines)| Tooltip::new(id, lines),
    )(input)
}

fn tooltip_name<'a>(input: &'a str) -> IResult<&'a str, SurfaceTargetCharacterId, ShellParseError> {
    brace_name_func(terminated(surface_target_character_id, tag(".tooltips")))(input)
}

fn tooltip_inner<'a>(input: &'a str) -> IResult<&'a str, TooltipInner, ShellParseError> {
    map(
        tuple((terminated(is_not(",\r\n"), tag(",")), is_not("\r\n"))),
        |(collision, description): (&'a str, &'a str)| {
            TooltipInner::new(collision.to_string(), description.to_string())
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod brace_tooltip {

        use crate::{ast::SurfaceTargetCharacterId, CommentLine, LineContainer};

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = r#"

sakura.tooltips
{
Bust,怒ります。
Head,つつかれると痛いです。
Shoulder,つつくとコミュニケートボックスを表示します。
}"#;
            let (remain, result) = brace_tooltip(case).unwrap();
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
                &Brace::Tooltip(Tooltip::new(
                    SurfaceTargetCharacterId::Sakura,
                    vec![
                        LineContainer::Body(TooltipInner::new(
                            "Bust".to_string(),
                            "怒ります。".to_string()
                        )),
                        LineContainer::Body(TooltipInner::new(
                            "Head".to_string(),
                            "つつかれると痛いです。".to_string()
                        )),
                        LineContainer::Body(TooltipInner::new(
                            "Shoulder".to_string(),
                            "つつくとコミュニケートボックスを表示します。".to_string()
                        )),
                    ]
                ))
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = r#"sakura.tooooltips
{
}"#;
            assert!(brace_tooltip(case).is_err());
        }
    }

    mod tooltip {
        use crate::{ast::SurfaceTargetCharacterId, LineContainer};

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = r#"sakura.tooltips
{
Bust,怒ります。
Head,つつかれると痛いです。
Shoulder,つつくとコミュニケートボックスを表示します。
}"#;
            let (remain, result) = tooltip(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result.id(), &SurfaceTargetCharacterId::Sakura);
            assert_eq!(
                result.lines(),
                &vec![
                    LineContainer::Body(TooltipInner::new(
                        "Bust".to_string(),
                        "怒ります。".to_string()
                    )),
                    LineContainer::Body(TooltipInner::new(
                        "Head".to_string(),
                        "つつかれると痛いです。".to_string()
                    )),
                    LineContainer::Body(TooltipInner::new(
                        "Shoulder".to_string(),
                        "つつくとコミュニケートボックスを表示します。".to_string()
                    )),
                ]
            )
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = r#"sakura.tooooltips
{
}"#;
            assert!(tooltip(case).is_err());
        }
    }

    mod tooltip_name {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.tooltips\r\n{";
            let (remain, result) = tooltip_name(case).unwrap();
            assert_eq!(remain, "{");
            assert_eq!(result, SurfaceTargetCharacterId::Sakura);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "sakura.tooltips";
            assert!(tooltip_name(case).is_err());
        }
    }

    mod tooltip_inner {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "Shoulder,つつくとコミュニケートボックスを表示します。\r\n";
            let (remain, result) = tooltip_inner(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(
                result,
                TooltipInner::new(
                    "Shoulder".to_string(),
                    "つつくとコミュニケートボックスを表示します。".to_string()
                )
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "Shoulder";
            assert!(tooltip_inner(case).is_err());
        }
    }
}

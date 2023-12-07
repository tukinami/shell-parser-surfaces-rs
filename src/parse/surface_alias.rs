use nom::{
    bytes::complete::{is_not, tag},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    IResult,
};

use crate::{
    ast::{SurfaceAlias, SurfaceAliasInner},
    Brace, BraceContainer, SurfaceTargetCharacterId,
};

use super::{
    parts::{
        brace_name_func, digit, header_comments_func, inner_brace_func, surface_target_character_id,
    },
    SerikoParseError,
};

pub(super) fn brace_surface_alias<'a>(
    input: &'a str,
) -> IResult<&'a str, BraceContainer, SerikoParseError> {
    map(
        tuple((header_comments_func(surface_alias_name), surface_alias)),
        |(header_comments, body)| BraceContainer::new(header_comments, Brace::SurfaceAlias(body)),
    )(input)
}

fn surface_alias<'a>(input: &'a str) -> IResult<&'a str, SurfaceAlias, SerikoParseError> {
    map(
        tuple((surface_alias_name, inner_brace_func(surface_alias_inner))),
        |(id, lines)| SurfaceAlias::new(id, lines),
    )(input)
}

fn surface_alias_name<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceTargetCharacterId, SerikoParseError> {
    brace_name_func(terminated(
        surface_target_character_id,
        tag(".surface.alias"),
    ))(input)
}

fn surface_alias_inner<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceAliasInner, SerikoParseError> {
    map(
        tuple((
            is_not("},\r\n"),
            tag(","),
            delimited(tag("["), separated_list1(tag(","), digit), tag("]")),
        )),
        |(target, _, v)| SurfaceAliasInner::new(target.to_string(), v),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod brace_surface_alias {

        use crate::{CommentLine, LineContainer};

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = r#"

sakura.surface.alias
{
素,[0]
照れ,[1,101,201]
驚き,[2]
}"#;
            let (remain, result) = brace_surface_alias(case).unwrap();
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
                &Brace::SurfaceAlias(SurfaceAlias::new(
                    crate::ast::SurfaceTargetCharacterId::Sakura,
                    vec![
                        LineContainer::Body(SurfaceAliasInner::new("素".to_string(), vec![0])),
                        LineContainer::Body(SurfaceAliasInner::new(
                            "照れ".to_string(),
                            vec![1, 101, 201]
                        )),
                        LineContainer::Body(SurfaceAliasInner::new("驚き".to_string(), vec![2])),
                    ]
                ))
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = r#"
sakura.surface.alias{
素,[0]
照れ,[1,101,201]
驚き,[2]
}"#;
            assert!(brace_surface_alias(case).is_err());
        }
    }

    mod surface_alias {
        use crate::LineContainer;

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = r#"sakura.surface.alias
{
素,[0]
照れ,[1,101,201]
驚き,[2]
}"#;
            let (remain, result) = surface_alias(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                SurfaceAlias::new(
                    crate::ast::SurfaceTargetCharacterId::Sakura,
                    vec![
                        LineContainer::Body(SurfaceAliasInner::new("素".to_string(), vec![0])),
                        LineContainer::Body(SurfaceAliasInner::new(
                            "照れ".to_string(),
                            vec![1, 101, 201]
                        )),
                        LineContainer::Body(SurfaceAliasInner::new("驚き".to_string(), vec![2])),
                    ]
                )
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = r#"
sakura.surface.alias{
素,[0]
照れ,[1,101,201]
驚き,[2]
}"#;
            assert!(surface_alias(case).is_err());
        }
    }

    mod surface_alias_name {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.surface.alias\r\n{";
            let (remain, result) = surface_alias_name(case).unwrap();
            assert_eq!(remain, "{");
            assert_eq!(result, SurfaceTargetCharacterId::Sakura);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "sakura.surface.alias";
            assert!(surface_alias_name(case).is_err());
        }
    }

    mod surface_alias_define {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "照れ,[1,101,201]\r\n";
            let (remain, result) = surface_alias_inner(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(
                result,
                SurfaceAliasInner::new("照れ".to_string(), vec![1, 101, 201])
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "照れ,1,101,201\r\n";
            assert!(surface_alias_inner(case).is_err());
        }
    }
}

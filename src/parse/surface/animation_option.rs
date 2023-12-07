use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::{
    ast::{AnimationOptionKind, SurfaceAnimationOption},
    parse::{parts::digit, SerikoParseError},
};

enum AnimationOptionKindTemp {
    Exclusive,
    Background,
    SharedIndex,
}

pub(super) fn animation_option<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceAnimationOption, SerikoParseError> {
    alt((animation_option_v0, animation_option_v1))(input)
}

fn animation_option_v0<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceAnimationOption, SerikoParseError> {
    map(
        tuple((digit, tag("option,"), animation_option_kinds)),
        |(id, _, options)| SurfaceAnimationOption::new(id, options),
    )(input)
}

fn animation_option_v1<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceAnimationOption, SerikoParseError> {
    map(
        tuple((
            tag("animation"),
            digit,
            tag(".option,"),
            animation_option_kinds,
        )),
        |(_, id, _, options)| SurfaceAnimationOption::new(id, options),
    )(input)
}

fn animation_option_kinds<'a>(
    input: &'a str,
) -> IResult<&'a str, Vec<AnimationOptionKind>, SerikoParseError> {
    map(
        tuple((
            animation_option_kind_temps,
            opt(preceded(
                tag(","),
                delimited(tag("("), separated_list1(tag(","), digit), tag(")")),
            )),
        )),
        |(temps, ids)| {
            temps
                .iter()
                .map(|v| match v {
                    AnimationOptionKindTemp::Exclusive => {
                        AnimationOptionKind::Exclusive(ids.clone())
                    }
                    AnimationOptionKindTemp::Background => AnimationOptionKind::Background,
                    AnimationOptionKindTemp::SharedIndex => AnimationOptionKind::SharedIndex,
                })
                .collect()
        },
    )(input)
}

fn animation_option_kind_temps<'a>(
    input: &'a str,
) -> IResult<&'a str, Vec<AnimationOptionKindTemp>, SerikoParseError> {
    separated_list1(tag("+"), animation_option_kind_temp)(input)
}

fn animation_option_kind_temp<'a>(
    input: &'a str,
) -> IResult<&'a str, AnimationOptionKindTemp, SerikoParseError> {
    alt((
        map(tag("exclusive"), |_| AnimationOptionKindTemp::Exclusive),
        map(tag("background"), |_| AnimationOptionKindTemp::Background),
        map(tag("shared-index"), |_| {
            AnimationOptionKindTemp::SharedIndex
        }),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod animation_option {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "5option,exclusive";
            let (remain, result) = animation_option(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                SurfaceAnimationOption::new(5, vec![AnimationOptionKind::Exclusive(None)])
            );

            let case = "animation5.option,exclusive+background,(1,3,5)";
            let (remain, result) = animation_option(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                SurfaceAnimationOption::new(
                    5,
                    vec![
                        AnimationOptionKind::Exclusive(Some(vec![1, 3, 5])),
                        AnimationOptionKind::Background,
                    ]
                )
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "animation5.option,exclusiv";
            assert!(animation_option(case).is_err());
        }
    }

    mod animation_option_v0 {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "5option,exclusive";
            let (remain, result) = animation_option_v0(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                SurfaceAnimationOption::new(5, vec![AnimationOptionKind::Exclusive(None)])
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "animation5.option,exclusive+background,(1,3,5)";
            assert!(animation_option_v0(case).is_err());
        }
    }

    mod animation_option_v1 {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "animation5.option,exclusive+background,(1,3,5)";
            let (remain, result) = animation_option_v1(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                SurfaceAnimationOption::new(
                    5,
                    vec![
                        AnimationOptionKind::Exclusive(Some(vec![1, 3, 5])),
                        AnimationOptionKind::Background,
                    ]
                )
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "5option,exclusive";
            assert!(animation_option_v1(case).is_err());
        }
    }

    mod animation_option_kinds {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "exclusive+background,(1,3,5)";
            let (remain, result) = animation_option_kinds(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                vec![
                    AnimationOptionKind::Exclusive(Some(vec![1, 3, 5])),
                    AnimationOptionKind::Background
                ]
            );
            let case = "shared-index";
            let (remain, result) = animation_option_kinds(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, vec![AnimationOptionKind::SharedIndex]);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "+background";
            assert!(animation_option_kinds(case).is_err());
        }
    }
}

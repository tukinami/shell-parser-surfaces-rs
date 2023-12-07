use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};

use crate::{
    ast::{
        AnimationPatternDrawMethod, AnimationPatternProperty, SurfaceAnimationPattern,
        SurfaceIdPointerType,
    },
    parse::{
        parts::{digit, digit_neg},
        SerikoParseError,
    },
};

use super::draw_method::{draw_method, draw_method_on_animation};

pub(super) fn animation_pattern<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceAnimationPattern, SerikoParseError> {
    alt((animation_pattern_v0, animation_pattern_v1))(input)
}

fn animation_pattern_v0<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceAnimationPattern, SerikoParseError> {
    map(
        tuple((
            digit,
            tag("pattern"),
            digit,
            alt((
                animation_pattern_draw_method_normal_v0,
                animation_pattern_draw_method_animation_v0,
            )),
        )),
        |(id, _, pattern_id, method)| SurfaceAnimationPattern::new(id, pattern_id, method),
    )(input)
}

fn animation_pattern_draw_method_normal_v0<'a>(
    input: &'a str,
) -> IResult<&'a str, AnimationPatternDrawMethod, SerikoParseError> {
    map(
        tuple((
            preceded(tag(","), digit_neg),
            preceded(tag(","), digit::<u32>),
            preceded(tag(","), draw_method),
            preceded(tag(","), digit_neg),
            preceded(tag(","), digit_neg),
        )),
        |(surface_id, weight, dm, x, y)| {
            let prop = AnimationPatternProperty::new(surface_id, weight * 10, x, y);
            AnimationPatternDrawMethod::Normal(dm, prop)
        },
    )(input)
}

fn animation_pattern_draw_method_animation_v0<'a>(
    input: &'a str,
) -> IResult<&'a str, AnimationPatternDrawMethod, SerikoParseError> {
    map(
        tuple((
            preceded(tag(","), digit_neg::<SurfaceIdPointerType>),
            preceded(tag(","), digit::<u32>),
            preceded(tag(","), draw_method_on_animation),
        )),
        |(_, _, dm)| AnimationPatternDrawMethod::Animation(dm),
    )(input)
}

fn animation_pattern_v1<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceAnimationPattern, SerikoParseError> {
    map(
        tuple((
            preceded(tag("animation"), digit),
            preceded(tag(".pattern"), digit),
            alt((
                animation_pattern_draw_method_normal_v1,
                animation_pattern_draw_method_animation_v1,
            )),
        )),
        |(id, pattern_id, dm)| SurfaceAnimationPattern::new(id, pattern_id, dm),
    )(input)
}

fn animation_pattern_draw_method_normal_v1<'a>(
    input: &'a str,
) -> IResult<&'a str, AnimationPatternDrawMethod, SerikoParseError> {
    map(
        tuple((
            preceded(tag(","), draw_method),
            animation_pattern_property_v1,
        )),
        |(dm, app)| AnimationPatternDrawMethod::Normal(dm, app),
    )(input)
}

fn animation_pattern_draw_method_animation_v1<'a>(
    input: &'a str,
) -> IResult<&'a str, AnimationPatternDrawMethod, SerikoParseError> {
    map(preceded(tag(","), draw_method_on_animation), |v| {
        AnimationPatternDrawMethod::Animation(v)
    })(input)
}

fn animation_pattern_property_v1<'a>(
    input: &'a str,
) -> IResult<&'a str, AnimationPatternProperty, SerikoParseError> {
    map(
        tuple((
            preceded(tag(","), digit_neg),
            preceded(tag(","), digit),
            preceded(tag(","), digit_neg),
            preceded(tag(","), digit_neg),
        )),
        |(surface_id, weight, x, y)| AnimationPatternProperty::new(surface_id, weight, x, y),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod animation_pattern {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "animation0.pattern1,overlay,-1,2,3,4";
            let (remain, result) = animation_pattern(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result.id(), &0);
            assert_eq!(result.pattern_id(), &1);
            assert_eq!(
                result.method(),
                &AnimationPatternDrawMethod::Normal(
                    crate::ast::DrawMethod::Overlay,
                    AnimationPatternProperty::new(-1, 2, 3, 4)
                )
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "0pattern0,0,0,alternativestart,";
            assert!(animation_pattern(case).is_err());
        }
    }

    mod animation_pattern_v0 {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "3pattern4,10,20,alternativestart,[1,2]";
            let (remain, result) = animation_pattern_v0(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result.id(), &3);
            assert_eq!(result.pattern_id(), &4);
            assert_eq!(
                result.method(),
                &AnimationPatternDrawMethod::Animation(
                    crate::ast::DrawMethodOnAnimation::Alternativestart(vec![1, 2])
                )
            );

            let case = "1pattern3,-1,70,overlay,0,0";
            let (remain, result) = animation_pattern_v0(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result.id(), &1);
            assert_eq!(result.pattern_id(), &3);
            assert_eq!(
                result.method(),
                &AnimationPatternDrawMethod::Normal(
                    crate::ast::DrawMethod::Overlay,
                    AnimationPatternProperty::new(-1, 700, 0, 0)
                )
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "animation0.pattern0,alternativestart,(1,2)";
            assert!(animation_pattern_v0(case).is_err());
        }
    }

    mod animation_pattern_draw_method_normal_v0 {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = ",101,70,overlay,200,100";
            let (remain, result) = animation_pattern_draw_method_normal_v0(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                AnimationPatternDrawMethod::Normal(
                    crate::ast::DrawMethod::Overlay,
                    AnimationPatternProperty::new(101, 700, 200, 100)
                )
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = ",0,0,0,0";
            assert!(animation_pattern_draw_method_normal_v0(case).is_err());
        }
    }

    mod animation_pattern_draw_method_animation_v0 {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = ",0,0,insert,10";
            let (remain, result) = animation_pattern_draw_method_animation_v0(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                AnimationPatternDrawMethod::Animation(crate::ast::DrawMethodOnAnimation::Insert(
                    10
                ))
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = ",insert,10";
            assert!(animation_pattern_draw_method_animation_v0(case).is_err());
        }
    }

    mod animation_pattern_v1 {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "animation3.pattern4,alternativestart,(1,2)";
            let (remain, result) = animation_pattern_v1(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result.id(), &3);
            assert_eq!(result.pattern_id(), &4);
            assert_eq!(
                result.method(),
                &AnimationPatternDrawMethod::Animation(
                    crate::ast::DrawMethodOnAnimation::Alternativestart(vec![1, 2])
                )
            );

            let case = "animation1.pattern3,overlay,-1,700,0,0";
            let (remain, result) = animation_pattern_v1(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result.id(), &1);
            assert_eq!(result.pattern_id(), &3);
            assert_eq!(
                result.method(),
                &AnimationPatternDrawMethod::Normal(
                    crate::ast::DrawMethod::Overlay,
                    AnimationPatternProperty::new(-1, 700, 0, 0)
                )
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "0pattern0,0,0,alternativestart,[1,2]";
            assert!(animation_pattern_v1(case).is_err());
        }
    }

    mod animation_pattern_property_v1 {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = ",-1,60,10,10\r\n";
            let (remain, result) = animation_pattern_property_v1(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result.surface_id(), &-1);
            assert_eq!(result.weight(), &60);
            assert_eq!(result.x(), &10);
            assert_eq!(result.y(), &10);
        }

        #[test]
        fn failed_when_inalid_str() {
            let case = ",-1,60,60";
            assert!(animation_pattern_property_v1(case).is_err());
        }
    }
}

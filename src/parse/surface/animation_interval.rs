use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::separated_list1, sequence::tuple,
    IResult,
};

use crate::{
    ast::{AnimationInterval, SurfaceAnimationInterval},
    parse::{parts::digit, SerikoParseError},
};

pub(super) fn animation_interval<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceAnimationInterval, SerikoParseError> {
    alt((animation_interval_v0, animation_interval_v1))(input)
}

fn animation_interval_v0<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceAnimationInterval, SerikoParseError> {
    map(
        tuple((digit, tag("interval,"), animation_interval_defines)),
        |(id, _, intervals)| SurfaceAnimationInterval::new(id, intervals),
    )(input)
}

fn animation_interval_v1<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceAnimationInterval, SerikoParseError> {
    map(
        tuple((
            tag("animation"),
            digit,
            tag(".interval,"),
            animation_interval_defines,
        )),
        |(_, id, _, intervals)| SurfaceAnimationInterval::new(id, intervals),
    )(input)
}

fn animation_interval_defines<'a>(
    input: &'a str,
) -> IResult<&'a str, Vec<AnimationInterval>, SerikoParseError> {
    separated_list1(tag("+"), animation_interval_define)(input)
}

fn animation_interval_define<'a>(
    input: &'a str,
) -> IResult<&'a str, AnimationInterval, SerikoParseError> {
    alt((
        animation_interval_define_sometimes,
        animation_interval_define_rarely,
        animation_interval_define_random,
        animation_interval_define_periodic,
        animation_interval_define_always,
        animation_interval_define_runonce,
        animation_interval_define_never,
        animation_interval_define_yen_e,
        animation_interval_define_talk,
        animation_interval_define_bind,
    ))(input)
}

fn animation_interval_define_sometimes<'a>(
    input: &'a str,
) -> IResult<&'a str, AnimationInterval, SerikoParseError> {
    map(tag("sometimes"), |_| AnimationInterval::Sometimes)(input)
}

fn animation_interval_define_rarely<'a>(
    input: &'a str,
) -> IResult<&'a str, AnimationInterval, SerikoParseError> {
    map(tag("rarely"), |_| AnimationInterval::Rarely)(input)
}

fn animation_interval_define_random<'a>(
    input: &'a str,
) -> IResult<&'a str, AnimationInterval, SerikoParseError> {
    map(tuple((tag("random,"), digit)), |(_, v)| {
        AnimationInterval::Random(v)
    })(input)
}

fn animation_interval_define_periodic<'a>(
    input: &'a str,
) -> IResult<&'a str, AnimationInterval, SerikoParseError> {
    map(tuple((tag("periodic,"), digit)), |(_, v)| {
        AnimationInterval::Periodic(v)
    })(input)
}

fn animation_interval_define_always<'a>(
    input: &'a str,
) -> IResult<&'a str, AnimationInterval, SerikoParseError> {
    map(tag("always"), |_| AnimationInterval::Always)(input)
}

fn animation_interval_define_runonce<'a>(
    input: &'a str,
) -> IResult<&'a str, AnimationInterval, SerikoParseError> {
    map(tag("runonce"), |_| AnimationInterval::Runonce)(input)
}

fn animation_interval_define_never<'a>(
    input: &'a str,
) -> IResult<&'a str, AnimationInterval, SerikoParseError> {
    map(tag("never"), |_| AnimationInterval::Never)(input)
}

fn animation_interval_define_yen_e<'a>(
    input: &'a str,
) -> IResult<&'a str, AnimationInterval, SerikoParseError> {
    map(tag("yen-e"), |_| AnimationInterval::YenE)(input)
}

fn animation_interval_define_talk<'a>(
    input: &'a str,
) -> IResult<&'a str, AnimationInterval, SerikoParseError> {
    map(tuple((tag("talk,"), digit)), |(_, v)| {
        AnimationInterval::Talk(v)
    })(input)
}

fn animation_interval_define_bind<'a>(
    input: &'a str,
) -> IResult<&'a str, AnimationInterval, SerikoParseError> {
    map(tag("bind"), |_| AnimationInterval::Bind)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod animation_interval {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "2interval,never";
            let (remain, result) = animation_interval(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                SurfaceAnimationInterval::new(2, vec![AnimationInterval::Never])
            );

            let case = "animation2.interval,never";
            let (remain, result) = animation_interval(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                SurfaceAnimationInterval::new(2, vec![AnimationInterval::Never])
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "animation2.interval,ne";
            assert!(animation_interval(case).is_err());
        }
    }

    mod animation_interval_v0 {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "2interval,never";
            let (remain, result) = animation_interval_v0(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                SurfaceAnimationInterval::new(2, vec![AnimationInterval::Never])
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "animation2.interval,never";
            assert!(animation_interval_v0(case).is_err());
        }
    }

    mod animation_interval_v1 {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "animation2.interval,never";
            let (remain, result) = animation_interval_v1(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                SurfaceAnimationInterval::new(2, vec![AnimationInterval::Never])
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "2interval,never";
            assert!(animation_interval_v1(case).is_err());
        }
    }

    mod animation_interval_defines {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "bind+always";
            let (remain, result) = animation_interval_defines(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                vec![AnimationInterval::Bind, AnimationInterval::Always]
            );

            let case = "bind+runonce";
            let (remain, result) = animation_interval_defines(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                vec![AnimationInterval::Bind, AnimationInterval::Runonce]
            );

            let case = "bind+random,10";
            let (remain, result) = animation_interval_defines(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                vec![AnimationInterval::Bind, AnimationInterval::Random(10)]
            );

            let case = "bind+periodic,20";
            let (remain, result) = animation_interval_defines(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                vec![AnimationInterval::Bind, AnimationInterval::Periodic(20)]
            );

            let case = "bind+runonce+random,5";
            let (remain, result) = animation_interval_defines(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                vec![
                    AnimationInterval::Bind,
                    AnimationInterval::Runonce,
                    AnimationInterval::Random(5)
                ]
            );

            let case = "bind";
            let (remain, result) = animation_interval_defines(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, vec![AnimationInterval::Bind]);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "somethingwrong";
            assert!(animation_interval_defines(case).is_err());
        }
    }

    mod animation_interval_define {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sometimes";
            let (remain, result) = animation_interval_define(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, AnimationInterval::Sometimes);

            let case = "rarely";
            let (remain, result) = animation_interval_define(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, AnimationInterval::Rarely);

            let case = "random,10";
            let (remain, result) = animation_interval_define(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, AnimationInterval::Random(10));

            let case = "periodic,20";
            let (remain, result) = animation_interval_define(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, AnimationInterval::Periodic(20));

            let case = "always";
            let (remain, result) = animation_interval_define(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, AnimationInterval::Always);

            let case = "runonce";
            let (remain, result) = animation_interval_define(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, AnimationInterval::Runonce);

            let case = "never";
            let (remain, result) = animation_interval_define(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, AnimationInterval::Never);

            let case = "yen-e";
            let (remain, result) = animation_interval_define(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, AnimationInterval::YenE);

            let case = "talk,5";
            let (remain, result) = animation_interval_define(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, AnimationInterval::Talk(5));

            let case = "bind";
            let (remain, result) = animation_interval_define(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, AnimationInterval::Bind);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "somethingwrong";
            assert!(animation_interval_define(case).is_err());
        }
    }
}

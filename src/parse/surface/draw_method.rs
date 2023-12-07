use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::many1,
    sequence::{delimited, pair, terminated},
    IResult,
};

use crate::{
    ast::{AnimationIdType, DrawMethod, DrawMethodOnAnimation},
    parse::{parts::digit, SerikoParseError},
};

pub(super) fn draw_method<'a>(input: &'a str) -> IResult<&'a str, DrawMethod, SerikoParseError> {
    alt((
        draw_method_base,
        draw_method_overlayfast,
        draw_method_overlaymultiply,
        draw_method_overlay,
        draw_method_replace,
        draw_method_interpolate,
        draw_method_asis,
        draw_method_move,
        draw_method_bind,
        draw_method_add,
        draw_method_reduce,
    ))(input)
}

pub(super) fn draw_method_on_animation<'a>(
    input: &'a str,
) -> IResult<&'a str, DrawMethodOnAnimation, SerikoParseError> {
    alt((
        draw_method_insert,
        draw_method_start,
        draw_method_stop,
        draw_method_alternativestart,
        draw_method_alternativestop,
        draw_method_parallelstart,
        draw_method_parallelstop,
    ))(input)
}

fn draw_method_base<'a>(input: &'a str) -> IResult<&'a str, DrawMethod, SerikoParseError> {
    map(tag("base"), |_| DrawMethod::Base)(input)
}

fn draw_method_overlay<'a>(input: &'a str) -> IResult<&'a str, DrawMethod, SerikoParseError> {
    map(tag("overlay"), |_| DrawMethod::Overlay)(input)
}

fn draw_method_overlayfast<'a>(input: &'a str) -> IResult<&'a str, DrawMethod, SerikoParseError> {
    map(tag("overlayfast"), |_| DrawMethod::Overlayfast)(input)
}

fn draw_method_overlaymultiply<'a>(
    input: &'a str,
) -> IResult<&'a str, DrawMethod, SerikoParseError> {
    map(tag("overlaymultiply"), |_| DrawMethod::Overlaymultiply)(input)
}

fn draw_method_replace<'a>(input: &'a str) -> IResult<&'a str, DrawMethod, SerikoParseError> {
    map(tag("replace"), |_| DrawMethod::Replace)(input)
}

fn draw_method_interpolate<'a>(input: &'a str) -> IResult<&'a str, DrawMethod, SerikoParseError> {
    map(tag("interpolate"), |_| DrawMethod::Interpolate)(input)
}

fn draw_method_asis<'a>(input: &'a str) -> IResult<&'a str, DrawMethod, SerikoParseError> {
    map(tag("asis"), |_| DrawMethod::Asis)(input)
}

fn draw_method_move<'a>(input: &'a str) -> IResult<&'a str, DrawMethod, SerikoParseError> {
    map(tag("move"), |_| DrawMethod::Move)(input)
}

fn draw_method_bind<'a>(input: &'a str) -> IResult<&'a str, DrawMethod, SerikoParseError> {
    map(tag("bind"), |_| DrawMethod::Bind)(input)
}

fn draw_method_add<'a>(input: &'a str) -> IResult<&'a str, DrawMethod, SerikoParseError> {
    map(tag("add"), |_| DrawMethod::Add)(input)
}

fn draw_method_reduce<'a>(input: &'a str) -> IResult<&'a str, DrawMethod, SerikoParseError> {
    map(tag("reduce"), |_| DrawMethod::Reduce)(input)
}

fn draw_method_insert<'a>(
    input: &'a str,
) -> IResult<&'a str, DrawMethodOnAnimation, SerikoParseError> {
    map(pair(tag("insert,"), digit), |(_, v)| {
        DrawMethodOnAnimation::Insert(v)
    })(input)
}

fn draw_method_start<'a>(
    input: &'a str,
) -> IResult<&'a str, DrawMethodOnAnimation, SerikoParseError> {
    map(pair(tag("start,"), digit), |(_, v)| {
        DrawMethodOnAnimation::Start(v)
    })(input)
}

fn draw_method_stop<'a>(
    input: &'a str,
) -> IResult<&'a str, DrawMethodOnAnimation, SerikoParseError> {
    map(pair(tag("stop,"), digit), |(_, v)| {
        DrawMethodOnAnimation::Stop(v)
    })(input)
}

fn draw_method_alternativestart<'a>(
    input: &'a str,
) -> IResult<&'a str, DrawMethodOnAnimation, SerikoParseError> {
    map(pair(tag("alternativestart,"), ids), |(_, v)| {
        DrawMethodOnAnimation::Alternativestart(v)
    })(input)
}

fn draw_method_alternativestop<'a>(
    input: &'a str,
) -> IResult<&'a str, DrawMethodOnAnimation, SerikoParseError> {
    map(pair(tag("alternativestop,"), ids), |(_, v)| {
        DrawMethodOnAnimation::Alternativestop(v)
    })(input)
}

fn draw_method_parallelstart<'a>(
    input: &'a str,
) -> IResult<&'a str, DrawMethodOnAnimation, SerikoParseError> {
    map(pair(tag("parallelstart,"), ids), |(_, v)| {
        DrawMethodOnAnimation::Parallelstart(v)
    })(input)
}

fn draw_method_parallelstop<'a>(
    input: &'a str,
) -> IResult<&'a str, DrawMethodOnAnimation, SerikoParseError> {
    map(pair(tag("parallelstop,"), ids), |(_, v)| {
        DrawMethodOnAnimation::Parallelstop(v)
    })(input)
}

fn ids<'a>(input: &'a str) -> IResult<&'a str, Vec<AnimationIdType>, SerikoParseError> {
    alt((ids_bracket, ids_parenthesis))(input)
}

fn ids_bracket<'a>(input: &'a str) -> IResult<&'a str, Vec<AnimationIdType>, SerikoParseError> {
    delimited(tag("["), ids_inner, tag("]"))(input)
}

fn ids_parenthesis<'a>(input: &'a str) -> IResult<&'a str, Vec<AnimationIdType>, SerikoParseError> {
    delimited(tag("("), ids_inner, tag(")"))(input)
}

fn ids_inner<'a>(input: &'a str) -> IResult<&'a str, Vec<AnimationIdType>, SerikoParseError> {
    alt((ids_inner_comma, ids_inner_period, map(digit, |v| vec![v])))(input)
}

fn ids_inner_comma<'a>(input: &'a str) -> IResult<&'a str, Vec<AnimationIdType>, SerikoParseError> {
    ids_inner_body(",")(input)
}

fn ids_inner_period<'a>(
    input: &'a str,
) -> IResult<&'a str, Vec<AnimationIdType>, SerikoParseError> {
    ids_inner_body(".")(input)
}

fn ids_inner_body<'a>(
    tag_str: &'static str,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<AnimationIdType>, SerikoParseError> {
    map(
        pair(many1(terminated(digit, tag(tag_str))), digit),
        |(mut vec, v)| {
            vec.push(v);
            vec
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    mod draw_method {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "base\r\n";
            let (remain, result) = draw_method(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, DrawMethod::Base);

            let case = "overlay";
            let (remain, result) = draw_method(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, DrawMethod::Overlay);

            let case = "overlayfast";
            let (remain, result) = draw_method(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, DrawMethod::Overlayfast);

            let case = "overlaymultiply";
            let (remain, result) = draw_method(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, DrawMethod::Overlaymultiply);

            let case = "replace";
            let (remain, result) = draw_method(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, DrawMethod::Replace);

            let case = "interpolate";
            let (remain, result) = draw_method(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, DrawMethod::Interpolate);

            let case = "asis";
            let (remain, result) = draw_method(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, DrawMethod::Asis);

            let case = "move";
            let (remain, result) = draw_method(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, DrawMethod::Move);

            let case = "bind";
            let (remain, result) = draw_method(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, DrawMethod::Bind);

            let case = "add";
            let (remain, result) = draw_method(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, DrawMethod::Add);

            let case = "reduce";
            let (remain, result) = draw_method(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, DrawMethod::Reduce);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "somethingwrong";
            assert!(draw_method(case).is_err());
        }
    }

    mod draw_method_on_animation {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "insert,10";
            let (remain, result) = draw_method_on_animation(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, DrawMethodOnAnimation::Insert(10));

            let case = "start,10";
            let (remain, result) = draw_method_on_animation(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, DrawMethodOnAnimation::Start(10));

            let case = "stop,10";
            let (remain, result) = draw_method_on_animation(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, DrawMethodOnAnimation::Stop(10));

            let case = "alternativestart,[10,20]";
            let (remain, result) = draw_method_on_animation(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                DrawMethodOnAnimation::Alternativestart(vec![10, 20])
            );

            let case = "alternativestop,[10,20]";
            let (remain, result) = draw_method_on_animation(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, DrawMethodOnAnimation::Alternativestop(vec![10, 20]));

            let case = "parallelstart,[10,20]";
            let (remain, result) = draw_method_on_animation(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, DrawMethodOnAnimation::Parallelstart(vec![10, 20]));

            let case = "parallelstop,[10,20]";
            let (remain, result) = draw_method_on_animation(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, DrawMethodOnAnimation::Parallelstop(vec![10, 20]));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "somethingwrong";
            assert!(draw_method_on_animation(case).is_err());
        }
    }

    mod ids {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "[10,20,30]";
            let (remain, result) = ids(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, vec![10, 20, 30]);

            let case = "(10.20.30)";
            let (remain, result) = ids(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, vec![10, 20, 30]);

            let case = "[10]a";
            let (remain, result) = ids(case).unwrap();
            assert_eq!(remain, "a");
            assert_eq!(result, vec![10]);

            let case = "(10)a";
            let (remain, result) = ids(case).unwrap();
            assert_eq!(remain, "a");
            assert_eq!(result, vec![10]);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "{10}";
            assert!(ids(case).is_err());
        }
    }

    mod ids_bracket {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "[10,20,30]";
            let (remain, result) = ids_bracket(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, vec![10, 20, 30]);

            let case = "[10.20.30]";
            let (remain, result) = ids_bracket(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, vec![10, 20, 30]);

            let case = "[10]a";
            let (remain, result) = ids_bracket(case).unwrap();
            assert_eq!(remain, "a");
            assert_eq!(result, vec![10]);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "(10,20,30)";
            assert!(ids_bracket(case).is_err());
        }
    }

    mod ids_parenthesis {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "(10,20,30)";
            let (remain, result) = ids_parenthesis(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, vec![10, 20, 30]);

            let case = "(10.20.30)";
            let (remain, result) = ids_parenthesis(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, vec![10, 20, 30]);

            let case = "(10)a";
            let (remain, result) = ids_parenthesis(case).unwrap();
            assert_eq!(remain, "a");
            assert_eq!(result, vec![10]);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "[10,20,30]";
            assert!(ids_parenthesis(case).is_err());
        }
    }

    mod ids_inner {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "10,20,30)";
            let (remain, result) = ids_inner(case).unwrap();
            assert_eq!(remain, ")");
            assert_eq!(result, vec![10, 20, 30]);

            let case = "10.20.30)";
            let (remain, result) = ids_inner(case).unwrap();
            assert_eq!(remain, ")");
            assert_eq!(result, vec![10, 20, 30]);

            let case = "10";
            let (remain, result) = ids_inner(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, vec![10]);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "a-20-30";
            assert!(ids_inner(case).is_err());
        }
    }

    mod ids_inner_comma {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "10,20,30";
            let (remain, result) = ids_inner_comma(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, vec![10, 20, 30]);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "aa,bb";
            assert!(ids_inner_comma(case).is_err());
        }
    }

    mod ids_inner_period {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "10.20.30";
            let (remain, result) = ids_inner_period(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, vec![10, 20, 30]);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "aa.bb";
            assert!(ids_inner_period(case).is_err());
        }
    }
}

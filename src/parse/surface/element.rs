use nom::{
    bytes::complete::{is_not, tag},
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};
use shell_parser_common_rs::ShellParseError;

use crate::{
    ast::SurfaceElement,
    parse::parts::{digit, digit_neg},
};

use super::draw_method::draw_method;

pub(super) fn element<'a>(input: &'a str) -> IResult<&'a str, SurfaceElement, ShellParseError> {
    map(
        tuple((
            tag("element"),
            digit,
            preceded(tag(","), draw_method),
            preceded(tag(","), is_not(",")),
            preceded(tag(","), digit_neg),
            preceded(tag(","), digit_neg),
        )),
        |(_, id, method, filename, x, y)| {
            SurfaceElement::new(id, method, filename.to_string(), x, y)
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod element {
        use crate::ast::DrawMethod;

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "element0,overlay,body0.png,0,0";
            let (remain, result) = element(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result.id(), &0);
            assert_eq!(result.method(), &DrawMethod::Overlay);
            assert_eq!(result.filename(), &"body0.png".to_string());
            assert_eq!(result.x(), &0);
            assert_eq!(result.y(), &0);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "element,overlay,s.png,0";
            assert!(element(case).is_err());
        }
    }
}

use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::tuple, IResult};
use shell_parser_common_rs::ShellParseError;

use crate::{
    ast::{Descript, SortOrder},
    Brace, BraceContainer, DescriptInner,
};

use super::parts::{brace_name_func, digit, header_comments_func, inner_brace_func};

pub(super) fn brace_descript<'a>(
    input: &'a str,
) -> IResult<&'a str, BraceContainer, ShellParseError> {
    map(
        tuple((header_comments_func(descript_name), descript)),
        |(header_comments, body)| BraceContainer::new(header_comments, Brace::Descript(body)),
    )(input)
}

fn descript<'a>(input: &'a str) -> IResult<&'a str, Descript, ShellParseError> {
    map(
        tuple((descript_name, inner_brace_func(descript_inner))),
        |(_, lines)| Descript::new(lines),
    )(input)
}

fn descript_name<'a>(input: &'a str) -> IResult<&'a str, &'a str, ShellParseError> {
    brace_name_func(tag("descript"))(input)
}

fn descript_inner<'a>(input: &'a str) -> IResult<&'a str, DescriptInner, ShellParseError> {
    alt((version, max_width, collision_sort, animation_sort))(input)
}

fn version<'a>(input: &'a str) -> IResult<&'a str, DescriptInner, ShellParseError> {
    map(tuple((tag("version,"), digit)), |(_, v)| {
        DescriptInner::Version(v)
    })(input)
}

fn max_width<'a>(input: &'a str) -> IResult<&'a str, DescriptInner, ShellParseError> {
    map(tuple((tag("maxwidth,"), digit)), |(_, v)| {
        DescriptInner::MaxWidth(v)
    })(input)
}

fn collision_sort<'a>(input: &'a str) -> IResult<&'a str, DescriptInner, ShellParseError> {
    map(tuple((tag("collision-sort,"), sort_order)), |(_, v)| {
        DescriptInner::CollistionSort(v)
    })(input)
}

fn animation_sort<'a>(input: &'a str) -> IResult<&'a str, DescriptInner, ShellParseError> {
    map(tuple((tag("animation-sort,"), sort_order)), |(_, v)| {
        DescriptInner::AnimationSort(v)
    })(input)
}

fn sort_order<'a>(input: &'a str) -> IResult<&'a str, SortOrder, ShellParseError> {
    alt((sort_order_ascend, sort_order_descend))(input)
}

fn sort_order_ascend<'a>(input: &'a str) -> IResult<&'a str, SortOrder, ShellParseError> {
    map(tag("ascend"), |_| SortOrder::Ascend)(input)
}

fn sort_order_descend<'a>(input: &'a str) -> IResult<&'a str, SortOrder, ShellParseError> {
    map(tag("descend"), |_| SortOrder::Descend)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod brace_descript {
        use super::*;
        use crate::{CommentLine, LineContainer};

        #[test]
        fn success_when_valid_str() {
            let case = r#"

descript
    {
        version,1
        maxwidth,320
    }
"#;
            let (remain, result) = brace_descript(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result.header_comments(),
                &vec![
                    CommentLine::new("".to_string()),
                    CommentLine::new("".to_string())
                ]
            );
            assert_eq!(
                result.body(),
                &Brace::Descript(Descript::new(vec![
                    LineContainer::Body(DescriptInner::Version(1)),
                    LineContainer::Body(DescriptInner::MaxWidth(320))
                ]))
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "descript{}";
            assert!(brace_descript(case).is_err());
        }
    }

    mod descript {
        use crate::{CommentLine, LineContainer};

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = r#"descript
    {
        version,1
        maxwidth,320
    }
"#;
            let (remain, result) = descript(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result.lines(),
                &vec![
                    LineContainer::Body(DescriptInner::Version(1)),
                    LineContainer::Body(DescriptInner::MaxWidth(320))
                ]
            );

            let case = r#"descript
    {
        version,1
        maxwidth,320
collision-sort,ascend
animation-sort,descend

    }"#;
            let (remain, result) = descript(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result.lines(),
                &vec![
                    LineContainer::Body(DescriptInner::Version(1)),
                    LineContainer::Body(DescriptInner::MaxWidth(320)),
                    LineContainer::Body(DescriptInner::CollistionSort(SortOrder::Ascend)),
                    LineContainer::Body(DescriptInner::AnimationSort(SortOrder::Descend)),
                    LineContainer::Comment(CommentLine::new("".to_string())),
                ]
            );

            let case = r#"descript
{
}
"#;
            assert!(descript(case).is_ok());
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = r#"
descript {
version,1
}
"#;
            assert!(descript(case).is_err());

            let case = r#"
descript
{
version,1}
"#;
            assert!(descript(case).is_err());
        }
    }

    mod descript_name {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "descript\r\n{";
            let (remain, result) = descript_name(case).unwrap();
            assert_eq!(remain, "{");
            assert_eq!(result, "descript");
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "descipt";
            assert!(descript_name(case).is_err());
        }
    }

    mod version {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "version,1\r\n";
            let (remain, result) = version(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, DescriptInner::Version(1));

            let case = "version,0";
            let (remain, result) = version(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, DescriptInner::Version(0));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "version,-1\r\n";
            assert!(version(case).is_err());

            let case = "vertion,1\r\n";
            assert!(version(case).is_err());
        }
    }

    mod max_width {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "maxwidth,100\r\n";
            let (remain, result) = max_width(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, DescriptInner::MaxWidth(100));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "maxwidth,-1\r\n";
            assert!(max_width(case).is_err());

            let case = "vertion,1\r\n";
            assert!(max_width(case).is_err());
        }
    }

    mod collision_sort {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "collision-sort,ascend\r\n";
            let (remain, result) = collision_sort(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, DescriptInner::CollistionSort(SortOrder::Ascend));

            let case = "collision-sort,descend\r\n";
            let (remain, result) = collision_sort(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, DescriptInner::CollistionSort(SortOrder::Descend));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "collision-sort,desssend\r\n";
            assert!(collision_sort(case).is_err());

            let case = "vertion,1\r\n";
            assert!(collision_sort(case).is_err());
        }
    }

    mod animation_sort {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "animation-sort,ascend\r\n";
            let (remain, result) = animation_sort(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, DescriptInner::AnimationSort(SortOrder::Ascend));

            let case = "animation-sort,descend\r\n";
            let (remain, result) = animation_sort(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, DescriptInner::AnimationSort(SortOrder::Descend));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "animation-sort,desssend\r\n";
            assert!(animation_sort(case).is_err());

            let case = "vertion,1\r\n";
            assert!(animation_sort(case).is_err());
        }
    }
}

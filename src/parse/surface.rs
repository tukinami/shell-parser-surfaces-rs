use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::separated_list1,
    sequence::tuple,
    sequence::{preceded, separated_pair},
    IResult,
};

use crate::{
    Brace, BraceContainer, SerikoParseError, Surface, SurfaceAppend, SurfaceId, SurfaceInner,
};

use self::{
    animation_collision::{animation_collision, animation_collision_ex},
    animation_interval::animation_interval,
    animation_option::animation_option,
    animation_pattern::animation_pattern,
    collision::{collision, collision_ex},
    element::element,
    offset::{
        balloon_offset_x, balloon_offset_y, kero_balloon_offset_x, kero_balloon_offset_y,
        point_basepos_x, point_basepos_y, point_center_x, point_center_y, point_kinoko_center_x,
        point_kinoko_center_y, sakura_balloon_offset_x, sakura_balloon_offset_y,
    },
};

use super::parts::{brace_name_func, digit, header_comments_func, inner_brace_func};

mod animation_collision;
mod animation_interval;
mod animation_option;
mod animation_pattern;
mod collision;
mod draw_method;
mod element;
mod offset;

pub(super) fn brace_surface<'a>(
    input: &'a str,
) -> IResult<&'a str, BraceContainer, SerikoParseError> {
    map(
        tuple((header_comments_func(surface_name), surface)),
        |(header_comments, body)| BraceContainer::new(header_comments, Brace::Surface(body)),
    )(input)
}

fn surface<'a>(input: &'a str) -> IResult<&'a str, Surface, SerikoParseError> {
    map(
        tuple((surface_name, inner_brace_func(surface_inner))),
        |(ids, lines)| Surface::new(ids, lines),
    )(input)
}

fn surface_name<'a>(input: &'a str) -> IResult<&'a str, Vec<SurfaceId>, SerikoParseError> {
    alt((surface_bracename_ssp, surface_bracename_materia))(input)
}

pub(super) fn brace_surface_append<'a>(
    input: &'a str,
) -> IResult<&'a str, BraceContainer, SerikoParseError> {
    map(
        tuple((header_comments_func(surface_append), surface_append)),
        |(header_comments, body)| BraceContainer::new(header_comments, Brace::SurfaceAppend(body)),
    )(input)
}

fn surface_append<'a>(input: &'a str) -> IResult<&'a str, SurfaceAppend, SerikoParseError> {
    map(
        tuple((surface_append_name, inner_brace_func(surface_inner))),
        |(ids, lines)| SurfaceAppend::new(ids, lines),
    )(input)
}

fn surface_append_name<'a>(input: &'a str) -> IResult<&'a str, Vec<SurfaceId>, SerikoParseError> {
    brace_name_func(preceded(tag("surface.append"), surface_ids))(input)
}

fn surface_bracename_ssp<'a>(input: &'a str) -> IResult<&'a str, Vec<SurfaceId>, SerikoParseError> {
    brace_name_func(preceded(tag("surface"), surface_ids))(input)
}

fn surface_bracename_materia<'a>(
    input: &'a str,
) -> IResult<&'a str, Vec<SurfaceId>, SerikoParseError> {
    brace_name_func(separated_list1(
        tag(","),
        preceded(tag("surface"), surface_id_unit),
    ))(input)
}

fn surface_ids<'a>(input: &'a str) -> IResult<&'a str, Vec<SurfaceId>, SerikoParseError> {
    separated_list1(tag(","), surface_id)(input)
}

fn surface_id<'a>(input: &'a str) -> IResult<&'a str, SurfaceId, SerikoParseError> {
    alt((surface_id_not, surface_id_range, surface_id_unit))(input)
}

fn surface_id_unit<'a>(input: &'a str) -> IResult<&'a str, SurfaceId, SerikoParseError> {
    map(digit, |v| SurfaceId::Unit(v))(input)
}

fn surface_id_range<'a>(input: &'a str) -> IResult<&'a str, SurfaceId, SerikoParseError> {
    map(separated_pair(digit, tag("-"), digit), |(v1, v2)| {
        SurfaceId::Range(v1, v2)
    })(input)
}

fn surface_id_not<'a>(input: &'a str) -> IResult<&'a str, SurfaceId, SerikoParseError> {
    map(
        preceded(tag("!"), alt((surface_id_range, surface_id_unit))),
        |v| SurfaceId::Not(Box::new(v)),
    )(input)
}

fn surface_inner<'a>(input: &'a str) -> IResult<&'a str, SurfaceInner, SerikoParseError> {
    alt((
        map(element, |v| SurfaceInner::Element(v)),
        map(animation_interval, |v| SurfaceInner::AnimationInterval(v)),
        map(animation_pattern, |v| SurfaceInner::AnimationPattern(v)),
        map(animation_option, |v| SurfaceInner::AnimationOption(v)),
        map(animation_collision, |v| SurfaceInner::AnimationCollision(v)),
        map(animation_collision_ex, |v| {
            SurfaceInner::AnimationCollisionEx(v)
        }),
        map(collision, |v| SurfaceInner::Collision(v)),
        map(collision_ex, |v| SurfaceInner::CollisionEx(v)),
        sakura_balloon_offset_x,
        sakura_balloon_offset_y,
        kero_balloon_offset_x,
        kero_balloon_offset_y,
        balloon_offset_x,
        balloon_offset_y,
        point_center_x,
        point_center_y,
        point_kinoko_center_x,
        point_kinoko_center_y,
        point_basepos_x,
        point_basepos_y,
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod brace_surface {

        use crate::{
            ast::{LineContainer, SurfaceCollision},
            CommentLine,
        };

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = r#"

surface1-3,!25-30
{
collision0,188,25,252,63,Head
collision1,180,191,220,222,Bust
}"#;
            let (remain, result) = brace_surface(case).unwrap();
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
                &Brace::Surface(Surface::new(
                    vec![
                        SurfaceId::Range(1, 3),
                        SurfaceId::Not(Box::new(SurfaceId::Range(25, 30)))
                    ],
                    vec![
                        LineContainer::Body(SurfaceInner::Collision(SurfaceCollision::new(
                            0,
                            188,
                            25,
                            252,
                            63,
                            "Head".to_string()
                        ))),
                        LineContainer::Body(SurfaceInner::Collision(SurfaceCollision::new(
                            1,
                            180,
                            191,
                            220,
                            222,
                            "Bust".to_string()
                        ))),
                    ]
                ))
            );

            let case = r#"

surface1,surface3,surface4
{
}"#;
            let (remain, result) = brace_surface(case).unwrap();
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
                &Brace::Surface(Surface::new(
                    vec![SurfaceId::Unit(1), SurfaceId::Unit(3), SurfaceId::Unit(4)],
                    vec![]
                ))
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = r#"surface1{}"#;
            assert!(surface(case).is_err());
        }
    }

    mod surface {
        use crate::ast::{LineContainer, SurfaceCollision};

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = r#"surface1-3,!25-30
{
collision0,188,25,252,63,Head
collision1,180,191,220,222,Bust
}"#;
            let (remain, result) = surface(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                Surface::new(
                    vec![
                        SurfaceId::Range(1, 3),
                        SurfaceId::Not(Box::new(SurfaceId::Range(25, 30)))
                    ],
                    vec![
                        LineContainer::Body(SurfaceInner::Collision(SurfaceCollision::new(
                            0,
                            188,
                            25,
                            252,
                            63,
                            "Head".to_string()
                        ))),
                        LineContainer::Body(SurfaceInner::Collision(SurfaceCollision::new(
                            1,
                            180,
                            191,
                            220,
                            222,
                            "Bust".to_string()
                        ))),
                    ]
                )
            );

            let case = r#"surface1,surface3,surface4
{
}"#;
            let (remain, result) = surface(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                Surface::new(
                    vec![SurfaceId::Unit(1), SurfaceId::Unit(3), SurfaceId::Unit(4)],
                    vec![]
                )
            );

            let case = r#"surface10
{
collision0,40,56,95,90,Head

sakura.balloon.offsetx,80
sakura.balloon.offsety,-100
kero.balloon.offsetx,-30
kero.balloon.offsety,20
}"#;
            let (remain, result) = surface(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result.ids(), &vec![SurfaceId::Unit(10)]);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = r#"surface1{}"#;
            assert!(surface(case).is_err());
        }
    }

    mod surface_name {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "surface0,1\r\n";
            let (remain, result) = surface_name(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, vec![SurfaceId::Unit(0), SurfaceId::Unit(1)]);

            let case = "surface0,surface1\r\n";
            let (remain, result) = surface_name(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, vec![SurfaceId::Unit(0), SurfaceId::Unit(1)]);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "surface0,";
            assert!(surface_name(case).is_err());
        }
    }

    mod brace_surface_append {
        use crate::{
            ast::{LineContainer, SurfaceCollision},
            CommentLine,
        };

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = r#"

surface.append1-3,!25-30
{
collision0,188,25,252,63,Head
collision1,180,191,220,222,Bust
}"#;
            let (remain, result) = brace_surface_append(case).unwrap();
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
                &Brace::SurfaceAppend(SurfaceAppend::new(
                    vec![
                        SurfaceId::Range(1, 3),
                        SurfaceId::Not(Box::new(SurfaceId::Range(25, 30)))
                    ],
                    vec![
                        LineContainer::Body(SurfaceInner::Collision(SurfaceCollision::new(
                            0,
                            188,
                            25,
                            252,
                            63,
                            "Head".to_string()
                        ))),
                        LineContainer::Body(SurfaceInner::Collision(SurfaceCollision::new(
                            1,
                            180,
                            191,
                            220,
                            222,
                            "Bust".to_string()
                        ))),
                    ]
                ))
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "surface.append0{}";
            assert!(brace_surface_append(case).is_err());
        }
    }

    mod surface_append {
        use crate::ast::{LineContainer, SurfaceCollision};

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = r#"surface.append1-3,!25-30
{
collision0,188,25,252,63,Head
collision1,180,191,220,222,Bust
}"#;
            let (remain, result) = surface_append(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                SurfaceAppend::new(
                    vec![
                        SurfaceId::Range(1, 3),
                        SurfaceId::Not(Box::new(SurfaceId::Range(25, 30)))
                    ],
                    vec![
                        LineContainer::Body(SurfaceInner::Collision(SurfaceCollision::new(
                            0,
                            188,
                            25,
                            252,
                            63,
                            "Head".to_string()
                        ))),
                        LineContainer::Body(SurfaceInner::Collision(SurfaceCollision::new(
                            1,
                            180,
                            191,
                            220,
                            222,
                            "Bust".to_string()
                        ))),
                    ]
                )
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = r#"
surface.append1,surface2
{
}
"#;
            assert!(surface_append(case).is_err());
        }
    }

    mod surface_append_name {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "surface.append0\r\n";
            let (remain, result) = surface_append_name(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, vec![SurfaceId::Unit(0)])
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "surface.append0";
            assert!(surface_append_name(case).is_err());
        }
    }

    mod surface_bracename_ssp {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "surface1,3,4,6,12\r\n";
            let (remain, result) = surface_bracename_ssp(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                vec![
                    SurfaceId::Unit(1),
                    SurfaceId::Unit(3),
                    SurfaceId::Unit(4),
                    SurfaceId::Unit(6),
                    SurfaceId::Unit(12),
                ]
            );

            let case = "surface1-12\r\n";
            let (remain, result) = surface_bracename_ssp(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, vec![SurfaceId::Range(1, 12)]);

            let case = "surface1-30,!15,!20-25\r\n";
            let (remain, result) = surface_bracename_ssp(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                vec![
                    SurfaceId::Range(1, 30),
                    SurfaceId::Not(Box::new(SurfaceId::Unit(15))),
                    SurfaceId::Not(Box::new(SurfaceId::Range(20, 25))),
                ]
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "surface,1,surface3,surface4";
            assert!(surface_bracename_ssp(case).is_err());
        }
    }

    mod surface_bracename_materia {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "surface1,surface3,surface4\r\n";
            let (remain, result) = surface_bracename_materia(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                vec![SurfaceId::Unit(1), SurfaceId::Unit(3), SurfaceId::Unit(4)]
            );

            let case = "surface1\r\n";
            let (remain, result) = surface_bracename_materia(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, vec![SurfaceId::Unit(1)]);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "surface,";
            assert!(surface_bracename_materia(case).is_err());
        }
    }

    mod surface_ids {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "0,1-30,!15,!20-25";
            let (remain, result) = surface_ids(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                vec![
                    SurfaceId::Unit(0),
                    SurfaceId::Range(1, 30),
                    SurfaceId::Not(Box::new(SurfaceId::Unit(15))),
                    SurfaceId::Not(Box::new(SurfaceId::Range(20, 25))),
                ]
            );

            let case = "0";
            let (remain, result) = surface_ids(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, vec![SurfaceId::Unit(0)]);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "!!30";
            assert!(surface_ids(case).is_err());
        }
    }
}

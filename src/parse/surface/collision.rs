use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::{map, opt},
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};

use crate::{
    parse::parts::{boolean, digit, digit_neg},
    CollisionExKind, SerikoParseError, SurfaceCollision, SurfaceCollisionEx,
};

pub(super) fn collision<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceCollision, SerikoParseError> {
    map(
        tuple((
            preceded(tag("collision"), digit),
            preceded(tag(","), digit_neg),
            preceded(tag(","), digit_neg),
            preceded(tag(","), digit_neg),
            preceded(tag(","), digit_neg),
            preceded(tag(","), is_not(",\r\n ")),
        )),
        |(id, start_x, start_y, end_x, end_y, collision_id)| {
            SurfaceCollision::new(id, start_x, start_y, end_x, end_y, collision_id.to_string())
        },
    )(input)
}

pub(super) fn collision_ex<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceCollisionEx, SerikoParseError> {
    map(
        tuple((
            tag("collisionex"),
            digit,
            preceded(tag(","), is_not(",")),
            preceded(tag(","), collision_ex_kind),
        )),
        |(_, id, collision_id, kind)| SurfaceCollisionEx::new(id, collision_id.to_string(), kind),
    )(input)
}

fn collision_ex_kind<'a>(input: &'a str) -> IResult<&'a str, CollisionExKind, SerikoParseError> {
    alt((
        collision_ex_kind_rect,
        collision_ex_kind_ellipse,
        collision_ex_kind_circle,
        collision_ex_kind_polygon,
        collision_ex_kind_region,
    ))(input)
}

fn collision_ex_kind_rect<'a>(
    input: &'a str,
) -> IResult<&'a str, CollisionExKind, SerikoParseError> {
    map(
        tuple((
            tag("rect"),
            preceded(tag(","), digit_neg),
            preceded(tag(","), digit_neg),
            preceded(tag(","), digit_neg),
            preceded(tag(","), digit_neg),
        )),
        |(_, start_x, start_y, end_x, end_y)| CollisionExKind::Rect(start_x, start_y, end_x, end_y),
    )(input)
}

fn collision_ex_kind_ellipse<'a>(
    input: &'a str,
) -> IResult<&'a str, CollisionExKind, SerikoParseError> {
    map(
        tuple((
            tag("ellipse"),
            preceded(tag(","), digit_neg),
            preceded(tag(","), digit_neg),
            preceded(tag(","), digit_neg),
            preceded(tag(","), digit_neg),
        )),
        |(_, start_x, start_y, end_x, end_y)| {
            CollisionExKind::Ellipse(start_x, start_y, end_x, end_y)
        },
    )(input)
}

fn collision_ex_kind_circle<'a>(
    input: &'a str,
) -> IResult<&'a str, CollisionExKind, SerikoParseError> {
    map(
        tuple((
            tag("circle"),
            preceded(tag(","), digit_neg),
            preceded(tag(","), digit_neg),
            preceded(tag(","), digit_neg),
        )),
        |(_, x, y, r)| CollisionExKind::Circle(x, y, r),
    )(input)
}

fn collision_ex_kind_polygon<'a>(
    input: &'a str,
) -> IResult<&'a str, CollisionExKind, SerikoParseError> {
    map(
        tuple((tag("polygon"), many1(preceded(tag(","), digit_neg)))),
        |(_, v)| CollisionExKind::Polygon(v),
    )(input)
}

fn collision_ex_kind_region<'a>(
    input: &'a str,
) -> IResult<&'a str, CollisionExKind, SerikoParseError> {
    map(
        tuple((
            tag("region"),
            preceded(tag(","), is_not(",")),
            preceded(tag(","), digit),
            preceded(tag(","), digit),
            preceded(tag(","), digit),
            opt(preceded(tag(","), boolean)),
        )),
        |(_, filename, r, g, b, flag)| CollisionExKind::Region(filename.to_string(), r, g, b, flag),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod collision {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "collision2,10,10,100,100,Head";
            let (remain, result) = collision(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                SurfaceCollision::new(2, 10, 10, 100, 100, "Head".to_string())
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "collision2,10,10,100,100";
            assert!(collision(case).is_err());
        }
    }

    mod collision_ex {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "collisionex0,Head,rect,100,100,200,300";
            let (remain, result) = collision_ex(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                SurfaceCollisionEx::new(
                    0,
                    "Head".to_string(),
                    CollisionExKind::Rect(100, 100, 200, 300)
                )
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "collisionex0,Head,rect,100,100,200";
            assert!(collision_ex(case).is_err());
        }
    }

    mod collision_ex_kind {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "rect,100,100,200,300";
            let (remain, result) = collision_ex_kind(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, CollisionExKind::Rect(100, 100, 200, 300));

            let case = "ellipse,100,100,200,300";
            let (remain, result) = collision_ex_kind(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, CollisionExKind::Ellipse(100, 100, 200, 300));

            let case = "circle,100,200,20";
            let (remain, result) = collision_ex_kind(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, CollisionExKind::Circle(100, 200, 20));

            let case = "polygon,100,100,200,300,50,200";
            let (remain, result) = collision_ex_kind(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                CollisionExKind::Polygon(vec![100, 100, 200, 300, 50, 200])
            );

            let case = "region,atari.png,0,255,0,true";
            let (remain, result) = collision_ex_kind(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                CollisionExKind::Region("atari.png".to_string(), 0, 255, 0, Some(true))
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "polygon,";
            assert!(collision_ex_kind(case).is_err());
        }
    }

    mod collision_ex_kind_rect {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "rect,100,100,200,300";
            let (remain, result) = collision_ex_kind_rect(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, CollisionExKind::Rect(100, 100, 200, 300));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "rect,100,100,200";
            assert!(collision_ex_kind_rect(case).is_err());
        }
    }

    mod collision_ex_kind_ellipse {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "ellipse,100,100,200,300";
            let (remain, result) = collision_ex_kind_ellipse(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, CollisionExKind::Ellipse(100, 100, 200, 300));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "ellipse,100,100,200";
            assert!(collision_ex_kind_ellipse(case).is_err());
        }
    }

    mod collision_ex_kind_circle {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "circle,100,200,20";
            let (remain, result) = collision_ex_kind_circle(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, CollisionExKind::Circle(100, 200, 20));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "circle,100,200";
            assert!(collision_ex_kind_circle(case).is_err());
        }
    }

    mod collision_ex_kind_polygon {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "polygon,100,100,200,300,50,200";
            let (remain, result) = collision_ex_kind_polygon(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                CollisionExKind::Polygon(vec![100, 100, 200, 300, 50, 200])
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "polygon,";
            assert!(collision_ex_kind_polygon(case).is_err());
        }
    }

    mod collision_ex_kind_region {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "region,atari.png,0,255,0,true";
            let (remain, result) = collision_ex_kind_region(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                CollisionExKind::Region("atari.png".to_string(), 0, 255, 0, Some(true))
            );

            let case = "region,atari.png,0,255,0";
            let (remain, result) = collision_ex_kind_region(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                CollisionExKind::Region("atari.png".to_string(), 0, 255, 0, None)
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "region,atari.png,0,255,";
            assert!(collision_ex_kind_region(case).is_err());
        }
    }
}

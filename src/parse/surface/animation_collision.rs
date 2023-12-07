use nom::{
    bytes::complete::tag,
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};

use crate::{
    ast::{SurfaceAnimationCollision, SurfaceAnimationCollisionEx},
    parse::{parts::digit, SerikoParseError},
};

use super::collision::{collision, collision_ex};

pub(super) fn animation_collision<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceAnimationCollision, SerikoParseError> {
    map(
        tuple((
            preceded(tag("animation"), digit),
            preceded(tag("."), collision),
        )),
        |(id, c)| SurfaceAnimationCollision::new(id, c),
    )(input)
}

pub(super) fn animation_collision_ex<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceAnimationCollisionEx, SerikoParseError> {
    map(
        tuple((
            preceded(tag("animation"), digit),
            preceded(tag("."), collision_ex),
        )),
        |(id, c)| SurfaceAnimationCollisionEx::new(id, c),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod animation_collision {
        use crate::ast::SurfaceCollision;

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "animation2.collision0,10,10,200,100,Head";
            let (remain, result) = animation_collision(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                SurfaceAnimationCollision::new(
                    2,
                    SurfaceCollision::new(0, 10, 10, 200, 100, "Head".to_string())
                )
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "animation2.collision0,10,10,200,100";
            assert!(animation_collision(case).is_err());
        }
    }

    mod animation_collision_ex {
        use crate::ast::SurfaceCollisionEx;

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "animation2.collisionex0,Head,rect,100,100,200,300";
            let (remain, result) = animation_collision_ex(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(
                result,
                SurfaceAnimationCollisionEx::new(
                    2,
                    SurfaceCollisionEx::new(
                        0,
                        "Head".to_string(),
                        crate::ast::CollisionExKind::Rect(100, 100, 200, 300)
                    )
                )
            );
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "animation2.collisionex0,Head,rect,100,100,200,";
            assert!(animation_collision_ex(case).is_err());
        }
    }
}

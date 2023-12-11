use std::str::FromStr;

use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};
use shell_parser_common_rs::ShellParseError;

use crate::{parse::parts::digit_neg, SurfaceInner};

pub(super) fn sakura_balloon_offset_x<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceInner, ShellParseError> {
    offset_base("sakura.balloon.offsetx,", |v| {
        SurfaceInner::SakuraBalloonOffsetX(v)
    })(input)
}

pub(super) fn sakura_balloon_offset_y<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceInner, ShellParseError> {
    offset_base("sakura.balloon.offsety,", |v| {
        SurfaceInner::SakuraBalloonOffsetY(v)
    })(input)
}

pub(super) fn kero_balloon_offset_x<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceInner, ShellParseError> {
    offset_base("kero.balloon.offsetx,", |v| {
        SurfaceInner::KeroBalloonOffsetX(v)
    })(input)
}

pub(super) fn kero_balloon_offset_y<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceInner, ShellParseError> {
    offset_base("kero.balloon.offsety,", |v| {
        SurfaceInner::KeroBalloonOffsetY(v)
    })(input)
}

pub(super) fn balloon_offset_x<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceInner, ShellParseError> {
    offset_base("balloon.offsetx,", |v| SurfaceInner::BalloonOffsetX(v))(input)
}

pub(super) fn balloon_offset_y<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceInner, ShellParseError> {
    offset_base("balloon.offsety,", |v| SurfaceInner::BalloonOffsetY(v))(input)
}

pub(super) fn point_center_x<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceInner, ShellParseError> {
    offset_base("point.centerx,", |v| SurfaceInner::PointCenterX(v))(input)
}

pub(super) fn point_center_y<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceInner, ShellParseError> {
    offset_base("point.centery,", |v| SurfaceInner::PointCenterY(v))(input)
}

pub(super) fn point_kinoko_center_x<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceInner, ShellParseError> {
    offset_base("point.kinoko.centerx,", |v| {
        SurfaceInner::PointKinokoCenterX(v)
    })(input)
}

pub(super) fn point_kinoko_center_y<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceInner, ShellParseError> {
    offset_base("point.kinoko.centery,", |v| {
        SurfaceInner::PointKinokoCenterY(v)
    })(input)
}

pub(super) fn point_basepos_x<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceInner, ShellParseError> {
    offset_base("point.basepos.x,", |v| SurfaceInner::PointBaseposX(v))(input)
}

pub(super) fn point_basepos_y<'a>(
    input: &'a str,
) -> IResult<&'a str, SurfaceInner, ShellParseError> {
    offset_base("point.basepos.y,", |v| SurfaceInner::PointBaseposY(v))(input)
}

fn offset_base<'a, F, T, O>(
    t: &'static str,
    mut f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, T, ShellParseError>
where
    F: FnMut(O) -> T,
    O: FromStr + std::ops::Neg<Output = O>,
{
    map(tuple((tag(t), digit_neg)), move |(_, v)| f(v))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod sakura_balloon_offset_x {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.balloon.offsetx,10";
            let (remain, result) = sakura_balloon_offset_x(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, SurfaceInner::SakuraBalloonOffsetX(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "sakura.balloon.offsetx,";
            assert!(sakura_balloon_offset_x(case).is_err());
        }
    }

    mod sakura_balloon_offset_y {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "sakura.balloon.offsety,10";
            let (remain, result) = sakura_balloon_offset_y(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, SurfaceInner::SakuraBalloonOffsetY(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "sakura.balloon.offsety,";
            assert!(sakura_balloon_offset_y(case).is_err());
        }
    }

    mod kero_balloon_offset_x {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "kero.balloon.offsetx,10";
            let (remain, result) = kero_balloon_offset_x(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, SurfaceInner::KeroBalloonOffsetX(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "kero.balloon.offsetx,";
            assert!(kero_balloon_offset_x(case).is_err());
        }
    }

    mod kero_balloon_offset_y {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "kero.balloon.offsety,10";
            let (remain, result) = kero_balloon_offset_y(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, SurfaceInner::KeroBalloonOffsetY(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "kero.balloon.offsety,";
            assert!(kero_balloon_offset_y(case).is_err());
        }
    }

    mod balloon_offset_x {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "balloon.offsetx,10";
            let (remain, result) = balloon_offset_x(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, SurfaceInner::BalloonOffsetX(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "balloon.offsetx,";
            assert!(balloon_offset_x(case).is_err());
        }
    }

    mod balloon_offset_y {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "balloon.offsety,10";
            let (remain, result) = balloon_offset_y(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, SurfaceInner::BalloonOffsetY(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "balloon.offsety,";
            assert!(balloon_offset_y(case).is_err());
        }
    }

    mod point_center_x {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "point.centerx,10";
            let (remain, result) = point_center_x(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, SurfaceInner::PointCenterX(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "point.centerx,";
            assert!(point_center_x(case).is_err());
        }
    }

    mod point_center_y {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "point.centery,10";
            let (remain, result) = point_center_y(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, SurfaceInner::PointCenterY(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "point.centery,";
            assert!(point_center_y(case).is_err());
        }
    }

    mod point_kinoko_center_x {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "point.kinoko.centerx,10";
            let (remain, result) = point_kinoko_center_x(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, SurfaceInner::PointKinokoCenterX(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "point.kinoko.centerx,";
            assert!(point_kinoko_center_x(case).is_err());
        }
    }

    mod point_kinoko_center_y {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "point.kinoko.centery,10";
            let (remain, result) = point_kinoko_center_y(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, SurfaceInner::PointKinokoCenterY(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "point.kinoko.centery,";
            assert!(point_kinoko_center_y(case).is_err());
        }
    }

    mod point_basepos_x {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "point.basepos.x,10";
            let (remain, result) = point_basepos_x(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, SurfaceInner::PointBaseposX(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "point.basepos.x,";
            assert!(point_basepos_x(case).is_err());
        }
    }

    mod point_basepos_y {
        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case = "point.basepos.y,10";
            let (remain, result) = point_basepos_y(case).unwrap();
            assert_eq!(remain, "");
            assert_eq!(result, SurfaceInner::PointBaseposY(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = "point.basepos.y,";
            assert!(point_basepos_y(case).is_err());
        }
    }

    mod offset_base {
        use crate::SurfaceInner;

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case_func = |v| SurfaceInner::SakuraBalloonOffsetX(v);
            let case = "sakura.balloon.offsetx,10\r\n";
            let (remain, result) = offset_base("sakura.balloon.offsetx,", case_func)(case).unwrap();
            assert_eq!(remain, "\r\n");
            assert_eq!(result, SurfaceInner::SakuraBalloonOffsetX(10));
        }

        #[test]
        fn failed_when_invalid_str() {
            let case_func = |v| SurfaceInner::SakuraBalloonOffsetX(v);
            let case = "sakura.balloon.offsetx,\r\n";
            assert!(offset_base("sakura.balloon.offsetx,", case_func)(case).is_err());
        }
    }
}

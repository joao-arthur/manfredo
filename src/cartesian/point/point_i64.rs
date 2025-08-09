use super::{point_i8::PointI8, point_i16::PointI16, point_i32::PointI32};

use crate::cartesian::point::point_u64::PointU64;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointI64 {
    pub x: i64,
    pub y: i64,
}

impl PointI64 {
    pub fn of(x: i64, y: i64) -> Self {
        PointI64 { x, y }
    }

    pub fn min() -> Self {
        PointI64 { x: i64::MIN, y: i64::MIN }
    }

    pub fn max() -> Self {
        PointI64 { x: i64::MAX, y: i64::MAX }
    }
}

impl From<PointI8> for PointI64 {
    fn from(p: PointI8) -> Self {
        PointI64 { x: p.x.into(), y: p.y.into() }
    }
}

impl From<PointI16> for PointI64 {
    fn from(p: PointI16) -> Self {
        PointI64 { x: p.x.into(), y: p.y.into() }
    }
}

impl From<PointI32> for PointI64 {
    fn from(p: PointI32) -> Self {
        PointI64 { x: p.x.into(), y: p.y.into() }
    }
}

impl std::fmt::Display for PointI64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointI64, p2: &PointI64) -> u64 {
    (i128::from(p2.x) - i128::from(p1.x)).unsigned_abs() as u64
}

pub fn delta_y(p1: &PointI64, p2: &PointI64) -> u64 {
    (i128::from(p2.y) - i128::from(p1.y)).unsigned_abs() as u64
}

pub fn delta(p1: &PointI64, p2: &PointI64) -> PointU64 {
    PointU64 { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

pub fn saturating_translate(p: &mut PointI64, delta: &PointI64) {
    p.x = p.x.saturating_add(delta.x);
    p.y = p.y.saturating_add(delta.y);
}

pub fn try_checked_translate(p: &mut PointI64, delta: &PointI64) -> Result<(), ()> {
    let x = p.x.checked_add(delta.x).ok_or(())?;
    let y = p.y.checked_add(delta.y).ok_or(())?;
    p.x = x;
    p.y = y;
    Ok(())
}

pub fn checked_translate(p: &mut PointI64, delta: &PointI64) {
    try_checked_translate(p, delta).unwrap()
}

pub fn saturating_translated(p: &PointI64, delta: &PointI64) -> PointI64 {
    PointI64::of(p.x.saturating_add(delta.x), p.y.saturating_add(delta.y))
}

pub fn try_checked_translated(p: &PointI64, delta: &PointI64) -> Option<PointI64> {
    let x = p.x.checked_add(delta.x)?;
    let y = p.y.checked_add(delta.y)?;
    Some(PointI64 { x, y })
}

pub fn checked_translated(p: &PointI64, delta: &PointI64) -> PointI64 {
    try_checked_translated(p, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::{point_i8::PointI8, point_i16::PointI16, point_i32::PointI32, point_u64::PointU64};

    use super::{
        PointI64, checked_translate, checked_translated, delta, delta_x, delta_y, saturating_translate, saturating_translated, try_checked_translate,
        try_checked_translated,
    };

    #[test]
    fn point_i64() {
        assert_eq!(PointI64::of(i64::MIN, i64::MAX), PointI64 { x: i64::MIN, y: i64::MAX });
        assert_eq!(PointI64::min(), PointI64 { x: i64::MIN, y: i64::MIN });
        assert_eq!(PointI64::max(), PointI64 { x: i64::MAX, y: i64::MAX });
    }

    #[test]
    fn from() {
        assert_eq!(PointI64::from(PointI8::min()), PointI64 { x: i8::MIN.into(), y: i8::MIN.into() });
        assert_eq!(PointI64::from(PointI8::max()), PointI64 { x: i8::MAX.into(), y: i8::MAX.into() });
        assert_eq!(PointI64::from(PointI16::min()), PointI64 { x: i16::MIN.into(), y: i16::MIN.into() });
        assert_eq!(PointI64::from(PointI16::max()), PointI64 { x: i16::MAX.into(), y: i16::MAX.into() });
        assert_eq!(PointI64::from(PointI32::min()), PointI64 { x: i32::MIN.into(), y: i32::MIN.into() });
        assert_eq!(PointI64::from(PointI32::max()), PointI64 { x: i32::MAX.into(), y: i32::MAX.into() });
    }

    #[test]
    fn to_string() {
        assert_eq!(PointI64::of(i64::MIN, i64::MAX).to_string(), "(-9223372036854775808, 9223372036854775807)");
        assert_eq!(PointI64::min().to_string(), "(-9223372036854775808, -9223372036854775808)");
        assert_eq!(PointI64::max().to_string(), "(9223372036854775807, 9223372036854775807)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointI64::of(0, i64::MIN), &PointI64::of(0, i64::MAX)), 0);
        assert_eq!(delta_x(&PointI64::of(i64::MIN, 0), &PointI64::of(i64::MAX, 0)), u64::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointI64::of(i64::MIN, 0), &PointI64::of(i64::MAX, 0)), 0);
        assert_eq!(delta_y(&PointI64::of(0, i64::MIN), &PointI64::of(0, i64::MAX)), u64::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointI64::of(0, 0), &PointI64::of(0, 0)), PointU64::min());
        assert_eq!(delta(&PointI64::min(), &PointI64::max()), PointU64::max());
    }

    #[test]
    fn delta_min() {
        let p = PointI64::min();
        assert_eq!(delta(&p, &PointI64::min()), PointU64::of(0, 0));
        assert_eq!(delta(&p, &PointI64::of(i64::MIN, i64::MIN + 1)), PointU64::of(0, 1));
        assert_eq!(delta(&p, &PointI64::of(i64::MIN, i64::MIN + 2)), PointU64::of(0, 2));

        assert_eq!(delta(&p, &PointI64::of(i64::MIN + 1, i64::MIN)), PointU64::of(1, 0));
        assert_eq!(delta(&p, &PointI64::of(i64::MIN + 1, i64::MIN + 1)), PointU64::of(1, 1));
        assert_eq!(delta(&p, &PointI64::of(i64::MIN + 1, i64::MIN + 2)), PointU64::of(1, 2));

        assert_eq!(delta(&p, &PointI64::of(i64::MIN + 2, i64::MIN)), PointU64::of(2, 0));
        assert_eq!(delta(&p, &PointI64::of(i64::MIN + 2, i64::MIN + 1)), PointU64::of(2, 1));
        assert_eq!(delta(&p, &PointI64::of(i64::MIN + 2, i64::MIN + 2)), PointU64::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p = PointI64::of(i64::MAX - 2, i64::MAX - 2);
        assert_eq!(delta(&p, &PointI64::of(i64::MAX - 2, i64::MAX - 2)), PointU64::of(0, 0));
        assert_eq!(delta(&p, &PointI64::of(i64::MAX - 2, i64::MAX - 1)), PointU64::of(0, 1));
        assert_eq!(delta(&p, &PointI64::of(i64::MAX - 2, i64::MAX)), PointU64::of(0, 2));

        assert_eq!(delta(&p, &PointI64::of(i64::MAX - 1, i64::MAX - 2)), PointU64::of(1, 0));
        assert_eq!(delta(&p, &PointI64::of(i64::MAX - 1, i64::MAX - 1)), PointU64::of(1, 1));
        assert_eq!(delta(&p, &PointI64::of(i64::MAX - 1, i64::MAX)), PointU64::of(1, 2));

        assert_eq!(delta(&p, &PointI64::of(i64::MAX, i64::MAX - 2)), PointU64::of(2, 0));
        assert_eq!(delta(&p, &PointI64::of(i64::MAX, i64::MAX - 1)), PointU64::of(2, 1));
        assert_eq!(delta(&p, &PointI64::max()), PointU64::of(2, 2));
    }

    #[test]
    fn test_saturating_translate() {
        let mut p = PointI64::of(0, 0);
        saturating_translate(&mut p, &PointI64::of(10, 15));
        assert_eq!(p, PointI64::of(10, 15));
        saturating_translate(&mut p, &PointI64::of(-15, -25));
        assert_eq!(p, PointI64::of(-5, -10));
        saturating_translate(&mut p, &PointI64::of(2, 3));
        assert_eq!(p, PointI64::of(-3, -7));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut p = PointI64::of(i64::MIN + 2, i64::MIN + 5);
        saturating_translate(&mut p, &PointI64::of(-10, -10));
        assert_eq!(p, PointI64::min());
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut p = PointI64::of(i64::MAX - 2, i64::MAX - 5);
        saturating_translate(&mut p, &PointI64::of(10, 10));
        assert_eq!(p, PointI64::max());
    }

    #[test]
    fn saturating_translate_min_bounds_min_delta() {
        let mut p = PointI64::of(i64::MIN + 1, i64::MIN + 1);
        saturating_translate(&mut p, &PointI64::min());
        assert_eq!(p, PointI64::min());
    }

    #[test]
    fn saturating_translate_max_bounds_max_delta() {
        let mut p = PointI64::of(i64::MAX - 1, i64::MAX - 1);
        saturating_translate(&mut p, &PointI64::max());
        assert_eq!(p, PointI64::max());
    }

    #[test]
    fn test_try_checked_translate() {
        let mut p = PointI64::of(0, 0);
        assert_eq!(try_checked_translate(&mut p, &PointI64::of(10, 15)), Ok(()));
        assert_eq!(p, PointI64::of(10, 15));
        assert_eq!(try_checked_translate(&mut p, &PointI64::of(-15, -25)), Ok(()));
        assert_eq!(p, PointI64::of(-5, -10));
        assert_eq!(try_checked_translate(&mut p, &PointI64::of(2, 3)), Ok(()));
        assert_eq!(p, PointI64::of(-3, -7));
    }

    #[test]
    fn try_checked_translate_min_bounds_err() {
        let mut p = PointI64::of(i64::MIN + 2, i64::MIN + 5);
        assert_eq!(try_checked_translate(&mut p, &PointI64::of(-10, -10)), Err(()));
        assert_eq!(p, PointI64::of(i64::MIN + 2, i64::MIN + 5));
    }

    #[test]
    fn try_checked_translate_max_bounds_err() {
        let mut p = PointI64::of(i64::MAX - 2, i64::MAX - 5);
        assert_eq!(try_checked_translate(&mut p, &PointI64::of(10, 10)), Err(()));
        assert_eq!(p, PointI64::of(i64::MAX - 2, i64::MAX - 5));
    }

    #[test]
    fn try_checked_translate_min_bounds_ok() {
        let mut p = PointI64::of(i64::MIN + 2, i64::MIN + 5);
        assert_eq!(try_checked_translate(&mut p, &PointI64::of(-2, -5)), Ok(()));
        assert_eq!(p, PointI64::min());
    }

    #[test]
    fn try_checked_translate_max_bounds_ok() {
        let mut p = PointI64::of(i64::MAX - 2, i64::MAX - 5);
        assert_eq!(try_checked_translate(&mut p, &PointI64::of(2, 5)), Ok(()));
        assert_eq!(p, PointI64::max());
    }

    #[test]
    fn try_checked_translate_min_bounds_min_delta() {
        let mut p = PointI64::of(i64::MIN + 1, i64::MIN + 1);
        assert_eq!(try_checked_translate(&mut p, &PointI64::min()), Err(()));
        assert_eq!(try_checked_translate(&mut p, &PointI64::of(i64::MIN, 0)), Err(()));
        assert_eq!(try_checked_translate(&mut p, &PointI64::of(0, i64::MIN)), Err(()));
        assert_eq!(p, PointI64::of(i64::MIN + 1, i64::MIN + 1));
    }

    #[test]
    fn try_checked_translate_max_bounds_max_delta() {
        let mut p = PointI64::of(i64::MAX - 1, i64::MAX - 1);
        assert_eq!(try_checked_translate(&mut p, &PointI64::max()), Err(()));
        assert_eq!(try_checked_translate(&mut p, &PointI64::of(i64::MAX, 0)), Err(()));
        assert_eq!(try_checked_translate(&mut p, &PointI64::of(0, i64::MAX)), Err(()));
        assert_eq!(p, PointI64::of(i64::MAX - 1, i64::MAX - 1));
    }

    #[test]
    fn test_checked_translate() {
        let mut p = PointI64::of(0, 0);
        checked_translate(&mut p, &PointI64::of(10, 15));
        assert_eq!(p, PointI64::of(10, 15));
        checked_translate(&mut p, &PointI64::of(-15, -25));
        assert_eq!(p, PointI64::of(-5, -10));
        checked_translate(&mut p, &PointI64::of(2, 3));
        assert_eq!(p, PointI64::of(-3, -7));
    }

    #[test]
    fn test_saturating_translated() {
        assert_eq!(saturating_translated(&PointI64::of(0, 0), &PointI64::of(10, 15)), PointI64::of(10, 15));
        assert_eq!(saturating_translated(&PointI64::of(0, 0), &PointI64::of(-15, -25)), PointI64::of(-15, -25));
    }

    #[test]
    fn saturating_translated_to_bounds() {
        assert_eq!(saturating_translated(&PointI64::of(i64::MIN + 2, i64::MIN + 5), &PointI64::of(-2, -5)), PointI64::min());
        assert_eq!(saturating_translated(&PointI64::of(i64::MAX - 2, i64::MAX - 5), &PointI64::of(2, 5)), PointI64::max());
    }

    #[test]
    fn saturating_translated_beyond_bounds() {
        assert_eq!(saturating_translated(&PointI64::of(i64::MIN + 2, i64::MIN + 5), &PointI64::of(-10, -10)), PointI64::min());
        assert_eq!(saturating_translated(&PointI64::of(i64::MAX - 2, i64::MAX - 5), &PointI64::of(10, 10)), PointI64::max());
    }

    #[test]
    fn saturating_translated_limits() {
        assert_eq!(saturating_translated(&PointI64::of(i64::MIN + 1, i64::MIN + 1), &PointI64::min()), PointI64::min());
        assert_eq!(saturating_translated(&PointI64::of(i64::MAX - 1, i64::MAX - 1), &PointI64::max()), PointI64::max());
    }

    #[test]
    fn try_checked_translated_min_bounds() {
        let p = PointI64::of(i64::MIN + 2, i64::MIN + 5);
        assert_eq!(try_checked_translated(&p, &PointI64::of(-2, 0)), Some(PointI64::of(i64::MIN, i64::MIN + 5)));
        assert_eq!(try_checked_translated(&p, &PointI64::of(0, -5)), Some(PointI64::of(i64::MIN + 2, i64::MIN)));
        assert_eq!(try_checked_translated(&p, &PointI64::of(-2, -5)), Some(PointI64::min()));
        assert_eq!(try_checked_translated(&p, &PointI64::of(-10, -10)), None);
        assert_eq!(try_checked_translated(&p, &PointI64::of(i64::MIN, 0)), None);
        assert_eq!(try_checked_translated(&p, &PointI64::of(0, i64::MIN)), None);
        assert_eq!(try_checked_translated(&p, &PointI64::min()), None);
    }

    #[test]
    fn try_checked_translated_max_bounds() {
        let p = PointI64::of(i64::MAX - 2, i64::MAX - 5);
        assert_eq!(try_checked_translated(&p, &PointI64::of(2, 0)), Some(PointI64::of(i64::MAX, i64::MAX - 5)));
        assert_eq!(try_checked_translated(&p, &PointI64::of(0, 5)), Some(PointI64::of(i64::MAX - 2, i64::MAX)));
        assert_eq!(try_checked_translated(&p, &PointI64::of(2, 5)), Some(PointI64::max()));
        assert_eq!(try_checked_translated(&p, &PointI64::of(10, 10)), None);
        assert_eq!(try_checked_translated(&p, &PointI64::of(i64::MAX, 0)), None);
        assert_eq!(try_checked_translated(&p, &PointI64::of(0, i64::MAX)), None);
        assert_eq!(try_checked_translated(&p, &PointI64::max()), None);
    }

    #[test]
    fn test_checked_translated() {
        assert_eq!(checked_translated(&PointI64::of(0, 0), &PointI64::of(10, 15)), PointI64::of(10, 15));
        assert_eq!(checked_translated(&PointI64::of(10, 15), &PointI64::of(-15, -25)), PointI64::of(-5, -10));
        assert_eq!(checked_translated(&PointI64::of(-5, -10), &PointI64::of(2, 3)), PointI64::of(-3, -7));
    }
}

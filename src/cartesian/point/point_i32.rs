use super::{point_i8::PointI8, point_i16::PointI16};

use crate::cartesian::point::point_u32::PointU32;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointI32 {
    pub x: i32,
    pub y: i32,
}

impl PointI32 {
    pub fn of(x: i32, y: i32) -> Self {
        PointI32 { x, y }
    }

    pub fn min() -> Self {
        PointI32 { x: i32::MIN, y: i32::MIN }
    }

    pub fn max() -> Self {
        PointI32 { x: i32::MAX, y: i32::MAX }
    }
}

impl From<PointI8> for PointI32 {
    fn from(p: PointI8) -> Self {
        PointI32 { x: p.x.into(), y: p.y.into() }
    }
}

impl From<PointI16> for PointI32 {
    fn from(p: PointI16) -> Self {
        PointI32 { x: p.x.into(), y: p.y.into() }
    }
}

impl std::fmt::Display for PointI32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointI32, p2: &PointI32) -> u32 {
    (i64::from(p2.x) - i64::from(p1.x)).unsigned_abs() as u32
}

pub fn delta_y(p1: &PointI32, p2: &PointI32) -> u32 {
    (i64::from(p2.y) - i64::from(p1.y)).unsigned_abs() as u32
}

pub fn delta(p1: &PointI32, p2: &PointI32) -> PointU32 {
    PointU32 { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

pub fn assign_saturating_add(p: &mut PointI32, delta: &PointI32) {
    p.x = p.x.saturating_add(delta.x);
    p.y = p.y.saturating_add(delta.y);
}

pub fn try_assign_checked_add(p: &mut PointI32, delta: &PointI32) -> Result<(), ()> {
    let x = p.x.checked_add(delta.x).ok_or(())?;
    let y = p.y.checked_add(delta.y).ok_or(())?;
    p.x = x;
    p.y = y;
    Ok(())
}

pub fn assign_checked_add(p: &mut PointI32, delta: &PointI32) {
    try_assign_checked_add(p, delta).unwrap()
}

pub fn saturating_add(p: &PointI32, delta: &PointI32) -> PointI32 {
    PointI32::of(p.x.saturating_add(delta.x), p.y.saturating_add(delta.y))
}

pub fn try_checked_add(p: &PointI32, delta: &PointI32) -> Option<PointI32> {
    let x = p.x.checked_add(delta.x)?;
    let y = p.y.checked_add(delta.y)?;
    Some(PointI32 { x, y })
}

pub fn checked_add(p: &PointI32, delta: &PointI32) -> PointI32 {
    try_checked_add(p, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::{point_i8::PointI8, point_i16::PointI16, point_u32::PointU32};

    use super::{
        PointI32, assign_checked_add, checked_add, delta, delta_x, delta_y, assign_saturating_add, saturating_add, try_assign_checked_add,
        try_checked_add,
    };

    #[test]
    fn point_i32() {
        assert_eq!(PointI32::of(i32::MIN, i32::MAX), PointI32 { x: i32::MIN, y: i32::MAX });
        assert_eq!(PointI32::min(), PointI32 { x: i32::MIN, y: i32::MIN });
        assert_eq!(PointI32::max(), PointI32 { x: i32::MAX, y: i32::MAX });
    }

    #[test]
    fn from() {
        assert_eq!(PointI32::from(PointI8::min()), PointI32 { x: i8::MIN.into(), y: i8::MIN.into() });
        assert_eq!(PointI32::from(PointI8::max()), PointI32 { x: i8::MAX.into(), y: i8::MAX.into() });
        assert_eq!(PointI32::from(PointI16::min()), PointI32 { x: i16::MIN.into(), y: i16::MIN.into() });
        assert_eq!(PointI32::from(PointI16::max()), PointI32 { x: i16::MAX.into(), y: i16::MAX.into() });
    }

    #[test]
    fn to_string() {
        assert_eq!(PointI32::of(i32::MIN, i32::MAX).to_string(), "(-2147483648, 2147483647)");
        assert_eq!(PointI32::min().to_string(), "(-2147483648, -2147483648)");
        assert_eq!(PointI32::max().to_string(), "(2147483647, 2147483647)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointI32::of(0, i32::MIN), &PointI32::of(0, i32::MAX)), 0);
        assert_eq!(delta_x(&PointI32::of(i32::MIN, 0), &PointI32::of(i32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointI32::of(i32::MIN, 0), &PointI32::of(i32::MAX, 0)), 0);
        assert_eq!(delta_y(&PointI32::of(0, i32::MIN), &PointI32::of(0, i32::MAX)), u32::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointI32::of(0, 0), &PointI32::of(0, 0)), PointU32::of(0, 0));
        assert_eq!(delta(&PointI32::min(), &PointI32::max()), PointU32::max());
    }

    #[test]
    fn delta_min() {
        let p = PointI32::min();
        assert_eq!(delta(&p, &PointI32::min()), PointU32::of(0, 0));
        assert_eq!(delta(&p, &PointI32::of(i32::MIN, i32::MIN + 1)), PointU32::of(0, 1));
        assert_eq!(delta(&p, &PointI32::of(i32::MIN, i32::MIN + 2)), PointU32::of(0, 2));

        assert_eq!(delta(&p, &PointI32::of(i32::MIN + 1, i32::MIN)), PointU32::of(1, 0));
        assert_eq!(delta(&p, &PointI32::of(i32::MIN + 1, i32::MIN + 1)), PointU32::of(1, 1));
        assert_eq!(delta(&p, &PointI32::of(i32::MIN + 1, i32::MIN + 2)), PointU32::of(1, 2));

        assert_eq!(delta(&p, &PointI32::of(i32::MIN + 2, i32::MIN)), PointU32::of(2, 0));
        assert_eq!(delta(&p, &PointI32::of(i32::MIN + 2, i32::MIN + 1)), PointU32::of(2, 1));
        assert_eq!(delta(&p, &PointI32::of(i32::MIN + 2, i32::MIN + 2)), PointU32::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p = PointI32::of(i32::MAX - 2, i32::MAX - 2);
        assert_eq!(delta(&p, &PointI32::of(i32::MAX - 2, i32::MAX - 2)), PointU32::of(0, 0));
        assert_eq!(delta(&p, &PointI32::of(i32::MAX - 2, i32::MAX - 1)), PointU32::of(0, 1));
        assert_eq!(delta(&p, &PointI32::of(i32::MAX - 2, i32::MAX)), PointU32::of(0, 2));

        assert_eq!(delta(&p, &PointI32::of(i32::MAX - 1, i32::MAX - 2)), PointU32::of(1, 0));
        assert_eq!(delta(&p, &PointI32::of(i32::MAX - 1, i32::MAX - 1)), PointU32::of(1, 1));
        assert_eq!(delta(&p, &PointI32::of(i32::MAX - 1, i32::MAX)), PointU32::of(1, 2));

        assert_eq!(delta(&p, &PointI32::of(i32::MAX, i32::MAX - 2)), PointU32::of(2, 0));
        assert_eq!(delta(&p, &PointI32::of(i32::MAX, i32::MAX - 1)), PointU32::of(2, 1));
        assert_eq!(delta(&p, &PointI32::max()), PointU32::of(2, 2));
    }

    #[test]
    fn test_assign_saturating_add() {
        let mut p = PointI32::of(0, 0);
        assign_saturating_add(&mut p, &PointI32::of(10, 15));
        assert_eq!(p, PointI32::of(10, 15));
        assign_saturating_add(&mut p, &PointI32::of(-15, -25));
        assert_eq!(p, PointI32::of(-5, -10));
        assign_saturating_add(&mut p, &PointI32::of(2, 3));
        assert_eq!(p, PointI32::of(-3, -7));
    }

    #[test]
    fn assign_saturating_add_min_bounds() {
        let mut p = PointI32::of(i32::MIN + 2, i32::MIN + 5);
        assign_saturating_add(&mut p, &PointI32::of(-10, -10));
        assert_eq!(p, PointI32::min());
    }

    #[test]
    fn assign_saturating_add_max_bounds() {
        let mut p = PointI32::of(i32::MAX - 2, i32::MAX - 5);
        assign_saturating_add(&mut p, &PointI32::of(10, 10));
        assert_eq!(p, PointI32::max());
    }

    #[test]
    fn assign_saturating_add_min_bounds_min_delta() {
        let mut p = PointI32::of(i32::MIN + 1, i32::MIN + 1);
        assign_saturating_add(&mut p, &PointI32::min());
        assert_eq!(p, PointI32::min());
    }

    #[test]
    fn assign_saturating_add_max_bounds_max_delta() {
        let mut p = PointI32::of(i32::MAX - 1, i32::MAX - 1);
        assign_saturating_add(&mut p, &PointI32::max());
        assert_eq!(p, PointI32::max());
    }

    #[test]
    fn test_try_assign_checked_add() {
        let mut p = PointI32::of(0, 0);
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::of(10, 15)), Ok(()));
        assert_eq!(p, PointI32::of(10, 15));
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::of(-15, -25)), Ok(()));
        assert_eq!(p, PointI32::of(-5, -10));
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::of(2, 3)), Ok(()));
        assert_eq!(p, PointI32::of(-3, -7));
    }

    #[test]
    fn try_assign_checked_add_min_bounds_err() {
        let mut p = PointI32::of(i32::MIN + 2, i32::MIN + 5);
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::of(-10, -10)), Err(()));
        assert_eq!(p, PointI32::of(i32::MIN + 2, i32::MIN + 5));
    }

    #[test]
    fn try_assign_checked_add_max_bounds_err() {
        let mut p = PointI32::of(i32::MAX - 2, i32::MAX - 5);
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::of(10, 10)), Err(()));
        assert_eq!(p, PointI32::of(i32::MAX - 2, i32::MAX - 5));
    }

    #[test]
    fn try_assign_checked_add_min_bounds_ok() {
        let mut p = PointI32::of(i32::MIN + 2, i32::MIN + 5);
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::of(-2, -5)), Ok(()));
        assert_eq!(p, PointI32::min());
    }

    #[test]
    fn try_assign_checked_add_max_bounds_ok() {
        let mut p = PointI32::of(i32::MAX - 2, i32::MAX - 5);
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::of(2, 5)), Ok(()));
        assert_eq!(p, PointI32::max());
    }

    #[test]
    fn try_assign_checked_add_min_bounds_min_delta() {
        let mut p = PointI32::of(i32::MIN + 1, i32::MIN + 1);
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::min()), Err(()));
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::of(i32::MIN, 0)), Err(()));
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::of(0, i32::MIN)), Err(()));
        assert_eq!(p, PointI32::of(i32::MIN + 1, i32::MIN + 1));
    }

    #[test]
    fn try_assign_checked_add_max_bounds_max_delta() {
        let mut p = PointI32::of(i32::MAX - 1, i32::MAX - 1);
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::max()), Err(()));
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::of(i32::MAX, 0)), Err(()));
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::of(0, i32::MAX)), Err(()));
        assert_eq!(p, PointI32::of(i32::MAX - 1, i32::MAX - 1));
    }

    #[test]
    fn test_assign_checked_add() {
        let mut p = PointI32::of(0, 0);
        assign_checked_add(&mut p, &PointI32::of(10, 15));
        assert_eq!(p, PointI32::of(10, 15));
        assign_checked_add(&mut p, &PointI32::of(-15, -25));
        assert_eq!(p, PointI32::of(-5, -10));
        assign_checked_add(&mut p, &PointI32::of(2, 3));
        assert_eq!(p, PointI32::of(-3, -7));
    }

    #[test]
    fn test_saturating_add() {
        assert_eq!(saturating_add(&PointI32::of(0, 0), &PointI32::of(10, 15)), PointI32::of(10, 15));
        assert_eq!(saturating_add(&PointI32::of(0, 0), &PointI32::of(-15, -25)), PointI32::of(-15, -25));
    }

    #[test]
    fn saturating_add_to_bounds() {
        assert_eq!(saturating_add(&PointI32::of(i32::MIN + 2, i32::MIN + 5), &PointI32::of(-2, -5)), PointI32::min());
        assert_eq!(saturating_add(&PointI32::of(i32::MAX - 2, i32::MAX - 5), &PointI32::of(2, 5)), PointI32::max());
    }

    #[test]
    fn saturating_add_beyond_bounds() {
        assert_eq!(saturating_add(&PointI32::of(i32::MIN + 2, i32::MIN + 5), &PointI32::of(-10, -10)), PointI32::min());
        assert_eq!(saturating_add(&PointI32::of(i32::MAX - 2, i32::MAX - 5), &PointI32::of(10, 10)), PointI32::max());
    }

    #[test]
    fn saturating_add_limits() {
        assert_eq!(saturating_add(&PointI32::of(i32::MIN + 1, i32::MIN + 1), &PointI32::min()), PointI32::min());
        assert_eq!(saturating_add(&PointI32::of(i32::MAX - 1, i32::MAX - 1), &PointI32::max()), PointI32::max());
    }

    #[test]
    fn try_checked_add_min_bounds() {
        let p = PointI32::of(i32::MIN + 2, i32::MIN + 5);
        assert_eq!(try_checked_add(&p, &PointI32::of(-2, 0)), Some(PointI32::of(i32::MIN, i32::MIN + 5)));
        assert_eq!(try_checked_add(&p, &PointI32::of(0, -5)), Some(PointI32::of(i32::MIN + 2, i32::MIN)));
        assert_eq!(try_checked_add(&p, &PointI32::of(-2, -5)), Some(PointI32::min()));
        assert_eq!(try_checked_add(&p, &PointI32::of(-10, -10)), None);
        assert_eq!(try_checked_add(&p, &PointI32::of(i32::MIN, 0)), None);
        assert_eq!(try_checked_add(&p, &PointI32::of(0, i32::MIN)), None);
        assert_eq!(try_checked_add(&p, &PointI32::min()), None);
    }

    #[test]
    fn try_checked_add_max_bounds() {
        let p = PointI32::of(i32::MAX - 2, i32::MAX - 5);
        assert_eq!(try_checked_add(&p, &PointI32::of(2, 0)), Some(PointI32::of(i32::MAX, i32::MAX - 5)));
        assert_eq!(try_checked_add(&p, &PointI32::of(0, 5)), Some(PointI32::of(i32::MAX - 2, i32::MAX)));
        assert_eq!(try_checked_add(&p, &PointI32::of(2, 5)), Some(PointI32::max()));
        assert_eq!(try_checked_add(&p, &PointI32::of(10, 10)), None);
        assert_eq!(try_checked_add(&p, &PointI32::of(i32::MAX, 0)), None);
        assert_eq!(try_checked_add(&p, &PointI32::of(0, i32::MAX)), None);
        assert_eq!(try_checked_add(&p, &PointI32::max()), None);
    }

    #[test]
    fn test_checked_add() {
        assert_eq!(checked_add(&PointI32::of(0, 0), &PointI32::of(10, 15)), PointI32::of(10, 15));
        assert_eq!(checked_add(&PointI32::of(10, 15), &PointI32::of(-15, -25)), PointI32::of(-5, -10));
        assert_eq!(checked_add(&PointI32::of(-5, -10), &PointI32::of(2, 3)), PointI32::of(-3, -7));
    }
}

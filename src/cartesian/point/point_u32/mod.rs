use super::{point_u8::PointU8, point_u16::PointU16};

use crate::cartesian::point::point_i32::PointI32;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointU32 {
    pub x: u32,
    pub y: u32,
}

impl PointU32 {
    pub fn of(x: u32, y: u32) -> Self {
        PointU32 { x, y }
    }

    pub fn min() -> Self {
        PointU32 { x: 0, y: 0 }
    }

    pub fn max() -> Self {
        PointU32 { x: u32::MAX, y: u32::MAX }
    }
}

impl From<PointU8> for PointU32 {
    fn from(p: PointU8) -> Self {
        PointU32 { x: p.x.into(), y: p.y.into() }
    }
}

impl From<PointU16> for PointU32 {
    fn from(p: PointU16) -> Self {
        PointU32 { x: p.x.into(), y: p.y.into() }
    }
}

impl std::fmt::Display for PointU32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointU32, p2: &PointU32) -> u32 {
    p2.x - p1.x
}

pub fn delta_y(p1: &PointU32, p2: &PointU32) -> u32 {
    p2.y - p1.y
}

pub fn delta(p1: &PointU32, p2: &PointU32) -> PointU32 {
    PointU32 { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

pub fn assign_saturating_add(p: &mut PointU32, delta: &PointI32) {
    let temp_x = i64::from(p.x) + i64::from(delta.x);
    let temp_y = i64::from(p.y) + i64::from(delta.y);
    let clamped_x = temp_x.clamp(0, i64::from(u32::MAX));
    let clamped_y = temp_y.clamp(0, i64::from(u32::MAX));
    p.x = clamped_x as u32;
    p.y = clamped_y as u32;
}

pub fn try_assign_checked_add(p: &mut PointU32, delta: &PointI32) -> Result<(), ()> {
    let x = u32::try_from(i64::from(p.x) + i64::from(delta.x)).map_err(|_| ())?;
    let y = u32::try_from(i64::from(p.y) + i64::from(delta.y)).map_err(|_| ())?;
    p.x = x;
    p.y = y;
    Ok(())
}

pub fn assign_checked_add(p: &mut PointU32, delta: &PointI32) {
    try_assign_checked_add(p, delta).unwrap()
}

pub fn saturating_add(p: &PointU32, delta: &PointI32) -> PointU32 {
    let temp_x = i64::from(p.x) + i64::from(delta.x);
    let temp_y = i64::from(p.y) + i64::from(delta.y);
    let clamped_x = temp_x.clamp(0, i64::from(u32::MAX));
    let clamped_y = temp_y.clamp(0, i64::from(u32::MAX));
    PointU32::of(clamped_x as u32, clamped_y as u32)
}

pub fn try_checked_add(p: &PointU32, delta: &PointI32) -> Option<PointU32> {
    let x = u32::try_from(i64::from(p.x) + i64::from(delta.x)).ok()?;
    let y = u32::try_from(i64::from(p.y) + i64::from(delta.y)).ok()?;
    Some(PointU32 { x, y })
}

pub fn checked_add(p: &PointU32, delta: &PointI32) -> PointU32 {
    try_checked_add(p, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::{point_i32::PointI32, point_u8::PointU8, point_u16::PointU16};

    use super::{
        PointU32, assign_checked_add, assign_saturating_add, checked_add, delta, delta_x, delta_y, saturating_add, try_assign_checked_add,
        try_checked_add,
    };

    #[test]
    fn point_u32() {
        assert_eq!(PointU32::of(0, u32::MAX), PointU32 { x: 0, y: u32::MAX });
        assert_eq!(PointU32::min(), PointU32 { x: 0, y: 0 });
        assert_eq!(PointU32::max(), PointU32 { x: u32::MAX, y: u32::MAX });
    }

    #[test]
    fn from() {
        assert_eq!(PointU32::from(PointU8::min()), PointU32 { x: u8::MIN.into(), y: u8::MIN.into() });
        assert_eq!(PointU32::from(PointU8::max()), PointU32 { x: u8::MAX.into(), y: u8::MAX.into() });
        assert_eq!(PointU32::from(PointU16::min()), PointU32 { x: u16::MIN.into(), y: u16::MIN.into() });
        assert_eq!(PointU32::from(PointU16::max()), PointU32 { x: u16::MAX.into(), y: u16::MAX.into() });
    }

    #[test]
    fn to_string() {
        assert_eq!(PointU32::of(0, u32::MAX).to_string(), "(0, 4294967295)");
        assert_eq!(PointU32::min().to_string(), "(0, 0)");
        assert_eq!(PointU32::max().to_string(), "(4294967295, 4294967295)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointU32::min(), &PointU32::of(0, u32::MAX)), 0);
        assert_eq!(delta_x(&PointU32::min(), &PointU32::of(u32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointU32::min(), &PointU32::of(u32::MAX, 0)), 0);
        assert_eq!(delta_y(&PointU32::min(), &PointU32::of(0, u32::MAX)), u32::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointU32::min(), &PointU32::min()), PointU32::min());
        assert_eq!(delta(&PointU32::min(), &PointU32::max()), PointU32::max());
    }

    #[test]
    fn delta_min() {
        let p = PointU32::min();
        assert_eq!(delta(&p, &PointU32::of(0, 0)), PointU32::of(0, 0));
        assert_eq!(delta(&p, &PointU32::of(0, 1)), PointU32::of(0, 1));
        assert_eq!(delta(&p, &PointU32::of(0, 2)), PointU32::of(0, 2));

        assert_eq!(delta(&p, &PointU32::of(1, 0)), PointU32::of(1, 0));
        assert_eq!(delta(&p, &PointU32::of(1, 1)), PointU32::of(1, 1));
        assert_eq!(delta(&p, &PointU32::of(1, 2)), PointU32::of(1, 2));

        assert_eq!(delta(&p, &PointU32::of(2, 0)), PointU32::of(2, 0));
        assert_eq!(delta(&p, &PointU32::of(2, 1)), PointU32::of(2, 1));
        assert_eq!(delta(&p, &PointU32::of(2, 2)), PointU32::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p = PointU32::of(u32::MAX - 2, u32::MAX - 2);
        assert_eq!(delta(&p, &PointU32::of(u32::MAX - 2, u32::MAX - 2)), PointU32::of(0, 0));
        assert_eq!(delta(&p, &PointU32::of(u32::MAX - 2, u32::MAX - 1)), PointU32::of(0, 1));
        assert_eq!(delta(&p, &PointU32::of(u32::MAX - 2, u32::MAX)), PointU32::of(0, 2));

        assert_eq!(delta(&p, &PointU32::of(u32::MAX - 1, u32::MAX - 2)), PointU32::of(1, 0));
        assert_eq!(delta(&p, &PointU32::of(u32::MAX - 1, u32::MAX - 1)), PointU32::of(1, 1));
        assert_eq!(delta(&p, &PointU32::of(u32::MAX - 1, u32::MAX)), PointU32::of(1, 2));

        assert_eq!(delta(&p, &PointU32::of(u32::MAX, u32::MAX - 2)), PointU32::of(2, 0));
        assert_eq!(delta(&p, &PointU32::of(u32::MAX, u32::MAX - 1)), PointU32::of(2, 1));
        assert_eq!(delta(&p, &PointU32::max()), PointU32::of(2, 2));
    }

    #[test]
    fn test_assign_saturating_add() {
        let mut p = PointU32::of(0, 0);
        assign_saturating_add(&mut p, &PointI32::of(10, 13));
        assert_eq!(p, PointU32::of(10, 13));
        assign_saturating_add(&mut p, &PointI32::of(-5, -3));
        assert_eq!(p, PointU32::of(5, 10));
        assign_saturating_add(&mut p, &PointI32::of(2, -4));
        assert_eq!(p, PointU32::of(7, 6));
    }

    #[test]
    fn assign_saturating_add_min_bounds() {
        let mut p = PointU32::of(2, 5);
        assign_saturating_add(&mut p, &PointI32::of(-10, -10));
        assert_eq!(p, PointU32::of(0, 0));
    }

    #[test]
    fn assign_saturating_add_max_bounds() {
        let mut p = PointU32::of(u32::MAX - 2, u32::MAX - 5);
        assign_saturating_add(&mut p, &PointI32::of(10, 10));
        assert_eq!(p, PointU32::max());
    }

    #[test]
    fn assign_saturating_add_min_bounds_min_delta() {
        let mut p = PointU32::of(1, 1);
        assign_saturating_add(&mut p, &PointI32::min());
        assert_eq!(p, PointU32::of(0, 0));
    }

    #[test]
    fn assign_saturating_add_max_bounds_max_delta() {
        let mut p = PointU32::of(u32::MAX - 1, u32::MAX - 1);
        assign_saturating_add(&mut p, &PointI32::max());
        assert_eq!(p, PointU32::max());
    }

    #[test]
    fn test_try_assign_checked_add() {
        let mut p = PointU32::of(0, 0);
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::of(10, 13)), Ok(()));
        assert_eq!(p, PointU32::of(10, 13));
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::of(-5, -3)), Ok(()));
        assert_eq!(p, PointU32::of(5, 10));
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::of(2, -4)), Ok(()));
        assert_eq!(p, PointU32::of(7, 6));
    }

    #[test]
    fn try_assign_checked_add_min_bounds_err() {
        let mut p = PointU32::of(2, 5);
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::of(-10, -10)), Err(()));
        assert_eq!(p, PointU32::of(2, 5));
    }

    #[test]
    fn try_assign_checked_add_max_bounds_err() {
        let mut p = PointU32::of(u32::MAX - 2, u32::MAX - 5);
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::of(10, 10)), Err(()));
        assert_eq!(p, PointU32::of(u32::MAX - 2, u32::MAX - 5));
    }

    #[test]
    fn try_assign_checked_add_min_bounds_ok() {
        let mut p = PointU32::of(2, 5);
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::of(-2, -5)), Ok(()));
        assert_eq!(p, PointU32::of(0, 0));
    }

    #[test]
    fn try_assign_checked_add_max_bounds_ok() {
        let mut p = PointU32::of(u32::MAX - 2, u32::MAX - 5);
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::of(2, 5)), Ok(()));
        assert_eq!(p, PointU32::max());
    }

    #[test]
    fn try_assign_checked_add_min_bounds_min_delta() {
        let mut p = PointU32::of(1, 1);
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::min()), Err(()));
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::of(i32::MIN, 0)), Err(()));
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::of(0, i32::MIN)), Err(()));
        assert_eq!(p, PointU32::of(1, 1));
    }

    #[test]
    fn try_assign_checked_add_max_bounds_max_delta() {
        let mut p = PointU32::of(u32::MAX - 1, u32::MAX - 1);
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::max()), Err(()));
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::of(i32::MAX, 0)), Err(()));
        assert_eq!(try_assign_checked_add(&mut p, &PointI32::of(0, i32::MAX)), Err(()));
        assert_eq!(p, PointU32::of(u32::MAX - 1, u32::MAX - 1));
    }

    #[test]
    fn test_assign_checked_add() {
        let mut p = PointU32::of(0, 0);
        assign_checked_add(&mut p, &PointI32::of(10, 13));
        assert_eq!(p, PointU32::of(10, 13));
        assign_checked_add(&mut p, &PointI32::of(-5, -3));
        assert_eq!(p, PointU32::of(5, 10));
        assign_checked_add(&mut p, &PointI32::of(2, -4));
        assert_eq!(p, PointU32::of(7, 6));
    }

    #[test]
    fn test_saturating_add() {
        assert_eq!(saturating_add(&PointU32::of(0, 0), &PointI32::of(10, 13)), PointU32::of(10, 13));
        assert_eq!(saturating_add(&PointU32::of(10, 10), &PointI32::of(-5, -3)), PointU32::of(5, 7));
    }

    #[test]
    fn saturating_add_to_bounds() {
        assert_eq!(saturating_add(&PointU32::of(2, 5), &PointI32::of(-2, -5)), PointU32::min());
        assert_eq!(saturating_add(&PointU32::of(u32::MAX - 2, u32::MAX - 5), &PointI32::of(2, 5)), PointU32::max());
    }

    #[test]
    fn saturating_add_beyond_bounds() {
        assert_eq!(saturating_add(&PointU32::of(2, 5), &PointI32::of(-10, -10)), PointU32::min());
        assert_eq!(saturating_add(&PointU32::of(u32::MAX - 2, u32::MAX - 5), &PointI32::of(10, 10)), PointU32::max());
    }

    #[test]
    fn saturating_add_limits() {
        assert_eq!(saturating_add(&PointU32::of(1, 1), &PointI32::min()), PointU32::min());
        assert_eq!(saturating_add(&PointU32::of(u32::MAX - 1, u32::MAX - 1), &PointI32::max()), PointU32::max());
    }

    #[test]
    fn try_checked_add_min_bounds() {
        let p = PointU32::of(2, 5);
        assert_eq!(try_checked_add(&p, &PointI32::of(-2, 0)), Some(PointU32::of(0, 5)));
        assert_eq!(try_checked_add(&p, &PointI32::of(0, -5)), Some(PointU32::of(2, 0)));
        assert_eq!(try_checked_add(&p, &PointI32::of(-2, -5)), Some(PointU32::min()));
        assert_eq!(try_checked_add(&p, &PointI32::of(-10, -10)), None);
        assert_eq!(try_checked_add(&p, &PointI32::of(i32::MIN, 0)), None);
        assert_eq!(try_checked_add(&p, &PointI32::of(0, i32::MIN)), None);
        assert_eq!(try_checked_add(&p, &PointI32::min()), None);
    }

    #[test]
    fn try_checked_add_max_bounds() {
        let p = PointU32::of(u32::MAX - 2, u32::MAX - 5);
        assert_eq!(try_checked_add(&p, &PointI32::of(2, 0)), Some(PointU32::of(u32::MAX, u32::MAX - 5)));
        assert_eq!(try_checked_add(&p, &PointI32::of(0, 5)), Some(PointU32::of(u32::MAX - 2, u32::MAX)));
        assert_eq!(try_checked_add(&p, &PointI32::of(2, 5)), Some(PointU32::max()));
        assert_eq!(try_checked_add(&p, &PointI32::of(10, 10)), None);
        assert_eq!(try_checked_add(&p, &PointI32::of(i32::MAX, 0)), None);
        assert_eq!(try_checked_add(&p, &PointI32::of(0, i32::MAX)), None);
        assert_eq!(try_checked_add(&p, &PointI32::max()), None);
    }

    #[test]
    fn test_checked_add() {
        assert_eq!(checked_add(&PointU32::of(0, 0), &PointI32::of(10, 13)), PointU32::of(10, 13));
        assert_eq!(checked_add(&PointU32::of(10, 13), &PointI32::of(-5, -3)), PointU32::of(5, 10));
        assert_eq!(checked_add(&PointU32::of(5, 10), &PointI32::of(2, -4)), PointU32::of(7, 6));
    }
}

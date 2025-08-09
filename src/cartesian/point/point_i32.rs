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

pub fn saturating_translate(p: &mut PointI32, delta: &PointI32) {
    p.x = p.x.saturating_add(delta.x);
    p.y = p.y.saturating_add(delta.y);
}

pub fn checked_translate(p: &mut PointI32, delta: &PointI32) -> Result<(), ()> {
    let x = p.x.checked_add(delta.x).ok_or(())?;
    let y = p.y.checked_add(delta.y).ok_or(())?;
    p.x = x;
    p.y = y;
    Ok(())
}

pub fn saturating_translated(p: &PointI32, delta: &PointI32) -> PointI32 {
    PointI32::of(p.x.saturating_add(delta.x), p.y.saturating_add(delta.y))
}

pub fn checked_translated(p: &PointI32, delta: &PointI32) -> Option<PointI32> {
    let x = p.x.checked_add(delta.x)?;
    let y = p.y.checked_add(delta.y)?;
    Some(PointI32 { x, y })
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_u32::PointU32;

    use super::{PointI32, checked_translate, checked_translated, delta, delta_x, delta_y, saturating_translate, saturating_translated};

    #[test]
    fn point_i32() {
        assert_eq!(PointI32::of(i32::MIN, i32::MAX), PointI32 { x: i32::MIN, y: i32::MAX });
        assert_eq!(PointI32::min(), PointI32 { x: i32::MIN, y: i32::MIN });
        assert_eq!(PointI32::max(), PointI32 { x: i32::MAX, y: i32::MAX });
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
        let p1 = PointI32::of(i32::MIN, i32::MIN);
        assert_eq!(delta(&p1, &PointI32::of(i32::MIN, i32::MIN)), PointU32::of(0, 0));
        assert_eq!(delta(&p1, &PointI32::of(i32::MIN, i32::MIN + 1)), PointU32::of(0, 1));
        assert_eq!(delta(&p1, &PointI32::of(i32::MIN, i32::MIN + 2)), PointU32::of(0, 2));

        assert_eq!(delta(&p1, &PointI32::of(i32::MIN + 1, i32::MIN)), PointU32::of(1, 0));
        assert_eq!(delta(&p1, &PointI32::of(i32::MIN + 1, i32::MIN + 1)), PointU32::of(1, 1));
        assert_eq!(delta(&p1, &PointI32::of(i32::MIN + 1, i32::MIN + 2)), PointU32::of(1, 2));

        assert_eq!(delta(&p1, &PointI32::of(i32::MIN + 2, i32::MIN)), PointU32::of(2, 0));
        assert_eq!(delta(&p1, &PointI32::of(i32::MIN + 2, i32::MIN + 1)), PointU32::of(2, 1));
        assert_eq!(delta(&p1, &PointI32::of(i32::MIN + 2, i32::MIN + 2)), PointU32::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p1 = PointI32::of(i32::MAX - 2, i32::MAX - 2);
        assert_eq!(delta(&p1, &PointI32::of(i32::MAX - 2, i32::MAX - 2)), PointU32::of(0, 0));
        assert_eq!(delta(&p1, &PointI32::of(i32::MAX - 2, i32::MAX - 1)), PointU32::of(0, 1));
        assert_eq!(delta(&p1, &PointI32::of(i32::MAX - 2, i32::MAX)), PointU32::of(0, 2));

        assert_eq!(delta(&p1, &PointI32::of(i32::MAX - 1, i32::MAX - 2)), PointU32::of(1, 0));
        assert_eq!(delta(&p1, &PointI32::of(i32::MAX - 1, i32::MAX - 1)), PointU32::of(1, 1));
        assert_eq!(delta(&p1, &PointI32::of(i32::MAX - 1, i32::MAX)), PointU32::of(1, 2));

        assert_eq!(delta(&p1, &PointI32::of(i32::MAX, i32::MAX - 2)), PointU32::of(2, 0));
        assert_eq!(delta(&p1, &PointI32::of(i32::MAX, i32::MAX - 1)), PointU32::of(2, 1));
        assert_eq!(delta(&p1, &PointI32::of(i32::MAX, i32::MAX)), PointU32::of(2, 2));
    }

    #[test]
    fn test_saturating_translate() {
        let mut r = PointI32::of(0, 0);
        saturating_translate(&mut r, &PointI32::of(10, 15));
        assert_eq!(r, PointI32::of(10, 15));
        saturating_translate(&mut r, &PointI32::of(-15, -25));
        assert_eq!(r, PointI32::of(-5, -10));
        saturating_translate(&mut r, &PointI32::of(2, 3));
        assert_eq!(r, PointI32::of(-3, -7));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut r = PointI32::of(i32::MIN + 2, i32::MIN + 5);
        saturating_translate(&mut r, &PointI32::of(-10, -10));
        assert_eq!(r, PointI32::of(i32::MIN, i32::MIN));
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut r = PointI32::of(i32::MAX - 2, i32::MAX - 5);
        saturating_translate(&mut r, &PointI32::of(10, 10));
        assert_eq!(r, PointI32::of(i32::MAX, i32::MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_min_delta() {
        let mut r = PointI32::of(i32::MIN + 1, i32::MIN + 1);
        saturating_translate(&mut r, &PointI32::min());
        assert_eq!(r, PointI32::of(i32::MIN, i32::MIN));
    }

    #[test]
    fn saturating_translate_max_bounds_max_delta() {
        let mut r = PointI32::of(i32::MAX - 1, i32::MAX - 1);
        saturating_translate(&mut r, &PointI32::max());
        assert_eq!(r, PointI32::of(i32::MAX, i32::MAX));
    }

    #[test]
    fn test_checked_translate() {
        let mut r = PointI32::of(0, 0);
        assert_eq!(checked_translate(&mut r, &PointI32::of(10, 15)), Ok(()));
        assert_eq!(r, PointI32::of(10, 15));
        assert_eq!(checked_translate(&mut r, &PointI32::of(-15, -25)), Ok(()));
        assert_eq!(r, PointI32::of(-5, -10));
        assert_eq!(checked_translate(&mut r, &PointI32::of(2, 3)), Ok(()));
        assert_eq!(r, PointI32::of(-3, -7));
    }

    #[test]
    fn checked_translate_min_bounds_err() {
        let mut r = PointI32::of(i32::MIN + 2, i32::MIN + 5);
        assert_eq!(checked_translate(&mut r, &PointI32::of(-10, -10)), Err(()));
        assert_eq!(r, PointI32::of(i32::MIN + 2, i32::MIN + 5));
    }

    #[test]
    fn checked_translate_max_bounds_err() {
        let mut r = PointI32::of(i32::MAX - 2, i32::MAX - 5);
        assert_eq!(checked_translate(&mut r, &PointI32::of(10, 10)), Err(()));
        assert_eq!(r, PointI32::of(i32::MAX - 2, i32::MAX - 5));
    }

    #[test]
    fn checked_translate_min_bounds_ok() {
        let mut r = PointI32::of(i32::MIN + 2, i32::MIN + 5);
        assert_eq!(checked_translate(&mut r, &PointI32::of(-2, -5)), Ok(()));
        assert_eq!(r, PointI32::of(i32::MIN, i32::MIN));
    }

    #[test]
    fn checked_translate_max_bounds_ok() {
        let mut r = PointI32::of(i32::MAX - 2, i32::MAX - 5);
        assert_eq!(checked_translate(&mut r, &PointI32::of(2, 5)), Ok(()));
        assert_eq!(r, PointI32::of(i32::MAX, i32::MAX));
    }

    #[test]
    fn checked_translate_min_bounds_min_delta() {
        let mut r = PointI32::of(i32::MIN + 1, i32::MIN + 1);
        assert_eq!(checked_translate(&mut r, &PointI32::min()), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI32::of(i32::MIN, 0)), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI32::of(0, i32::MIN)), Err(()));
        assert_eq!(r, PointI32::of(i32::MIN + 1, i32::MIN + 1));
    }

    #[test]
    fn checked_translate_max_bounds_max_delta() {
        let mut r = PointI32::of(i32::MAX - 1, i32::MAX - 1);
        assert_eq!(checked_translate(&mut r, &PointI32::max()), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI32::of(i32::MAX, 0)), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI32::of(0, i32::MAX)), Err(()));
        assert_eq!(r, PointI32::of(i32::MAX - 1, i32::MAX - 1));
    }

    #[test]
    fn test_saturating_translated() {
        assert_eq!(saturating_translated(&PointI32::of(0, 0), &PointI32::of(10, 15)), PointI32::of(10, 15));
        assert_eq!(saturating_translated(&PointI32::of(0, 0), &PointI32::of(-15, -25)), PointI32::of(-15, -25));
    }

    #[test]
    fn saturating_translated_to_bounds() {
        assert_eq!(saturating_translated(&PointI32::of(i32::MIN + 2, i32::MIN + 5), &PointI32::of(-2, -5)), PointI32::min());
        assert_eq!(saturating_translated(&PointI32::of(i32::MAX - 2, i32::MAX - 5), &PointI32::of(2, 5)), PointI32::max());
    }

    #[test]
    fn saturating_translated_beyond_bounds() {
        assert_eq!(saturating_translated(&PointI32::of(i32::MIN + 2, i32::MIN + 5), &PointI32::of(-10, -10)), PointI32::min());
        assert_eq!(saturating_translated(&PointI32::of(i32::MAX - 2, i32::MAX - 5), &PointI32::of(10, 10)), PointI32::max());
    }

    #[test]
    fn saturating_translated_limits() {
        assert_eq!(saturating_translated(&PointI32::of(i32::MIN + 1, i32::MIN + 1), &PointI32::min()), PointI32::min());
        assert_eq!(saturating_translated(&PointI32::of(i32::MAX - 1, i32::MAX - 1), &PointI32::max()), PointI32::max());
    }

    #[test]
    fn checked_translated_min_bounds() {
        let p = PointI32::of(i32::MIN + 2, i32::MIN + 5);
        assert_eq!(checked_translated(&p, &PointI32::of(-2, 0)), Some(PointI32::of(i32::MIN, i32::MIN + 5)));
        assert_eq!(checked_translated(&p, &PointI32::of(0, -5)), Some(PointI32::of(i32::MIN + 2, i32::MIN)));
        assert_eq!(checked_translated(&p, &PointI32::of(-2, -5)), Some(PointI32::min()));
        assert_eq!(checked_translated(&p, &PointI32::of(-10, -10)), None);
        assert_eq!(checked_translated(&p, &PointI32::of(i32::MIN, 0)), None);
        assert_eq!(checked_translated(&p, &PointI32::of(0, i32::MIN)), None);
        assert_eq!(checked_translated(&p, &PointI32::min()), None);
    }

    #[test]
    fn checked_translated_max_bounds() {
        let p = PointI32::of(i32::MAX - 2, i32::MAX - 5);
        assert_eq!(checked_translated(&p, &PointI32::of(2, 0)), Some(PointI32::of(i32::MAX, i32::MAX - 5)));
        assert_eq!(checked_translated(&p, &PointI32::of(0, 5)), Some(PointI32::of(i32::MAX - 2, i32::MAX)));
        assert_eq!(checked_translated(&p, &PointI32::of(2, 5)), Some(PointI32::max()));
        assert_eq!(checked_translated(&p, &PointI32::of(10, 10)), None);
        assert_eq!(checked_translated(&p, &PointI32::of(i32::MAX, 0)), None);
        assert_eq!(checked_translated(&p, &PointI32::of(0, i32::MAX)), None);
        assert_eq!(checked_translated(&p, &PointI32::max()), None);
    }
}

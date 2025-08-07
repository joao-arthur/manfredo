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

pub fn checked_translate(p: &mut PointI64, delta: &PointI64) -> Result<(), ()> {
    let x = p.x.checked_add(delta.x).ok_or(())?;
    let y = p.y.checked_add(delta.y).ok_or(())?;
    p.x = x;
    p.y = y;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_u64::PointU64;

    use super::{PointI64, checked_translate, delta, delta_x, delta_y, saturating_translate};

    #[test]
    fn point_i64() {
        assert_eq!(PointI64::of(i64::MIN, i64::MAX), PointI64 { x: i64::MIN, y: i64::MAX });
        assert_eq!(PointI64::min(), PointI64 { x: i64::MIN, y: i64::MIN });
        assert_eq!(PointI64::max(), PointI64 { x: i64::MAX, y: i64::MAX });
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
        let p1 = PointI64::of(i64::MIN, i64::MIN);
        assert_eq!(delta(&p1, &PointI64::of(i64::MIN, i64::MIN)), PointU64::of(0, 0));
        assert_eq!(delta(&p1, &PointI64::of(i64::MIN, i64::MIN + 1)), PointU64::of(0, 1));
        assert_eq!(delta(&p1, &PointI64::of(i64::MIN, i64::MIN + 2)), PointU64::of(0, 2));

        assert_eq!(delta(&p1, &PointI64::of(i64::MIN + 1, i64::MIN)), PointU64::of(1, 0));
        assert_eq!(delta(&p1, &PointI64::of(i64::MIN + 1, i64::MIN + 1)), PointU64::of(1, 1));
        assert_eq!(delta(&p1, &PointI64::of(i64::MIN + 1, i64::MIN + 2)), PointU64::of(1, 2));

        assert_eq!(delta(&p1, &PointI64::of(i64::MIN + 2, i64::MIN)), PointU64::of(2, 0));
        assert_eq!(delta(&p1, &PointI64::of(i64::MIN + 2, i64::MIN + 1)), PointU64::of(2, 1));
        assert_eq!(delta(&p1, &PointI64::of(i64::MIN + 2, i64::MIN + 2)), PointU64::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p1 = PointI64::of(i64::MAX - 2, i64::MAX - 2);
        assert_eq!(delta(&p1, &PointI64::of(i64::MAX - 2, i64::MAX - 2)), PointU64::of(0, 0));
        assert_eq!(delta(&p1, &PointI64::of(i64::MAX - 2, i64::MAX - 1)), PointU64::of(0, 1));
        assert_eq!(delta(&p1, &PointI64::of(i64::MAX - 2, i64::MAX)), PointU64::of(0, 2));

        assert_eq!(delta(&p1, &PointI64::of(i64::MAX - 1, i64::MAX - 2)), PointU64::of(1, 0));
        assert_eq!(delta(&p1, &PointI64::of(i64::MAX - 1, i64::MAX - 1)), PointU64::of(1, 1));
        assert_eq!(delta(&p1, &PointI64::of(i64::MAX - 1, i64::MAX)), PointU64::of(1, 2));

        assert_eq!(delta(&p1, &PointI64::of(i64::MAX, i64::MAX - 2)), PointU64::of(2, 0));
        assert_eq!(delta(&p1, &PointI64::of(i64::MAX, i64::MAX - 1)), PointU64::of(2, 1));
        assert_eq!(delta(&p1, &PointI64::of(i64::MAX, i64::MAX)), PointU64::of(2, 2));
    }

    #[test]
    fn test_saturating_translate() {
        let mut r = PointI64::of(0, 0);
        saturating_translate(&mut r, &PointI64::of(10, 10));
        assert_eq!(r, PointI64::of(10, 10));
        saturating_translate(&mut r, &PointI64::of(-15, -15));
        assert_eq!(r, PointI64::of(-5, -5));
        saturating_translate(&mut r, &PointI64::of(2, 2));
        assert_eq!(r, PointI64::of(-3, -3));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut r = PointI64::of(i64::MIN + 2, i64::MIN + 5);
        saturating_translate(&mut r, &PointI64::of(-10, -10));
        assert_eq!(r, PointI64::of(i64::MIN, i64::MIN));
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut r = PointI64::of(i64::MAX - 2, i64::MAX - 5);
        saturating_translate(&mut r, &PointI64::of(10, 10));
        assert_eq!(r, PointI64::of(i64::MAX, i64::MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_min_delta() {
        let mut r = PointI64::of(i64::MIN + 1, i64::MIN + 1);
        saturating_translate(&mut r, &PointI64::min());
        assert_eq!(r, PointI64::of(i64::MIN, i64::MIN));
    }

    #[test]
    fn saturating_translate_max_bounds_max_delta() {
        let mut r = PointI64::of(i64::MAX - 1, i64::MAX - 1);
        saturating_translate(&mut r, &PointI64::max());
        assert_eq!(r, PointI64::of(i64::MAX, i64::MAX));
    }

    #[test]
    fn test_checked_translate() {
        let mut r = PointI64::of(0, 0);
        assert_eq!(checked_translate(&mut r, &PointI64::of(10, 10)), Ok(()));
        assert_eq!(r, PointI64::of(10, 10));
        assert_eq!(checked_translate(&mut r, &PointI64::of(-15, -15)), Ok(()));
        assert_eq!(r, PointI64::of(-5, -5));
        assert_eq!(checked_translate(&mut r, &PointI64::of(2, 2)), Ok(()));
        assert_eq!(r, PointI64::of(-3, -3));
    }

    #[test]
    fn checked_translate_min_bounds_err() {
        let mut r = PointI64::of(i64::MIN + 2, i64::MIN + 5);
        assert_eq!(checked_translate(&mut r, &PointI64::of(-10, -10)), Err(()));
        assert_eq!(r, PointI64::of(i64::MIN + 2, i64::MIN + 5));
    }

    #[test]
    fn checked_translate_max_bounds_err() {
        let mut r = PointI64::of(i64::MAX - 2, i64::MAX - 5);
        assert_eq!(checked_translate(&mut r, &PointI64::of(10, 10)), Err(()));
        assert_eq!(r, PointI64::of(i64::MAX - 2, i64::MAX - 5));
    }

    #[test]
    fn checked_translate_min_bounds_ok() {
        let mut r = PointI64::of(i64::MIN + 2, i64::MIN + 5);
        assert_eq!(checked_translate(&mut r, &PointI64::of(-2, -5)), Ok(()));
        assert_eq!(r, PointI64::of(i64::MIN, i64::MIN));
    }

    #[test]
    fn checked_translate_max_bounds_ok() {
        let mut r = PointI64::of(i64::MAX - 2, i64::MAX - 5);
        assert_eq!(checked_translate(&mut r, &PointI64::of(2, 5)), Ok(()));
        assert_eq!(r, PointI64::of(i64::MAX, i64::MAX));
    }

    #[test]
    fn checked_translate_min_bounds_min_delta() {
        let mut r = PointI64::of(i64::MIN + 1, i64::MIN + 1);
        assert_eq!(checked_translate(&mut r, &PointI64::min()), Err(()));
        assert_eq!(r, PointI64::of(i64::MIN + 1, i64::MIN + 1));
    }

    #[test]
    fn checked_translate_max_bounds_max_delta() {
        let mut r = PointI64::of(i64::MAX - 1, i64::MAX - 1);
        assert_eq!(checked_translate(&mut r, &PointI64::max()), Err(()));
        assert_eq!(r, PointI64::of(i64::MAX - 1, i64::MAX - 1));
    }
}

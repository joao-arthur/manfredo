use crate::cartesian::point::point_i64::PointI64;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointU64 {
    pub x: u64,
    pub y: u64,
}

impl PointU64 {
    pub fn of(x: u64, y: u64) -> Self {
        PointU64 { x, y }
    }

    pub fn min() -> Self {
        PointU64 { x: 0, y: 0 }
    }

    pub fn max() -> Self {
        PointU64 { x: u64::MAX, y: u64::MAX }
    }
}

impl std::fmt::Display for PointU64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointU64, p2: &PointU64) -> u64 {
    p2.x - p1.x
}

pub fn delta_y(p1: &PointU64, p2: &PointU64) -> u64 {
    p2.y - p1.y
}

pub fn delta(p1: &PointU64, p2: &PointU64) -> PointU64 {
    PointU64 { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

pub fn saturating_translate(p: &mut PointU64, delta: &PointI64) {
    let temp_x = i128::from(p.x) + i128::from(delta.x);
    let temp_y = i128::from(p.y) + i128::from(delta.y);
    let clamped_x = temp_x.clamp(0, i128::from(u64::MAX));
    let clamped_y = temp_y.clamp(0, i128::from(u64::MAX));
    p.x = clamped_x as u64;
    p.y = clamped_y as u64;
}

pub fn checked_translate(p: &mut PointU64, delta: &PointI64) -> Result<(), ()> {
    let x = u64::try_from(i128::from(p.x) + i128::from(delta.x)).map_err(|_| ())?;
    let y = u64::try_from(i128::from(p.y) + i128::from(delta.y)).map_err(|_| ())?;
    p.x = x;
    p.y = y;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_i64::PointI64;

    use super::{PointU64, checked_translate, delta, delta_x, delta_y, saturating_translate};

    #[test]
    fn point_u64() {
        assert_eq!(PointU64::of(0, u64::MAX), PointU64 { x: 0, y: u64::MAX });
        assert_eq!(PointU64::min(), PointU64 { x: 0, y: 0 });
        assert_eq!(PointU64::max(), PointU64 { x: u64::MAX, y: u64::MAX });
        assert_eq!(PointU64::of(0, u64::MAX).to_string(), "(0, 18446744073709551615)");
        assert_eq!(PointU64::min().to_string(), "(0, 0)");
        assert_eq!(PointU64::max().to_string(), "(18446744073709551615, 18446744073709551615)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointU64::min(), &PointU64::of(0, u64::MAX)), 0);
        assert_eq!(delta_x(&PointU64::min(), &PointU64::of(u64::MAX, 0)), u64::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointU64::min(), &PointU64::of(u64::MAX, 0)), 0);
        assert_eq!(delta_y(&PointU64::min(), &PointU64::of(0, u64::MAX)), u64::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointU64::min(), &PointU64::min()), PointU64::min());
        assert_eq!(delta(&PointU64::min(), &PointU64::max()), PointU64::max());
    }

    #[test]
    fn delta_min() {
        let p1 = PointU64::min();
        assert_eq!(delta(&p1, &PointU64::of(0, 0)), PointU64::of(0, 0));
        assert_eq!(delta(&p1, &PointU64::of(0, 1)), PointU64::of(0, 1));
        assert_eq!(delta(&p1, &PointU64::of(0, 2)), PointU64::of(0, 2));

        assert_eq!(delta(&p1, &PointU64::of(1, 0)), PointU64::of(1, 0));
        assert_eq!(delta(&p1, &PointU64::of(1, 1)), PointU64::of(1, 1));
        assert_eq!(delta(&p1, &PointU64::of(1, 2)), PointU64::of(1, 2));

        assert_eq!(delta(&p1, &PointU64::of(2, 0)), PointU64::of(2, 0));
        assert_eq!(delta(&p1, &PointU64::of(2, 1)), PointU64::of(2, 1));
        assert_eq!(delta(&p1, &PointU64::of(2, 2)), PointU64::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p1 = PointU64::of(u64::MAX - 2, u64::MAX - 2);
        assert_eq!(delta(&p1, &PointU64::of(u64::MAX - 2, u64::MAX - 2)), PointU64::of(0, 0));
        assert_eq!(delta(&p1, &PointU64::of(u64::MAX - 2, u64::MAX - 1)), PointU64::of(0, 1));
        assert_eq!(delta(&p1, &PointU64::of(u64::MAX - 2, u64::MAX)), PointU64::of(0, 2));

        assert_eq!(delta(&p1, &PointU64::of(u64::MAX - 1, u64::MAX - 2)), PointU64::of(1, 0));
        assert_eq!(delta(&p1, &PointU64::of(u64::MAX - 1, u64::MAX - 1)), PointU64::of(1, 1));
        assert_eq!(delta(&p1, &PointU64::of(u64::MAX - 1, u64::MAX)), PointU64::of(1, 2));

        assert_eq!(delta(&p1, &PointU64::of(u64::MAX, u64::MAX - 2)), PointU64::of(2, 0));
        assert_eq!(delta(&p1, &PointU64::of(u64::MAX, u64::MAX - 1)), PointU64::of(2, 1));
        assert_eq!(delta(&p1, &PointU64::of(u64::MAX, u64::MAX)), PointU64::of(2, 2));
    }

    #[test]
    fn test_saturating_translate() {
        let mut r = PointU64::of(0, 0);
        saturating_translate(&mut r, &PointI64::of(10, 13));
        assert_eq!(r, PointU64::of(10, 13));
        saturating_translate(&mut r, &PointI64::of(-5, -3));
        assert_eq!(r, PointU64::of(5, 10));
        saturating_translate(&mut r, &PointI64::of(2, -4));
        assert_eq!(r, PointU64::of(7, 6));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut r = PointU64::of(2, 5);
        saturating_translate(&mut r, &PointI64::of(-10, -10));
        assert_eq!(r, PointU64::of(0, 0));
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut r = PointU64::of(u64::MAX - 2, u64::MAX - 5);
        saturating_translate(&mut r, &PointI64::of(10, 10));
        assert_eq!(r, PointU64::of(u64::MAX, u64::MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_min_delta() {
        let mut r = PointU64::of(1, 1);
        saturating_translate(&mut r, &PointI64::min());
        assert_eq!(r, PointU64::of(0, 0));
    }

    #[test]
    fn saturating_translate_max_bounds_max_delta() {
        let mut r = PointU64::of(u64::MAX - 1, u64::MAX - 1);
        saturating_translate(&mut r, &PointI64::max());
        assert_eq!(r, PointU64::of(u64::MAX, u64::MAX));
    }

    #[test]
    fn test_checked_translate() {
        let mut r = PointU64::of(0, 0);
        assert_eq!(checked_translate(&mut r, &PointI64::of(10, 13)), Ok(()));
        assert_eq!(r, PointU64::of(10, 13));
        assert_eq!(checked_translate(&mut r, &PointI64::of(-5, -3)), Ok(()));
        assert_eq!(r, PointU64::of(5, 10));
        assert_eq!(checked_translate(&mut r, &PointI64::of(2, -4)), Ok(()));
        assert_eq!(r, PointU64::of(7, 6));
    }

    #[test]
    fn checked_translate_min_bounds_err() {
        let mut r = PointU64::of(2, 5);
        assert_eq!(checked_translate(&mut r, &PointI64::of(-10, -10)), Err(()));
        assert_eq!(r, PointU64::of(2, 5));
    }

    #[test]
    fn checked_translate_max_bounds_err() {
        let mut r = PointU64::of(u64::MAX - 2, u64::MAX - 5);
        assert_eq!(checked_translate(&mut r, &PointI64::of(10, 10)), Err(()));
        assert_eq!(r, PointU64::of(u64::MAX - 2, u64::MAX - 5));
    }

    #[test]
    fn checked_translate_min_bounds_ok() {
        let mut r = PointU64::of(2, 5);
        assert_eq!(checked_translate(&mut r, &PointI64::of(-2, -5)), Ok(()));
        assert_eq!(r, PointU64::of(0, 0));
    }

    #[test]
    fn checked_translate_max_bounds_ok() {
        let mut r = PointU64::of(u64::MAX - 2, u64::MAX - 5);
        assert_eq!(checked_translate(&mut r, &PointI64::of(2, 5)), Ok(()));
        assert_eq!(r, PointU64::of(u64::MAX, u64::MAX));
    }

    #[test]
    fn checked_translate_min_bounds_min_delta() {
        let mut r = PointU64::of(1, 1);
        assert_eq!(checked_translate(&mut r, &PointI64::min()), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI64::of(i64::MIN, 0)), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI64::of(0, i64::MIN)), Err(()));
        assert_eq!(r, PointU64::of(1, 1));
    }

    #[test]
    fn checked_translate_max_bounds_max_delta() {
        let mut r = PointU64::of(u64::MAX - 1, u64::MAX - 1);
        assert_eq!(checked_translate(&mut r, &PointI64::max()), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI64::of(i64::MAX, 0)), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI64::of(0, i64::MAX)), Err(()));
        assert_eq!(r, PointU64::of(u64::MAX - 1, u64::MAX - 1));
    }
}

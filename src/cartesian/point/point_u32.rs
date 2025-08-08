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

pub fn saturating_translate(p: &mut PointU32, delta: &PointI32) {
    let temp_x = i64::from(p.x) + i64::from(delta.x);
    let temp_y = i64::from(p.y) + i64::from(delta.y);
    let clamped_x = temp_x.clamp(0, i64::from(u32::MAX));
    let clamped_y = temp_y.clamp(0, i64::from(u32::MAX));
    p.x = clamped_x as u32;
    p.y = clamped_y as u32;
}

pub fn checked_translate(p: &mut PointU32, delta: &PointI32) -> Result<(), ()> {
    let x = u32::try_from(i64::from(p.x) + i64::from(delta.x)).map_err(|_| ())?;
    let y = u32::try_from(i64::from(p.y) + i64::from(delta.y)).map_err(|_| ())?;
    p.x = x;
    p.y = y;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_i32::PointI32;

    use super::{PointU32, checked_translate, delta, delta_x, delta_y, saturating_translate};

    #[test]
    fn point_u32() {
        assert_eq!(PointU32::of(0, u32::MAX), PointU32 { x: 0, y: u32::MAX });
        assert_eq!(PointU32::min(), PointU32 { x: 0, y: 0 });
        assert_eq!(PointU32::max(), PointU32 { x: u32::MAX, y: u32::MAX });
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
        let p1 = PointU32::min();
        assert_eq!(delta(&p1, &PointU32::of(0, 0)), PointU32::of(0, 0));
        assert_eq!(delta(&p1, &PointU32::of(0, 1)), PointU32::of(0, 1));
        assert_eq!(delta(&p1, &PointU32::of(0, 2)), PointU32::of(0, 2));

        assert_eq!(delta(&p1, &PointU32::of(1, 0)), PointU32::of(1, 0));
        assert_eq!(delta(&p1, &PointU32::of(1, 1)), PointU32::of(1, 1));
        assert_eq!(delta(&p1, &PointU32::of(1, 2)), PointU32::of(1, 2));

        assert_eq!(delta(&p1, &PointU32::of(2, 0)), PointU32::of(2, 0));
        assert_eq!(delta(&p1, &PointU32::of(2, 1)), PointU32::of(2, 1));
        assert_eq!(delta(&p1, &PointU32::of(2, 2)), PointU32::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p1 = PointU32::of(u32::MAX - 2, u32::MAX - 2);
        assert_eq!(delta(&p1, &PointU32::of(u32::MAX - 2, u32::MAX - 2)), PointU32::of(0, 0));
        assert_eq!(delta(&p1, &PointU32::of(u32::MAX - 2, u32::MAX - 1)), PointU32::of(0, 1));
        assert_eq!(delta(&p1, &PointU32::of(u32::MAX - 2, u32::MAX)), PointU32::of(0, 2));

        assert_eq!(delta(&p1, &PointU32::of(u32::MAX - 1, u32::MAX - 2)), PointU32::of(1, 0));
        assert_eq!(delta(&p1, &PointU32::of(u32::MAX - 1, u32::MAX - 1)), PointU32::of(1, 1));
        assert_eq!(delta(&p1, &PointU32::of(u32::MAX - 1, u32::MAX)), PointU32::of(1, 2));

        assert_eq!(delta(&p1, &PointU32::of(u32::MAX, u32::MAX - 2)), PointU32::of(2, 0));
        assert_eq!(delta(&p1, &PointU32::of(u32::MAX, u32::MAX - 1)), PointU32::of(2, 1));
        assert_eq!(delta(&p1, &PointU32::of(u32::MAX, u32::MAX)), PointU32::of(2, 2));
    }

    #[test]
    fn test_saturating_translate() {
        let mut r = PointU32::of(0, 0);
        saturating_translate(&mut r, &PointI32::of(10, 13));
        assert_eq!(r, PointU32::of(10, 13));
        saturating_translate(&mut r, &PointI32::of(-5, -3));
        assert_eq!(r, PointU32::of(5, 10));
        saturating_translate(&mut r, &PointI32::of(2, -4));
        assert_eq!(r, PointU32::of(7, 6));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut r = PointU32::of(2, 5);
        saturating_translate(&mut r, &PointI32::of(-10, -10));
        assert_eq!(r, PointU32::of(0, 0));
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut r = PointU32::of(u32::MAX - 2, u32::MAX - 5);
        saturating_translate(&mut r, &PointI32::of(10, 10));
        assert_eq!(r, PointU32::of(u32::MAX, u32::MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_min_delta() {
        let mut r = PointU32::of(1, 1);
        saturating_translate(&mut r, &PointI32::min());
        assert_eq!(r, PointU32::of(0, 0));
    }

    #[test]
    fn saturating_translate_max_bounds_max_delta() {
        let mut r = PointU32::of(u32::MAX - 1, u32::MAX - 1);
        saturating_translate(&mut r, &PointI32::max());
        assert_eq!(r, PointU32::of(u32::MAX, u32::MAX));
    }

    #[test]
    fn test_checked_translate() {
        let mut r = PointU32::of(0, 0);
        assert_eq!(checked_translate(&mut r, &PointI32::of(10, 13)), Ok(()));
        assert_eq!(r, PointU32::of(10, 13));
        assert_eq!(checked_translate(&mut r, &PointI32::of(-5, -3)), Ok(()));
        assert_eq!(r, PointU32::of(5, 10));
        assert_eq!(checked_translate(&mut r, &PointI32::of(2, -4)), Ok(()));
        assert_eq!(r, PointU32::of(7, 6));
    }

    #[test]
    fn checked_translate_min_bounds_err() {
        let mut r = PointU32::of(2, 5);
        assert_eq!(checked_translate(&mut r, &PointI32::of(-10, -10)), Err(()));
        assert_eq!(r, PointU32::of(2, 5));
    }

    #[test]
    fn checked_translate_max_bounds_err() {
        let mut r = PointU32::of(u32::MAX - 2, u32::MAX - 5);
        assert_eq!(checked_translate(&mut r, &PointI32::of(10, 10)), Err(()));
        assert_eq!(r, PointU32::of(u32::MAX - 2, u32::MAX - 5));
    }

    #[test]
    fn checked_translate_min_bounds_ok() {
        let mut r = PointU32::of(2, 5);
        assert_eq!(checked_translate(&mut r, &PointI32::of(-2, -5)), Ok(()));
        assert_eq!(r, PointU32::of(0, 0));
    }

    #[test]
    fn checked_translate_max_bounds_ok() {
        let mut r = PointU32::of(u32::MAX - 2, u32::MAX - 5);
        assert_eq!(checked_translate(&mut r, &PointI32::of(2, 5)), Ok(()));
        assert_eq!(r, PointU32::of(u32::MAX, u32::MAX));
    }

    #[test]
    fn checked_translate_min_bounds_min_delta() {
        let mut r = PointU32::of(1, 1);
        assert_eq!(checked_translate(&mut r, &PointI32::min()), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI32::of(i32::MIN, 0)), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI32::of(0, i32::MIN)), Err(()));
        assert_eq!(r, PointU32::of(1, 1));
    }

    #[test]
    fn checked_translate_max_bounds_max_delta() {
        let mut r = PointU32::of(u32::MAX - 1, u32::MAX - 1);
        assert_eq!(checked_translate(&mut r, &PointI32::max()), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI32::of(i32::MAX, 0)), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI32::of(0, i32::MAX)), Err(()));
        assert_eq!(r, PointU32::of(u32::MAX - 1, u32::MAX - 1));
    }
}

use crate::cartesian::point::point_u16::PointU16;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointI16 {
    pub x: i16,
    pub y: i16,
}

impl PointI16 {
    pub fn of(x: i16, y: i16) -> Self {
        PointI16 { x, y }
    }

    pub fn min() -> Self {
        PointI16 { x: i16::MIN, y: i16::MIN }
    }

    pub fn max() -> Self {
        PointI16 { x: i16::MAX, y: i16::MAX }
    }
}

impl std::fmt::Display for PointI16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointI16, p2: &PointI16) -> u16 {
    (i32::from(p2.x) - i32::from(p1.x)).unsigned_abs() as u16
}

pub fn delta_y(p1: &PointI16, p2: &PointI16) -> u16 {
    (i32::from(p2.y) - i32::from(p1.y)).unsigned_abs() as u16
}

pub fn delta(p1: &PointI16, p2: &PointI16) -> PointU16 {
    PointU16 { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

pub fn saturating_translate(p: &mut PointI16, delta: &PointI16) {
    p.x = p.x.saturating_add(delta.x);
    p.y = p.y.saturating_add(delta.y);
}

pub fn checked_translate(p: &mut PointI16, delta: &PointI16) -> Result<(), ()> {
    let x = p.x.checked_add(delta.x).ok_or(())?;
    let y = p.y.checked_add(delta.y).ok_or(())?;
    p.x = x;
    p.y = y;
    Ok(())
}

pub fn saturating_translated(p: &PointI16, delta: &PointI16) -> PointI16 {
    PointI16::of(p.x.saturating_add(delta.x), p.y.saturating_add(delta.y))
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_u16::PointU16;

    use super::{PointI16, checked_translate, delta, delta_x, delta_y, saturating_translate, saturating_translated};

    #[test]
    fn point_i16() {
        assert_eq!(PointI16::of(i16::MIN, i16::MAX), PointI16 { x: i16::MIN, y: i16::MAX });
        assert_eq!(PointI16::min(), PointI16 { x: i16::MIN, y: i16::MIN });
        assert_eq!(PointI16::max(), PointI16 { x: i16::MAX, y: i16::MAX });
        assert_eq!(PointI16::of(i16::MIN, i16::MAX).to_string(), "(-32768, 32767)");
        assert_eq!(PointI16::min().to_string(), "(-32768, -32768)");
        assert_eq!(PointI16::max().to_string(), "(32767, 32767)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointI16::of(0, i16::MIN), &PointI16::of(0, i16::MAX)), 0);
        assert_eq!(delta_x(&PointI16::of(i16::MIN, 0), &PointI16::of(i16::MAX, 0)), u16::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointI16::of(i16::MIN, 0), &PointI16::of(i16::MAX, 0)), 0);
        assert_eq!(delta_y(&PointI16::of(0, i16::MIN), &PointI16::of(0, i16::MAX)), u16::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointI16::of(0, 0), &PointI16::of(0, 0)), PointU16::of(0, 0));
        assert_eq!(delta(&PointI16::min(), &PointI16::max()), PointU16::max());
    }

    #[test]
    fn delta_min() {
        let p1 = PointI16::of(i16::MIN, i16::MIN);
        assert_eq!(delta(&p1, &PointI16::of(i16::MIN, i16::MIN)), PointU16::of(0, 0));
        assert_eq!(delta(&p1, &PointI16::of(i16::MIN, i16::MIN + 1)), PointU16::of(0, 1));
        assert_eq!(delta(&p1, &PointI16::of(i16::MIN, i16::MIN + 2)), PointU16::of(0, 2));

        assert_eq!(delta(&p1, &PointI16::of(i16::MIN + 1, i16::MIN)), PointU16::of(1, 0));
        assert_eq!(delta(&p1, &PointI16::of(i16::MIN + 1, i16::MIN + 1)), PointU16::of(1, 1));
        assert_eq!(delta(&p1, &PointI16::of(i16::MIN + 1, i16::MIN + 2)), PointU16::of(1, 2));

        assert_eq!(delta(&p1, &PointI16::of(i16::MIN + 2, i16::MIN)), PointU16::of(2, 0));
        assert_eq!(delta(&p1, &PointI16::of(i16::MIN + 2, i16::MIN + 1)), PointU16::of(2, 1));
        assert_eq!(delta(&p1, &PointI16::of(i16::MIN + 2, i16::MIN + 2)), PointU16::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p1 = PointI16::of(i16::MAX - 2, i16::MAX - 2);
        assert_eq!(delta(&p1, &PointI16::of(i16::MAX - 2, i16::MAX - 2)), PointU16::of(0, 0));
        assert_eq!(delta(&p1, &PointI16::of(i16::MAX - 2, i16::MAX - 1)), PointU16::of(0, 1));
        assert_eq!(delta(&p1, &PointI16::of(i16::MAX - 2, i16::MAX)), PointU16::of(0, 2));

        assert_eq!(delta(&p1, &PointI16::of(i16::MAX - 1, i16::MAX - 2)), PointU16::of(1, 0));
        assert_eq!(delta(&p1, &PointI16::of(i16::MAX - 1, i16::MAX - 1)), PointU16::of(1, 1));
        assert_eq!(delta(&p1, &PointI16::of(i16::MAX - 1, i16::MAX)), PointU16::of(1, 2));

        assert_eq!(delta(&p1, &PointI16::of(i16::MAX, i16::MAX - 2)), PointU16::of(2, 0));
        assert_eq!(delta(&p1, &PointI16::of(i16::MAX, i16::MAX - 1)), PointU16::of(2, 1));
        assert_eq!(delta(&p1, &PointI16::of(i16::MAX, i16::MAX)), PointU16::of(2, 2));
    }

    #[test]
    fn test_saturating_translate() {
        let mut r = PointI16::of(0, 0);
        saturating_translate(&mut r, &PointI16::of(10, 15));
        assert_eq!(r, PointI16::of(10, 15));
        saturating_translate(&mut r, &PointI16::of(-15, -25));
        assert_eq!(r, PointI16::of(-5, -10));
        saturating_translate(&mut r, &PointI16::of(2, 3));
        assert_eq!(r, PointI16::of(-3, -7));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut r = PointI16::of(i16::MIN + 2, i16::MIN + 5);
        saturating_translate(&mut r, &PointI16::of(-10, -10));
        assert_eq!(r, PointI16::of(i16::MIN, i16::MIN));
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut r = PointI16::of(i16::MAX - 2, i16::MAX - 5);
        saturating_translate(&mut r, &PointI16::of(10, 10));
        assert_eq!(r, PointI16::of(i16::MAX, i16::MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_min_delta() {
        let mut r = PointI16::of(i16::MIN + 1, i16::MIN + 1);
        saturating_translate(&mut r, &PointI16::min());
        assert_eq!(r, PointI16::of(i16::MIN, i16::MIN));
    }

    #[test]
    fn saturating_translate_max_bounds_max_delta() {
        let mut r = PointI16::of(i16::MAX - 1, i16::MAX - 1);
        saturating_translate(&mut r, &PointI16::max());
        assert_eq!(r, PointI16::of(i16::MAX, i16::MAX));
    }

    #[test]
    fn test_checked_translate() {
        let mut r = PointI16::of(0, 0);
        assert_eq!(checked_translate(&mut r, &PointI16::of(10, 15)), Ok(()));
        assert_eq!(r, PointI16::of(10, 15));
        assert_eq!(checked_translate(&mut r, &PointI16::of(-15, -25)), Ok(()));
        assert_eq!(r, PointI16::of(-5, -10));
        assert_eq!(checked_translate(&mut r, &PointI16::of(2, 3)), Ok(()));
        assert_eq!(r, PointI16::of(-3, -7));
    }

    #[test]
    fn checked_translate_min_bounds_err() {
        let mut r = PointI16::of(i16::MIN + 2, i16::MIN + 5);
        assert_eq!(checked_translate(&mut r, &PointI16::of(-10, -10)), Err(()));
        assert_eq!(r, PointI16::of(i16::MIN + 2, i16::MIN + 5));
    }

    #[test]
    fn checked_translate_max_bounds_err() {
        let mut r = PointI16::of(i16::MAX - 2, i16::MAX - 5);
        assert_eq!(checked_translate(&mut r, &PointI16::of(10, 10)), Err(()));
        assert_eq!(r, PointI16::of(i16::MAX - 2, i16::MAX - 5));
    }

    #[test]
    fn checked_translate_min_bounds_ok() {
        let mut r = PointI16::of(i16::MIN + 2, i16::MIN + 5);
        assert_eq!(checked_translate(&mut r, &PointI16::of(-2, -5)), Ok(()));
        assert_eq!(r, PointI16::of(i16::MIN, i16::MIN));
    }

    #[test]
    fn checked_translate_max_bounds_ok() {
        let mut r = PointI16::of(i16::MAX - 2, i16::MAX - 5);
        assert_eq!(checked_translate(&mut r, &PointI16::of(2, 5)), Ok(()));
        assert_eq!(r, PointI16::of(i16::MAX, i16::MAX));
    }

    #[test]
    fn checked_translate_min_bounds_min_delta() {
        let mut r = PointI16::of(i16::MIN + 1, i16::MIN + 1);
        assert_eq!(checked_translate(&mut r, &PointI16::min()), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI16::of(i16::MIN, 0)), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI16::of(0, i16::MIN)), Err(()));
        assert_eq!(r, PointI16::of(i16::MIN + 1, i16::MIN + 1));
    }

    #[test]
    fn checked_translate_max_bounds_max_delta() {
        let mut r = PointI16::of(i16::MAX - 1, i16::MAX - 1);
        assert_eq!(checked_translate(&mut r, &PointI16::max()), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI16::of(i16::MAX, 0)), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI16::of(0, i16::MAX)), Err(()));
        assert_eq!(r, PointI16::of(i16::MAX - 1, i16::MAX - 1));
    }

    #[test]
    fn test_saturating_translated() {
        assert_eq!(saturating_translated(&PointI16::of(0, 0), &PointI16::of(10, 15)), PointI16::of(10, 15));
        assert_eq!(saturating_translated(&PointI16::of(0, 0), &PointI16::of(-15, -25)), PointI16::of(-15, -25));
    }

    #[test]
    fn saturating_translated_to_bounds() {
        assert_eq!(saturating_translated(&PointI16::of(i16::MIN + 2, i16::MIN + 5), &PointI16::of(-2, -5)), PointI16::min());
        assert_eq!(saturating_translated(&PointI16::of(i16::MAX - 2, i16::MAX - 5), &PointI16::of(2, 5)), PointI16::max());
    }

    #[test]
    fn saturating_translated_beyond_bounds() {
        assert_eq!(saturating_translated(&PointI16::of(i16::MIN + 2, i16::MIN + 5), &PointI16::of(-10, -10)), PointI16::min());
        assert_eq!(saturating_translated(&PointI16::of(i16::MAX - 2, i16::MAX - 5), &PointI16::of(10, 10)), PointI16::max());
    }

    #[test]
    fn saturating_translated_limits() {
        assert_eq!(saturating_translated(&PointI16::of(i16::MIN + 1, i16::MIN + 1), &PointI16::min()), PointI16::min());
        assert_eq!(saturating_translated(&PointI16::of(i16::MAX - 1, i16::MAX - 1), &PointI16::max()), PointI16::max());
    }
}

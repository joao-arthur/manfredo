use crate::cartesian::point::point_u8::PointU8;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointI8 {
    pub x: i8,
    pub y: i8,
}

impl PointI8 {
    pub fn of(x: i8, y: i8) -> Self {
        PointI8 { x, y }
    }

    pub fn min() -> Self {
        PointI8 { x: i8::MIN, y: i8::MIN }
    }

    pub fn max() -> Self {
        PointI8 { x: i8::MAX, y: i8::MAX }
    }
}

impl std::fmt::Display for PointI8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointI8, p2: &PointI8) -> u8 {
    (i16::from(p2.x) - i16::from(p1.x)).unsigned_abs() as u8
}

pub fn delta_y(p1: &PointI8, p2: &PointI8) -> u8 {
    (i16::from(p2.y) - i16::from(p1.y)).unsigned_abs() as u8
}

pub fn delta(p1: &PointI8, p2: &PointI8) -> PointU8 {
    PointU8 { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

pub fn saturating_translate(p: &mut PointI8, delta: &PointI8) {
    p.x = p.x.saturating_add(delta.x);
    p.y = p.y.saturating_add(delta.y);
}

pub fn try_checked_translate(p: &mut PointI8, delta: &PointI8) -> Result<(), ()> {
    let x = p.x.checked_add(delta.x).ok_or(())?;
    let y = p.y.checked_add(delta.y).ok_or(())?;
    p.x = x;
    p.y = y;
    Ok(())
}

pub fn saturating_translated(p: &PointI8, delta: &PointI8) -> PointI8 {
    PointI8::of(p.x.saturating_add(delta.x), p.y.saturating_add(delta.y))
}

pub fn try_checked_translated(p: &PointI8, delta: &PointI8) -> Option<PointI8> {
    let x = p.x.checked_add(delta.x)?;
    let y = p.y.checked_add(delta.y)?;
    Some(PointI8 { x, y })
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_u8::PointU8;

    use super::{PointI8, try_checked_translate, try_checked_translated, delta, delta_x, delta_y, saturating_translate, saturating_translated};

    #[test]
    fn point_i8() {
        assert_eq!(PointI8::of(i8::MIN, i8::MAX), PointI8 { x: i8::MIN, y: i8::MAX });
        assert_eq!(PointI8::min(), PointI8 { x: i8::MIN, y: i8::MIN });
        assert_eq!(PointI8::max(), PointI8 { x: i8::MAX, y: i8::MAX });
        assert_eq!(PointI8::of(i8::MIN, i8::MAX).to_string(), "(-128, 127)");
        assert_eq!(PointI8::min().to_string(), "(-128, -128)");
        assert_eq!(PointI8::max().to_string(), "(127, 127)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointI8::of(0, i8::MIN), &PointI8::of(0, i8::MAX)), 0);
        assert_eq!(delta_x(&PointI8::of(i8::MIN, 0), &PointI8::of(i8::MAX, 0)), u8::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointI8::of(i8::MIN, 0), &PointI8::of(i8::MAX, 0)), 0);
        assert_eq!(delta_y(&PointI8::of(0, i8::MIN), &PointI8::of(0, i8::MAX)), u8::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointI8::of(0, 0), &PointI8::of(0, 0)), PointU8::of(0, 0));
        assert_eq!(delta(&PointI8::min(), &PointI8::max()), PointU8::max());
    }

    #[test]
    fn delta_min() {
        let p = PointI8::min();
        assert_eq!(delta(&p, &PointI8::min()), PointU8::of(0, 0));
        assert_eq!(delta(&p, &PointI8::of(i8::MIN, i8::MIN + 1)), PointU8::of(0, 1));
        assert_eq!(delta(&p, &PointI8::of(i8::MIN, i8::MIN + 2)), PointU8::of(0, 2));

        assert_eq!(delta(&p, &PointI8::of(i8::MIN + 1, i8::MIN)), PointU8::of(1, 0));
        assert_eq!(delta(&p, &PointI8::of(i8::MIN + 1, i8::MIN + 1)), PointU8::of(1, 1));
        assert_eq!(delta(&p, &PointI8::of(i8::MIN + 1, i8::MIN + 2)), PointU8::of(1, 2));

        assert_eq!(delta(&p, &PointI8::of(i8::MIN + 2, i8::MIN)), PointU8::of(2, 0));
        assert_eq!(delta(&p, &PointI8::of(i8::MIN + 2, i8::MIN + 1)), PointU8::of(2, 1));
        assert_eq!(delta(&p, &PointI8::of(i8::MIN + 2, i8::MIN + 2)), PointU8::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p = PointI8::of(i8::MAX - 2, i8::MAX - 2);
        assert_eq!(delta(&p, &PointI8::of(i8::MAX - 2, i8::MAX - 2)), PointU8::of(0, 0));
        assert_eq!(delta(&p, &PointI8::of(i8::MAX - 2, i8::MAX - 1)), PointU8::of(0, 1));
        assert_eq!(delta(&p, &PointI8::of(i8::MAX - 2, i8::MAX)), PointU8::of(0, 2));

        assert_eq!(delta(&p, &PointI8::of(i8::MAX - 1, i8::MAX - 2)), PointU8::of(1, 0));
        assert_eq!(delta(&p, &PointI8::of(i8::MAX - 1, i8::MAX - 1)), PointU8::of(1, 1));
        assert_eq!(delta(&p, &PointI8::of(i8::MAX - 1, i8::MAX)), PointU8::of(1, 2));

        assert_eq!(delta(&p, &PointI8::of(i8::MAX, i8::MAX - 2)), PointU8::of(2, 0));
        assert_eq!(delta(&p, &PointI8::of(i8::MAX, i8::MAX - 1)), PointU8::of(2, 1));
        assert_eq!(delta(&p, &PointI8::max()), PointU8::of(2, 2));
    }

    #[test]
    fn test_saturating_translate() {
        let mut p = PointI8::of(0, 0);
        saturating_translate(&mut p, &PointI8::of(10, 15));
        assert_eq!(p, PointI8::of(10, 15));
        saturating_translate(&mut p, &PointI8::of(-15, -25));
        assert_eq!(p, PointI8::of(-5, -10));
        saturating_translate(&mut p, &PointI8::of(2, 3));
        assert_eq!(p, PointI8::of(-3, -7));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut p = PointI8::of(i8::MIN + 2, i8::MIN + 5);
        saturating_translate(&mut p, &PointI8::of(-10, -10));
        assert_eq!(p, PointI8::min());
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut p = PointI8::of(i8::MAX - 2, i8::MAX - 5);
        saturating_translate(&mut p, &PointI8::of(10, 10));
        assert_eq!(p, PointI8::max());
    }

    #[test]
    fn saturating_translate_min_bounds_min_delta() {
        let mut p = PointI8::of(i8::MIN + 1, i8::MIN + 1);
        saturating_translate(&mut p, &PointI8::min());
        assert_eq!(p, PointI8::min());
    }

    #[test]
    fn saturating_translate_max_bounds_max_delta() {
        let mut p = PointI8::of(i8::MAX - 1, i8::MAX - 1);
        saturating_translate(&mut p, &PointI8::max());
        assert_eq!(p, PointI8::max());
    }

    #[test]
    fn test_checked_translate() {
        let mut p = PointI8::of(0, 0);
        assert_eq!(try_checked_translate(&mut p, &PointI8::of(10, 15)), Ok(()));
        assert_eq!(p, PointI8::of(10, 15));
        assert_eq!(try_checked_translate(&mut p, &PointI8::of(-15, -25)), Ok(()));
        assert_eq!(p, PointI8::of(-5, -10));
        assert_eq!(try_checked_translate(&mut p, &PointI8::of(2, 3)), Ok(()));
        assert_eq!(p, PointI8::of(-3, -7));
    }

    #[test]
    fn checked_translate_min_bounds_err() {
        let mut p = PointI8::of(i8::MIN + 2, i8::MIN + 5);
        assert_eq!(try_checked_translate(&mut p, &PointI8::of(-10, -10)), Err(()));
        assert_eq!(p, PointI8::of(i8::MIN + 2, i8::MIN + 5));
    }

    #[test]
    fn checked_translate_max_bounds_err() {
        let mut p = PointI8::of(i8::MAX - 2, i8::MAX - 5);
        assert_eq!(try_checked_translate(&mut p, &PointI8::of(10, 10)), Err(()));
        assert_eq!(p, PointI8::of(i8::MAX - 2, i8::MAX - 5));
    }

    #[test]
    fn checked_translate_min_bounds_ok() {
        let mut p = PointI8::of(i8::MIN + 2, i8::MIN + 5);
        assert_eq!(try_checked_translate(&mut p, &PointI8::of(-2, -5)), Ok(()));
        assert_eq!(p, PointI8::min());
    }

    #[test]
    fn checked_translate_max_bounds_ok() {
        let mut p = PointI8::of(i8::MAX - 2, i8::MAX - 5);
        assert_eq!(try_checked_translate(&mut p, &PointI8::of(2, 5)), Ok(()));
        assert_eq!(p, PointI8::max());
    }

    #[test]
    fn checked_translate_min_bounds_min_delta() {
        let mut p = PointI8::of(i8::MIN + 1, i8::MIN + 1);
        assert_eq!(try_checked_translate(&mut p, &PointI8::min()), Err(()));
        assert_eq!(try_checked_translate(&mut p, &PointI8::of(i8::MIN, 0)), Err(()));
        assert_eq!(try_checked_translate(&mut p, &PointI8::of(0, i8::MIN)), Err(()));
        assert_eq!(p, PointI8::of(i8::MIN + 1, i8::MIN + 1));
    }

    #[test]
    fn checked_translate_max_bounds_max_delta() {
        let mut p = PointI8::of(i8::MAX - 1, i8::MAX - 1);
        assert_eq!(try_checked_translate(&mut p, &PointI8::max()), Err(()));
        assert_eq!(try_checked_translate(&mut p, &PointI8::of(i8::MAX, 0)), Err(()));
        assert_eq!(try_checked_translate(&mut p, &PointI8::of(0, i8::MAX)), Err(()));
        assert_eq!(p, PointI8::of(i8::MAX - 1, i8::MAX - 1));
    }

    #[test]
    fn test_saturating_translated() {
        assert_eq!(saturating_translated(&PointI8::of(0, 0), &PointI8::of(10, 15)), PointI8::of(10, 15));
        assert_eq!(saturating_translated(&PointI8::of(0, 0), &PointI8::of(-15, -25)), PointI8::of(-15, -25));
    }

    #[test]
    fn saturating_translated_to_bounds() {
        assert_eq!(saturating_translated(&PointI8::of(i8::MIN + 2, i8::MIN + 5), &PointI8::of(-2, -5)), PointI8::min());
        assert_eq!(saturating_translated(&PointI8::of(i8::MAX - 2, i8::MAX - 5), &PointI8::of(2, 5)), PointI8::max());
    }

    #[test]
    fn saturating_translated_beyond_bounds() {
        assert_eq!(saturating_translated(&PointI8::of(i8::MIN + 2, i8::MIN + 5), &PointI8::of(-10, -10)), PointI8::min());
        assert_eq!(saturating_translated(&PointI8::of(i8::MAX - 2, i8::MAX - 5), &PointI8::of(10, 10)), PointI8::max());
    }

    #[test]
    fn saturating_translated_limits() {
        assert_eq!(saturating_translated(&PointI8::of(i8::MIN + 1, i8::MIN + 1), &PointI8::min()), PointI8::min());
        assert_eq!(saturating_translated(&PointI8::of(i8::MAX - 1, i8::MAX - 1), &PointI8::max()), PointI8::max());
    }

    #[test]
    fn checked_translated_min_bounds() {
        let p = PointI8::of(i8::MIN + 2, i8::MIN + 5);
        assert_eq!(try_checked_translated(&p, &PointI8::of(-2, 0)), Some(PointI8::of(i8::MIN, i8::MIN + 5)));
        assert_eq!(try_checked_translated(&p, &PointI8::of(0, -5)), Some(PointI8::of(i8::MIN + 2, i8::MIN)));
        assert_eq!(try_checked_translated(&p, &PointI8::of(-2, -5)), Some(PointI8::min()));
        assert_eq!(try_checked_translated(&p, &PointI8::of(-10, -10)), None);
        assert_eq!(try_checked_translated(&p, &PointI8::of(i8::MIN, 0)), None);
        assert_eq!(try_checked_translated(&p, &PointI8::of(0, i8::MIN)), None);
        assert_eq!(try_checked_translated(&p, &PointI8::min()), None);
    }

    #[test]
    fn checked_translated_max_bounds() {
        let p = PointI8::of(i8::MAX - 2, i8::MAX - 5);
        assert_eq!(try_checked_translated(&p, &PointI8::of(2, 0)), Some(PointI8::of(i8::MAX, i8::MAX - 5)));
        assert_eq!(try_checked_translated(&p, &PointI8::of(0, 5)), Some(PointI8::of(i8::MAX - 2, i8::MAX)));
        assert_eq!(try_checked_translated(&p, &PointI8::of(2, 5)), Some(PointI8::max()));
        assert_eq!(try_checked_translated(&p, &PointI8::of(10, 10)), None);
        assert_eq!(try_checked_translated(&p, &PointI8::of(i8::MAX, 0)), None);
        assert_eq!(try_checked_translated(&p, &PointI8::of(0, i8::MAX)), None);
        assert_eq!(try_checked_translated(&p, &PointI8::max()), None);
    }
}

use crate::cartesian::point::point_i8::PointI8;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointU8 {
    pub x: u8,
    pub y: u8,
}

impl PointU8 {
    pub fn of(x: u8, y: u8) -> Self {
        PointU8 { x, y }
    }

    pub fn min() -> Self {
        PointU8 { x: 0, y: 0 }
    }

    pub fn max() -> Self {
        PointU8 { x: u8::MAX, y: u8::MAX }
    }
}

impl std::fmt::Display for PointU8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointU8, p2: &PointU8) -> u8 {
    p2.x - p1.x
}

pub fn delta_y(p1: &PointU8, p2: &PointU8) -> u8 {
    p2.y - p1.y
}

pub fn delta(p1: &PointU8, p2: &PointU8) -> PointU8 {
    PointU8 { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

pub fn saturating_translate(p: &mut PointU8, delta: &PointI8) {
    let temp_x = i16::from(p.x) + i16::from(delta.x);
    let temp_y = i16::from(p.y) + i16::from(delta.y);
    let clamped_x = temp_x.clamp(0, i16::from(u8::MAX));
    let clamped_y = temp_y.clamp(0, i16::from(u8::MAX));
    p.x = clamped_x as u8;
    p.y = clamped_y as u8;
}

pub fn checked_translate(p: &mut PointU8, delta: &PointI8) -> Result<(), ()> {
    let x = u8::try_from(i16::from(p.x) + i16::from(delta.x)).map_err(|_| ())?;
    let y = u8::try_from(i16::from(p.y) + i16::from(delta.y)).map_err(|_| ())?;
    p.x = x;
    p.y = y;
    Ok(())
}

pub fn saturating_translated(p: &PointU8, delta: &PointI8) -> PointU8 {
    let temp_x = i16::from(p.x) + i16::from(delta.x);
    let temp_y = i16::from(p.y) + i16::from(delta.y);
    let clamped_x = temp_x.clamp(0, i16::from(u8::MAX));
    let clamped_y = temp_y.clamp(0, i16::from(u8::MAX));
    PointU8::of(clamped_x as u8, clamped_y as u8)
}

pub fn checked_translated(p: &PointU8, delta: &PointI8) -> Option<PointU8> {
    let x = u8::try_from(i16::from(p.x) + i16::from(delta.x)).ok()?;
    let y = u8::try_from(i16::from(p.y) + i16::from(delta.y)).ok()?;
    Some(PointU8 { x, y })
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_i8::PointI8;

    use super::{PointU8, checked_translate, checked_translated, delta, delta_x, delta_y, saturating_translate, saturating_translated};

    #[test]
    fn point_u8() {
        assert_eq!(PointU8::of(0, u8::MAX), PointU8 { x: 0, y: u8::MAX });
        assert_eq!(PointU8::min(), PointU8 { x: 0, y: 0 });
        assert_eq!(PointU8::max(), PointU8 { x: u8::MAX, y: u8::MAX });
        assert_eq!(PointU8::of(0, u8::MAX).to_string(), "(0, 255)");
        assert_eq!(PointU8::min().to_string(), "(0, 0)");
        assert_eq!(PointU8::max().to_string(), "(255, 255)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointU8::min(), &PointU8::of(0, u8::MAX)), 0);
        assert_eq!(delta_x(&PointU8::min(), &PointU8::of(u8::MAX, 0)), u8::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointU8::min(), &PointU8::of(u8::MAX, 0)), 0);
        assert_eq!(delta_y(&PointU8::min(), &PointU8::of(0, u8::MAX)), u8::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointU8::min(), &PointU8::min()), PointU8::min());
        assert_eq!(delta(&PointU8::min(), &PointU8::max()), PointU8::max());
    }

    #[test]
    fn delta_min() {
        let p = PointU8::min();
        assert_eq!(delta(&p, &PointU8::of(0, 0)), PointU8::of(0, 0));
        assert_eq!(delta(&p, &PointU8::of(0, 1)), PointU8::of(0, 1));
        assert_eq!(delta(&p, &PointU8::of(0, 2)), PointU8::of(0, 2));

        assert_eq!(delta(&p, &PointU8::of(1, 0)), PointU8::of(1, 0));
        assert_eq!(delta(&p, &PointU8::of(1, 1)), PointU8::of(1, 1));
        assert_eq!(delta(&p, &PointU8::of(1, 2)), PointU8::of(1, 2));

        assert_eq!(delta(&p, &PointU8::of(2, 0)), PointU8::of(2, 0));
        assert_eq!(delta(&p, &PointU8::of(2, 1)), PointU8::of(2, 1));
        assert_eq!(delta(&p, &PointU8::of(2, 2)), PointU8::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p = PointU8::of(u8::MAX - 2, u8::MAX - 2);
        assert_eq!(delta(&p, &PointU8::of(u8::MAX - 2, u8::MAX - 2)), PointU8::of(0, 0));
        assert_eq!(delta(&p, &PointU8::of(u8::MAX - 2, u8::MAX - 1)), PointU8::of(0, 1));
        assert_eq!(delta(&p, &PointU8::of(u8::MAX - 2, u8::MAX)), PointU8::of(0, 2));

        assert_eq!(delta(&p, &PointU8::of(u8::MAX - 1, u8::MAX - 2)), PointU8::of(1, 0));
        assert_eq!(delta(&p, &PointU8::of(u8::MAX - 1, u8::MAX - 1)), PointU8::of(1, 1));
        assert_eq!(delta(&p, &PointU8::of(u8::MAX - 1, u8::MAX)), PointU8::of(1, 2));

        assert_eq!(delta(&p, &PointU8::of(u8::MAX, u8::MAX - 2)), PointU8::of(2, 0));
        assert_eq!(delta(&p, &PointU8::of(u8::MAX, u8::MAX - 1)), PointU8::of(2, 1));
        assert_eq!(delta(&p, &PointU8::max()), PointU8::of(2, 2));
    }

    #[test]
    fn test_saturating_translate() {
        let mut p = PointU8::of(0, 0);
        saturating_translate(&mut p, &PointI8::of(10, 13));
        assert_eq!(p, PointU8::of(10, 13));
        saturating_translate(&mut p, &PointI8::of(-5, -3));
        assert_eq!(p, PointU8::of(5, 10));
        saturating_translate(&mut p, &PointI8::of(2, -4));
        assert_eq!(p, PointU8::of(7, 6));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut p = PointU8::of(2, 5);
        saturating_translate(&mut p, &PointI8::of(-10, -10));
        assert_eq!(p, PointU8::of(0, 0));
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut p = PointU8::of(u8::MAX - 2, u8::MAX - 5);
        saturating_translate(&mut p, &PointI8::of(10, 10));
        assert_eq!(p, PointU8::max());
    }

    #[test]
    fn saturating_translate_min_bounds_min_delta() {
        let mut p = PointU8::of(1, 1);
        saturating_translate(&mut p, &PointI8::min());
        assert_eq!(p, PointU8::of(0, 0));
    }

    #[test]
    fn saturating_translate_max_bounds_max_delta() {
        let mut p = PointU8::of(u8::MAX - 1, u8::MAX - 1);
        saturating_translate(&mut p, &PointI8::max());
        assert_eq!(p, PointU8::max());
    }

    #[test]
    fn test_checked_translate() {
        let mut p = PointU8::of(0, 0);
        assert_eq!(checked_translate(&mut p, &PointI8::of(10, 13)), Ok(()));
        assert_eq!(p, PointU8::of(10, 13));
        assert_eq!(checked_translate(&mut p, &PointI8::of(-5, -3)), Ok(()));
        assert_eq!(p, PointU8::of(5, 10));
        assert_eq!(checked_translate(&mut p, &PointI8::of(2, -4)), Ok(()));
        assert_eq!(p, PointU8::of(7, 6));
    }

    #[test]
    fn checked_translate_min_bounds_err() {
        let mut p = PointU8::of(2, 5);
        assert_eq!(checked_translate(&mut p, &PointI8::of(-10, -10)), Err(()));
        assert_eq!(p, PointU8::of(2, 5));
    }

    #[test]
    fn checked_translate_max_bounds_err() {
        let mut p = PointU8::of(u8::MAX - 2, u8::MAX - 5);
        assert_eq!(checked_translate(&mut p, &PointI8::of(10, 10)), Err(()));
        assert_eq!(p, PointU8::of(u8::MAX - 2, u8::MAX - 5));
    }

    #[test]
    fn checked_translate_min_bounds_ok() {
        let mut p = PointU8::of(2, 5);
        assert_eq!(checked_translate(&mut p, &PointI8::of(-2, -5)), Ok(()));
        assert_eq!(p, PointU8::of(0, 0));
    }

    #[test]
    fn checked_translate_max_bounds_ok() {
        let mut p = PointU8::of(u8::MAX - 2, u8::MAX - 5);
        assert_eq!(checked_translate(&mut p, &PointI8::of(2, 5)), Ok(()));
        assert_eq!(p, PointU8::max());
    }

    #[test]
    fn checked_translate_min_bounds_min_delta() {
        let mut p = PointU8::of(1, 1);
        assert_eq!(checked_translate(&mut p, &PointI8::min()), Err(()));
        assert_eq!(checked_translate(&mut p, &PointI8::of(i8::MIN, 0)), Err(()));
        assert_eq!(checked_translate(&mut p, &PointI8::of(0, i8::MIN)), Err(()));
        assert_eq!(p, PointU8::of(1, 1));
    }

    #[test]
    fn checked_translate_max_bounds_max_delta() {
        let mut p = PointU8::of(u8::MAX - 1, u8::MAX - 1);
        assert_eq!(checked_translate(&mut p, &PointI8::max()), Err(()));
        assert_eq!(checked_translate(&mut p, &PointI8::of(i8::MAX, 0)), Err(()));
        assert_eq!(checked_translate(&mut p, &PointI8::of(0, i8::MAX)), Err(()));
        assert_eq!(p, PointU8::of(u8::MAX - 1, u8::MAX - 1));
    }

    #[test]
    fn test_saturating_translated() {
        assert_eq!(saturating_translated(&PointU8::of(0, 0), &PointI8::of(10, 13)), PointU8::of(10, 13));
        assert_eq!(saturating_translated(&PointU8::of(10, 10), &PointI8::of(-5, -3)), PointU8::of(5, 7));
    }

    #[test]
    fn saturating_translated_to_bounds() {
        assert_eq!(saturating_translated(&PointU8::of(2, 5), &PointI8::of(-2, -5)), PointU8::min());
        assert_eq!(saturating_translated(&PointU8::of(u8::MAX - 2, u8::MAX - 5), &PointI8::of(2, 5)), PointU8::max());
    }

    #[test]
    fn saturating_translated_beyond_bounds() {
        assert_eq!(saturating_translated(&PointU8::of(2, 5), &PointI8::of(-10, -10)), PointU8::min());
        assert_eq!(saturating_translated(&PointU8::of(u8::MAX - 2, u8::MAX - 5), &PointI8::of(10, 10)), PointU8::max());
    }

    #[test]
    fn saturating_translated_limits() {
        assert_eq!(saturating_translated(&PointU8::of(1, 1), &PointI8::min()), PointU8::min());
        assert_eq!(saturating_translated(&PointU8::of(u8::MAX - 1, u8::MAX - 1), &PointI8::max()), PointU8::max());
    }

    #[test]
    fn checked_translated_min_bounds() {
        let p = PointU8::of(2, 5);
        assert_eq!(checked_translated(&p, &PointI8::of(-2, 0)), Some(PointU8::of(0, 5)));
        assert_eq!(checked_translated(&p, &PointI8::of(0, -5)), Some(PointU8::of(2, 0)));
        assert_eq!(checked_translated(&p, &PointI8::of(-2, -5)), Some(PointU8::min()));
        assert_eq!(checked_translated(&p, &PointI8::of(-10, -10)), None);
        assert_eq!(checked_translated(&p, &PointI8::of(i8::MIN, 0)), None);
        assert_eq!(checked_translated(&p, &PointI8::of(0, i8::MIN)), None);
        assert_eq!(checked_translated(&p, &PointI8::min()), None);
    }

    #[test]
    fn checked_translated_max_bounds() {
        let p = PointU8::of(u8::MAX - 2, u8::MAX - 5);
        assert_eq!(checked_translated(&p, &PointI8::of(2, 0)), Some(PointU8::of(u8::MAX, u8::MAX - 5)));
        assert_eq!(checked_translated(&p, &PointI8::of(0, 5)), Some(PointU8::of(u8::MAX - 2, u8::MAX)));
        assert_eq!(checked_translated(&p, &PointI8::of(2, 5)), Some(PointU8::max()));
        assert_eq!(checked_translated(&p, &PointI8::of(10, 10)), None);
        assert_eq!(checked_translated(&p, &PointI8::of(i8::MAX, 0)), None);
        assert_eq!(checked_translated(&p, &PointI8::of(0, i8::MAX)), None);
        assert_eq!(checked_translated(&p, &PointI8::max()), None);
    }
}

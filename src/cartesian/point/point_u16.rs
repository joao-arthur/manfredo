use crate::cartesian::point::point_i16::PointI16;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointU16 {
    pub x: u16,
    pub y: u16,
}

impl PointU16 {
    pub fn of(x: u16, y: u16) -> Self {
        PointU16 { x, y }
    }

    pub fn min() -> Self {
        PointU16 { x: 0, y: 0 }
    }

    pub fn max() -> Self {
        PointU16 { x: u16::MAX, y: u16::MAX }
    }
}

impl std::fmt::Display for PointU16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointU16, p2: &PointU16) -> u16 {
    p2.x - p1.x
}

pub fn delta_y(p1: &PointU16, p2: &PointU16) -> u16 {
    p2.y - p1.y
}

pub fn delta(p1: &PointU16, p2: &PointU16) -> PointU16 {
    PointU16 { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

pub fn saturating_translate(p: &mut PointU16, delta: &PointI16) {
    let temp_x = i32::from(p.x) + i32::from(delta.x);
    let temp_y = i32::from(p.y) + i32::from(delta.y);
    let clamped_x = temp_x.clamp(0, i32::from(u16::MAX));
    let clamped_y = temp_y.clamp(0, i32::from(u16::MAX));
    p.x = clamped_x as u16;
    p.y = clamped_y as u16;
}

pub fn checked_translate(p: &mut PointU16, delta: &PointI16) -> Result<(), ()> {
    let x = u16::try_from(i32::from(p.x) + i32::from(delta.x)).map_err(|_| ())?;
    let y = u16::try_from(i32::from(p.y) + i32::from(delta.y)).map_err(|_| ())?;
    p.x = x;
    p.y = y;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_i16::PointI16;

    use super::{PointU16, checked_translate, delta, delta_x, delta_y, saturating_translate};

    #[test]
    fn point_u16() {
        assert_eq!(PointU16::of(0, u16::MAX), PointU16 { x: 0, y: u16::MAX });
        assert_eq!(PointU16::min(), PointU16 { x: 0, y: 0 });
        assert_eq!(PointU16::max(), PointU16 { x: u16::MAX, y: u16::MAX });
        assert_eq!(PointU16::of(0, u16::MAX).to_string(), "(0, 65535)");
        assert_eq!(PointU16::min().to_string(), "(0, 0)");
        assert_eq!(PointU16::max().to_string(), "(65535, 65535)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointU16::min(), &PointU16::of(0, u16::MAX)), 0);
        assert_eq!(delta_x(&PointU16::min(), &PointU16::of(u16::MAX, 0)), u16::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointU16::min(), &PointU16::of(u16::MAX, 0)), 0);
        assert_eq!(delta_y(&PointU16::min(), &PointU16::of(0, u16::MAX)), u16::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointU16::min(), &PointU16::min()), PointU16::min());
        assert_eq!(delta(&PointU16::min(), &PointU16::max()), PointU16::max());
    }

    #[test]
    fn delta_min() {
        let p1 = PointU16::min();
        assert_eq!(delta(&p1, &PointU16::of(0, 0)), PointU16::of(0, 0));
        assert_eq!(delta(&p1, &PointU16::of(0, 1)), PointU16::of(0, 1));
        assert_eq!(delta(&p1, &PointU16::of(0, 2)), PointU16::of(0, 2));

        assert_eq!(delta(&p1, &PointU16::of(1, 0)), PointU16::of(1, 0));
        assert_eq!(delta(&p1, &PointU16::of(1, 1)), PointU16::of(1, 1));
        assert_eq!(delta(&p1, &PointU16::of(1, 2)), PointU16::of(1, 2));

        assert_eq!(delta(&p1, &PointU16::of(2, 0)), PointU16::of(2, 0));
        assert_eq!(delta(&p1, &PointU16::of(2, 1)), PointU16::of(2, 1));
        assert_eq!(delta(&p1, &PointU16::of(2, 2)), PointU16::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p1 = PointU16::of(u16::MAX - 2, u16::MAX - 2);
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 2, u16::MAX - 2)), PointU16::of(0, 0));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 2, u16::MAX - 1)), PointU16::of(0, 1));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 2, u16::MAX)), PointU16::of(0, 2));

        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 1, u16::MAX - 2)), PointU16::of(1, 0));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 1, u16::MAX - 1)), PointU16::of(1, 1));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 1, u16::MAX)), PointU16::of(1, 2));

        assert_eq!(delta(&p1, &PointU16::of(u16::MAX, u16::MAX - 2)), PointU16::of(2, 0));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX, u16::MAX - 1)), PointU16::of(2, 1));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX, u16::MAX)), PointU16::of(2, 2));
    }

    #[test]
    fn test_saturating_translate() {
        let mut r = PointU16::of(0, 0);
        saturating_translate(&mut r, &PointI16::of(10, 10));
        assert_eq!(r, PointU16::of(10, 10));
        saturating_translate(&mut r, &PointI16::of(-5, -5));
        assert_eq!(r, PointU16::of(5, 5));
        saturating_translate(&mut r, &PointI16::of(2, 2));
        assert_eq!(r, PointU16::of(7, 7));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut r = PointU16::of(2, 5);
        saturating_translate(&mut r, &PointI16::of(-10, -10));
        assert_eq!(r, PointU16::of(0, 0));
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut r = PointU16::of(u16::MAX - 2, u16::MAX - 5);
        saturating_translate(&mut r, &PointI16::of(10, 10));
        assert_eq!(r, PointU16::of(u16::MAX, u16::MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_min_delta() {
        let mut r = PointU16::of(1, 1);
        saturating_translate(&mut r, &PointI16::min());
        assert_eq!(r, PointU16::of(0, 0));
    }

    #[test]
    fn saturating_translate_max_bounds_max_delta() {
        let mut r = PointU16::of(u16::MAX - 1, u16::MAX - 1);
        saturating_translate(&mut r, &PointI16::max());
        assert_eq!(r, PointU16::of(u16::MAX, u16::MAX));
    }

    #[test]
    fn test_checked_translate() {
        let mut r = PointU16::of(0, 0);
        assert_eq!(checked_translate(&mut r, &PointI16::of(10, 10)), Ok(()));
        assert_eq!(r, PointU16::of(10, 10));
        assert_eq!(checked_translate(&mut r, &PointI16::of(-5, -5)), Ok(()));
        assert_eq!(r, PointU16::of(5, 5));
        assert_eq!(checked_translate(&mut r, &PointI16::of(2, 2)), Ok(()));
        assert_eq!(r, PointU16::of(7, 7));
    }

    #[test]
    fn checked_translate_min_bounds_err() {
        let mut r = PointU16::of(2, 5);
        assert_eq!(checked_translate(&mut r, &PointI16::of(-10, -10)), Err(()));
        assert_eq!(r, PointU16::of(2, 5));
    }

    #[test]
    fn checked_translate_max_bounds_err() {
        let mut r = PointU16::of(u16::MAX - 2, u16::MAX - 5);
        assert_eq!(checked_translate(&mut r, &PointI16::of(10, 10)), Err(()));
        assert_eq!(r, PointU16::of(u16::MAX - 2, u16::MAX - 5));
    }

    #[test]
    fn checked_translate_min_bounds_ok() {
        let mut r = PointU16::of(2, 5);
        assert_eq!(checked_translate(&mut r, &PointI16::of(-2, -5)), Ok(()));
        assert_eq!(r, PointU16::of(0, 0));
    }

    #[test]
    fn checked_translate_max_bounds_ok() {
        let mut r = PointU16::of(u16::MAX - 2, u16::MAX - 5);
        assert_eq!(checked_translate(&mut r, &PointI16::of(2, 5)), Ok(()));
        assert_eq!(r, PointU16::of(u16::MAX, u16::MAX));
    }

    #[test]
    fn checked_translate_min_bounds_min_delta() {
        let mut r = PointU16::of(1, 1);
        assert_eq!(checked_translate(&mut r, &PointI16::min()), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI16::of(i16::MIN, 0)), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI16::of(0, i16::MIN)), Err(()));
        assert_eq!(r, PointU16::of(1, 1));
    }

    #[test]
    fn checked_translate_max_bounds_max_delta() {
        let mut r = PointU16::of(u16::MAX - 1, u16::MAX - 1);
        assert_eq!(checked_translate(&mut r, &PointI16::max()), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI16::of(i16::MAX, 0)), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI16::of(0, i16::MAX)), Err(()));
        assert_eq!(r, PointU16::of(u16::MAX - 1, u16::MAX - 1));
    }
}

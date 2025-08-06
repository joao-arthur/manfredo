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

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_u8::PointU8;

    use super::{PointI8, delta, delta_x, delta_y, saturating_translate};

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
        let p1 = PointI8::of(i8::MIN, i8::MIN);
        assert_eq!(delta(&p1, &PointI8::of(i8::MIN, i8::MIN)), PointU8::of(0, 0));
        assert_eq!(delta(&p1, &PointI8::of(i8::MIN, i8::MIN + 1)), PointU8::of(0, 1));
        assert_eq!(delta(&p1, &PointI8::of(i8::MIN, i8::MIN + 2)), PointU8::of(0, 2));

        assert_eq!(delta(&p1, &PointI8::of(i8::MIN + 1, i8::MIN)), PointU8::of(1, 0));
        assert_eq!(delta(&p1, &PointI8::of(i8::MIN + 1, i8::MIN + 1)), PointU8::of(1, 1));
        assert_eq!(delta(&p1, &PointI8::of(i8::MIN + 1, i8::MIN + 2)), PointU8::of(1, 2));

        assert_eq!(delta(&p1, &PointI8::of(i8::MIN + 2, i8::MIN)), PointU8::of(2, 0));
        assert_eq!(delta(&p1, &PointI8::of(i8::MIN + 2, i8::MIN + 1)), PointU8::of(2, 1));
        assert_eq!(delta(&p1, &PointI8::of(i8::MIN + 2, i8::MIN + 2)), PointU8::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p1 = PointI8::of(i8::MAX - 2, i8::MAX - 2);
        assert_eq!(delta(&p1, &PointI8::of(i8::MAX - 2, i8::MAX - 2)), PointU8::of(0, 0));
        assert_eq!(delta(&p1, &PointI8::of(i8::MAX - 2, i8::MAX - 1)), PointU8::of(0, 1));
        assert_eq!(delta(&p1, &PointI8::of(i8::MAX - 2, i8::MAX)), PointU8::of(0, 2));

        assert_eq!(delta(&p1, &PointI8::of(i8::MAX - 1, i8::MAX - 2)), PointU8::of(1, 0));
        assert_eq!(delta(&p1, &PointI8::of(i8::MAX - 1, i8::MAX - 1)), PointU8::of(1, 1));
        assert_eq!(delta(&p1, &PointI8::of(i8::MAX - 1, i8::MAX)), PointU8::of(1, 2));

        assert_eq!(delta(&p1, &PointI8::of(i8::MAX, i8::MAX - 2)), PointU8::of(2, 0));
        assert_eq!(delta(&p1, &PointI8::of(i8::MAX, i8::MAX - 1)), PointU8::of(2, 1));
        assert_eq!(delta(&p1, &PointI8::of(i8::MAX, i8::MAX)), PointU8::of(2, 2));
    }

    #[test]
    fn test_saturating_translate() {
        let mut r = PointI8::of(0, 0);
        saturating_translate(&mut r, &PointI8::of(10, 10));
        assert_eq!(r, PointI8::of(10, 10));
        saturating_translate(&mut r, &PointI8::of(-15, -15));
        assert_eq!(r, PointI8::of(-5, -5));
        saturating_translate(&mut r, &PointI8::of(2, 2));
        assert_eq!(r, PointI8::of(-3, -3));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut r = PointI8::of(i8::MIN + 2, i8::MIN + 5);
        saturating_translate(&mut r, &PointI8::of(-10, -10));
        assert_eq!(r, PointI8::of(i8::MIN, i8::MIN));
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut r = PointI8::of(i8::MAX - 2, i8::MAX - 5);
        saturating_translate(&mut r, &PointI8::of(10, 10));
        assert_eq!(r, PointI8::of(i8::MAX, i8::MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_min_delta() {
        let mut r = PointI8::of(i8::MIN + 1, i8::MIN + 1);
        saturating_translate(&mut r, &PointI8::min());
        assert_eq!(r, PointI8::of(i8::MIN, i8::MIN));
    }

    #[test]
    fn saturating_translate_max_bounds_max_delta() {
        let mut r = PointI8::of(i8::MAX - 1, i8::MAX - 1);
        saturating_translate(&mut r, &PointI8::max());
        assert_eq!(r, PointI8::of(i8::MAX, i8::MAX));
    }
}

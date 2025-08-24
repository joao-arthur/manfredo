use super::{point_i8::PointI8, point_u16::PointU16};

pub mod add;

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

impl From<PointI8> for PointI16 {
    fn from(p: PointI8) -> Self {
        PointI16 { x: p.x.into(), y: p.y.into() }
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

#[cfg(test)]
mod tests {
    use super::{PointI16, delta, delta_x, delta_y};
    use crate::cartesian::point::{point_i8::PointI8, point_u16::PointU16};

    #[test]
    fn point_i16() {
        assert_eq!(PointI16::of(i16::MIN, i16::MAX), PointI16 { x: i16::MIN, y: i16::MAX });
        assert_eq!(PointI16::min(), PointI16 { x: i16::MIN, y: i16::MIN });
        assert_eq!(PointI16::max(), PointI16 { x: i16::MAX, y: i16::MAX });
    }

    #[test]
    fn from() {
        assert_eq!(PointI16::from(PointI8::min()), PointI16 { x: i8::MIN.into(), y: i8::MIN.into() });
        assert_eq!(PointI16::from(PointI8::max()), PointI16 { x: i8::MAX.into(), y: i8::MAX.into() });
    }

    #[test]
    fn to_string() {
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
        assert_eq!(delta(&PointI16::of(0, 0), &PointI16::of(0, 0)), PointU16::min());
        assert_eq!(delta(&PointI16::min(), &PointI16::max()), PointU16::max());
    }

    #[test]
    fn delta_min() {
        let p = PointI16::min();
        assert_eq!(delta(&p, &PointI16::min()), PointU16::min());
        assert_eq!(delta(&p, &PointI16::of(i16::MIN, i16::MIN + 1)), PointU16::of(0, 1));
        assert_eq!(delta(&p, &PointI16::of(i16::MIN, i16::MIN + 2)), PointU16::of(0, 2));

        assert_eq!(delta(&p, &PointI16::of(i16::MIN + 1, i16::MIN)), PointU16::of(1, 0));
        assert_eq!(delta(&p, &PointI16::of(i16::MIN + 1, i16::MIN + 1)), PointU16::of(1, 1));
        assert_eq!(delta(&p, &PointI16::of(i16::MIN + 1, i16::MIN + 2)), PointU16::of(1, 2));

        assert_eq!(delta(&p, &PointI16::of(i16::MIN + 2, i16::MIN)), PointU16::of(2, 0));
        assert_eq!(delta(&p, &PointI16::of(i16::MIN + 2, i16::MIN + 1)), PointU16::of(2, 1));
        assert_eq!(delta(&p, &PointI16::of(i16::MIN + 2, i16::MIN + 2)), PointU16::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p = PointI16::of(i16::MAX - 2, i16::MAX - 2);
        assert_eq!(delta(&p, &PointI16::of(i16::MAX - 2, i16::MAX - 2)), PointU16::min());
        assert_eq!(delta(&p, &PointI16::of(i16::MAX - 2, i16::MAX - 1)), PointU16::of(0, 1));
        assert_eq!(delta(&p, &PointI16::of(i16::MAX - 2, i16::MAX)), PointU16::of(0, 2));

        assert_eq!(delta(&p, &PointI16::of(i16::MAX - 1, i16::MAX - 2)), PointU16::of(1, 0));
        assert_eq!(delta(&p, &PointI16::of(i16::MAX - 1, i16::MAX - 1)), PointU16::of(1, 1));
        assert_eq!(delta(&p, &PointI16::of(i16::MAX - 1, i16::MAX)), PointU16::of(1, 2));

        assert_eq!(delta(&p, &PointI16::of(i16::MAX, i16::MAX - 2)), PointU16::of(2, 0));
        assert_eq!(delta(&p, &PointI16::of(i16::MAX, i16::MAX - 1)), PointU16::of(2, 1));
        assert_eq!(delta(&p, &PointI16::max()), PointU16::of(2, 2));
    }
}

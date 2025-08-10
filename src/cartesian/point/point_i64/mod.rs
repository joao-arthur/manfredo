use super::{point_i8::PointI8, point_i16::PointI16, point_i32::PointI32};

use crate::cartesian::point::point_u64::PointU64;

pub mod checked;
pub mod saturating;
pub mod wrapping;

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

impl From<PointI8> for PointI64 {
    fn from(p: PointI8) -> Self {
        PointI64 { x: p.x.into(), y: p.y.into() }
    }
}

impl From<PointI16> for PointI64 {
    fn from(p: PointI16) -> Self {
        PointI64 { x: p.x.into(), y: p.y.into() }
    }
}

impl From<PointI32> for PointI64 {
    fn from(p: PointI32) -> Self {
        PointI64 { x: p.x.into(), y: p.y.into() }
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

#[cfg(test)]
mod tests {
    use crate::cartesian::point::{point_i8::PointI8, point_i16::PointI16, point_i32::PointI32, point_u64::PointU64};

    use super::{PointI64, delta, delta_x, delta_y};

    #[test]
    fn point_i64() {
        assert_eq!(PointI64::of(i64::MIN, i64::MAX), PointI64 { x: i64::MIN, y: i64::MAX });
        assert_eq!(PointI64::min(), PointI64 { x: i64::MIN, y: i64::MIN });
        assert_eq!(PointI64::max(), PointI64 { x: i64::MAX, y: i64::MAX });
    }

    #[test]
    fn from() {
        assert_eq!(PointI64::from(PointI8::min()), PointI64 { x: i8::MIN.into(), y: i8::MIN.into() });
        assert_eq!(PointI64::from(PointI8::max()), PointI64 { x: i8::MAX.into(), y: i8::MAX.into() });
        assert_eq!(PointI64::from(PointI16::min()), PointI64 { x: i16::MIN.into(), y: i16::MIN.into() });
        assert_eq!(PointI64::from(PointI16::max()), PointI64 { x: i16::MAX.into(), y: i16::MAX.into() });
        assert_eq!(PointI64::from(PointI32::min()), PointI64 { x: i32::MIN.into(), y: i32::MIN.into() });
        assert_eq!(PointI64::from(PointI32::max()), PointI64 { x: i32::MAX.into(), y: i32::MAX.into() });
    }

    #[test]
    fn to_string() {
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
        let p = PointI64::min();
        assert_eq!(delta(&p, &PointI64::min()), PointU64::of(0, 0));
        assert_eq!(delta(&p, &PointI64::of(i64::MIN, i64::MIN + 1)), PointU64::of(0, 1));
        assert_eq!(delta(&p, &PointI64::of(i64::MIN, i64::MIN + 2)), PointU64::of(0, 2));

        assert_eq!(delta(&p, &PointI64::of(i64::MIN + 1, i64::MIN)), PointU64::of(1, 0));
        assert_eq!(delta(&p, &PointI64::of(i64::MIN + 1, i64::MIN + 1)), PointU64::of(1, 1));
        assert_eq!(delta(&p, &PointI64::of(i64::MIN + 1, i64::MIN + 2)), PointU64::of(1, 2));

        assert_eq!(delta(&p, &PointI64::of(i64::MIN + 2, i64::MIN)), PointU64::of(2, 0));
        assert_eq!(delta(&p, &PointI64::of(i64::MIN + 2, i64::MIN + 1)), PointU64::of(2, 1));
        assert_eq!(delta(&p, &PointI64::of(i64::MIN + 2, i64::MIN + 2)), PointU64::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p = PointI64::of(i64::MAX - 2, i64::MAX - 2);
        assert_eq!(delta(&p, &PointI64::of(i64::MAX - 2, i64::MAX - 2)), PointU64::of(0, 0));
        assert_eq!(delta(&p, &PointI64::of(i64::MAX - 2, i64::MAX - 1)), PointU64::of(0, 1));
        assert_eq!(delta(&p, &PointI64::of(i64::MAX - 2, i64::MAX)), PointU64::of(0, 2));

        assert_eq!(delta(&p, &PointI64::of(i64::MAX - 1, i64::MAX - 2)), PointU64::of(1, 0));
        assert_eq!(delta(&p, &PointI64::of(i64::MAX - 1, i64::MAX - 1)), PointU64::of(1, 1));
        assert_eq!(delta(&p, &PointI64::of(i64::MAX - 1, i64::MAX)), PointU64::of(1, 2));

        assert_eq!(delta(&p, &PointI64::of(i64::MAX, i64::MAX - 2)), PointU64::of(2, 0));
        assert_eq!(delta(&p, &PointI64::of(i64::MAX, i64::MAX - 1)), PointU64::of(2, 1));
        assert_eq!(delta(&p, &PointI64::max()), PointU64::of(2, 2));
    }
}

use super::{point_i8::PointI8, point_i16::PointI16, point_u32::PointU32};

pub mod add;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointI32 {
    pub row: i32,
    pub col: i32,
}

impl PointI32 {
    pub fn of(row: i32, col: i32) -> Self {
        PointI32 { row, col }
    }

    pub fn min() -> Self {
        PointI32 { row: i32::MIN, col: i32::MIN }
    }

    pub fn max() -> Self {
        PointI32 { row: i32::MAX, col: i32::MAX }
    }
}

impl From<PointI8> for PointI32 {
    fn from(p: PointI8) -> Self {
        PointI32 { row: p.row.into(), col: p.col.into() }
    }
}

impl From<PointI16> for PointI32 {
    fn from(p: PointI16) -> Self {
        PointI32 { row: p.row.into(), col: p.col.into() }
    }
}

impl std::fmt::Display for PointI32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

pub fn delta_row(p1: &PointI32, p2: &PointI32) -> u32 {
    (i64::from(p2.row) - i64::from(p1.row)).unsigned_abs() as u32
}

pub fn delta_col(p1: &PointI32, p2: &PointI32) -> u32 {
    (i64::from(p2.col) - i64::from(p1.col)).unsigned_abs() as u32
}

pub fn delta(p1: &PointI32, p2: &PointI32) -> PointU32 {
    PointU32 { row: delta_row(p1, p2), col: delta_col(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{PointI32, delta, delta_col, delta_row};
    use crate::matrix::point::{point_i8::PointI8, point_i16::PointI16, point_u32::PointU32};

    #[test]
    fn point_i32() {
        assert_eq!(PointI32::of(i32::MIN, i32::MAX), PointI32 { row: i32::MIN, col: i32::MAX });
        assert_eq!(PointI32::min(), PointI32 { row: i32::MIN, col: i32::MIN });
        assert_eq!(PointI32::max(), PointI32 { row: i32::MAX, col: i32::MAX });
    }

    #[test]
    fn from() {
        assert_eq!(PointI32::from(PointI8::min()), PointI32 { row: i8::MIN.into(), col: i8::MIN.into() });
        assert_eq!(PointI32::from(PointI8::max()), PointI32 { row: i8::MAX.into(), col: i8::MAX.into() });
        assert_eq!(PointI32::from(PointI16::min()), PointI32 { row: i16::MIN.into(), col: i16::MIN.into() });
        assert_eq!(PointI32::from(PointI16::max()), PointI32 { row: i16::MAX.into(), col: i16::MAX.into() });
    }

    #[test]
    fn to_string() {
        assert_eq!(PointI32::of(i32::MIN, i32::MAX).to_string(), "(-2147483648, 2147483647)");
        assert_eq!(PointI32::min().to_string(), "(-2147483648, -2147483648)");
        assert_eq!(PointI32::max().to_string(), "(2147483647, 2147483647)");
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&PointI32::of(0, i32::MIN), &PointI32::of(0, i32::MAX)), 0);
        assert_eq!(delta_row(&PointI32::of(i32::MIN, 0), &PointI32::of(i32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&PointI32::of(i32::MIN, 0), &PointI32::of(i32::MAX, 0)), 0);
        assert_eq!(delta_col(&PointI32::of(0, i32::MIN), &PointI32::of(0, i32::MAX)), u32::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointI32::of(0, 0), &PointI32::of(0, 0)), PointU32::min());
        assert_eq!(delta(&PointI32::min(), &PointI32::max()), PointU32::max());
    }

    #[test]
    fn delta_min() {
        let p = PointI32::min();
        assert_eq!(delta(&p, &PointI32::min()), PointU32::min());
        assert_eq!(delta(&p, &PointI32::of(i32::MIN, i32::MIN + 1)), PointU32::of(0, 1));
        assert_eq!(delta(&p, &PointI32::of(i32::MIN, i32::MIN + 2)), PointU32::of(0, 2));

        assert_eq!(delta(&p, &PointI32::of(i32::MIN + 1, i32::MIN)), PointU32::of(1, 0));
        assert_eq!(delta(&p, &PointI32::of(i32::MIN + 1, i32::MIN + 1)), PointU32::of(1, 1));
        assert_eq!(delta(&p, &PointI32::of(i32::MIN + 1, i32::MIN + 2)), PointU32::of(1, 2));

        assert_eq!(delta(&p, &PointI32::of(i32::MIN + 2, i32::MIN)), PointU32::of(2, 0));
        assert_eq!(delta(&p, &PointI32::of(i32::MIN + 2, i32::MIN + 1)), PointU32::of(2, 1));
        assert_eq!(delta(&p, &PointI32::of(i32::MIN + 2, i32::MIN + 2)), PointU32::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p = PointI32::of(i32::MAX - 2, i32::MAX - 2);
        assert_eq!(delta(&p, &PointI32::of(i32::MAX - 2, i32::MAX - 2)), PointU32::min());
        assert_eq!(delta(&p, &PointI32::of(i32::MAX - 2, i32::MAX - 1)), PointU32::of(0, 1));
        assert_eq!(delta(&p, &PointI32::of(i32::MAX - 2, i32::MAX)), PointU32::of(0, 2));

        assert_eq!(delta(&p, &PointI32::of(i32::MAX - 1, i32::MAX - 2)), PointU32::of(1, 0));
        assert_eq!(delta(&p, &PointI32::of(i32::MAX - 1, i32::MAX - 1)), PointU32::of(1, 1));
        assert_eq!(delta(&p, &PointI32::of(i32::MAX - 1, i32::MAX)), PointU32::of(1, 2));

        assert_eq!(delta(&p, &PointI32::of(i32::MAX, i32::MAX - 2)), PointU32::of(2, 0));
        assert_eq!(delta(&p, &PointI32::of(i32::MAX, i32::MAX - 1)), PointU32::of(2, 1));
        assert_eq!(delta(&p, &PointI32::max()), PointU32::of(2, 2));
    }
}

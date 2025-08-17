use super::{point_u8::PointU8, point_u16::PointU16, point_u32::PointU32};

pub mod add;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointU64 {
    pub row: u64,
    pub col: u64,
}

impl PointU64 {
    pub fn of(row: u64, col: u64) -> Self {
        PointU64 { row, col }
    }

    pub fn min() -> Self {
        PointU64 { row: 0, col: 0 }
    }

    pub fn max() -> Self {
        PointU64 { row: u64::MAX, col: u64::MAX }
    }
}

impl From<PointU8> for PointU64 {
    fn from(p: PointU8) -> Self {
        PointU64 { row: p.row.into(), col: p.col.into() }
    }
}

impl From<PointU16> for PointU64 {
    fn from(p: PointU16) -> Self {
        PointU64 { row: p.row.into(), col: p.col.into() }
    }
}

impl From<PointU32> for PointU64 {
    fn from(p: PointU32) -> Self {
        PointU64 { row: p.row.into(), col: p.col.into() }
    }
}

impl std::fmt::Display for PointU64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

pub fn delta_row(p1: &PointU64, p2: &PointU64) -> u64 {
    p2.row - p1.row
}

pub fn delta_col(p1: &PointU64, p2: &PointU64) -> u64 {
    p2.col - p1.col
}

pub fn delta(p1: &PointU64, p2: &PointU64) -> PointU64 {
    PointU64 { row: delta_row(p1, p2), col: delta_col(p1, p2) }
}

#[cfg(test)]
mod tests {
    use crate::matrix::point::{point_u8::PointU8, point_u16::PointU16, point_u32::PointU32};

    use super::{PointU64, delta, delta_col, delta_row};

    #[test]
    fn point_u64() {
        assert_eq!(PointU64::of(0, u64::MAX), PointU64 { row: 0, col: u64::MAX });
        assert_eq!(PointU64::min(), PointU64 { row: 0, col: 0 });
        assert_eq!(PointU64::max(), PointU64 { row: u64::MAX, col: u64::MAX });
    }

    #[test]
    fn from() {
        assert_eq!(PointU64::from(PointU8::min()), PointU64 { row: u8::MIN.into(), col: u8::MIN.into() });
        assert_eq!(PointU64::from(PointU8::max()), PointU64 { row: u8::MAX.into(), col: u8::MAX.into() });
        assert_eq!(PointU64::from(PointU16::min()), PointU64 { row: u16::MIN.into(), col: u16::MIN.into() });
        assert_eq!(PointU64::from(PointU16::max()), PointU64 { row: u16::MAX.into(), col: u16::MAX.into() });
        assert_eq!(PointU64::from(PointU32::min()), PointU64 { row: u32::MIN.into(), col: u32::MIN.into() });
        assert_eq!(PointU64::from(PointU32::max()), PointU64 { row: u32::MAX.into(), col: u32::MAX.into() });
    }

    #[test]
    fn to_string() {
        assert_eq!(PointU64::of(0, u64::MAX).to_string(), "(0, 18446744073709551615)");
        assert_eq!(PointU64::min().to_string(), "(0, 0)");
        assert_eq!(PointU64::max().to_string(), "(18446744073709551615, 18446744073709551615)");
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&PointU64::min(), &PointU64::of(0, u64::MAX)), 0);
        assert_eq!(delta_row(&PointU64::min(), &PointU64::of(u64::MAX, 0)), u64::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&PointU64::min(), &PointU64::of(u64::MAX, 0)), 0);
        assert_eq!(delta_col(&PointU64::min(), &PointU64::of(0, u64::MAX)), u64::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointU64::min(), &PointU64::min()), PointU64::min());
        assert_eq!(delta(&PointU64::min(), &PointU64::max()), PointU64::max());
    }

    #[test]
    fn delta_min() {
        let p = PointU64::min();
        assert_eq!(delta(&p, &PointU64::of(0, 0)), PointU64::of(0, 0));
        assert_eq!(delta(&p, &PointU64::of(0, 1)), PointU64::of(0, 1));
        assert_eq!(delta(&p, &PointU64::of(0, 2)), PointU64::of(0, 2));

        assert_eq!(delta(&p, &PointU64::of(1, 0)), PointU64::of(1, 0));
        assert_eq!(delta(&p, &PointU64::of(1, 1)), PointU64::of(1, 1));
        assert_eq!(delta(&p, &PointU64::of(1, 2)), PointU64::of(1, 2));

        assert_eq!(delta(&p, &PointU64::of(2, 0)), PointU64::of(2, 0));
        assert_eq!(delta(&p, &PointU64::of(2, 1)), PointU64::of(2, 1));
        assert_eq!(delta(&p, &PointU64::of(2, 2)), PointU64::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p = PointU64::of(u64::MAX - 2, u64::MAX - 2);
        assert_eq!(delta(&p, &PointU64::of(u64::MAX - 2, u64::MAX - 2)), PointU64::of(0, 0));
        assert_eq!(delta(&p, &PointU64::of(u64::MAX - 2, u64::MAX - 1)), PointU64::of(0, 1));
        assert_eq!(delta(&p, &PointU64::of(u64::MAX - 2, u64::MAX)), PointU64::of(0, 2));

        assert_eq!(delta(&p, &PointU64::of(u64::MAX - 1, u64::MAX - 2)), PointU64::of(1, 0));
        assert_eq!(delta(&p, &PointU64::of(u64::MAX - 1, u64::MAX - 1)), PointU64::of(1, 1));
        assert_eq!(delta(&p, &PointU64::of(u64::MAX - 1, u64::MAX)), PointU64::of(1, 2));

        assert_eq!(delta(&p, &PointU64::of(u64::MAX, u64::MAX - 2)), PointU64::of(2, 0));
        assert_eq!(delta(&p, &PointU64::of(u64::MAX, u64::MAX - 1)), PointU64::of(2, 1));
        assert_eq!(delta(&p, &PointU64::max()), PointU64::of(2, 2));
    }
}

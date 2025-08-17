use super::{point_u8::PointU8, point_u16::PointU16};

pub mod add;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointU32 {
    pub row: u32,
    pub col: u32,
}

impl PointU32 {
    pub fn of(row: u32, col: u32) -> Self {
        PointU32 { row, col }
    }

    pub fn min() -> Self {
        PointU32 { row: 0, col: 0 }
    }

    pub fn max() -> Self {
        PointU32 { row: u32::MAX, col: u32::MAX }
    }
}

impl From<PointU8> for PointU32 {
    fn from(p: PointU8) -> Self {
        PointU32 { row: p.row.into(), col: p.col.into() }
    }
}

impl From<PointU16> for PointU32 {
    fn from(p: PointU16) -> Self {
        PointU32 { row: p.row.into(), col: p.col.into() }
    }
}

impl std::fmt::Display for PointU32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

pub fn delta_row(p1: &PointU32, p2: &PointU32) -> u32 {
    p2.row - p1.row
}

pub fn delta_col(p1: &PointU32, p2: &PointU32) -> u32 {
    p2.col - p1.col
}

pub fn delta(p1: &PointU32, p2: &PointU32) -> PointU32 {
    PointU32 { row: delta_row(p1, p2), col: delta_col(p1, p2) }
}

#[cfg(test)]
mod tests {
    use crate::matrix::point::{point_u8::PointU8, point_u16::PointU16};

    use super::{PointU32, delta, delta_col, delta_row};

    #[test]
    fn point_u32() {
        assert_eq!(PointU32::of(0, u32::MAX), PointU32 { row: 0, col: u32::MAX });
        assert_eq!(PointU32::min(), PointU32 { row: 0, col: 0 });
        assert_eq!(PointU32::max(), PointU32 { row: u32::MAX, col: u32::MAX });
    }

    #[test]
    fn from() {
        assert_eq!(PointU32::from(PointU8::min()), PointU32 { row: u8::MIN.into(), col: u8::MIN.into() });
        assert_eq!(PointU32::from(PointU8::max()), PointU32 { row: u8::MAX.into(), col: u8::MAX.into() });
        assert_eq!(PointU32::from(PointU16::min()), PointU32 { row: u16::MIN.into(), col: u16::MIN.into() });
        assert_eq!(PointU32::from(PointU16::max()), PointU32 { row: u16::MAX.into(), col: u16::MAX.into() });
    }

    #[test]
    fn to_string() {
        assert_eq!(PointU32::of(0, u32::MAX).to_string(), "(0, 4294967295)");
        assert_eq!(PointU32::min().to_string(), "(0, 0)");
        assert_eq!(PointU32::max().to_string(), "(4294967295, 4294967295)");
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&PointU32::min(), &PointU32::of(0, u32::MAX)), 0);
        assert_eq!(delta_row(&PointU32::min(), &PointU32::of(u32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&PointU32::min(), &PointU32::of(u32::MAX, 0)), 0);
        assert_eq!(delta_col(&PointU32::min(), &PointU32::of(0, u32::MAX)), u32::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointU32::min(), &PointU32::min()), PointU32::min());
        assert_eq!(delta(&PointU32::min(), &PointU32::max()), PointU32::max());
    }

    #[test]
    fn delta_min() {
        let p = PointU32::min();
        assert_eq!(delta(&p, &PointU32::of(0, 0)), PointU32::of(0, 0));
        assert_eq!(delta(&p, &PointU32::of(0, 1)), PointU32::of(0, 1));
        assert_eq!(delta(&p, &PointU32::of(0, 2)), PointU32::of(0, 2));

        assert_eq!(delta(&p, &PointU32::of(1, 0)), PointU32::of(1, 0));
        assert_eq!(delta(&p, &PointU32::of(1, 1)), PointU32::of(1, 1));
        assert_eq!(delta(&p, &PointU32::of(1, 2)), PointU32::of(1, 2));

        assert_eq!(delta(&p, &PointU32::of(2, 0)), PointU32::of(2, 0));
        assert_eq!(delta(&p, &PointU32::of(2, 1)), PointU32::of(2, 1));
        assert_eq!(delta(&p, &PointU32::of(2, 2)), PointU32::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p = PointU32::of(u32::MAX - 2, u32::MAX - 2);
        assert_eq!(delta(&p, &PointU32::of(u32::MAX - 2, u32::MAX - 2)), PointU32::of(0, 0));
        assert_eq!(delta(&p, &PointU32::of(u32::MAX - 2, u32::MAX - 1)), PointU32::of(0, 1));
        assert_eq!(delta(&p, &PointU32::of(u32::MAX - 2, u32::MAX)), PointU32::of(0, 2));

        assert_eq!(delta(&p, &PointU32::of(u32::MAX - 1, u32::MAX - 2)), PointU32::of(1, 0));
        assert_eq!(delta(&p, &PointU32::of(u32::MAX - 1, u32::MAX - 1)), PointU32::of(1, 1));
        assert_eq!(delta(&p, &PointU32::of(u32::MAX - 1, u32::MAX)), PointU32::of(1, 2));

        assert_eq!(delta(&p, &PointU32::of(u32::MAX, u32::MAX - 2)), PointU32::of(2, 0));
        assert_eq!(delta(&p, &PointU32::of(u32::MAX, u32::MAX - 1)), PointU32::of(2, 1));
        assert_eq!(delta(&p, &PointU32::max()), PointU32::of(2, 2));
    }
}

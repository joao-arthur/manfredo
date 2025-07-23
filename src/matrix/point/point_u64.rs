#[derive(Debug, PartialEq)]
pub struct PointU64 {
    pub row: u64,
    pub col: u64,
}

impl PointU64 {
    pub fn of(row: u64, col: u64) -> Self {
        PointU64 { row, col }
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

#[cfg(test)]
mod tests {
    use super::{PointU64, delta_col, delta_row};

    #[test]
    fn point_u64() {
        assert_eq!(PointU64::of(0, u64::MAX), PointU64 { row: 0, col: u64::MAX });
        assert_eq!(PointU64::of(0, u64::MAX).to_string(), "(0, 18446744073709551615)");
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&PointU64::of(0, 0), &PointU64::of(0, u64::MAX)), 0);
        assert_eq!(delta_row(&PointU64::of(0, 0), &PointU64::of(u64::MAX, 0)), u64::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&PointU64::of(0, 0), &PointU64::of(u64::MAX, 0)), 0);
        assert_eq!(delta_col(&PointU64::of(0, 0), &PointU64::of(0, u64::MAX)), u64::MAX);
    }
}

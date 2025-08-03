#[derive(Eq, PartialEq, Debug, Clone, Hash)]
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

pub fn delta(p1: &PointU64, p2: &PointU64) -> PointU64 {
    PointU64 { row: delta_row(p1, p2), col: delta_col(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{PointU64, delta, delta_col, delta_row};

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

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointU64::of(0, 0), &PointU64::of(0, 0)), PointU64::of(0, 0));
        assert_eq!(delta(&PointU64::of(0, 0), &PointU64::of(u64::MAX, u64::MAX)), PointU64::of(u64::MAX, u64::MAX));
    }

    #[test]
    fn delta_min() {
        let p1 = PointU64::of(0, 0);
        assert_eq!(delta(&p1, &PointU64::of(0, 0)), PointU64::of(0, 0));
        assert_eq!(delta(&p1, &PointU64::of(0, 1)), PointU64::of(0, 1));
        assert_eq!(delta(&p1, &PointU64::of(0, 2)), PointU64::of(0, 2));

        assert_eq!(delta(&p1, &PointU64::of(1, 0)), PointU64::of(1, 0));
        assert_eq!(delta(&p1, &PointU64::of(1, 1)), PointU64::of(1, 1));
        assert_eq!(delta(&p1, &PointU64::of(1, 2)), PointU64::of(1, 2));

        assert_eq!(delta(&p1, &PointU64::of(2, 0)), PointU64::of(2, 0));
        assert_eq!(delta(&p1, &PointU64::of(2, 1)), PointU64::of(2, 1));
        assert_eq!(delta(&p1, &PointU64::of(2, 2)), PointU64::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p1 = PointU64::of(u64::MAX - 2, u64::MAX - 2);
        assert_eq!(delta(&p1, &PointU64::of(u64::MAX - 2, u64::MAX - 2)), PointU64::of(0, 0));
        assert_eq!(delta(&p1, &PointU64::of(u64::MAX - 2, u64::MAX - 1)), PointU64::of(0, 1));
        assert_eq!(delta(&p1, &PointU64::of(u64::MAX - 2, u64::MAX)), PointU64::of(0, 2));

        assert_eq!(delta(&p1, &PointU64::of(u64::MAX - 1, u64::MAX - 2)), PointU64::of(1, 0));
        assert_eq!(delta(&p1, &PointU64::of(u64::MAX - 1, u64::MAX - 1)), PointU64::of(1, 1));
        assert_eq!(delta(&p1, &PointU64::of(u64::MAX - 1, u64::MAX)), PointU64::of(1, 2));

        assert_eq!(delta(&p1, &PointU64::of(u64::MAX, u64::MAX - 2)), PointU64::of(2, 0));
        assert_eq!(delta(&p1, &PointU64::of(u64::MAX, u64::MAX - 1)), PointU64::of(2, 1));
        assert_eq!(delta(&p1, &PointU64::of(u64::MAX, u64::MAX)), PointU64::of(2, 2));
    }
}

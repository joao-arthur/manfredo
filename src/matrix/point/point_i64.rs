use crate::matrix::point::point_u64::PointU64;

#[derive(PartialEq, Debug, Clone)]
pub struct PointI64 {
    pub row: i64,
    pub col: i64,
}

impl PointI64 {
    pub fn of(row: i64, col: i64) -> Self {
        PointI64 { row, col }
    }
}

impl std::fmt::Display for PointI64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

pub fn delta_row(p1: &PointI64, p2: &PointI64) -> u64 {
    (i128::from(p2.row) - i128::from(p1.row)).unsigned_abs() as u64
}

pub fn delta_col(p1: &PointI64, p2: &PointI64) -> u64 {
    (i128::from(p2.col) - i128::from(p1.col)).unsigned_abs() as u64
}

pub fn delta(p1: &PointI64, p2: &PointI64) -> PointU64 {
    PointU64 { row: delta_row(p1, p2), col: delta_col(p1, p2) }
}

#[cfg(test)]
mod tests {
    use crate::matrix::point::point_u64::PointU64;

    use super::{PointI64, delta, delta_col, delta_row};

    #[test]
    fn point_i64() {
        assert_eq!(PointI64::of(i64::MIN, i64::MAX), PointI64 { row: i64::MIN, col: i64::MAX });
        assert_eq!(PointI64::of(i64::MIN, i64::MAX).to_string(), "(-9223372036854775808, 9223372036854775807)");
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&PointI64::of(0, i64::MIN), &PointI64::of(0, i64::MAX)), 0);
        assert_eq!(delta_row(&PointI64::of(i64::MIN, 0), &PointI64::of(i64::MAX, 0)), u64::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&PointI64::of(i64::MIN, 0), &PointI64::of(i64::MAX, 0)), 0);
        assert_eq!(delta_col(&PointI64::of(0, i64::MIN), &PointI64::of(0, i64::MAX)), u64::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointI64::of(0, 0), &PointI64::of(0, 0)), PointU64::of(0, 0));
        assert_eq!(delta(&PointI64::of(i64::MIN, i64::MIN), &PointI64::of(i64::MAX, i64::MAX)), PointU64::of(u64::MAX, u64::MAX));
    }

    #[test]
    fn delta_min() {
        let p1 = PointI64::of(i64::MIN, i64::MIN);
        assert_eq!(delta(&p1, &PointI64::of(i64::MIN, i64::MIN)), PointU64::of(0, 0));
        assert_eq!(delta(&p1, &PointI64::of(i64::MIN, i64::MIN + 1)), PointU64::of(0, 1));
        assert_eq!(delta(&p1, &PointI64::of(i64::MIN, i64::MIN + 2)), PointU64::of(0, 2));

        assert_eq!(delta(&p1, &PointI64::of(i64::MIN + 1, i64::MIN)), PointU64::of(1, 0));
        assert_eq!(delta(&p1, &PointI64::of(i64::MIN + 1, i64::MIN + 1)), PointU64::of(1, 1));
        assert_eq!(delta(&p1, &PointI64::of(i64::MIN + 1, i64::MIN + 2)), PointU64::of(1, 2));

        assert_eq!(delta(&p1, &PointI64::of(i64::MIN + 2, i64::MIN)), PointU64::of(2, 0));
        assert_eq!(delta(&p1, &PointI64::of(i64::MIN + 2, i64::MIN + 1)), PointU64::of(2, 1));
        assert_eq!(delta(&p1, &PointI64::of(i64::MIN + 2, i64::MIN + 2)), PointU64::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p1 = PointI64::of(i64::MAX - 2, i64::MAX - 2);
        assert_eq!(delta(&p1, &PointI64::of(i64::MAX - 2, i64::MAX - 2)), PointU64::of(0, 0));
        assert_eq!(delta(&p1, &PointI64::of(i64::MAX - 2, i64::MAX - 1)), PointU64::of(0, 1));
        assert_eq!(delta(&p1, &PointI64::of(i64::MAX - 2, i64::MAX)), PointU64::of(0, 2));

        assert_eq!(delta(&p1, &PointI64::of(i64::MAX - 1, i64::MAX - 2)), PointU64::of(1, 0));
        assert_eq!(delta(&p1, &PointI64::of(i64::MAX - 1, i64::MAX - 1)), PointU64::of(1, 1));
        assert_eq!(delta(&p1, &PointI64::of(i64::MAX - 1, i64::MAX)), PointU64::of(1, 2));

        assert_eq!(delta(&p1, &PointI64::of(i64::MAX, i64::MAX - 2)), PointU64::of(2, 0));
        assert_eq!(delta(&p1, &PointI64::of(i64::MAX, i64::MAX - 1)), PointU64::of(2, 1));
        assert_eq!(delta(&p1, &PointI64::of(i64::MAX, i64::MAX)), PointU64::of(2, 2));
    }
}

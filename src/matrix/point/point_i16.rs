use crate::matrix::point::point_u16::PointU16;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointI16 {
    pub row: i16,
    pub col: i16,
}

impl PointI16 {
    pub fn of(row: i16, col: i16) -> Self {
        PointI16 { row, col }
    }

    pub fn min() -> Self {
        PointI16 { row: i16::MIN, col: i16::MIN }
    }

    pub fn max() -> Self {
        PointI16 { row: i16::MAX, col: i16::MAX }
    }
}

impl std::fmt::Display for PointI16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

pub fn delta_row(p1: &PointI16, p2: &PointI16) -> u16 {
    (i32::from(p2.row) - i32::from(p1.row)).unsigned_abs() as u16
}

pub fn delta_col(p1: &PointI16, p2: &PointI16) -> u16 {
    (i32::from(p2.col) - i32::from(p1.col)).unsigned_abs() as u16
}

pub fn delta(p1: &PointI16, p2: &PointI16) -> PointU16 {
    PointU16 { row: delta_row(p1, p2), col: delta_col(p1, p2) }
}

#[cfg(test)]
mod tests {
    use crate::matrix::point::point_u16::PointU16;

    use super::{PointI16, delta, delta_col, delta_row};

    #[test]
    fn point_i16() {
        assert_eq!(PointI16::of(i16::MIN, i16::MAX), PointI16 { row: i16::MIN, col: i16::MAX });
        assert_eq!(PointI16::min(), PointI16 { row: i16::MIN, col: i16::MIN });
        assert_eq!(PointI16::max(), PointI16 { row: i16::MAX, col: i16::MAX });
        assert_eq!(PointI16::of(i16::MIN, i16::MAX).to_string(), "(-32768, 32767)");
        assert_eq!(PointI16::min().to_string(), "(-32768, -32768)");
        assert_eq!(PointI16::max().to_string(), "(32767, 32767)");
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&PointI16::of(0, i16::MIN), &PointI16::of(0, i16::MAX)), 0);
        assert_eq!(delta_row(&PointI16::of(i16::MIN, 0), &PointI16::of(i16::MAX, 0)), u16::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&PointI16::of(i16::MIN, 0), &PointI16::of(i16::MAX, 0)), 0);
        assert_eq!(delta_col(&PointI16::of(0, i16::MIN), &PointI16::of(0, i16::MAX)), u16::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointI16::of(0, 0), &PointI16::of(0, 0)), PointU16::of(0, 0));
        assert_eq!(delta(&PointI16::min(), &PointI16::of(i16::MAX, i16::MAX)), PointU16::of(u16::MAX, u16::MAX));
    }

    #[test]
    fn delta_min() {
        let p1 = PointI16::of(i16::MIN, i16::MIN);
        assert_eq!(delta(&p1, &PointI16::of(i16::MIN, i16::MIN)), PointU16::of(0, 0));
        assert_eq!(delta(&p1, &PointI16::of(i16::MIN, i16::MIN + 1)), PointU16::of(0, 1));
        assert_eq!(delta(&p1, &PointI16::of(i16::MIN, i16::MIN + 2)), PointU16::of(0, 2));

        assert_eq!(delta(&p1, &PointI16::of(i16::MIN + 1, i16::MIN)), PointU16::of(1, 0));
        assert_eq!(delta(&p1, &PointI16::of(i16::MIN + 1, i16::MIN + 1)), PointU16::of(1, 1));
        assert_eq!(delta(&p1, &PointI16::of(i16::MIN + 1, i16::MIN + 2)), PointU16::of(1, 2));

        assert_eq!(delta(&p1, &PointI16::of(i16::MIN + 2, i16::MIN)), PointU16::of(2, 0));
        assert_eq!(delta(&p1, &PointI16::of(i16::MIN + 2, i16::MIN + 1)), PointU16::of(2, 1));
        assert_eq!(delta(&p1, &PointI16::of(i16::MIN + 2, i16::MIN + 2)), PointU16::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p1 = PointI16::of(i16::MAX - 2, i16::MAX - 2);
        assert_eq!(delta(&p1, &PointI16::of(i16::MAX - 2, i16::MAX - 2)), PointU16::of(0, 0));
        assert_eq!(delta(&p1, &PointI16::of(i16::MAX - 2, i16::MAX - 1)), PointU16::of(0, 1));
        assert_eq!(delta(&p1, &PointI16::of(i16::MAX - 2, i16::MAX)), PointU16::of(0, 2));

        assert_eq!(delta(&p1, &PointI16::of(i16::MAX - 1, i16::MAX - 2)), PointU16::of(1, 0));
        assert_eq!(delta(&p1, &PointI16::of(i16::MAX - 1, i16::MAX - 1)), PointU16::of(1, 1));
        assert_eq!(delta(&p1, &PointI16::of(i16::MAX - 1, i16::MAX)), PointU16::of(1, 2));

        assert_eq!(delta(&p1, &PointI16::of(i16::MAX, i16::MAX - 2)), PointU16::of(2, 0));
        assert_eq!(delta(&p1, &PointI16::of(i16::MAX, i16::MAX - 1)), PointU16::of(2, 1));
        assert_eq!(delta(&p1, &PointI16::of(i16::MAX, i16::MAX)), PointU16::of(2, 2));
    }
}

use crate::matrix::point::point_u8::PointU8;

#[derive(PartialEq, Debug, Clone)]
pub struct PointI8 {
    pub row: i8,
    pub col: i8,
}

impl PointI8 {
    pub fn of(row: i8, col: i8) -> Self {
        PointI8 { row, col }
    }
}

impl std::fmt::Display for PointI8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

pub fn delta_row(p1: &PointI8, p2: &PointI8) -> u8 {
    (i16::from(p2.row) - i16::from(p1.row)).unsigned_abs() as u8
}

pub fn delta_col(p1: &PointI8, p2: &PointI8) -> u8 {
    (i16::from(p2.col) - i16::from(p1.col)).unsigned_abs() as u8
}

pub fn delta(p1: &PointI8, p2: &PointI8) -> PointU8 {
    PointU8 { row: delta_row(p1, p2), col: delta_col(p1, p2) }
}

#[cfg(test)]
mod tests {
    use crate::matrix::point::point_u8::PointU8;

    use super::{PointI8, delta, delta_col, delta_row};

    #[test]
    fn point_i8() {
        assert_eq!(PointI8::of(i8::MIN, i8::MAX), PointI8 { row: i8::MIN, col: i8::MAX });
        assert_eq!(PointI8::of(i8::MIN, i8::MAX).to_string(), "(-128, 127)");
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&PointI8::of(0, i8::MIN), &PointI8::of(0, i8::MAX)), 0);
        assert_eq!(delta_row(&PointI8::of(i8::MIN, 0), &PointI8::of(i8::MAX, 0)), u8::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&PointI8::of(i8::MIN, 0), &PointI8::of(i8::MAX, 0)), 0);
        assert_eq!(delta_col(&PointI8::of(0, i8::MIN), &PointI8::of(0, i8::MAX)), u8::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointI8::of(0, 0), &PointI8::of(0, 0)), PointU8::of(0, 0));
        assert_eq!(delta(&PointI8::of(i8::MIN, i8::MIN), &PointI8::of(i8::MAX, i8::MAX)), PointU8::of(u8::MAX, u8::MAX));
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
}

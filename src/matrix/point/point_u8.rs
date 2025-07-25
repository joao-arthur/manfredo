#[derive(Debug, PartialEq, Clone)]
pub struct PointU8 {
    pub row: u8,
    pub col: u8,
}

impl PointU8 {
    pub fn of(row: u8, col: u8) -> Self {
        PointU8 { row, col }
    }
}

impl std::fmt::Display for PointU8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

pub fn delta_row(p1: &PointU8, p2: &PointU8) -> u8 {
    p2.row - p1.row
}

pub fn delta_col(p1: &PointU8, p2: &PointU8) -> u8 {
    p2.col - p1.col
}

pub fn delta(p1: &PointU8, p2: &PointU8) -> PointU8 {
    PointU8 { row: delta_row(p1, p2), col: delta_col(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{PointU8, delta, delta_col, delta_row};

    #[test]
    fn point_u8() {
        assert_eq!(PointU8::of(0, u8::MAX), PointU8 { row: 0, col: u8::MAX });
        assert_eq!(PointU8::of(0, u8::MAX).to_string(), "(0, 255)");
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&PointU8::of(0, 0), &PointU8::of(0, u8::MAX)), 0);
        assert_eq!(delta_row(&PointU8::of(0, 0), &PointU8::of(u8::MAX, 0)), u8::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&PointU8::of(0, 0), &PointU8::of(u8::MAX, 0)), 0);
        assert_eq!(delta_col(&PointU8::of(0, 0), &PointU8::of(0, u8::MAX)), u8::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointU8::of(0, 0), &PointU8::of(0, 0)), PointU8::of(0, 0));
        assert_eq!(delta(&PointU8::of(0, 0), &PointU8::of(u8::MAX, u8::MAX)), PointU8::of(u8::MAX, u8::MAX));
    }

    #[test]
    fn delta_min() {
        let p1 = PointU8::of(0, 0);
        assert_eq!(delta(&p1, &PointU8::of(0, 0)), PointU8::of(0, 0));
        assert_eq!(delta(&p1, &PointU8::of(0, 1)), PointU8::of(0, 1));
        assert_eq!(delta(&p1, &PointU8::of(0, 2)), PointU8::of(0, 2));

        assert_eq!(delta(&p1, &PointU8::of(1, 0)), PointU8::of(1, 0));
        assert_eq!(delta(&p1, &PointU8::of(1, 1)), PointU8::of(1, 1));
        assert_eq!(delta(&p1, &PointU8::of(1, 2)), PointU8::of(1, 2));

        assert_eq!(delta(&p1, &PointU8::of(2, 0)), PointU8::of(2, 0));
        assert_eq!(delta(&p1, &PointU8::of(2, 1)), PointU8::of(2, 1));
        assert_eq!(delta(&p1, &PointU8::of(2, 2)), PointU8::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p1 = PointU8::of(u8::MAX - 2, u8::MAX - 2);
        assert_eq!(delta(&p1, &PointU8::of(u8::MAX - 2, u8::MAX - 2)), PointU8::of(0, 0));
        assert_eq!(delta(&p1, &PointU8::of(u8::MAX - 2, u8::MAX - 1)), PointU8::of(0, 1));
        assert_eq!(delta(&p1, &PointU8::of(u8::MAX - 2, u8::MAX)), PointU8::of(0, 2));

        assert_eq!(delta(&p1, &PointU8::of(u8::MAX - 1, u8::MAX - 2)), PointU8::of(1, 0));
        assert_eq!(delta(&p1, &PointU8::of(u8::MAX - 1, u8::MAX - 1)), PointU8::of(1, 1));
        assert_eq!(delta(&p1, &PointU8::of(u8::MAX - 1, u8::MAX)), PointU8::of(1, 2));

        assert_eq!(delta(&p1, &PointU8::of(u8::MAX, u8::MAX - 2)), PointU8::of(2, 0));
        assert_eq!(delta(&p1, &PointU8::of(u8::MAX, u8::MAX - 1)), PointU8::of(2, 1));
        assert_eq!(delta(&p1, &PointU8::of(u8::MAX, u8::MAX)), PointU8::of(2, 2));
    }
}

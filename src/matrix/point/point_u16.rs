#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointU16 {
    pub row: u16,
    pub col: u16,
}

impl PointU16 {
    pub fn of(row: u16, col: u16) -> Self {
        PointU16 { row, col }
    }

    pub fn min() -> Self {
        PointU16 { row: 0, col: 0 }
    }

    pub fn max() -> Self {
        PointU16 { row: u16::MAX, col: u16::MAX }
    }
}

impl std::fmt::Display for PointU16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

pub fn delta_row(p1: &PointU16, p2: &PointU16) -> u16 {
    p2.row - p1.row
}

pub fn delta_col(p1: &PointU16, p2: &PointU16) -> u16 {
    p2.col - p1.col
}

pub fn delta(p1: &PointU16, p2: &PointU16) -> PointU16 {
    PointU16 { row: delta_row(p1, p2), col: delta_col(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{PointU16, delta, delta_col, delta_row};

    #[test]
    fn point_u16() {
        assert_eq!(PointU16::of(0, u16::MAX), PointU16 { row: 0, col: u16::MAX });
        assert_eq!(PointU16::min(), PointU16 { row: 0, col: 0 });
        assert_eq!(PointU16::max(), PointU16 { row: u16::MAX, col: u16::MAX });
        assert_eq!(PointU16::of(0, u16::MAX).to_string(), "(0, 65535)");
        assert_eq!(PointU16::min().to_string(), "(0, 0)");
        assert_eq!(PointU16::max().to_string(), "(65535, 65535)");
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&PointU16::of(0, 0), &PointU16::of(0, u16::MAX)), 0);
        assert_eq!(delta_row(&PointU16::of(0, 0), &PointU16::of(u16::MAX, 0)), u16::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&PointU16::of(0, 0), &PointU16::of(u16::MAX, 0)), 0);
        assert_eq!(delta_col(&PointU16::of(0, 0), &PointU16::of(0, u16::MAX)), u16::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointU16::of(0, 0), &PointU16::of(0, 0)), PointU16::of(0, 0));
        assert_eq!(delta(&PointU16::of(0, 0), &PointU16::of(u16::MAX, u16::MAX)), PointU16::of(u16::MAX, u16::MAX));
    }

    #[test]
    fn delta_min() {
        let p1 = PointU16::of(0, 0);
        assert_eq!(delta(&p1, &PointU16::of(0, 0)), PointU16::of(0, 0));
        assert_eq!(delta(&p1, &PointU16::of(0, 1)), PointU16::of(0, 1));
        assert_eq!(delta(&p1, &PointU16::of(0, 2)), PointU16::of(0, 2));

        assert_eq!(delta(&p1, &PointU16::of(1, 0)), PointU16::of(1, 0));
        assert_eq!(delta(&p1, &PointU16::of(1, 1)), PointU16::of(1, 1));
        assert_eq!(delta(&p1, &PointU16::of(1, 2)), PointU16::of(1, 2));

        assert_eq!(delta(&p1, &PointU16::of(2, 0)), PointU16::of(2, 0));
        assert_eq!(delta(&p1, &PointU16::of(2, 1)), PointU16::of(2, 1));
        assert_eq!(delta(&p1, &PointU16::of(2, 2)), PointU16::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p1 = PointU16::of(u16::MAX - 2, u16::MAX - 2);
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 2, u16::MAX - 2)), PointU16::of(0, 0));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 2, u16::MAX - 1)), PointU16::of(0, 1));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 2, u16::MAX)), PointU16::of(0, 2));

        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 1, u16::MAX - 2)), PointU16::of(1, 0));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 1, u16::MAX - 1)), PointU16::of(1, 1));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 1, u16::MAX)), PointU16::of(1, 2));

        assert_eq!(delta(&p1, &PointU16::of(u16::MAX, u16::MAX - 2)), PointU16::of(2, 0));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX, u16::MAX - 1)), PointU16::of(2, 1));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX, u16::MAX)), PointU16::of(2, 2));
    }
}

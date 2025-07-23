#[derive(Debug, PartialEq)]
pub struct PointU16 {
    pub row: u16,
    pub col: u16,
}

impl PointU16 {
    pub fn of(row: u16, col: u16) -> Self {
        PointU16 { row, col }
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

#[cfg(test)]
mod tests {
    use super::{PointU16, delta_col, delta_row};

    #[test]
    fn point_u16() {
        assert_eq!(PointU16::of(0, u16::MAX), PointU16 { row: 0, col: u16::MAX });
        assert_eq!(PointU16::of(0, u16::MAX).to_string(), "(0, 65535)");
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
}

#[derive(PartialEq, Debug, Clone)]
pub struct PointI64 {
    pub x: i64,
    pub y: i64,
}

impl PointI64 {
    fn of(x: i64, y: i64) -> Self {
        PointI64 { x, y }
    }
}

impl std::fmt::Display for PointI64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointI64, p2: &PointI64) -> u64 {
    (i128::from(p2.x) - i128::from(p1.x)).unsigned_abs() as u64
}

pub fn delta_y(p1: &PointI64, p2: &PointI64) -> u64 {
    (i128::from(p2.y) - i128::from(p1.y)).unsigned_abs() as u64
}

#[cfg(test)]
mod tests {
    use super::{PointI64, delta_x, delta_y};

    #[test]
    fn point_u8() {
        assert_eq!(PointI64::of(i64::MIN, i64::MAX), PointI64 { x: i64::MIN, y: i64::MAX });
        assert_eq!(
            PointI64::of(i64::MIN, i64::MAX).to_string(),
            "(-9223372036854775808, 9223372036854775807)"
        );
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointI64::of(0, i64::MIN), &PointI64::of(0, i64::MAX)), 0);
        assert_eq!(delta_x(&PointI64::of(i64::MIN, 0), &PointI64::of(i64::MAX, 0)), u64::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointI64::of(i64::MIN, 0), &PointI64::of(i64::MAX, 0)), 0);
        assert_eq!(delta_y(&PointI64::of(0, i64::MIN), &PointI64::of(0, i64::MAX)), u64::MAX);
    }
}

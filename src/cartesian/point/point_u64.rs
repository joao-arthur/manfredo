#[derive(PartialEq, Debug, Clone)]
pub struct PointU64 {
    pub x: u64,
    pub y: u64,
}

impl PointU64 {
    pub fn of(x: u64, y: u64) -> Self {
        PointU64 { x, y }
    }
}

impl std::fmt::Display for PointU64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointU64, p2: &PointU64) -> u64 {
    p2.x - p1.x
}

pub fn delta_y(p1: &PointU64, p2: &PointU64) -> u64 {
    p2.y - p1.y
}

#[cfg(test)]
mod tests {
    use super::{PointU64, delta_x, delta_y};

    #[test]
    fn point_u64() {
        assert_eq!(PointU64::of(0, u64::MAX), PointU64 { x: 0, y: u64::MAX });
        assert_eq!(PointU64::of(0, u64::MAX).to_string(), "(0, 18446744073709551615)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointU64::of(0, 0), &PointU64::of(0, u64::MAX)), 0);
        assert_eq!(delta_x(&PointU64::of(0, 0), &PointU64::of(u64::MAX, 0)), u64::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointU64::of(0, 0), &PointU64::of(u64::MAX, 0)), 0);
        assert_eq!(delta_y(&PointU64::of(0, 0), &PointU64::of(0, u64::MAX)), u64::MAX);
    }
}

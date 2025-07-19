#[derive(PartialEq, Debug, Clone)]
pub struct PointU64 {
    pub x: u64,
    pub y: u64,
}

impl PointU64 {
    fn of(x: u64, y: u64) -> Self {
        PointU64 { x, y }
    }
}

impl std::fmt::Display for PointU64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: PointU64, p2: PointU64) -> u64 {
    p2.x - p1.x
}

pub fn delta_y(p1: PointU64, p2: PointU64) -> u64 {
    p2.y - p1.y
}

#[cfg(test)]
mod tests {
    use super::{PointU64, delta_x, delta_y};

    #[test]
    fn point_u8() {
        assert_eq!(
            PointU64::of(0, 18_446_744_073_709_551_615),
            PointU64 { x: 0, y: 18_446_744_073_709_551_615 }
        );
        assert_eq!(
            PointU64::of(4_611_686_018_427_387_904, 9_223_372_036_854_775_808).to_string(),
            "(4611686018427387904, 9223372036854775808)"
        );
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(
            delta_x(
                PointU64::of(0, 18_446_744_073_709_551_615),
                PointU64::of(18_446_744_073_709_551_615, 18_446_744_073_709_551_615)
            ),
            18_446_744_073_709_551_615
        );
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(
            delta_y(
                PointU64::of(18_446_744_073_709_551_615, 0),
                PointU64::of(18_446_744_073_709_551_615, 18_446_744_073_709_551_615)
            ),
            18_446_744_073_709_551_615
        );
    }
}

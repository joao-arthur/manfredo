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
    }

    #[test]
    fn test_distance_x() {
        assert_eq!(
            delta_x(
                PointU64::of(0, 18_446_744_073_709_551_615),
                PointU64::of(18_446_744_073_709_551_615, 18_446_744_073_709_551_615)
            ),
            18_446_744_073_709_551_615
        );
    }

    #[test]
    fn test_distance_y() {
        assert_eq!(
            delta_y(
                PointU64::of(18_446_744_073_709_551_615, 0),
                PointU64::of(18_446_744_073_709_551_615, 18_446_744_073_709_551_615)
            ),
            18_446_744_073_709_551_615
        );
    }
}

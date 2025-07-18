#[derive(PartialEq, Debug, Clone)]
pub struct PointU64 {
    pub x: u64,
    pub y: u64,
}

impl PointU64 {
    fn from(x: u64, y: u64) -> Self {
        PointU64 { x, y }
    }
}

pub fn distance_x(p1: PointU64, p2: PointU64) -> u64 {
    p2.x - p1.x
}

pub fn distance_y(p1: PointU64, p2: PointU64) -> u64 {
    p2.y - p1.y
}

#[cfg(test)]
mod tests {
    use super::{PointU64, distance_x, distance_y};

    #[test]
    fn point_u8() {
        assert_eq!(
            PointU64::from(0, 18_446_744_073_709_551_615),
            PointU64 { x: 0, y: 18_446_744_073_709_551_615 }
        );
    }

    #[test]
    fn test_distance_x() {
        assert_eq!(
            distance_x(
                PointU64::from(0, 18_446_744_073_709_551_615),
                PointU64::from(18_446_744_073_709_551_615, 18_446_744_073_709_551_615)
            ),
            18_446_744_073_709_551_615
        );
    }

    #[test]
    fn test_distance_y() {
        assert_eq!(
            distance_y(
                PointU64::from(18_446_744_073_709_551_615, 0),
                PointU64::from(18_446_744_073_709_551_615, 18_446_744_073_709_551_615)
            ),
            18_446_744_073_709_551_615
        );
    }
}

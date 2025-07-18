#[derive(PartialEq, Debug, Clone)]
pub struct PointU16 {
    pub x: u16,
    pub y: u16,
}

impl PointU16 {
    fn from(x: u16, y: u16) -> Self {
        PointU16 { x, y }
    }
}

pub fn distance_x(p1: PointU16, p2: PointU16) -> u16 {
    p2.x - p1.x
}

pub fn distance_y(p1: PointU16, p2: PointU16) -> u16 {
    p2.y - p1.y
}

#[cfg(test)]
mod tests {
    use super::{PointU16, distance_x, distance_y};

    #[test]
    fn point_u8() {
        assert_eq!(PointU16::from(0, 65_535), PointU16 { x: 0, y: 65_535 });
    }

    #[test]
    fn test_distance_x() {
        assert_eq!(distance_x(PointU16::from(0, 65_535), PointU16::from(65_535, 65_535)), 65_535);
    }

    #[test]
    fn test_distance_y() {
        assert_eq!(distance_y(PointU16::from(65_535, 0), PointU16::from(65_535, 65_535)), 65_535);
    }
}

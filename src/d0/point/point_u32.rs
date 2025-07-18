#[derive(PartialEq, Debug, Clone)]
pub struct PointU32 {
    pub x: u32,
    pub y: u32,
}

impl PointU32 {
    fn from(x: u32, y: u32) -> Self {
        PointU32 { x, y }
    }
}

pub fn distance_x(p1: PointU32, p2: PointU32) -> u32 {
    p2.x - p1.x
}

pub fn distance_y(p1: PointU32, p2: PointU32) -> u32 {
    p2.y - p1.y
}

#[cfg(test)]
mod tests {
    use super::{PointU32, distance_x, distance_y};

    #[test]
    fn point_u8() {
        assert_eq!(PointU32::from(0, 4_294_967_295), PointU32 { x: 0, y: 4_294_967_295 });
    }

    #[test]
    fn test_distance_x() {
        assert_eq!(
            distance_x(
                PointU32::from(0, 4_294_967_295),
                PointU32::from(4_294_967_295, 4_294_967_295)
            ),
            4_294_967_295
        );
    }

    #[test]
    fn test_distance_y() {
        assert_eq!(
            distance_y(
                PointU32::from(4_294_967_295, 0),
                PointU32::from(4_294_967_295, 4_294_967_295)
            ),
            4_294_967_295
        );
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct PointU8 {
    pub x: u8,
    pub y: u8,
}

impl PointU8 {
    fn of(x: u8, y: u8) -> Self {
        PointU8 { x, y }
    }
}

impl std::fmt::Display for PointU8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointU8, p2: &PointU8) -> u8 {
    p2.x - p1.x
}

pub fn delta_y(p1: &PointU8, p2: &PointU8) -> u8 {
    p2.y - p1.y
}

#[cfg(test)]
mod tests {
    use super::{PointU8, delta_x, delta_y};

    #[test]
    fn point_u8() {
        assert_eq!(PointU8::of(0, 255), PointU8 { x: 0, y: 255 });
        assert_eq!(PointU8::of(64, 128).to_string(), "(64, 128)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointU8::of(0, 0), &PointU8::of(0, 255)), 0);
        assert_eq!(delta_x(&PointU8::of(0, 0), &PointU8::of(255, 0)), 255);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointU8::of(0, 0), &PointU8::of(255, 0)), 0);
        assert_eq!(delta_y(&PointU8::of(0, 0), &PointU8::of(0, 255)), 255);
    }
}

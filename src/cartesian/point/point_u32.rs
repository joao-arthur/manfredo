#[derive(PartialEq, Debug, Clone)]
pub struct PointU32 {
    pub x: u32,
    pub y: u32,
}

impl PointU32 {
    fn of(x: u32, y: u32) -> Self {
        PointU32 { x, y }
    }
}

impl std::fmt::Display for PointU32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointU32, p2: &PointU32) -> u32 {
    p2.x - p1.x
}

pub fn delta_y(p1: &PointU32, p2: &PointU32) -> u32 {
    p2.y - p1.y
}

#[cfg(test)]
mod tests {
    use super::{PointU32, delta_x, delta_y};

    #[test]
    fn point_u8() {
        assert_eq!(PointU32::of(0, u32::MAX), PointU32 { x: 0, y: u32::MAX });
        assert_eq!(PointU32::of(0, u32::MAX).to_string(), "(0, 4294967295)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointU32::of(0, 0), &PointU32::of(0, u32::MAX)), 0);
        assert_eq!(delta_x(&PointU32::of(0, 0), &PointU32::of(u32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointU32::of(0, 0), &PointU32::of(u32::MAX, 0)), 0);
        assert_eq!(delta_y(&PointU32::of(0, 0), &PointU32::of(0, u32::MAX)), u32::MAX);
    }
}

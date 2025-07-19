#[derive(PartialEq, Debug, Clone)]
pub struct PointI16 {
    pub x: i16,
    pub y: i16,
}

impl PointI16 {
    fn of(x: i16, y: i16) -> Self {
        PointI16 { x, y }
    }
}

impl std::fmt::Display for PointI16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointI16, p2: &PointI16) -> u16 {
    (i32::from(p2.x) - i32::from(p1.x)).unsigned_abs() as u16
}

pub fn delta_y(p1: &PointI16, p2: &PointI16) -> u16 {
    (i32::from(p2.y) - i32::from(p1.y)).unsigned_abs() as u16
}

#[cfg(test)]
mod tests {
    use super::{PointI16, delta_x, delta_y};

    #[test]
    fn point_u8() {
        assert_eq!(PointI16::of(-32_768, 32_767), PointI16 { x: -32_768, y: 32_767 });
        assert_eq!(PointI16::of(-32_768, 32_767).to_string(), "(-32768, 32767)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointI16::of(0, -32_768), &PointI16::of(0, 32_767)), 0);
        assert_eq!(delta_x(&PointI16::of(-32_768, 0), &PointI16::of(32_767, 0)), 65_535);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointI16::of(-32_768, 0), &PointI16::of(32_767, 0)), 0);
        assert_eq!(delta_y(&PointI16::of(0, -32_768), &PointI16::of(0, 32_767)), 65_535);
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct PointI8 {
    pub x: i8,
    pub y: i8,
}

impl PointI8 {
    fn of(x: i8, y: i8) -> Self {
        PointI8 { x, y }
    }
}

impl std::fmt::Display for PointI8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointI8, p2: &PointI8) -> u8 {
    (i16::from(p2.x) - i16::from(p1.x)).unsigned_abs() as u8
}

pub fn delta_y(p1: &PointI8, p2: &PointI8) -> u8 {
    (i16::from(p2.y) - i16::from(p1.y)).unsigned_abs() as u8
}

#[cfg(test)]
mod tests {
    use super::{PointI8, delta_x, delta_y};

    #[test]
    fn point_u8() {
        assert_eq!(PointI8::of(i8::MIN, i8::MAX), PointI8 { x: i8::MIN, y: i8::MAX });
        assert_eq!(PointI8::of(i8::MIN, i8::MAX).to_string(), "(-128, 127)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointI8::of(0, i8::MIN), &PointI8::of(0, i8::MAX)), 0);
        assert_eq!(delta_x(&PointI8::of(i8::MIN, 0), &PointI8::of(i8::MAX, 0)), u8::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointI8::of(i8::MIN, 0), &PointI8::of(i8::MAX, 0)), 0);
        assert_eq!(delta_y(&PointI8::of(0, i8::MIN), &PointI8::of(0, i8::MAX)), u8::MAX);
    }
}

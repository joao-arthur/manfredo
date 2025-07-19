#[derive(PartialEq, Debug, Clone)]
pub struct PointI16 {
    pub x: i16,
    pub y: i16,
}

impl PointI16 {
    pub fn of(x: i16, y: i16) -> Self {
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
    fn point_i16() {
        assert_eq!(PointI16::of(i16::MIN, i16::MAX), PointI16 { x: i16::MIN, y: i16::MAX });
        assert_eq!(PointI16::of(i16::MIN, i16::MAX).to_string(), "(-32768, 32767)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointI16::of(0, i16::MIN), &PointI16::of(0, i16::MAX)), 0);
        assert_eq!(delta_x(&PointI16::of(i16::MIN, 0), &PointI16::of(i16::MAX, 0)), u16::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointI16::of(i16::MIN, 0), &PointI16::of(i16::MAX, 0)), 0);
        assert_eq!(delta_y(&PointI16::of(0, i16::MIN), &PointI16::of(0, i16::MAX)), u16::MAX);
    }
}

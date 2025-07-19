#[derive(PartialEq, Debug, Clone)]
pub struct PointF32 {
    pub x: f32,
    pub y: f32,
}

impl PointF32 {
    pub fn of(x: f32, y: f32) -> Self {
        PointF32 { x, y }
    }
}

impl std::fmt::Display for PointF32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointF32, p2: &PointF32) -> f32 {
    p2.x - p1.x
}

pub fn delta_y(p1: &PointF32, p2: &PointF32) -> f32 {
    p2.y - p1.y
}

#[cfg(test)]
mod tests {
    use super::{PointF32, delta_x, delta_y};

    #[test]
    fn point_f32() {
        assert_eq!(PointF32::of(f32::MIN, f32::MAX), PointF32 { x: f32::MIN, y: f32::MAX });
        assert_eq!(
            PointF32::of(f32::MIN, f32::MAX).to_string(),
            "(-340282350000000000000000000000000000000, 340282350000000000000000000000000000000)"
        );
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointF32::of(0.0, 0.0), &PointF32::of(0.0, 4294967295.0)), 0.0);
        assert_eq!(delta_x(&PointF32::of(0.0, 0.0), &PointF32::of(4294967295.0, 0.0)), 4294967295.0);
        assert_eq!(delta_x(&PointF32::of(-2147483648.0, 0.0), &PointF32::of(2147483647.0, 0.0)), 4294967295.0);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointF32::of(0.0, 0.0), &PointF32::of(4294967295.0, 0.0)), 0.0);
        assert_eq!(delta_y(&PointF32::of(0.0, 0.0), &PointF32::of(0.0, 4294967295.0)), 4294967295.0);
        assert_eq!(delta_y(&PointF32::of(0.0, -2147483648.0), &PointF32::of(0.0, 2147483647.0)), 4294967295.0);
    }
}

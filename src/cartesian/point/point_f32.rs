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

const ART_MIN: f32 = -16_777_216.0;
const ART_MAX: f32 = 16_777_215.0;

pub fn delta_x(p1: &PointF32, p2: &PointF32) -> f32 {
    p2.x - p1.x
}

pub fn delta_y(p1: &PointF32, p2: &PointF32) -> f32 {
    p2.y - p1.y
}

#[cfg(test)]
mod tests {
    use super::{ART_MAX, ART_MIN, PointF32, delta_x, delta_y};

    #[test]
    fn point_f32() {
        assert_eq!(PointF32::of(ART_MIN, ART_MAX), PointF32 { x: ART_MIN, y: ART_MAX });
        assert_eq!(PointF32::of(ART_MIN, ART_MAX).to_string(), "(-16777216, 16777215)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointF32::of(0.0, 0.0), &PointF32::of(0.0, ART_MAX)), 0.0);
        assert_eq!(delta_x(&PointF32::of(0.0, -8_388_608.0), &PointF32::of(0.0, 8_388_607.0)), 0.0);
        assert_eq!(delta_x(&PointF32::of(0.0, 0.0), &PointF32::of(ART_MAX, 0.0)), ART_MAX);
        assert_eq!(delta_x(&PointF32::of(-8_388_608.0, 0.0), &PointF32::of(8_388_607.0, 0.0)), ART_MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointF32::of(0.0, 0.0), &PointF32::of(ART_MAX, 0.0)), 0.0);
        assert_eq!(delta_y(&PointF32::of(0.0, 0.0), &PointF32::of(0.0, ART_MAX)), ART_MAX);
        assert_eq!(delta_y(&PointF32::of(0.0, -8_388_608.0), &PointF32::of(0.0, 8_388_607.0)), ART_MAX);
    }
}

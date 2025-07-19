use crate::cartesian::point::point_f32;

#[derive(PartialEq, Debug, Clone)]
pub struct RectF32 {
    pub min: point_f32::PointF32,
    pub max: point_f32::PointF32,
}

impl RectF32 {
    pub fn of(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        RectF32 {
            min: point_f32::PointF32 { x: x1, y: y1 },
            max: point_f32::PointF32 { x: x2, y: y2 },
        }
    }
}

impl std::fmt::Display for RectF32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min.to_string(), self.max.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_f32::PointF32;

    use super::RectF32;

    #[test]
    fn rect_f32() {
        assert_eq!(
            RectF32::of(-2147483648.0, -1.0, 1.0, 2147483647.0),
            RectF32 {
                min: PointF32 { x: -2147483648.0, y: -1.0 },
                max: PointF32 { x: 1.0, y: 2147483647.0 }
            }
        );
        assert_eq!(
            RectF32::of(-2147483648.0, -0.0, 0.0, 2147483647.0).to_string(),
            "((-2147483600, -0), (0, 2147483600))"
        );
    }
}

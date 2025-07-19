use crate::cartesian::point::point_f64;

#[derive(PartialEq, Debug, Clone)]
pub struct RectF64 {
    pub min: point_f64::PointF64,
    pub max: point_f64::PointF64,
}

impl RectF64 {
    pub fn of(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        RectF64 {
            min: point_f64::PointF64 { x: x1, y: y1 },
            max: point_f64::PointF64 { x: x2, y: y2 },
        }
    }
}

impl std::fmt::Display for RectF64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min.to_string(), self.max.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_f64::PointF64;

    use super::RectF64;

    #[test]
    fn rect_f64() {
        assert_eq!(
            RectF64::of(-9223372036854775808.0, -1.0, 1.0, 9223372036854775807.0),
            RectF64 {
                min: PointF64 { x: -9223372036854775808.0, y: -1.0 },
                max: PointF64 { x: 1.0, y: 9223372036854775807.0 }
            }
        );
        assert_eq!(
            RectF64::of(-9223372036854775808.0, -0.0, 0.0, 9223372036854775807.0).to_string(),
            "((-9223372036854776000, -0), (0, 9223372036854776000))"
        );
    }
}

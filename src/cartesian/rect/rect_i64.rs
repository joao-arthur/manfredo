use crate::cartesian::point::point_i64;

#[derive(PartialEq, Debug, Clone)]
pub struct RectI64 {
    pub min: point_i64::PointI64,
    pub max: point_i64::PointI64,
}

impl RectI64 {
    pub fn of(x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
        RectI64 { min: point_i64::PointI64::of(x1, y1), max: point_i64::PointI64::of(x2, y2) }
    }
}

impl std::fmt::Display for RectI64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min.to_string(), self.max.to_string())
    }
}

pub fn delta_x(r: &RectI64) -> u64 {
    point_i64::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectI64) -> u64 {
    point_i64::delta_y(&r.min, &r.max)
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_i64::PointI64;

    use super::{RectI64, delta_x, delta_y};

    #[test]
    fn rect_i64() {
        assert_eq!(
            RectI64::of(i64::MIN, -1, 1, i64::MAX),
            RectI64 { min: PointI64 { x: i64::MIN, y: -1 }, max: PointI64 { x: 1, y: i64::MAX } }
        );
        assert_eq!(
            RectI64::of(i64::MIN, -0, 0, i64::MAX).to_string(),
            "((-9223372036854775808, 0), (0, 9223372036854775807))"
        );
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectI64::of(0, i64::MIN, 0, i64::MAX)), 0);
        assert_eq!(delta_x(&RectI64::of(i64::MIN, 0, i64::MAX, 0)), u64::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectI64::of(i64::MIN, 0, i64::MAX, 0)), 0);
        assert_eq!(delta_y(&RectI64::of(0, i64::MIN, 0, i64::MAX)), u64::MAX);
    }
}

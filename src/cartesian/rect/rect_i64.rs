use crate::cartesian::point::point_i64;

#[derive(PartialEq, Debug, Clone)]
pub struct RectI64 {
    pub min: point_i64::PointI64,
    pub max: point_i64::PointI64,
}

impl RectI64 {
    pub fn of(x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
        RectI64 {
            min: point_i64::PointI64 { x: x1, y: y1 },
            max: point_i64::PointI64 { x: x2, y: y2 },
        }
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
            RectI64::of(0, 4, 64, 255),
            RectI64 { min: PointI64 { x: 0, y: 4 }, max: PointI64 { x: 64, y: 255 } }
        );
        assert_eq!(RectI64::of(0, 64, 2048, 65536).to_string(), "((0, 64), (2048, 65536))")
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(
            delta_x(&RectI64::of(0, -9_223_372_036_854_775_808, 0, 9_223_372_036_854_775_807)),
            0
        );
        assert_eq!(
            delta_x(&RectI64::of(-9_223_372_036_854_775_808, 0, 9_223_372_036_854_775_807, 0)),
            18_446_744_073_709_551_615
        );
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(
            delta_y(&RectI64::of(-9_223_372_036_854_775_808, 0, 9_223_372_036_854_775_807, 0)),
            0
        );
        assert_eq!(
            delta_y(&RectI64::of(0, -9_223_372_036_854_775_808, 0, 9_223_372_036_854_775_807)),
            18_446_744_073_709_551_615
        );
    }
}

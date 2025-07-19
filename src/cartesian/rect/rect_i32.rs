use crate::cartesian::point::point_i32;

#[derive(PartialEq, Debug, Clone)]
pub struct RectI32 {
    pub min: point_i32::PointI32,
    pub max: point_i32::PointI32,
}

impl RectI32 {
    pub fn of(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        RectI32 {
            min: point_i32::PointI32 { x: x1, y: y1 },
            max: point_i32::PointI32 { x: x2, y: y2 },
        }
    }
}

impl std::fmt::Display for RectI32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min.to_string(), self.max.to_string())
    }
}

pub fn delta_x(r: &RectI32) -> u32 {
    point_i32::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectI32) -> u32 {
    point_i32::delta_y(&r.min, &r.max)
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_i32::PointI32;

    use super::{RectI32, delta_x, delta_y};

    #[test]
    fn rect_i32() {
        assert_eq!(
            RectI32::of(i32::MIN, -1, 1, i32::MAX),
            RectI32 { min: PointI32 { x: i32::MIN, y: -1 }, max: PointI32 { x: 1, y: i32::MAX } }
        );
        assert_eq!(
            RectI32::of(i32::MIN, -0, 0, i32::MAX).to_string(),
            "((-2147483648, 0), (0, 2147483647))"
        );
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectI32::of(0, i32::MIN, 0, i32::MAX)), 0);
        assert_eq!(delta_x(&RectI32::of(i32::MIN, 0, i32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectI32::of(i32::MIN, 0, i32::MAX, 0)), 0);
        assert_eq!(delta_y(&RectI32::of(0, i32::MIN, 0, i32::MAX)), u32::MAX);
    }
}

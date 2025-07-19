use crate::cartesian::point::point_i8;

#[derive(PartialEq, Debug, Clone)]
pub struct RectI8 {
    pub min: point_i8::PointI8,
    pub max: point_i8::PointI8,
}

impl RectI8 {
    pub fn of(x1: i8, y1: i8, x2: i8, y2: i8) -> Self {
        RectI8 { min: point_i8::PointI8::of(x1, y1), max: point_i8::PointI8::of(x2, y2) }
    }
}

impl std::fmt::Display for RectI8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min.to_string(), self.max.to_string())
    }
}

pub fn delta_x(r: &RectI8) -> u8 {
    point_i8::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectI8) -> u8 {
    point_i8::delta_y(&r.min, &r.max)
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_i8::PointI8;

    use super::{RectI8, delta_x, delta_y};

    #[test]
    fn rect_i8() {
        assert_eq!(
            RectI8::of(i8::MIN, -1, 1, i8::MAX),
            RectI8 { min: PointI8 { x: i8::MIN, y: -1 }, max: PointI8 { x: 1, y: i8::MAX } }
        );
        assert_eq!(RectI8::of(i8::MIN, -0, 0, i8::MAX).to_string(), "((-128, 0), (0, 127))");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectI8::of(0, i8::MIN, 0, i8::MAX)), 0);
        assert_eq!(delta_x(&RectI8::of(i8::MIN, 0, i8::MAX, 0)), u8::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectI8::of(i8::MIN, 0, i8::MAX, 0)), 0);
        assert_eq!(delta_y(&RectI8::of(0, i8::MIN, 0, i8::MAX)), u8::MAX);
    }
}

use crate::cartesian::point::point_i8;

#[derive(PartialEq, Debug, Clone)]
pub struct RectI8 {
    pub min: point_i8::PointI8,
    pub max: point_i8::PointI8,
}

impl RectI8 {
    pub fn of(x1: i8, y1: i8, x2: i8, y2: i8) -> Self {
        RectI8 { min: point_i8::PointI8 { x: x1, y: y1 }, max: point_i8::PointI8 { x: x2, y: y2 } }
    }
}

pub fn delta_x(r: &RectI8) -> u8 {
    point_i8::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectI8) -> u8 {
    point_i8::delta_y(&r.min, &r.max)
}

impl std::fmt::Display for RectI8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min.to_string(), self.max.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_i8::PointI8;

    use super::{RectI8, delta_x, delta_y};

    #[test]
    fn rect_i8() {
        assert_eq!(
            RectI8::of(-128, -0, 0, 127),
            RectI8 { min: PointI8 { x: -128, y: 0 }, max: PointI8 { x: 0, y: 127 } }
        );
        assert_eq!(RectI8::of(-128, -0, 0, 127).to_string(), "((-128, 0), (0, 127))")
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectI8::of(0, -128, 0, 127)), 0);
        assert_eq!(delta_x(&RectI8::of(-128, 0, 127, 0)), 255);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectI8::of(-128, 0, 127, 0)), 0);
        assert_eq!(delta_y(&RectI8::of(0, -128, 0, 127)), 255);
    }
}

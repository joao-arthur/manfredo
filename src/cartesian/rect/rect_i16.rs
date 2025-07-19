use crate::cartesian::point::point_i16;

#[derive(PartialEq, Debug, Clone)]
pub struct RectI16 {
    pub min: point_i16::PointI16,
    pub max: point_i16::PointI16,
}

impl RectI16 {
    pub fn of(x1: i16, y1: i16, x2: i16, y2: i16) -> Self {
        RectI16 { min: point_i16::PointI16::of(x1, y1), max: point_i16::PointI16::of(x2, y2) }
    }
}

impl std::fmt::Display for RectI16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min.to_string(), self.max.to_string())
    }
}

pub fn delta_x(r: &RectI16) -> u16 {
    point_i16::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectI16) -> u16 {
    point_i16::delta_y(&r.min, &r.max)
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_i16::PointI16;

    use super::{RectI16, delta_x, delta_y};

    #[test]
    fn rect_i16() {
        assert_eq!(
            RectI16::of(i16::MIN, -1, 1, i16::MAX),
            RectI16 { min: PointI16 { x: i16::MIN, y: -1 }, max: PointI16 { x: 1, y: i16::MAX } }
        );
        assert_eq!(RectI16::of(i16::MIN, -0, 0, i16::MAX).to_string(), "((-32768, 0), (0, 32767))");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectI16::of(0, i16::MIN, 0, i16::MAX)), 0);
        assert_eq!(delta_x(&RectI16::of(i16::MIN, 0, i16::MAX, 0)), u16::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectI16::of(i16::MIN, 0, i16::MAX, 0)), 0);
        assert_eq!(delta_y(&RectI16::of(0, i16::MIN, 0, i16::MAX)), u16::MAX);
    }
}

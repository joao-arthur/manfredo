use crate::cartesian::point::point_i16;

#[derive(PartialEq, Debug, Clone)]
pub struct RectI16 {
    pub min: point_i16::PointI16,
    pub max: point_i16::PointI16,
}

impl RectI16 {
    pub fn of(x1: i16, y1: i16, x2: i16, y2: i16) -> Self {
        RectI16 {
            min: point_i16::PointI16 { x: x1, y: y1 },
            max: point_i16::PointI16 { x: x2, y: y2 },
        }
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
            RectI16::of(0, 4, 64, 255),
            RectI16 { min: PointI16 { x: 0, y: 4 }, max: PointI16 { x: 64, y: 255 } }
        );
        assert_eq!(RectI16::of(0, 4, 64, 255).to_string(), "((0, 4), (64, 255))")
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectI16::of(0, -32_768, 0, 32_767)), 0);
        assert_eq!(delta_x(&RectI16::of(-32_768, 0, 32_767, 0)), 65_535);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectI16::of(-32_768, 0, 32_767, 0)), 0);
        assert_eq!(delta_y(&RectI16::of(0, -32_768, 0, 32_767)), 65_535);
    }
}

use crate::d0::point::point_u8::PointU8;

#[derive(PartialEq, Debug, Clone)]
pub struct RectU8 {
    pub min: PointU8,
    pub max: PointU8,
}

impl RectU8 {
    pub fn of(x1: u8, y1: u8, x2: u8, y2: u8) -> Self {
        RectU8 { min: PointU8 { x: x1, y: y1 }, max: PointU8 { x: x2, y: y2 } }
    }
}

pub fn delta_x(r: &RectU8) -> u8 {
    r.max.x - r.min.x
}

pub fn delta_y(r: &RectU8) -> u8 {
    r.max.y - r.min.y
}

impl std::fmt::Display for RectU8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min.to_string(), self.max.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::d0::point::point_u8::PointU8;

    use super::{RectU8, delta_x, delta_y};

    #[test]
    fn point_u8() {
        assert_eq!(
            RectU8::of(0, 4, 64, 255),
            RectU8 { min: PointU8 { x: 0, y: 4 }, max: PointU8 { x: 64, y: 255 } }
        );
        assert_eq!(RectU8::of(0, 4, 64, 255).to_string(), "((0, 4), (64, 255))")
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectU8::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_x(&RectU8::of(0, 0, 255, 0)), 255);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectU8::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_y(&RectU8::of(0, 0, 0, 255)), 255);
    }
}

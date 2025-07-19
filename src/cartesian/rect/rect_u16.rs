use crate::cartesian::point::point_u16;

#[derive(PartialEq, Debug, Clone)]
pub struct RectU16 {
    pub min: point_u16::PointU16,
    pub max: point_u16::PointU16,
}

impl RectU16 {
    pub fn of(x1: u16, y1: u16, x2: u16, y2: u16) -> Self {
        RectU16 {
            min: point_u16::PointU16 { x: x1, y: y1 },
            max: point_u16::PointU16 { x: x2, y: y2 },
        }
    }
}

impl std::fmt::Display for RectU16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min.to_string(), self.max.to_string())
    }
}

pub fn delta_x(r: &RectU16) -> u16 {
    point_u16::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectU16) -> u16 {
    point_u16::delta_y(&r.min, &r.max)
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_u16::PointU16;

    use super::{RectU16, delta_x, delta_y};

    #[test]
    fn rect_u16() {
        assert_eq!(
            RectU16::of(16, 32, 64, 128),
            RectU16 { min: PointU16 { x: 16, y: 32 }, max: PointU16 { x: 64, y: 128 } }
        );
        assert_eq!(RectU16::of(u16::MAX, 0, 0, u16::MAX).to_string(), "((65535, 0), (0, 65535))");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectU16::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_x(&RectU16::of(0, 0, u16::MAX, 0)), u16::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectU16::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_y(&RectU16::of(0, 0, 0, u16::MAX)), u16::MAX);
    }
}

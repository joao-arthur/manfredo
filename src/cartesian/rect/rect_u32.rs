use crate::cartesian::point::point_u32;

#[derive(PartialEq, Debug, Clone)]
pub struct RectU32 {
    pub min: point_u32::PointU32,
    pub max: point_u32::PointU32,
}

impl RectU32 {
    pub fn of(x1: u32, y1: u32, x2: u32, y2: u32) -> Self {
        RectU32 {
            min: point_u32::PointU32 { x: x1, y: y1 },
            max: point_u32::PointU32 { x: x2, y: y2 },
        }
    }
}

pub fn delta_x(r: &RectU32) -> u32 {
    point_u32::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectU32) -> u32 {
    point_u32::delta_y(&r.min, &r.max)
}

impl std::fmt::Display for RectU32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min.to_string(), self.max.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_u32::PointU32;

    use super::{RectU32, delta_x, delta_y};

    #[test]
    fn rect_u32() {
        assert_eq!(
            RectU32::of(0, 4, 64, 255),
            RectU32 { min: PointU32 { x: 0, y: 4 }, max: PointU32 { x: 64, y: 255 } }
        );
        assert_eq!(RectU32::of(0, 8, 128, 1024).to_string(), "((0, 8), (128, 1024))")
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectU32::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_x(&RectU32::of(0, 0, 4_294_967_295, 0)), 4_294_967_295);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectU32::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_y(&RectU32::of(0, 0, 0, 4_294_967_295)), 4_294_967_295);
    }
}

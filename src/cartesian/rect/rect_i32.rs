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

pub fn delta_x(r: &RectI32) -> u32 {
    point_i32::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectI32) -> u32 {
    point_i32::delta_y(&r.min, &r.max)
}

impl std::fmt::Display for RectI32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min.to_string(), self.max.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_i32::PointI32;

    use super::{RectI32, delta_x, delta_y};

    #[test]
    fn rect_i32() {
        assert_eq!(
            RectI32::of(0, 4, 64, 255),
            RectI32 { min: PointI32 { x: 0, y: 4 }, max: PointI32 { x: 64, y: 255 } }
        );
        assert_eq!(RectI32::of(0, 8, 128, 1024).to_string(), "((0, 8), (128, 1024))")
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectI32::of(0, -2_147_483_648, 0, 2_147_483_647)), 0);
        assert_eq!(delta_x(&RectI32::of(-2_147_483_648, 0, 2_147_483_647, 0)), 4_294_967_295);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectI32::of(-2_147_483_648, 0, 2_147_483_647, 0)), 0);
        assert_eq!(delta_y(&RectI32::of(0, -2_147_483_648, 0, 2_147_483_647)), 4_294_967_295);
    }
}

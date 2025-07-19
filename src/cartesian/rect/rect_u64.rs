use crate::cartesian::point::point_u64;

#[derive(PartialEq, Debug, Clone)]
pub struct RectU64 {
    pub min: point_u64::PointU64,
    pub max: point_u64::PointU64,
}

impl RectU64 {
    pub fn of(x1: u64, y1: u64, x2: u64, y2: u64) -> Self {
        RectU64 {
            min: point_u64::PointU64 { x: x1, y: y1 },
            max: point_u64::PointU64 { x: x2, y: y2 },
        }
    }
}

impl std::fmt::Display for RectU64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min.to_string(), self.max.to_string())
    }
}

pub fn delta_x(r: &RectU64) -> u64 {
    point_u64::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectU64) -> u64 {
    point_u64::delta_y(&r.min, &r.max)
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_u64::PointU64;

    use super::{RectU64, delta_x, delta_y};

    #[test]
    fn rect_u64() {
        assert_eq!(
            RectU64::of(4096, 8192, 16384, 32768),
            RectU64 { min: PointU64 { x: 4096, y: 8192 }, max: PointU64 { x: 16384, y: 32768 } }
        );
        assert_eq!(
            RectU64::of(u64::MAX, 0, 0, u64::MAX).to_string(),
            "((18446744073709551615, 0), (0, 18446744073709551615))"
        );
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectU64::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_x(&RectU64::of(0, 0, u64::MAX, 0)), u64::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectU64::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_y(&RectU64::of(0, 0, 0, u64::MAX)), u64::MAX);
    }
}

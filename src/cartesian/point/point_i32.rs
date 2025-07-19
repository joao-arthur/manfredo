#[derive(PartialEq, Debug, Clone)]
pub struct PointI32 {
    pub x: i32,
    pub y: i32,
}

impl PointI32 {
    fn of(x: i32, y: i32) -> Self {
        PointI32 { x, y }
    }
}

impl std::fmt::Display for PointI32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: PointI32, p2: PointI32) -> u32 {
    (i64::from(p2.x) - i64::from(p1.x)).unsigned_abs() as u32
}

pub fn delta_y(p1: PointI32, p2: PointI32) -> u32 {
    (i64::from(p2.y) - i64::from(p1.y)).unsigned_abs() as u32
}

#[cfg(test)]
mod tests {
    use super::{PointI32, delta_x, delta_y};

    #[test]
    fn point_u8() {
        assert_eq!(
            PointI32::of(-2_147_483_648, 2_147_483_647),
            PointI32 { x: -2_147_483_648, y: 2_147_483_647 }
        );
        assert_eq!(
            PointI32::of(-2_147_483_648, 2_147_483_647).to_string(),
            "(-2147483648, 2147483647)"
        );
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(
            delta_x(PointI32::of(-2_147_483_648, 0), PointI32::of(2_147_483_647, 0)),
            4_294_967_295
        );
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(
            delta_y(PointI32::of(0, -2_147_483_648), PointI32::of(0, 2_147_483_647)),
            4_294_967_295
        );
    }
}

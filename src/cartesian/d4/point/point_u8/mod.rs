use crate::cartesian::d1::point::point_u8::MAX;

mod delta;

pub use self::delta::{delta, delta_max, delta_min, delta_w, delta_x, delta_y, delta_z};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub x: u8,
    pub y: u8,
    pub z: u8,
    pub w: u8,
}

impl Point {
    pub fn of(x: u8, y: u8, z: u8, w: u8) -> Self {
        Point { x, y, z, w }
    }

    pub fn min() -> Self {
        Point { x: 0, y: 0, z: 0, w: 0 }
    }

    pub fn max() -> Self {
        Point { x: MAX, y: MAX, z: MAX, w: MAX }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::cartesian::d1::point::point_u8::MAX;

    #[test]
    fn point() {
        assert_eq!(Point::of(10, 20, 30, 40), Point { x: 10, y: 20, z: 30, w: 40 });
        assert_eq!(Point::min(), Point { x: 0, y: 0, z: 0, w: 0 });
        assert_eq!(Point::max(), Point { x: MAX, y: MAX, z: MAX, w: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(10, 20, 30, 40).to_string(), "(10, 20, 30, 40)");
        assert_eq!(Point::min().to_string(), "(0, 0, 0, 0)");
        assert_eq!(Point::max().to_string(), "(255, 255, 255, 255)");
    }
}

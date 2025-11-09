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
    pub fn new(x: u8, y: u8, z: u8, w: u8) -> Self {
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
        assert_eq!(Point::new(1, 2, 3, 4), Point { x: 1, y: 2, z: 3, w: 4 });
        assert_eq!(Point::new(4, 3, 2, 1), Point { x: 4, y: 3, z: 2, w: 1 });
        assert_eq!(Point::min(), Point { x: 0, y: 0, z: 0, w: 0 });
        assert_eq!(Point::max(), Point { x: MAX, y: MAX, z: MAX, w: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::new(1, 2, 3, 4).to_string(), "(1, 2, 3, 4)");
        assert_eq!(Point::min().to_string(), "(0, 0, 0, 0)");
        assert_eq!(Point::max().to_string(), "(255, 255, 255, 255)");
    }
}

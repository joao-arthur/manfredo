use crate::cartesian::d1::point::point_i8::{MAX, MIN};

mod delta;

pub use self::delta::{delta, delta_max, delta_min, delta_w, delta_x, delta_y, delta_z};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub x: i8,
    pub y: i8,
    pub z: i8,
    pub w: i8,
}

impl Point {
    pub fn of(x: i8, y: i8, z: i8, w: i8) -> Self {
        Point { x, y, z, w }
    }

    pub fn min() -> Self {
        Point { x: MIN, y: MIN, z: MIN, w: MIN }
    }

    pub fn max() -> Self {
        Point { x: MAX, y: MAX, z: MAX, w: MAX }
    }

    pub fn zero() -> Self {
        Point { x: 0, y: 0, z: 0, w: 0 }
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
    use crate::cartesian::d1::point::point_i8::{MAX, MIN};

    #[test]
    fn point() {
        assert_eq!(Point::of(-20, -10, 10, 20), Point { x: -20, y: -10, z: 10, w: 20 });
        assert_eq!(Point::min(), Point { x: MIN, y: MIN, z: MIN, w: MIN });
        assert_eq!(Point::max(), Point { x: MAX, y: MAX, z: MAX, w: MAX });
        assert_eq!(Point::zero(), Point { x: 0, y: 0, z: 0, w: 0 });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(-20, -10, 10, 20).to_string(), "(-20, -10, 10, 20)");
        assert_eq!(Point::min().to_string(), "(-128, -128, -128, -128)");
        assert_eq!(Point::max().to_string(), "(127, 127, 127, 127)");
        assert_eq!(Point::zero().to_string(), "(0, 0, 0, 0)");
    }
}

use crate::cartesian::d1::point::point_f32::{MAX, MIN};

mod delta;

pub use self::delta::{delta, delta_max, delta_min, delta_x, delta_y, delta_z};

#[derive(PartialEq, Debug, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn of(x: f32, y: f32, z: f32) -> Self {
        Point { x, y, z }
    }

    pub fn min() -> Self {
        Point { x: MIN, y: MIN, z: MIN }
    }

    pub fn max() -> Self {
        Point { x: MAX, y: MAX, z: MAX }
    }

    pub fn zero() -> Self {
        Point { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::cartesian::d1::point::point_f32::{MAX, MIN};

    #[test]
    fn point() {
        assert_eq!(Point::of(-10.0, 0.0, 10.0), Point { x: -10.0, y: 0.0, z: 10.0 });
        assert_eq!(Point::min(), Point { x: MIN, y: MIN, z: MIN });
        assert_eq!(Point::max(), Point { x: MAX, y: MAX, z: MAX });
        assert_eq!(Point::zero(), Point { x: 0.0, y: 0.0, z: 0.0 });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(-10.0, 0.0, 10.0).to_string(), "(-10, 0, 10)");
        assert_eq!(Point::min().to_string(), "(-16777216, -16777216, -16777216)");
        assert_eq!(Point::max().to_string(), "(16777215, 16777215, 16777215)");
        assert_eq!(Point::zero().to_string(), "(0, 0, 0)");
    }
}

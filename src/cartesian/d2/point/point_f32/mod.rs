use crate::cartesian::d1::point::point_f32::{MAX, MIN};

mod add;
mod delta;
mod distance;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::{delta, delta_max, delta_min, delta_x, delta_y};
pub use self::distance::distance;

#[derive(PartialEq, Debug, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn of(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    pub fn min() -> Self {
        Point { x: MIN, y: MIN }
    }

    pub fn max() -> Self {
        Point { x: MAX, y: MAX }
    }

    pub fn zero() -> Self {
        Point { x: 0.0, y: 0.0 }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::cartesian::d1::point::point_f32::{MAX, MIN};

    #[test]
    fn point() {
        assert_eq!(Point::of(-1.0, 1.0), Point { x: -1.0, y: 1.0 });
        assert_eq!(Point::of(1.0, -1.0), Point { x: 1.0, y: -1.0 });
        assert_eq!(Point::min(), Point { x: MIN, y: MIN });
        assert_eq!(Point::max(), Point { x: MAX, y: MAX });
        assert_eq!(Point::zero(), Point { x: 0.0, y: 0.0 });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(-1.0, 1.0).to_string(), "(-1, 1)");
        assert_eq!(Point::min().to_string(), "(-16777216, -16777216)");
        assert_eq!(Point::max().to_string(), "(16777215, 16777215)");
        assert_eq!(Point::zero().to_string(), "(0, 0)");
    }
}

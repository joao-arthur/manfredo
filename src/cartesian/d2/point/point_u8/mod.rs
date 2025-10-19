use crate::cartesian::d1::point::point_u8::MAX;

mod add;
mod delta;
mod distance;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::{delta, delta_max, delta_min, delta_x, delta_y};
pub use self::distance::distance;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

impl Point {
    pub fn of(x: u8, y: u8) -> Self {
        Point { x, y }
    }

    pub fn min() -> Self {
        Point { x: 0, y: 0 }
    }

    pub fn max() -> Self {
        Point { x: MAX, y: MAX }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX, Point};

    #[test]
    fn point() {
        assert_eq!(Point::of(0, MAX), Point { x: 0, y: MAX });
        assert_eq!(Point::of(MAX, 0), Point { x: MAX, y: 0 });
        assert_eq!(Point::min(), Point { x: 0, y: 0 });
        assert_eq!(Point::max(), Point { x: MAX, y: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(0, MAX).to_string(), "(0, 255)");
        assert_eq!(Point::of(MAX, 0).to_string(), "(255, 0)");
        assert_eq!(Point::min().to_string(), "(0, 0)");
        assert_eq!(Point::max().to_string(), "(255, 255)");
    }
}

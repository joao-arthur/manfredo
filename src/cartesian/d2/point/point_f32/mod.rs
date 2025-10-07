pub const MIN: f32 = -16_777_216.0;
pub const MAX: f32 = 16_777_215.0;

mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::{delta, delta_max, delta_min, delta_x, delta_y};

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
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX, MIN, Point};

    #[test]
    fn point() {
        assert_eq!(Point::of(MIN, MAX), Point { x: MIN, y: MAX });
        assert_eq!(Point::min(), Point { x: MIN, y: MIN });
        assert_eq!(Point::max(), Point { x: MAX, y: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(MIN, MAX).to_string(), "(-16777216, 16777215)");
        assert_eq!(Point::min().to_string(), "(-16777216, -16777216)");
        assert_eq!(Point::max().to_string(), "(16777215, 16777215)");
    }
}

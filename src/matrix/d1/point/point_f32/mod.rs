pub const MIN: f32 = -16_777_216.0;
pub const MAX: f32 = 16_777_215.0;

mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::delta;

#[derive(PartialEq, Debug, Clone)]
pub struct Point {
    pub i: f32,
}

impl Point {
    pub fn of(i: f32) -> Self {
        Point { i }
    }

    pub fn min() -> Self {
        Point { i: MIN }
    }

    pub fn max() -> Self {
        Point { i: MAX }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.i)
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX, MIN, Point};

    #[test]
    fn point() {
        assert_eq!(Point::of(-10.0), Point { i: -10.0 });
        assert_eq!(Point::of(10.0), Point { i: 10.0 });
        assert_eq!(Point::min(), Point { i: MIN });
        assert_eq!(Point::max(), Point { i: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(-10.0).to_string(), "(-10)");
        assert_eq!(Point::of(10.0).to_string(), "(10)");
        assert_eq!(Point::min().to_string(), "(-16777216)");
        assert_eq!(Point::max().to_string(), "(16777215)");
    }
}

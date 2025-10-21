pub const MIN: f32 = -16_777_216.0;
pub const MAX: f32 = 16_777_215.0;

mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::delta;

#[derive(PartialEq, Debug, Clone)]
pub struct Point {
    pub x: f32,
}

impl Point {
    pub fn of(x: f32) -> Self {
        Point { x }
    }

    pub fn min() -> Self {
        Point { x: MIN }
    }

    pub fn max() -> Self {
        Point { x: MAX }
    }

    pub fn zero() -> Self {
        Point { x: 0.0 }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.x)
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX, MIN, Point};

    #[test]
    fn point() {
        assert_eq!(Point::of(-10.0), Point { x: -10.0 });
        assert_eq!(Point::of(10.0), Point { x: 10.0 });
        assert_eq!(Point::min(), Point { x: MIN });
        assert_eq!(Point::max(), Point { x: MAX });
        assert_eq!(Point::zero(), Point { x: 0.0 });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(-10.0).to_string(), "(-10)");
        assert_eq!(Point::of(10.0).to_string(), "(10)");
        assert_eq!(Point::min().to_string(), "(-16777216)");
        assert_eq!(Point::max().to_string(), "(16777215)");
        assert_eq!(Point::zero().to_string(), "(0)");
    }
}

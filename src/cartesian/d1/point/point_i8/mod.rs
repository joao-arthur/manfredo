mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::delta;

const MIN: i8 = i8::MIN;
const MAX: i8 = i8::MAX;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub x: i8,
}

impl Point {
    pub fn of(x: i8) -> Self {
        Point { x }
    }

    pub fn min() -> Self {
        Point { x: MIN }
    }

    pub fn max() -> Self {
        Point { x: MAX }
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
        assert_eq!(Point::of(-10), Point { x: -10 });
        assert_eq!(Point::of(10), Point { x: 10 });
        assert_eq!(Point::min(), Point { x: MIN });
        assert_eq!(Point::max(), Point { x: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(-10).to_string(), "(-10)");
        assert_eq!(Point::of(10).to_string(), "(10)");
        assert_eq!(Point::min().to_string(), "(-128)");
        assert_eq!(Point::max().to_string(), "(127)");
    }
}

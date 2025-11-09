mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::delta;

pub const MAX: u8 = u8::MAX;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub x: u8,
}

impl Point {
    pub fn new(x: u8) -> Self {
        Point { x }
    }

    pub fn min() -> Self {
        Point { x: 0 }
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
    use super::{MAX, Point};

    #[test]
    fn point() {
        assert_eq!(Point::new(1), Point { x: 1 });
        assert_eq!(Point::new(2), Point { x: 2 });
        assert_eq!(Point::min(), Point { x: 0 });
        assert_eq!(Point::max(), Point { x: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::new(1).to_string(), "(1)");
        assert_eq!(Point::min().to_string(), "(0)");
        assert_eq!(Point::max().to_string(), "(255)");
    }
}

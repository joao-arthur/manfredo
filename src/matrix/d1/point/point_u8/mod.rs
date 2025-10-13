mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::delta;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub i: u8,
}

impl Point {
    pub fn of(i: u8) -> Self {
        Point { i }
    }

    pub fn min() -> Self {
        Point { i: 0 }
    }

    pub fn max() -> Self {
        Point { i: u8::MAX }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.i)
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn point() {
        assert_eq!(Point::of(10), Point { i: 10 });
        assert_eq!(Point::min(), Point { i: 0 });
        assert_eq!(Point::max(), Point { i: u8::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(10).to_string(), "(10)");
        assert_eq!(Point::min().to_string(), "(0)");
        assert_eq!(Point::max().to_string(), "(255)");
    }
}

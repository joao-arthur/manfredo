mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::{delta, delta_x, delta_y};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}

impl Point {
    pub fn of(x: i8, y: i8) -> Self {
        Point { x, y }
    }

    pub fn min() -> Self {
        Point { x: i8::MIN, y: i8::MIN }
    }

    pub fn max() -> Self {
        Point { x: i8::MAX, y: i8::MAX }
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

    #[test]
    fn point() {
        assert_eq!(Point::of(i8::MIN, i8::MAX), Point { x: i8::MIN, y: i8::MAX });
        assert_eq!(Point::min(), Point { x: i8::MIN, y: i8::MIN });
        assert_eq!(Point::max(), Point { x: i8::MAX, y: i8::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(i8::MIN, i8::MAX).to_string(), "(-128, 127)");
        assert_eq!(Point::min().to_string(), "(-128, -128)");
        assert_eq!(Point::max().to_string(), "(127, 127)");
    }
}

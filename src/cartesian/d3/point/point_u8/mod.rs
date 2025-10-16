pub const MAX: u8 = u8::MAX;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl Point {
    pub fn of(x: u8, y: u8, z: u8) -> Self {
        Point { x, y, z }
    }

    pub fn min() -> Self {
        Point { x: 0, y: 0, z: 0 }
    }

    pub fn max() -> Self {
        Point { x: MAX, y: MAX, z: MAX }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX, Point};

    #[test]
    fn point() {
        assert_eq!(Point::of(0, MAX, 0), Point { x: 0, y: MAX, z: 0 });
        assert_eq!(Point::of(MAX, 0, MAX), Point { x: MAX, y: 0, z: MAX });
        assert_eq!(Point::min(), Point { x: 0, y: 0, z: 0 });
        assert_eq!(Point::max(), Point { x: MAX, y: MAX, z: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(0, MAX, 0).to_string(), "(0, 255, 0)");
        assert_eq!(Point::of(MAX, 0, MAX).to_string(), "(255, 0, 255)");
        assert_eq!(Point::min().to_string(), "(0, 0, 0)");
        assert_eq!(Point::max().to_string(), "(255, 255, 255)");
    }
}

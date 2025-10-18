mod delta;

pub use self::delta::{delta, delta_max, delta_min, delta_x, delta_y, delta_z};

const MIN: i8 = i8::MIN;
const MAX: i8 = i8::MAX;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl Point {
    pub fn of(x: i8, y: i8, z: i8) -> Self {
        Point { x, y, z }
    }

    pub fn min() -> Self {
        Point { x: MIN, y: MIN, z: MIN }
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
    use super::{MAX, MIN, Point};

    #[test]
    fn point() {
        assert_eq!(Point::of(MIN, MAX, MIN), Point { x: MIN, y: MAX, z: MIN });
        assert_eq!(Point::of(MAX, MIN, MAX), Point { x: MAX, y: MIN, z: MAX });
        assert_eq!(Point::min(), Point { x: MIN, y: MIN, z: MIN });
        assert_eq!(Point::max(), Point { x: MAX, y: MAX, z: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(MIN, MAX, MIN).to_string(), "(-128, 127, -128)");
        assert_eq!(Point::of(MAX, MIN, MAX).to_string(), "(127, -128, 127)");
        assert_eq!(Point::min().to_string(), "(-128, -128, -128)");
        assert_eq!(Point::max().to_string(), "(127, 127, 127)");
    }
}

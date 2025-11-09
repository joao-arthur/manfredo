use crate::cartesian::d3::point::point_u8::Point;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Rect {
    pub min: Point,
    pub max: Point,
}

impl Rect {
    pub fn new(min: (u8, u8, u8), max: (u8, u8, u8)) -> Self {
        Rect { min: Point { x: min.0, y: min.1, z: min.2 }, max: Point { x: max.0, y: max.1, z: max.2 } }
    }

    pub fn largest() -> Self {
        Rect { min: Point::min(), max: Point::max() }
    }

    pub fn min() -> Self {
        Rect { min: Point::min(), max: Point::min() }
    }

    pub fn max() -> Self {
        Rect { min: Point::max(), max: Point::max() }
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

#[cfg(test)]
mod tests {
    use super::Rect;
    use crate::cartesian::d3::point::point_u8::Point;

    #[test]
    fn rect() {
        assert_eq!(Rect::new((0, 1, 2), (3, 4, 5)), Rect { min: Point { x: 0, y: 1, z: 2 }, max: Point { x: 3, y: 4, z: 5 } });
        assert_eq!(Rect::largest(), Rect { min: Point::min(), max: Point::max() });
        assert_eq!(Rect::min(), Rect { min: Point::min(), max: Point::min() });
        assert_eq!(Rect::max(), Rect { min: Point::max(), max: Point::max() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::new((0, 1, 2), (3, 4, 5)).to_string(), "((0, 1, 2), (3, 4, 5))");
        assert_eq!(Rect::largest().to_string(), "((0, 0, 0), (255, 255, 255))");
        assert_eq!(Rect::min().to_string(), "((0, 0, 0), (0, 0, 0))");
        assert_eq!(Rect::max().to_string(), "((255, 255, 255), (255, 255, 255))");
    }
}

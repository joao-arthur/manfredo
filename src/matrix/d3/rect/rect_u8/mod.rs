use crate::matrix::d3::point::point_u8::Point;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Rect {
    pub min: Point,
    pub max: Point,
}

impl Rect {
    pub fn of(min: Point, max: Point) -> Self {
        Rect { min, max }
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
    use crate::matrix::d3::point::point_u8::Point;

    #[test]
    fn rect() {
        assert_eq!(Rect::largest(), Rect { min: Point::min(), max: Point::max() });
        assert_eq!(Rect::min(), Rect { min: Point::min(), max: Point::min() });
        assert_eq!(Rect::max(), Rect { min: Point::max(), max: Point::max() });
        assert_eq!(Rect::of(Point::of(0, 1, 2), Point::of(3, 4, 5)), Rect { min: Point { row: 0, col: 1, depth: 2 }, max: Point { row: 3, col: 4, depth: 5 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::largest().to_string(), "((0, 0, 0), (255, 255, 255))");
        assert_eq!(Rect::min().to_string(), "((0, 0, 0), (0, 0, 0))");
        assert_eq!(Rect::max().to_string(), "((255, 255, 255), (255, 255, 255))");
        assert_eq!(Rect::of(Point::of(0, 1, 2), Point::of(3, 4, 5)).to_string(), "((0, 1, 2), (3, 4, 5))");
    }
}

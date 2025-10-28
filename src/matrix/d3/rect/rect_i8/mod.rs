use crate::matrix::d3::point::point_i8::Point;

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
    use crate::matrix::d3::point::point_i8::Point;

    #[test]
    fn rect() {
        assert_eq!(Rect::largest(), Rect { min: Point::min(), max: Point::max() });
        assert_eq!(Rect::min(), Rect { min: Point::min(), max: Point::min() });
        assert_eq!(Rect::max(), Rect { min: Point::max(), max: Point::max() });
        assert_eq!(Rect::of(Point::of(-3, -2, -1), Point::of(1, 2, 3)), Rect { min: Point { row: -3, col: -2, depth: -1 }, max: Point { row: 1, col: 2, depth: 3 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::largest().to_string(), "((-128, -128, -128), (127, 127, 127))");
        assert_eq!(Rect::min().to_string(), "((-128, -128, -128), (-128, -128, -128))");
        assert_eq!(Rect::max().to_string(), "((127, 127, 127), (127, 127, 127))");
        assert_eq!(Rect::of(Point::of(-3, -2, -1), Point::of(1, 2, 3)).to_string(), "((-3, -2, -1), (1, 2, 3))");
    }
}

use crate::cartesian::d4::point::point_i8::Point;

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
    use crate::cartesian::d4::point::point_i8::Point;

    #[test]
    fn rect() {
        assert_eq!(Rect::largest(), Rect { min: Point::min(), max: Point::max() });
        assert_eq!(Rect::min(), Rect { min: Point::min(), max: Point::min() });
        assert_eq!(Rect::max(), Rect { min: Point::max(), max: Point::max() });
        assert_eq!(Rect::of(Point::of(-4, -3, -2, -1), Point::of(1, 2, 3, 4)), Rect { min: Point { x: -4, y: -3, z: -2, w: -1 }, max: Point { x: 1, y: 2, z: 3, w: 4 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::largest().to_string(), "((-128, -128, -128, -128), (127, 127, 127, 127))");
        assert_eq!(Rect::min().to_string(), "((-128, -128, -128, -128), (-128, -128, -128, -128))");
        assert_eq!(Rect::max().to_string(), "((127, 127, 127, 127), (127, 127, 127, 127))");
        assert_eq!(Rect::of(Point::of(-4, -3, -2, -1), Point::of(1, 2, 3, 4)).to_string(), "((-4, -3, -2, -1), (1, 2, 3, 4))");
    }
}

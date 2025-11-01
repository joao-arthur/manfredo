use crate::matrix::d3::{point::point_u16::Point, rect::rect_u8};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Rect {
    pub min: Point,
    pub max: Point,
}

impl Rect {
    pub fn of(min: (u16, u16, u16), max: (u16, u16, u16)) -> Self {
        Rect { min: Point { row: min.0, col: min.1, depth: min.2 }, max: Point { row: max.0, col: max.1, depth: max.2 } }
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

impl From<rect_u8::Rect> for Rect {
    fn from(l: rect_u8::Rect) -> Self {
        Rect { min: Point::from(l.min), max: Point::from(l.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Rect;
    use crate::matrix::d3::{point::point_u16::Point, rect::rect_u8};

    #[test]
    fn rect() {
        assert_eq!(Rect::of((0, 1, 2), (3, 4, 5)), Rect { min: Point { row: 0, col: 1, depth: 2 }, max: Point { row: 3, col: 4, depth: 5 } });
        assert_eq!(Rect::largest(), Rect { min: Point::min(), max: Point::max() });
        assert_eq!(Rect::min(), Rect { min: Point::min(), max: Point::min() });
        assert_eq!(Rect::max(), Rect { min: Point::max(), max: Point::max() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::of((0, 1, 2), (3, 4, 5)).to_string(), "((0, 1, 2), (3, 4, 5))");
        assert_eq!(Rect::largest().to_string(), "((0, 0, 0), (65535, 65535, 65535))");
        assert_eq!(Rect::min().to_string(), "((0, 0, 0), (0, 0, 0))");
        assert_eq!(Rect::max().to_string(), "((65535, 65535, 65535), (65535, 65535, 65535))");
    }

    #[test]
    fn from() {
        assert_eq!(Rect::from(rect_u8::Rect::largest()), Rect { min: Point::min(), max: Point { row: u8::MAX.into(), col: u8::MAX.into(), depth: u8::MAX.into() } });
    }
}

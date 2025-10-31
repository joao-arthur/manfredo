use crate::cartesian::d4::{point::point_u16::Point, rect::rect_u8};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Rect {
    pub min: Point,
    pub max: Point,
}

impl Rect {
    pub fn of(min: (u16, u16, u16, u16), max: (u16, u16, u16, u16)) -> Self {
        Rect { min: Point { x: min.0, y: min.1, z: min.2, w: min.3 }, max: Point { x: max.0, y: max.1, z: max.2, w: max.3 } }
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
    use crate::cartesian::d4::{point::point_u16::Point, rect::rect_u8};

    #[test]
    fn rect() {
        assert_eq!(Rect::largest(), Rect { min: Point::min(), max: Point::max() });
        assert_eq!(Rect::min(), Rect { min: Point::min(), max: Point::min() });
        assert_eq!(Rect::max(), Rect { min: Point::max(), max: Point::max() });
        assert_eq!(Rect::of((0, 1, 2, 3), (4, 5, 6, 7)), Rect { min: Point { x: 0, y: 1, z: 2, w: 3 }, max: Point { x: 4, y: 5, z: 6, w: 7 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::largest().to_string(), "((0, 0, 0, 0), (65535, 65535, 65535, 65535))");
        assert_eq!(Rect::min().to_string(), "((0, 0, 0, 0), (0, 0, 0, 0))");
        assert_eq!(Rect::max().to_string(), "((65535, 65535, 65535, 65535), (65535, 65535, 65535, 65535))");
        assert_eq!(Rect::of((0, 1, 2, 3), (4, 5, 6, 7)).to_string(), "((0, 1, 2, 3), (4, 5, 6, 7))");
    }

    #[test]
    fn from() {
        assert_eq!(Rect::from(rect_u8::Rect::largest()), Rect { min: Point::min(), max: Point { x: u8::MAX.into(), y: u8::MAX.into(), z: u8::MAX.into(), w: u8::MAX.into() } });
    }
}

use crate::matrix::d3::{
    point::point_i64::Point,
    rect::{rect_i8, rect_i16, rect_i32},
};

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

impl From<rect_i8::Rect> for Rect {
    fn from(l: rect_i8::Rect) -> Self {
        Rect { min: Point::from(l.min), max: Point::from(l.max) }
    }
}

impl From<rect_i16::Rect> for Rect {
    fn from(l: rect_i16::Rect) -> Self {
        Rect { min: Point::from(l.min), max: Point::from(l.max) }
    }
}

impl From<rect_i32::Rect> for Rect {
    fn from(l: rect_i32::Rect) -> Self {
        Rect { min: Point::from(l.min), max: Point::from(l.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Rect;
    use crate::matrix::d3::{
        point::point_i64::Point,
        rect::{rect_i8, rect_i16, rect_i32},
    };

    #[test]
    fn rect() {
        assert_eq!(Rect::largest(), Rect { min: Point::min(), max: Point::max() });
        assert_eq!(Rect::min(), Rect { min: Point::min(), max: Point::min() });
        assert_eq!(Rect::max(), Rect { min: Point::max(), max: Point::max() });
        assert_eq!(Rect::of(Point::of(-3, -2, -1), Point::of(1, 2, 3)), Rect { min: Point { row: -3, col: -2, depth: -1 }, max: Point { row: 1, col: 2, depth: 3 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::largest().to_string(), "((-9223372036854775808, -9223372036854775808, -9223372036854775808), (9223372036854775807, 9223372036854775807, 9223372036854775807))");
        assert_eq!(Rect::min().to_string(), "((-9223372036854775808, -9223372036854775808, -9223372036854775808), (-9223372036854775808, -9223372036854775808, -9223372036854775808))");
        assert_eq!(Rect::max().to_string(), "((9223372036854775807, 9223372036854775807, 9223372036854775807), (9223372036854775807, 9223372036854775807, 9223372036854775807))");
        assert_eq!(Rect::of(Point::of(-3, -2, -1), Point::of(1, 2, 3)).to_string(), "((-3, -2, -1), (1, 2, 3))");
    }

    #[test]
    fn from() {
        assert_eq!(
            Rect::from(rect_i8::Rect::largest()),
            Rect { min: Point { row: i8::MIN.into(), col: i8::MIN.into(), depth: i8::MIN.into() }, max: Point { row: i8::MAX.into(), col: i8::MAX.into(), depth: i8::MAX.into() } }
        );
        assert_eq!(
            Rect::from(rect_i16::Rect::largest()),
            Rect { min: Point { row: i16::MIN.into(), col: i16::MIN.into(), depth: i16::MIN.into() }, max: Point { row: i16::MAX.into(), col: i16::MAX.into(), depth: i16::MAX.into() } }
        );
        assert_eq!(
            Rect::from(rect_i32::Rect::largest()),
            Rect { min: Point { row: i32::MIN.into(), col: i32::MIN.into(), depth: i32::MIN.into() }, max: Point { row: i32::MAX.into(), col: i32::MAX.into(), depth: i32::MAX.into() } }
        );
    }
}

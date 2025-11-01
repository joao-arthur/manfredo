use crate::cartesian::d4::{point::point_i16::Point, rect::rect_i8};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Rect {
    pub min: Point,
    pub max: Point,
}

impl Rect {
    pub fn of(min: (i16, i16, i16, i16), max: (i16, i16, i16, i16)) -> Self {
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

impl From<rect_i8::Rect> for Rect {
    fn from(l: rect_i8::Rect) -> Self {
        Rect { min: Point::from(l.min), max: Point::from(l.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Rect;
    use crate::cartesian::d4::{point::point_i16::Point, rect::rect_i8};

    #[test]
    fn rect() {
        assert_eq!(Rect::of((-4, -3, -2, -1), (1, 2, 3, 4)), Rect { min: Point { x: -4, y: -3, z: -2, w: -1 }, max: Point { x: 1, y: 2, z: 3, w: 4 } });
        assert_eq!(Rect::largest(), Rect { min: Point::min(), max: Point::max() });
        assert_eq!(Rect::min(), Rect { min: Point::min(), max: Point::min() });
        assert_eq!(Rect::max(), Rect { min: Point::max(), max: Point::max() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::of((-4, -3, -2, -1), (1, 2, 3, 4)).to_string(), "((-4, -3, -2, -1), (1, 2, 3, 4))");
        assert_eq!(Rect::largest().to_string(), "((-32768, -32768, -32768, -32768), (32767, 32767, 32767, 32767))");
        assert_eq!(Rect::min().to_string(), "((-32768, -32768, -32768, -32768), (-32768, -32768, -32768, -32768))");
        assert_eq!(Rect::max().to_string(), "((32767, 32767, 32767, 32767), (32767, 32767, 32767, 32767))");
    }

    #[test]
    fn from() {
        assert_eq!(
            Rect::from(rect_i8::Rect::largest()),
            Rect {
                min: Point { x: i8::MIN.into(), y: i8::MIN.into(), z: i8::MIN.into(), w: i8::MIN.into() },
                max: Point { x: i8::MAX.into(), y: i8::MAX.into(), z: i8::MAX.into(), w: i8::MAX.into() }
            }
        );
    }
}

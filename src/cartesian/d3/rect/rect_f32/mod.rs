use crate::cartesian::d3::point::point_f32::Point;

#[derive(PartialEq, Debug, Clone)]
pub struct Rect {
    pub min: Point,
    pub max: Point,
}

impl Rect {
    pub fn of(min: (f32, f32, f32), max: (f32, f32, f32)) -> Self {
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
    use crate::cartesian::d3::point::point_f32::Point;

    #[test]
    fn rect() {
        assert_eq!(Rect::largest(), Rect { min: Point::min(), max: Point::max() });
        assert_eq!(Rect::min(), Rect { min: Point::min(), max: Point::min() });
        assert_eq!(Rect::max(), Rect { min: Point::max(), max: Point::max() });
        assert_eq!(Rect::of((-3.0, -2.0, -1.0), (1.0, 2.0, 3.0)), Rect { min: Point { x: -3.0, y: -2.0, z: -1.0 }, max: Point { x: 1.0, y: 2.0, z: 3.0 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::largest().to_string(), "((-16777216, -16777216, -16777216), (16777215, 16777215, 16777215))");
        assert_eq!(Rect::min().to_string(), "((-16777216, -16777216, -16777216), (-16777216, -16777216, -16777216))");
        assert_eq!(Rect::max().to_string(), "((16777215, 16777215, 16777215), (16777215, 16777215, 16777215))");
        assert_eq!(Rect::of((-3.0, -2.0, -1.0), (1.0, 2.0, 3.0)).to_string(), "((-3, -2, -1), (1, 2, 3))");
    }
}

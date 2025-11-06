use crate::cartesian::d3::point::point_f32::Point;
use crate::iter::iter_f32;

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

    pub fn largest_min() -> Self {
        Rect { min: Point::min(), max: Point::of(-1.0, -1.0, -1.0) }
    }

    pub fn largest_max() -> Self {
        Rect { min: Point::zero(), max: Point::max() }
    }

    pub fn min() -> Self {
        Rect { min: Point::min(), max: Point::min() }
    }

    pub fn max() -> Self {
        Rect { min: Point::max(), max: Point::max() }
    }

    pub fn zero() -> Self {
        Rect { min: Point::zero(), max: Point::zero() }
    }

    pub fn iter_x(&self) -> iter_f32::Iter {
        iter_f32::Iter::new(self.min.x, self.max.x)
    }

    pub fn iter_y(&self) -> iter_f32::Iter {
        iter_f32::Iter::new(self.min.y, self.max.y)
    }

    pub fn iter_z(&self) -> iter_f32::Iter {
        iter_f32::Iter::new(self.min.z, self.max.z)
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
        assert_eq!(Rect::of((-3.0, -2.0, -1.0), (1.0, 2.0, 3.0)), Rect { min: Point { x: -3.0, y: -2.0, z: -1.0 }, max: Point { x: 1.0, y: 2.0, z: 3.0 } });
        assert_eq!(Rect::largest(), Rect { min: Point::min(), max: Point::max() });
        assert_eq!(Rect::min(), Rect { min: Point::min(), max: Point::min() });
        assert_eq!(Rect::max(), Rect { min: Point::max(), max: Point::max() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::of((-3.0, -2.0, -1.0), (1.0, 2.0, 3.0)).to_string(), "((-3, -2, -1), (1, 2, 3))");
        assert_eq!(Rect::largest().to_string(), "((-16777216, -16777216, -16777216), (16777215, 16777215, 16777215))");
        assert_eq!(Rect::min().to_string(), "((-16777216, -16777216, -16777216), (-16777216, -16777216, -16777216))");
        assert_eq!(Rect::max().to_string(), "((16777215, 16777215, 16777215), (16777215, 16777215, 16777215))");
    }

    #[test]
    fn iter_x() {
        assert_eq!(Rect::of((-8.0, -7.0, -6.0), (-9.0, -6.0, -5.0)).iter_x().collect::<Vec<f32>>(), []);
        assert_eq!(Rect::of((-8.0, -7.0, -6.0), (-8.0, -6.0, -5.0)).iter_x().collect::<Vec<f32>>(), [-8.0]);
        assert_eq!(Rect::of((-8.0, -7.0, -6.0), (-7.0, -6.0, -5.0)).iter_x().collect::<Vec<f32>>(), [-8.0, -7.0]);
        assert_eq!(Rect::of((-8.0, -7.0, -6.0), (-7.0, -6.0, -5.0)).iter_x().rev().collect::<Vec<f32>>(), [-7.0, -8.0]);
        assert_eq!(Rect::of((-8.0, -7.0, -6.0), (-8.0, -6.0, -5.0)).iter_x().rev().collect::<Vec<f32>>(), [-8.0]);
        assert_eq!(Rect::of((-8.0, -7.0, -6.0), (-9.0, -6.0, -5.0)).iter_x().rev().collect::<Vec<f32>>(), []);
    }

    #[test]
    fn iter_y() {
        assert_eq!(Rect::of((-8.0, -7.0, -6.0), (-7.0, -8.0, -5.0)).iter_y().collect::<Vec<f32>>(), []);
        assert_eq!(Rect::of((-8.0, -7.0, -6.0), (-7.0, -7.0, -5.0)).iter_y().collect::<Vec<f32>>(), [-7.0]);
        assert_eq!(Rect::of((-8.0, -7.0, -6.0), (-7.0, -6.0, -5.0)).iter_y().collect::<Vec<f32>>(), [-7.0, -6.0]);
        assert_eq!(Rect::of((-8.0, -7.0, -6.0), (-7.0, -6.0, -5.0)).iter_y().rev().collect::<Vec<f32>>(), [-6.0, -7.0]);
        assert_eq!(Rect::of((-8.0, -7.0, -6.0), (-7.0, -7.0, -5.0)).iter_y().rev().collect::<Vec<f32>>(), [-7.0]);
        assert_eq!(Rect::of((-8.0, -7.0, -6.0), (-7.0, -8.0, -5.0)).iter_y().rev().collect::<Vec<f32>>(), []);
    }

    #[test]
    fn iter_z() {
        assert_eq!(Rect::of((-8.0, -7.0, -6.0), (-7.0, -6.0, -7.0)).iter_z().collect::<Vec<f32>>(), []);
        assert_eq!(Rect::of((-8.0, -7.0, -6.0), (-7.0, -6.0, -6.0)).iter_z().collect::<Vec<f32>>(), [-6.0]);
        assert_eq!(Rect::of((-8.0, -7.0, -6.0), (-7.0, -6.0, -5.0)).iter_z().collect::<Vec<f32>>(), [-6.0, -5.0]);
        assert_eq!(Rect::of((-8.0, -7.0, -6.0), (-7.0, -6.0, -5.0)).iter_z().rev().collect::<Vec<f32>>(), [-5.0, -6.0]);
        assert_eq!(Rect::of((-8.0, -7.0, -6.0), (-7.0, -6.0, -6.0)).iter_z().rev().collect::<Vec<f32>>(), [-6.0]);
        assert_eq!(Rect::of((-8.0, -7.0, -6.0), (-7.0, -6.0, -7.0)).iter_z().rev().collect::<Vec<f32>>(), []);
    }
}

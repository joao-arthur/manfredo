use crate::cartesian::d2::{
    circle::{circle_i8, circle_i16},
    point::point_i32::Point,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Circle {
    pub center: Point,
    pub radius: u32,
}

impl Circle {
    pub fn of(center: Point, radius: u32) -> Self {
       Circle { center, radius }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.center, self.radius)
    }
}

impl From<circle_i8::Circle> for Circle {
    fn from(circle: circle_i8::Circle) -> Self {
        Circle { center: Point::from(circle.center), radius: u32::from(circle.radius) }
    }
}

impl From<circle_i16::Circle> for Circle {
    fn from(circle: circle_i16::Circle) -> Self {
        Circle { center: Point::from(circle.center), radius: u32::from(circle.radius) }
    }
}

#[cfg(test)]
mod tests {
    use super::Circle;
    use crate::cartesian::d2::{
        circle::{circle_i8, circle_i16},
        point::{point_i32::Point, point_i8, point_i16},
    };

    const MIN: i32 = i32::MIN;
    const MAX: i32 = i32::MAX;

    #[test]
    fn circle() {
        assert_eq!(Circle::of(Point::of(MIN, MIN), u32::MAX), Circle { center: Point { x: MIN, y: MIN }, radius: u32::MAX });
        assert_eq!(Circle::of(Point::of(MIN, MAX), u32::MAX), Circle { center: Point { x: MIN, y: MAX }, radius: u32::MAX });
        assert_eq!(Circle::of(Point::of(MAX, MIN), u32::MAX), Circle { center: Point { x: MAX, y: MIN }, radius: u32::MAX });
        assert_eq!(Circle::of(Point::of(MAX, MAX), u32::MAX), Circle { center: Point { x: MAX, y: MAX }, radius: u32::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Circle::of(Point::of(MIN, MIN), u32::MAX).to_string(), "((-2147483648, -2147483648), 4294967295)");
        assert_eq!(Circle::of(Point::of(MIN, MAX), u32::MAX).to_string(), "((-2147483648, 2147483647), 4294967295)");
        assert_eq!(Circle::of(Point::of(MAX, MIN), u32::MAX).to_string(), "((2147483647, -2147483648), 4294967295)");
        assert_eq!(Circle::of(Point::of(MAX, MAX), u32::MAX).to_string(), "((2147483647, 2147483647), 4294967295)");
    }

    #[test]
    fn from() {
        assert_eq!(
            Circle::from(circle_i8::Circle::of(point_i8::Point::of(i8::MIN, i8::MAX), u8::MAX)),
            Circle { center: Point { x: i8::MIN.into(), y: i8::MAX.into() }, radius: u8::MAX.into() }
        );
        assert_eq!(
            Circle::from(circle_i16::Circle::of(point_i16::Point::of(i16::MIN, i16::MAX), u16::MAX)),
            Circle { center: Point { x: i16::MIN.into(), y: i16::MAX.into() }, radius: u16::MAX.into() }
        );
    }
}

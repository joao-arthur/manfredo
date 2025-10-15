use crate::cartesian::d2::{
    circle::{circle_i8, circle_i16, circle_i32},
    point::point_i64::Point,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Circle {
    pub center: Point,
    pub radius: u64,
}

impl Circle {
    pub fn of(center: Point, radius: u64) -> Self {
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
        Circle { center: Point::from(circle.center), radius: u64::from(circle.radius) }
    }
}

impl From<circle_i16::Circle> for Circle {
    fn from(circle: circle_i16::Circle) -> Self {
        Circle { center: Point::from(circle.center), radius: u64::from(circle.radius) }
    }
}

impl From<circle_i32::Circle> for Circle {
    fn from(circle: circle_i32::Circle) -> Self {
        Circle { center: Point::from(circle.center), radius: u64::from(circle.radius) }
    }
}

#[cfg(test)]
mod tests {
    use super::Circle;
    use crate::cartesian::d2::{
        circle::{circle_i8, circle_i16, circle_i32},
        point::{point_i64::Point, point_i8, point_i16, point_i32},
    };

    const MIN: i64 = i64::MIN;
    const MAX: i64 = i64::MAX;

    #[test]
    fn circle() {
        assert_eq!(Circle::of(Point::of(MIN, MIN), u64::MAX), Circle { center: Point { x: MIN, y: MIN }, radius: u64::MAX });
        assert_eq!(Circle::of(Point::of(MIN, MAX), u64::MAX), Circle { center: Point { x: MIN, y: MAX }, radius: u64::MAX });
        assert_eq!(Circle::of(Point::of(MAX, MIN), u64::MAX), Circle { center: Point { x: MAX, y: MIN }, radius: u64::MAX });
        assert_eq!(Circle::of(Point::of(MAX, MAX), u64::MAX), Circle { center: Point { x: MAX, y: MAX }, radius: u64::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Circle::of(Point::of(MIN, MIN), u64::MAX).to_string(), "((-9223372036854775808, -9223372036854775808), 18446744073709551615)");
        assert_eq!(Circle::of(Point::of(MIN, MAX), u64::MAX).to_string(), "((-9223372036854775808, 9223372036854775807), 18446744073709551615)");
        assert_eq!(Circle::of(Point::of(MAX, MIN), u64::MAX).to_string(), "((9223372036854775807, -9223372036854775808), 18446744073709551615)");
        assert_eq!(Circle::of(Point::of(MAX, MAX), u64::MAX).to_string(), "((9223372036854775807, 9223372036854775807), 18446744073709551615)");
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
        assert_eq!(
            Circle::from(circle_i32::Circle::of(point_i32::Point::of(i32::MIN, i32::MAX), u32::MAX)),
            Circle { center: Point { x: i32::MIN.into(), y: i32::MAX.into() }, radius: u32::MAX.into() }
        );
    }
}

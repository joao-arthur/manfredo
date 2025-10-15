use crate::cartesian::d2::{
    circle::{circle_u8, circle_u16},
    point::point_u32::Point,
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

impl From<circle_u8::Circle> for Circle {
    fn from(circle: circle_u8::Circle) -> Self {
        Circle { center: Point::from(circle.center), radius: u32::from(circle.radius) }
    }
}

impl From<circle_u16::Circle> for Circle {
    fn from(circle: circle_u16::Circle) -> Self {
        Circle { center: Point::from(circle.center), radius: u32::from(circle.radius) }
    }
}

#[cfg(test)]
mod tests {
    use super::Circle;
    use crate::cartesian::d2::{
        circle::{circle_u8, circle_u16},
        point::{point_u32::Point, point_u8, point_u16},
    };

    const MAX: u32 = u32::MAX;

    #[test]
    fn circle() {
        assert_eq!(Circle::of(Point::of(0, 0), u32::MAX), Circle { center: Point { x: 0, y: 0 }, radius: u32::MAX });
        assert_eq!(Circle::of(Point::of(0, MAX), u32::MAX), Circle { center: Point { x: 0, y: MAX }, radius: u32::MAX });
        assert_eq!(Circle::of(Point::of(MAX, 0), u32::MAX), Circle { center: Point { x: MAX, y: 0 }, radius: u32::MAX });
        assert_eq!(Circle::of(Point::of(MAX, MAX), u32::MAX), Circle { center: Point { x: MAX, y: MAX }, radius: u32::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Circle::of(Point::of(0, 0), u32::MAX).to_string(), "((0, 0), 4294967295)");
        assert_eq!(Circle::of(Point::of(0, MAX), u32::MAX).to_string(), "((0, 4294967295), 4294967295)");
        assert_eq!(Circle::of(Point::of(MAX, 0), u32::MAX).to_string(), "((4294967295, 0), 4294967295)");
        assert_eq!(Circle::of(Point::of(MAX, MAX), u32::MAX).to_string(), "((4294967295, 4294967295), 4294967295)");
    }

    #[test]
    fn from() {
        assert_eq!(
            Circle::from(circle_u8::Circle::of(point_u8::Point::of(0, u8::MAX), u8::MAX)),
            Circle { center: Point { x: 0, y: u8::MAX.into() }, radius: u8::MAX.into() }
        );
        assert_eq!(
            Circle::from(circle_u16::Circle::of(point_u16::Point::of(0, u16::MAX), u16::MAX)),
            Circle { center: Point { x: 0, y: u16::MAX.into() }, radius: u16::MAX.into() }
        );
    }
}

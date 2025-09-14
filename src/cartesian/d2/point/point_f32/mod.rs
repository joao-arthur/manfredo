pub const MIN: f32 = -16_777_216.0;
pub const MAX: f32 = 16_777_215.0;

mod add;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};

#[derive(PartialEq, Debug, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn of(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    pub fn min() -> Self {
        Point { x: MIN, y: MIN }
    }

    pub fn max() -> Self {
        Point { x: MAX, y: MAX }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &Point, p2: &Point) -> f32 {
    p2.x - p1.x
}

pub fn delta_y(p1: &Point, p2: &Point) -> f32 {
    p2.y - p1.y
}

pub fn delta(p1: &Point, p2: &Point) -> Point {
    Point { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{MAX, MIN, Point, delta, delta_x, delta_y};

    #[test]
    fn point_f32() {
        assert_eq!(Point::of(MIN, MAX), Point { x: MIN, y: MAX });
        assert_eq!(Point::min(), Point { x: MIN, y: MIN });
        assert_eq!(Point::max(), Point { x: MAX, y: MAX });
        assert_eq!(Point::of(MIN, MAX).to_string(), "(-16777216, 16777215)");
        assert_eq!(Point::min().to_string(), "(-16777216, -16777216)");
        assert_eq!(Point::max().to_string(), "(16777215, 16777215)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Point::of(0.0, 0.0), &Point::of(0.0, MAX)), 0.0);
        assert_eq!(delta_x(&Point::of(0.0, -8_388_608.0), &Point::of(0.0, 8_388_607.0)), 0.0);
        assert_eq!(delta_x(&Point::of(0.0, 0.0), &Point::of(MAX, 0.0)), MAX);
        assert_eq!(delta_x(&Point::of(-8_388_608.0, 0.0), &Point::of(8_388_607.0, 0.0)), MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&Point::of(0.0, 0.0), &Point::of(MAX, 0.0)), 0.0);
        assert_eq!(delta_y(&Point::of(-8_388_608.0, 0.0), &Point::of(8_388_607.0, 0.0)), 0.0);
        assert_eq!(delta_y(&Point::of(0.0, 0.0), &Point::of(0.0, MAX)), MAX);
        assert_eq!(delta_y(&Point::of(0.0, -8_388_608.0), &Point::of(0.0, 8_388_607.0)), MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::of(0.0, 0.0), &Point::of(0.0, 0.0)), Point::of(0.0, 0.0));
        assert_eq!(delta(&Point::of(-8_388_608.0, -8_388_608.0), &Point::of(8_388_607.0, 8_388_607.0)), Point::of(MAX, MAX));
    }

    #[test]
    fn delta_min() {
        let p = Point::of(-8_388_608.0, -8_388_608.0);
        assert_eq!(delta(&p, &Point::of(-8_388_608.0, -8_388_608.0)), Point::of(0.0, 0.0));
        assert_eq!(delta(&p, &Point::of(-8_388_608.0, -8_388_607.0)), Point::of(0.0, 1.0));
        assert_eq!(delta(&p, &Point::of(-8_388_608.0, -8_388_606.0)), Point::of(0.0, 2.0));

        assert_eq!(delta(&p, &Point::of(-8_388_607.0, -8_388_608.0)), Point::of(1.0, 0.0));
        assert_eq!(delta(&p, &Point::of(-8_388_607.0, -8_388_607.0)), Point::of(1.0, 1.0));
        assert_eq!(delta(&p, &Point::of(-8_388_607.0, -8_388_606.0)), Point::of(1.0, 2.0));

        assert_eq!(delta(&p, &Point::of(-8_388_606.0, -8_388_608.0)), Point::of(2.0, 0.0));
        assert_eq!(delta(&p, &Point::of(-8_388_606.0, -8_388_607.0)), Point::of(2.0, 1.0));
        assert_eq!(delta(&p, &Point::of(-8_388_606.0, -8_388_606.0)), Point::of(2.0, 2.0));
    }

    #[test]
    fn delta_max() {
        let p = Point::of(8_388_605.0, 8_388_605.0);
        assert_eq!(delta(&p, &Point::of(8_388_605.0, 8_388_605.0)), Point::of(0.0, 0.0));
        assert_eq!(delta(&p, &Point::of(8_388_605.0, 8_388_606.0)), Point::of(0.0, 1.0));
        assert_eq!(delta(&p, &Point::of(8_388_605.0, 8_388_607.0)), Point::of(0.0, 2.0));

        assert_eq!(delta(&p, &Point::of(8_388_606.0, 8_388_605.0)), Point::of(1.0, 0.0));
        assert_eq!(delta(&p, &Point::of(8_388_606.0, 8_388_606.0)), Point::of(1.0, 1.0));
        assert_eq!(delta(&p, &Point::of(8_388_606.0, 8_388_607.0)), Point::of(1.0, 2.0));

        assert_eq!(delta(&p, &Point::of(8_388_607.0, 8_388_605.0)), Point::of(2.0, 0.0));
        assert_eq!(delta(&p, &Point::of(8_388_607.0, 8_388_606.0)), Point::of(2.0, 1.0));
        assert_eq!(delta(&p, &Point::of(8_388_607.0, 8_388_607.0)), Point::of(2.0, 2.0));
    }
}

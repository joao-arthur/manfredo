use crate::cartesian::d2::point::point_f32::Point;

pub fn delta_x(p1: &Point, p2: &Point) -> f32 {
    p2.x - p1.x
}

pub fn delta_y(p1: &Point, p2: &Point) -> f32 {
    p2.y - p1.y
}

pub fn delta_min(p1: &Point, p2: &Point) -> f32 {
    delta_x(p1, p2).min(delta_y(p1, p2))
}

pub fn delta_max(p1: &Point, p2: &Point) -> f32 {
    delta_x(p1, p2).max(delta_y(p1, p2))
}

pub fn delta(p1: &Point, p2: &Point) -> Point {
    Point { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{delta, delta_x, delta_y};
    use crate::cartesian::d2::point::point_f32::{MAX, Point};

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

use super::Point;

pub fn distance(p1: &Point, p2: &Point) -> f64 {
    let dx = (p1.x as f64) - (p2.x as f64);
    let dy = (p1.y as f64) - (p2.y as f64);
    (dx.mul_add(dx, dy * dy)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::distance;
    use crate::cartesian::d2::point::point_u8::Point;

    #[test]
    fn delta_0() {
        assert_eq!(distance(&Point::of(0, 0), &Point::of(0, 0)), 0.0);
        assert_eq!(distance(&Point::of(0, 1), &Point::of(0, 1)), 0.0);
        assert_eq!(distance(&Point::of(1, 0), &Point::of(1, 0)), 0.0);
        assert_eq!(distance(&Point::of(1, 1), &Point::of(1, 1)), 0.0);
    }

    #[test]
    fn delta_x() {
        assert_eq!(distance(&Point::of(10, 0), &Point::of(10, 50)), 50.0);
        assert_eq!(distance(&Point::of(10, 100), &Point::of(10, 200)), 100.0);
    }

    #[test]
    fn delta_y() {
        assert_eq!(distance(&Point::of(0, 10), &Point::of(50, 10)), 50.0);
        assert_eq!(distance(&Point::of(100, 10), &Point::of(200, 10)), 100.0);
    }

    #[test]
    fn pythagorean() {
        assert_eq!(distance(&Point::of(0, 0), &Point::of(3, 4)), 5.0);
        assert_eq!(distance(&Point::of(0, 0), &Point::of(5, 12)), 13.0);
        assert_eq!(distance(&Point::of(0, 0), &Point::of(8, 15)), 17.0);
    }

    #[test]
    fn square() {
        assert_eq!(distance(&Point::of(0, 0), &Point::of(100, 100)), 141.4213562373095);
        assert_eq!(distance(&Point::of(0, 0), &Point::of(200, 200)), 282.842712474619);
    }
}

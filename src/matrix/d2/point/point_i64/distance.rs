use super::Point;

pub fn distance(p1: &Point, p2: &Point) -> f64 {
    let dx = (p1.row as f64) - (p2.row as f64);
    let dy = (p1.col as f64) - (p2.col as f64);
    (dx.mul_add(dx, dy * dy)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::distance;
    use crate::matrix::d2::point::point_i64::Point;

    #[test]
    fn delta_0() {
        assert_eq!(distance(&Point::of(-1, -1), &Point::of(-1, -1)), 0.0);
        assert_eq!(distance(&Point::of(-1, 1), &Point::of(-1, 1)), 0.0);
        assert_eq!(distance(&Point::of(1, -1), &Point::of(1, -1)), 0.0);
        assert_eq!(distance(&Point::of(1, 1), &Point::of(1, 1)), 0.0);
    }

    #[test]
    fn delta_row() {
        assert_eq!(distance(&Point::of(-10, 0), &Point::of(-10, 50)), 50.0);
        assert_eq!(distance(&Point::of(10, 100), &Point::of(10, 200)), 100.0);
    }

    #[test]
    fn delta_col() {
        assert_eq!(distance(&Point::of(0, -10), &Point::of(50, -10)), 50.0);
        assert_eq!(distance(&Point::of(100, 10), &Point::of(200, 10)), 100.0);
    }

    #[test]
    fn pythagorean() {
        assert_eq!(distance(&Point::of(0, 0), &Point::of(3, 4)), 5.0);
        assert_eq!(distance(&Point::of(0, 0), &Point::of(30, 40)), 50.0);
        assert_eq!(distance(&Point::of(0, 0), &Point::of(300, 400)), 500.0);
        assert_eq!(distance(&Point::of(0, 0), &Point::of(5000, 12000)), 13000.0);
        assert_eq!(distance(&Point::of(0, 0), &Point::of(50000, 120000)), 130000.0);
        assert_eq!(distance(&Point::of(0, 0), &Point::of(500000, 1200000)), 1300000.0);
        assert_eq!(distance(&Point::of(0, 0), &Point::of(8000000, 15000000)), 17000000.0);
        assert_eq!(distance(&Point::of(0, 0), &Point::of(80000000, 150000000)), 170000000.0);
        assert_eq!(distance(&Point::of(0, 0), &Point::of(800000000, 1500000000)), 1700000000.0);
    }

    #[test]
    fn square() {
        assert_eq!(distance(&Point::of(-50, -50), &Point::of(50, 50)), 141.4213562373095);
        assert_eq!(distance(&Point::of(-100, -100), &Point::of(100, 100)), 282.842712474619);
    }
}

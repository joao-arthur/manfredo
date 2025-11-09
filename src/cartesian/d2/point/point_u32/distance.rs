use super::Point;

pub fn distance(p1: &Point, p2: &Point) -> f64 {
    let dx = (p1.x as f64) - (p2.x as f64);
    let dy = (p1.y as f64) - (p2.y as f64);
    (dx.mul_add(dx, dy * dy)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::distance;
    use crate::cartesian::d2::point::point_u32::Point;

    #[test]
    fn delta_0() {
        assert_eq!(distance(&Point::min(), &Point::min()), 0.0);
        assert_eq!(distance(&Point::new(0, 1), &Point::new(0, 1)), 0.0);
        assert_eq!(distance(&Point::new(1, 0), &Point::new(1, 0)), 0.0);
        assert_eq!(distance(&Point::new(1, 1), &Point::new(1, 1)), 0.0);
    }

    #[test]
    fn delta_x() {
        assert_eq!(distance(&Point::new(10, 0), &Point::new(10, 50)), 50.0);
        assert_eq!(distance(&Point::new(10, 100), &Point::new(10, 200)), 100.0);
    }

    #[test]
    fn delta_y() {
        assert_eq!(distance(&Point::new(0, 10), &Point::new(50, 10)), 50.0);
        assert_eq!(distance(&Point::new(100, 10), &Point::new(200, 10)), 100.0);
    }

    #[test]
    fn pythagorean() {
        assert_eq!(distance(&Point::min(), &Point::new(3, 4)), 5.0);
        assert_eq!(distance(&Point::min(), &Point::new(30, 40)), 50.0);
        assert_eq!(distance(&Point::min(), &Point::new(300, 400)), 500.0);
        assert_eq!(distance(&Point::min(), &Point::new(5000, 12000)), 13000.0);
        assert_eq!(distance(&Point::min(), &Point::new(50000, 120000)), 130000.0);
        assert_eq!(distance(&Point::min(), &Point::new(500000, 1200000)), 1300000.0);
        assert_eq!(distance(&Point::min(), &Point::new(8000000, 15000000)), 17000000.0);
        assert_eq!(distance(&Point::min(), &Point::new(80000000, 150000000)), 170000000.0);
        assert_eq!(distance(&Point::min(), &Point::new(800000000, 1500000000)), 1700000000.0);
    }

    #[test]
    fn square() {
        assert_eq!(distance(&Point::min(), &Point::new(100, 100)), 141.4213562373095);
        assert_eq!(distance(&Point::new(100, 100), &Point::new(300, 300)), 282.842712474619);
    }
}

use super::Point;

pub fn distance(p1: &Point, p2: &Point) -> f64 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    (dx.mul_add(dx, dy * dy)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::distance;
    use crate::cartesian::d2::point::point_f64::Point;

    #[test]
    fn delta_0() {
        assert_eq!(distance(&Point::new(-1.0, -1.0), &Point::new(-1.0, -1.0)), 0.0);
        assert_eq!(distance(&Point::new(-1.0, 1.0), &Point::new(-1.0, 1.0)), 0.0);
        assert_eq!(distance(&Point::new(1.0, -1.0), &Point::new(1.0, -1.0)), 0.0);
        assert_eq!(distance(&Point::new(1.0, 1.0), &Point::new(1.0, 1.0)), 0.0);
    }

    #[test]
    fn delta_x() {
        assert_eq!(distance(&Point::new(-10.0, 0.0), &Point::new(-10.0, 50.0)), 50.0);
        assert_eq!(distance(&Point::new(10.0, 100.0), &Point::new(10.0, 200.0)), 100.0);
    }

    #[test]
    fn delta_y() {
        assert_eq!(distance(&Point::new(0.0, -10.0), &Point::new(50.0, -10.0)), 50.0);
        assert_eq!(distance(&Point::new(100.0, 10.0), &Point::new(200.0, 10.0)), 100.0);
    }

    #[test]
    fn pythagorean() {
        assert_eq!(distance(&Point::zero(), &Point::new(3.0, 4.0)), 5.0);
        assert_eq!(distance(&Point::zero(), &Point::new(30.0, 40.0)), 50.0);
        assert_eq!(distance(&Point::zero(), &Point::new(300.0, 400.0)), 500.0);
        assert_eq!(distance(&Point::zero(), &Point::new(5000.0, 12000.0)), 13000.0);
        assert_eq!(distance(&Point::zero(), &Point::new(50000.0, 120000.0)), 130000.0);
        assert_eq!(distance(&Point::zero(), &Point::new(500000.0, 1200000.0)), 1300000.0);
        assert_eq!(distance(&Point::zero(), &Point::new(8000000.0, 15000000.0)), 17000000.0);
        assert_eq!(distance(&Point::zero(), &Point::new(80000000.0, 150000000.0)), 170000000.0);
        assert_eq!(distance(&Point::zero(), &Point::new(800000000.0, 1500000000.0)), 1700000000.0);
    }

    #[test]
    fn square() {
        assert_eq!(distance(&Point::new(-50.0, -50.0), &Point::new(50.0, 50.0)), 141.4213562373095);
        assert_eq!(distance(&Point::new(-100.0, -100.0), &Point::new(100.0, 100.0)), 282.842712474619);

        assert_eq!(distance(&Point::zero(), &Point::new(141.4213562373095, 141.4213562373095)), 200.0);
        assert_eq!(distance(&Point::zero(), &Point::new(282.842712474619, 282.842712474619)), 400.0);
    }
}

use super::Point;

pub fn distance(p1: &Point, p2: &Point) -> f32 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    (dx.mul_add(dx, dy * dy)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::distance;
    use crate::cartesian::d2::point::point_f32::Point;

    #[test]
    fn distance_delta_0() {
        assert_eq!(distance(&Point::of(-1.0, -1.0), &Point::of(-1.0, -1.0)), 0.0);
        assert_eq!(distance(&Point::of(-1.0, 1.0), &Point::of(-1.0, 1.0)), 0.0);
        assert_eq!(distance(&Point::of(1.0, -1.0), &Point::of(1.0, -1.0)), 0.0);
        assert_eq!(distance(&Point::of(1.0, 1.0), &Point::of(1.0, 1.0)), 0.0);
    }

    #[test]
    fn distance_delta_x() {
        assert_eq!(distance(&Point::of(-10.0, 0.0), &Point::of(-10.0, 50.0)), 50.0);
        assert_eq!(distance(&Point::of(10.0, 100.0), &Point::of(10.0, 200.0)), 100.0);
    }

    #[test]
    fn distance_delta_y() {
        assert_eq!(distance(&Point::of(0.0, -10.0), &Point::of(50.0, -10.0)), 50.0);
        assert_eq!(distance(&Point::of(100.0, 10.0), &Point::of(200.0, 10.0)), 100.0);
    }

    #[test]
    fn distance_pythagorean() {
        assert_eq!(distance(&Point::of(0.0, 0.0), &Point::of(3.0, 4.0)), 5.0);
        assert_eq!(distance(&Point::of(0.0, 0.0), &Point::of(30.0, 40.0)), 50.0);
        assert_eq!(distance(&Point::of(0.0, 0.0), &Point::of(300.0, 400.0)), 500.0);
        assert_eq!(distance(&Point::of(0.0, 0.0), &Point::of(5000.0, 12000.0)), 13000.0);
        assert_eq!(distance(&Point::of(0.0, 0.0), &Point::of(50000.0, 120000.0)), 130000.0);
        assert_eq!(distance(&Point::of(0.0, 0.0), &Point::of(500000.0, 1200000.0)), 1300000.0);
        assert_eq!(distance(&Point::of(0.0, 0.0), &Point::of(8000000.0, 15000000.0)), 17000000.0);
        assert_eq!(distance(&Point::of(0.0, 0.0), &Point::of(80000000.0, 150000000.0)), 170000000.0);
        assert_eq!(distance(&Point::of(0.0, 0.0), &Point::of(800000000.0, 1500000000.0)), 1700000000.0);
    }

    #[test]
    fn distance_square() {
        assert_eq!(distance(&Point::of(-50.0, -50.0), &Point::of(50.0, 50.0)), 141.4213562373095);
        assert_eq!(distance(&Point::of(-100.0, -100.0), &Point::of(100.0, 100.0)), 282.842712474619);

        assert_eq!(distance(&Point::of(0.0, 0.0), &Point::of(141.4213562373095, 141.4213562373095)), 200.0);
        assert_eq!(distance(&Point::of(0.0, 0.0), &Point::of(282.842712474619, 282.842712474619)), 400.0);
    }
}

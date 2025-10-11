use super::Point;

pub fn delta(p1: &Point, p2: &Point) -> f32 {
    p2.x - p1.x
}

#[cfg(test)]
mod tests {
    use super::delta;
    use crate::cartesian::d1::point::point_f32::{MAX, Point};

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::of(0.0), &Point::of(0.0)), 0.0);
        assert_eq!(delta(&Point::of(0.0), &Point::max()), MAX);
        assert_eq!(delta(&Point::min(), &Point::of(0.0)), MAX + 1.0);
        assert_eq!(delta(&Point::of(-8_388_608.0), &Point::of(8_388_607.0)), MAX);
    }
}

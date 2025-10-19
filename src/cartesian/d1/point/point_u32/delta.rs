use super::Point;

pub fn delta(p1: &Point, p2: &Point) -> u32 {
    p2.x - p1.x
}

#[cfg(test)]
mod tests {
    use super::delta;
    use crate::cartesian::d1::point::point_u32::{MAX, Point};

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::min(), &Point::min()), 0);
        assert_eq!(delta(&Point::min(), &Point::max()), MAX);
    }
}

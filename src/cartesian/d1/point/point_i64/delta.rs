use super::Point;

pub fn delta(p1: &Point, p2: &Point) -> u64 {
    (i128::from(p2.x) - i128::from(p1.x)).unsigned_abs() as u64
}

#[cfg(test)]
mod tests {
    use super::delta;
    use crate::cartesian::d1::point::point_i64::Point;

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::of(0), &Point::of(0)), 0);
        assert_eq!(delta(&Point::min(), &Point::max()), u64::MAX);
    }
}

use super::Point;

pub fn delta(p1: &Point, p2: &Point) -> u32 {
    (i64::from(p2.x) - i64::from(p1.x)).unsigned_abs() as u32
}

#[cfg(test)]
mod tests {
    use super::delta;
    use crate::cartesian::d1::point::point_i32::Point;

    const MIN: i32 = i32::MIN;
    const MAX: i32 = i32::MAX;

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::of(0), &Point::of(0)), 0);
        assert_eq!(delta(&Point::min(), &Point::max()), u32::MAX);
    }
}

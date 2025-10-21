use super::Point;

pub fn delta(p1: &Point, p2: &Point) -> u16 {
    (i32::from(p2.x) - i32::from(p1.x)).unsigned_abs() as u16
}

#[cfg(test)]
mod tests {
    use super::delta;
    use crate::cartesian::d1::point::point_i16::Point;

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::zero(), &Point::zero()), 0);
        assert_eq!(delta(&Point::min(), &Point::max()), u16::MAX);
    }
}

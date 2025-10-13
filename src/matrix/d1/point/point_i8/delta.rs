use super::Point;

pub fn delta(p1: &Point, p2: &Point) -> u8 {
    (i16::from(p2.i) - i16::from(p1.i)).unsigned_abs() as u8
}

#[cfg(test)]
mod tests {
    use super::delta;
    use crate::matrix::d1::point::point_i8::Point;

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::of(0), &Point::of(0)), 0);
        assert_eq!(delta(&Point::min(), &Point::max()), u8::MAX);
    }
}

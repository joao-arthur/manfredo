use super::checked_add_assign;
use crate::matrix::d2::point::point_i32::Point;

#[test]
fn test() {
    let mut p = Point::zero();
    checked_add_assign(&mut p, &Point::of(10, 13));
    assert_eq!(p, Point::of(10, 13));
    checked_add_assign(&mut p, &Point::of(-25, -30));
    assert_eq!(p, Point::of(-15, -17));
}

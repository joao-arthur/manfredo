use super::saturating_add_assign;
use crate::cartesian::d2::point::point_i64::Point;

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

#[test]
fn test() {
    let mut p = Point::of(0, 0);
    saturating_add_assign(&mut p, &Point::of(10, 13));
    assert_eq!(p, Point::of(10, 13));
    saturating_add_assign(&mut p, &Point::of(-5, -3));
    assert_eq!(p, Point::of(5, 10));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::of(MIN + 2, MIN + 5);
    saturating_add_assign(&mut p_min, &Point::of(-2, -5));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::of(MAX - 2, MAX - 5);
    saturating_add_assign(&mut p_max, &Point::of(2, 5));
    assert_eq!(p_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::of(MIN + 2, MIN + 5);
    saturating_add_assign(&mut p_min, &Point::of(-10, -10));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::of(MAX - 2, MAX - 5);
    saturating_add_assign(&mut p_max, &Point::of(10, 10));
    assert_eq!(p_max, Point::max());
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::of(MIN + 1, MIN + 1);
    saturating_add_assign(&mut p_min, &Point::min());
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::of(MAX - 1, MAX - 1);
    saturating_add_assign(&mut p_max, &Point::max());
    assert_eq!(p_max, Point::max());
}

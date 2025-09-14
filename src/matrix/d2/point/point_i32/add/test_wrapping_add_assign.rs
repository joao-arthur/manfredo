use super::wrapping_add_assign;
use crate::matrix::d2::point::point_i32::Point;

const MIN: i32 = i32::MIN;
const MAX: i32 = i32::MAX;

#[test]
fn test() {
    let mut p = Point::of(0, 0);
    wrapping_add_assign(&mut p, &Point::of(10, 13));
    assert_eq!(p, Point::of(10, 13));
    wrapping_add_assign(&mut p, &Point::of(-5, -3));
    assert_eq!(p, Point::of(5, 10));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::of(MIN + 2, MIN + 5);
    wrapping_add_assign(&mut p_min, &Point::of(-2, -5));
    assert_eq!(p_min, Point::min());

    let mut m_max = Point::of(MAX - 2, MAX - 5);
    wrapping_add_assign(&mut m_max, &Point::of(2, 5));
    assert_eq!(m_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::of(MIN + 2, MIN + 5);
    wrapping_add_assign(&mut p_min, &Point::of(-10, -10));
    assert_eq!(p_min, Point::of(MAX - 7, MAX - 4));

    let mut m_max = Point::of(MAX - 2, MAX - 5);
    wrapping_add_assign(&mut m_max, &Point::of(10, 10));
    assert_eq!(m_max, Point::of(MIN + 7, MIN + 4));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::of(MIN + 1, MIN + 1);
    wrapping_add_assign(&mut p_min, &Point::min());
    assert_eq!(p_min, Point::of(1, 1));

    let mut m_max = Point::of(MAX - 1, MAX - 1);
    wrapping_add_assign(&mut m_max, &Point::max());
    assert_eq!(m_max, Point::of(-3, -3));
}

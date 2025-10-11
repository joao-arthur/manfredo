use super::wrapping_add_assign;
use crate::cartesian::d1::point::point_i16::Point;

const MIN: i16 = i16::MIN;
const MAX: i16 = i16::MAX;

#[test]
fn test() {
    let mut p = Point::of(0);
    wrapping_add_assign(&mut p, &Point::of(10));
    assert_eq!(p, Point::of(10));
    wrapping_add_assign(&mut p, &Point::of(-5));
    assert_eq!(p, Point::of(5));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::of(MIN + 2);
    wrapping_add_assign(&mut p_min, &Point::of(-2));
    assert_eq!(p_min, Point::min());

    let mut m_max = Point::of(MAX - 2);
    wrapping_add_assign(&mut m_max, &Point::of(2));
    assert_eq!(m_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::of(MIN + 2);
    wrapping_add_assign(&mut p_min, &Point::of(-10));
    assert_eq!(p_min, Point::of(MAX - 7));

    let mut m_max = Point::of(MAX - 2);
    wrapping_add_assign(&mut m_max, &Point::of(10));
    assert_eq!(m_max, Point::of(MIN + 7));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::of(MIN + 1);
    wrapping_add_assign(&mut p_min, &Point::min());
    assert_eq!(p_min, Point::of(1));

    let mut m_max = Point::of(MAX - 1);
    wrapping_add_assign(&mut m_max, &Point::max());
    assert_eq!(m_max, Point::of(-3));
}

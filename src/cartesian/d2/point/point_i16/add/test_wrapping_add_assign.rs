use super::wrapping_add_assign;
use crate::cartesian::{
    d1::point::point_i16::{MAX, MIN},
    d2::point::point_i16::Point,
};

#[test]
fn test() {
    let mut p = Point::zero();
    wrapping_add_assign(&mut p, &Point::new(10, 13));
    assert_eq!(p, Point::new(10, 13));
    wrapping_add_assign(&mut p, &Point::new(-5, -3));
    assert_eq!(p, Point::new(5, 10));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::new(MIN + 2, MIN + 5);
    wrapping_add_assign(&mut p_min, &Point::new(-2, -5));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::new(MAX - 2, MAX - 5);
    wrapping_add_assign(&mut p_max, &Point::new(2, 5));
    assert_eq!(p_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::new(MIN + 2, MIN + 5);
    wrapping_add_assign(&mut p_min, &Point::new(-10, -10));
    assert_eq!(p_min, Point::new(MAX - 7, MAX - 4));

    let mut p_max = Point::new(MAX - 2, MAX - 5);
    wrapping_add_assign(&mut p_max, &Point::new(10, 10));
    assert_eq!(p_max, Point::new(MIN + 7, MIN + 4));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::new(MIN + 1, MIN + 1);
    wrapping_add_assign(&mut p_min, &Point::min());
    assert_eq!(p_min, Point::new(1, 1));

    let mut p_max = Point::new(MAX - 1, MAX - 1);
    wrapping_add_assign(&mut p_max, &Point::max());
    assert_eq!(p_max, Point::new(-3, -3));
}

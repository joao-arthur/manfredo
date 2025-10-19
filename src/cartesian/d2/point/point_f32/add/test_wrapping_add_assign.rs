use super::wrapping_add_assign;
use crate::cartesian::{
    d1::point::point_f32::{MAX, MIN},
    d2::point::point_f32::Point,
};

#[test]
fn test() {
    let mut p = Point::of(0.0, 0.0);
    wrapping_add_assign(&mut p, &Point::of(10.0, 13.0));
    assert_eq!(p, Point::of(10.0, 13.0));
    wrapping_add_assign(&mut p, &Point::of(-5.0, -3.0));
    assert_eq!(p, Point::of(5.0, 10.0));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::of(MIN + 2.0, MIN + 5.0);
    wrapping_add_assign(&mut p_min, &Point::of(-2.0, -5.0));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::of(MAX - 2.0, MAX - 5.0);
    wrapping_add_assign(&mut p_max, &Point::of(2.0, 5.0));
    assert_eq!(p_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::of(MIN + 2.0, MIN + 5.0);
    wrapping_add_assign(&mut p_min, &Point::of(-10.0, -10.0));
    assert_eq!(p_min, Point::of(MAX - 7.0, MAX - 4.0));

    let mut p_max = Point::of(MAX - 2.0, MAX - 5.0);
    wrapping_add_assign(&mut p_max, &Point::of(10.0, 10.0));
    assert_eq!(p_max, Point::of(MIN + 7.0, MIN + 4.0));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::of(MIN + 1.0, MIN + 1.0);
    wrapping_add_assign(&mut p_min, &Point::min());
    assert_eq!(p_min, Point::of(1.0, 1.0));

    let mut p_max = Point::of(MAX - 1.0, MAX - 1.0);
    wrapping_add_assign(&mut p_max, &Point::max());
    assert_eq!(p_max, Point::of(-3.0, -3.0));
}

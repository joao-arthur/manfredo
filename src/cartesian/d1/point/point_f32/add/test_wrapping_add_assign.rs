use super::wrapping_add_assign;
use crate::cartesian::d1::point::point_f32::{MAX, MIN, Point};

#[test]
fn test() {
    let mut p = Point::zero();
    wrapping_add_assign(&mut p, &Point::new(10.0));
    assert_eq!(p, Point::new(10.0));
    wrapping_add_assign(&mut p, &Point::new(-5.0));
    assert_eq!(p, Point::new(5.0));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::new(MIN + 2.0);
    wrapping_add_assign(&mut p_min, &Point::new(-2.0));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::new(MAX - 2.0);
    wrapping_add_assign(&mut p_max, &Point::new(2.0));
    assert_eq!(p_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::new(MIN + 2.0);
    wrapping_add_assign(&mut p_min, &Point::new(-10.0));
    assert_eq!(p_min, Point::new(MAX - 7.0));

    let mut p_max = Point::new(MAX - 2.0);
    wrapping_add_assign(&mut p_max, &Point::new(10.0));
    assert_eq!(p_max, Point::new(MIN + 7.0));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::new(MIN + 1.0);
    wrapping_add_assign(&mut p_min, &Point::min());
    assert_eq!(p_min, Point::new(1.0));

    let mut p_max = Point::new(MAX - 1.0);
    wrapping_add_assign(&mut p_max, &Point::max());
    assert_eq!(p_max, Point::new(-3.0));
}

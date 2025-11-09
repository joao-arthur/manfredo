use super::wrapping_add_assign;
use crate::matrix::{
    d1::point::point_u32::MAX,
    d2::point::{point_i32::Point as PointI, point_u32::Point},
};

#[test]
fn test() {
    let mut p = Point::min();
    wrapping_add_assign(&mut p, &PointI::new(10, 13));
    assert_eq!(p, Point::new(10, 13));
    wrapping_add_assign(&mut p, &PointI::new(-5, -3));
    assert_eq!(p, Point::new(5, 10));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::new(2, 5);
    wrapping_add_assign(&mut p_min, &PointI::new(-2, -5));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::new(MAX - 2, MAX - 5);
    wrapping_add_assign(&mut p_max, &PointI::new(2, 5));
    assert_eq!(p_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::new(2, 5);
    wrapping_add_assign(&mut p_min, &PointI::new(-10, -10));
    assert_eq!(p_min, Point::new(MAX - 7, MAX - 4));

    let mut p_max = Point::new(MAX - 2, MAX - 5);
    wrapping_add_assign(&mut p_max, &PointI::new(10, 10));
    assert_eq!(p_max, Point::new(7, 4));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::new(1, 1);
    wrapping_add_assign(&mut p_min, &PointI::min());
    assert_eq!(p_min, Point::new(2147483649, 2147483649));

    let mut p_max = Point::new(MAX - 1, MAX - 1);
    wrapping_add_assign(&mut p_max, &PointI::max());
    assert_eq!(p_max, Point::new(2147483645, 2147483645));
}

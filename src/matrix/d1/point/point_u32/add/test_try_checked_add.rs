use super::try_checked_add;
use crate::matrix::d1::point::{
    point_i32::Point as PointI,
    point_u32::{MAX, Point},
};

#[test]
fn test() {
    assert_eq!(try_checked_add(&Point::min(), &PointI::new(10)), Some(Point::new(10)));
    assert_eq!(try_checked_add(&Point::new(10), &PointI::new(-5)), Some(Point::new(5)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&Point::new(2), &PointI::new(-2)), Some(Point::min()));
    assert_eq!(try_checked_add(&Point::new(MAX - 2), &PointI::new(2)), Some(Point::max()));
}

#[test]
fn out_of_bounds() {
    let p_min = Point::new(2);
    assert_eq!(try_checked_add(&p_min, &PointI::new(-10)), None);

    let p_max = Point::new(MAX - 2);
    assert_eq!(try_checked_add(&p_max, &PointI::new(10)), None);
}

#[test]
fn limits_out_of_bounds() {
    let p_min = Point::new(1);
    assert_eq!(try_checked_add(&p_min, &PointI::min()), None);

    let p_max = Point::new(MAX - 1);
    assert_eq!(try_checked_add(&p_max, &PointI::max()), None);
}

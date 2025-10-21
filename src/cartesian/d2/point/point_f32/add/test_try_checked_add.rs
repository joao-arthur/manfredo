use super::try_checked_add;
use crate::cartesian::{
    d1::point::point_f32::{MAX, MIN},
    d2::point::point_f32::Point,
};

#[test]
fn test() {
    assert_eq!(try_checked_add(&Point::zero(), &Point::of(10.0, 13.0)), Some(Point::of(10.0, 13.0)));
    assert_eq!(try_checked_add(&Point::of(10.0, 10.0), &Point::of(-5.0, -3.0)), Some(Point::of(5.0, 7.0)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&Point::of(MIN + 2.0, MIN + 5.0), &Point::of(-2.0, -5.0)), Some(Point::min()));
    assert_eq!(try_checked_add(&Point::of(MAX - 2.0, MAX - 5.0), &Point::of(2.0, 5.0)), Some(Point::max()));
}

#[test]
fn out_of_bounds() {
    let p_min = Point::of(MIN + 2.0, MIN + 5.0);
    assert_eq!(try_checked_add(&p_min, &Point::of(-10.0, 0.0)), None);
    assert_eq!(try_checked_add(&p_min, &Point::of(0.0, -10.0)), None);
    assert_eq!(try_checked_add(&p_min, &Point::of(-10.0, -10.0)), None);

    let p_max = Point::of(MAX - 2.0, MAX - 5.0);
    assert_eq!(try_checked_add(&p_max, &Point::of(10.0, 0.0)), None);
    assert_eq!(try_checked_add(&p_max, &Point::of(0.0, 10.0)), None);
    assert_eq!(try_checked_add(&p_max, &Point::of(10.0, 10.0)), None);
}

#[test]
fn limits_out_of_bounds() {
    let p_min = Point::of(MIN + 1.0, MIN + 1.0);
    assert_eq!(try_checked_add(&p_min, &Point::of(MIN, 0.0)), None);
    assert_eq!(try_checked_add(&p_min, &Point::of(0.0, MIN)), None);
    assert_eq!(try_checked_add(&p_min, &Point::min()), None);

    let p_max = Point::of(MAX - 1.0, MAX - 1.0);
    assert_eq!(try_checked_add(&p_max, &Point::of(MAX, 0.0)), None);
    assert_eq!(try_checked_add(&p_max, &Point::of(0.0, MAX)), None);
    assert_eq!(try_checked_add(&p_max, &Point::max()), None);
}

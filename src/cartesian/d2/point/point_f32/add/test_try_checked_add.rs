use super::try_checked_add;
use crate::cartesian::{
    d1::point::point_f32::{MAX, MIN},
    d2::point::point_f32::Point,
};

#[test]
fn test() {
    assert_eq!(try_checked_add(&Point::zero(), &Point::new(10.0, 13.0)), Some(Point::new(10.0, 13.0)));
    assert_eq!(try_checked_add(&Point::new(10.0, 10.0), &Point::new(-5.0, -3.0)), Some(Point::new(5.0, 7.0)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&Point::new(MIN + 2.0, MIN + 5.0), &Point::new(-2.0, -5.0)), Some(Point::min()));
    assert_eq!(try_checked_add(&Point::new(MAX - 2.0, MAX - 5.0), &Point::new(2.0, 5.0)), Some(Point::max()));
}

#[test]
fn out_of_bounds() {
    let p_min = Point::new(MIN + 2.0, MIN + 5.0);
    assert_eq!(try_checked_add(&p_min, &Point::new(-10.0, 0.0)), None);
    assert_eq!(try_checked_add(&p_min, &Point::new(0.0, -10.0)), None);
    assert_eq!(try_checked_add(&p_min, &Point::new(-10.0, -10.0)), None);

    let p_max = Point::new(MAX - 2.0, MAX - 5.0);
    assert_eq!(try_checked_add(&p_max, &Point::new(10.0, 0.0)), None);
    assert_eq!(try_checked_add(&p_max, &Point::new(0.0, 10.0)), None);
    assert_eq!(try_checked_add(&p_max, &Point::new(10.0, 10.0)), None);
}

#[test]
fn limits_out_of_bounds() {
    let p_min = Point::new(MIN + 1.0, MIN + 1.0);
    assert_eq!(try_checked_add(&p_min, &Point::new(MIN, 0.0)), None);
    assert_eq!(try_checked_add(&p_min, &Point::new(0.0, MIN)), None);
    assert_eq!(try_checked_add(&p_min, &Point::min()), None);

    let p_max = Point::new(MAX - 1.0, MAX - 1.0);
    assert_eq!(try_checked_add(&p_max, &Point::new(MAX, 0.0)), None);
    assert_eq!(try_checked_add(&p_max, &Point::new(0.0, MAX)), None);
    assert_eq!(try_checked_add(&p_max, &Point::max()), None);
}

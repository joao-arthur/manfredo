use super::try_checked_translate;
use crate::cartesian::{
    d1::point::point_f64::{MAX, MIN},
    d2::{point::point_f64::Point, rect::rect_f64::Rect},
};

#[test]
fn test() {
    assert_eq!(try_checked_translate(&Rect::new((0.0, 0.0), (10.0, 10.0)), &Point::new(10.0, 20.0)), Some(Rect::new((10.0, 20.0), (20.0, 30.0))));
    assert_eq!(try_checked_translate(&Rect::new((10.0, 20.0), (20.0, 30.0)), &Point::new(-20.0, -15.0)), Some(Rect::new((-10.0, 5.0), (0.0, 15.0))));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_translate(&Rect::new((MIN + 2.0, MIN + 5.0), (MAX, MAX)), &Point::new(-2.0, -5.0)), Some(Rect::new((MIN, MIN), (MAX - 2.0, MAX - 5.0))));
    assert_eq!(try_checked_translate(&Rect::new((MIN, MIN), (MAX - 2.0, MAX - 5.0)), &Point::new(2.0, 5.0)), Some(Rect::new((MIN + 2.0, MIN + 5.0), (MAX, MAX))));
}

#[test]
fn out_of_bounds() {
    let r = Rect::new((MIN + 10.0, MIN + 10.0), (MAX - 10.0, MAX - 10.0));
    assert_eq!(try_checked_translate(&r, &Point::new(-20.0, 0.0)), None);
    assert_eq!(try_checked_translate(&r, &Point::new(0.0, -20.0)), None);
    assert_eq!(try_checked_translate(&r, &Point::new(20.0, 0.0)), None);
    assert_eq!(try_checked_translate(&r, &Point::new(0.0, 20.0)), None);
}

#[test]
fn edge_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(try_checked_translate(&r, &Point::new(-1.0, 0.0)), None);
    assert_eq!(try_checked_translate(&r, &Point::new(0.0, -1.0)), None);
    assert_eq!(try_checked_translate(&r, &Point::new(1.0, 0.0)), None);
    assert_eq!(try_checked_translate(&r, &Point::new(0.0, 1.0)), None);
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(try_checked_translate(&r, &Point::new(MIN, 0.0)), None);
    assert_eq!(try_checked_translate(&r, &Point::new(0.0, MIN)), None);
    assert_eq!(try_checked_translate(&r, &Point::new(MAX, 0.0)), None);
    assert_eq!(try_checked_translate(&r, &Point::new(0.0, MAX)), None);
}

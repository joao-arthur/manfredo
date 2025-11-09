use super::try_checked_translate_assign;
use crate::cartesian::{
    d1::point::point_f64::{MAX, MIN},
    d2::{point::point_f64::Point, rect::rect_f64::Rect},
};

#[test]
fn test() {
    let mut r = Rect::new((0.0, 0.0), (10.0, 10.0));
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(10.0, 20.0)), Some(()));
    assert_eq!(r, Rect::new((10.0, 20.0), (20.0, 30.0)));
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(-20.0, -15.0)), Some(()));
    assert_eq!(r, Rect::new((-10.0, 5.0), (0.0, 15.0)));
}

#[test]
fn to_bounds() {
    let mut r_min = Rect::new((MIN + 2.0, MIN + 5.0), (MAX, MAX));
    assert_eq!(try_checked_translate_assign(&mut r_min, &Point::new(-2.0, -5.0)), Some(()));
    assert_eq!(r_min, Rect::new((MIN, MIN), (MAX - 2.0, MAX - 5.0)));

    let mut r_max = Rect::new((MIN, MIN), (MAX - 2.0, MAX - 5.0));
    assert_eq!(try_checked_translate_assign(&mut r_max, &Point::new(2.0, 5.0)), Some(()));
    assert_eq!(r_max, Rect::new((MIN + 2.0, MIN + 5.0), (MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    let mut r = Rect::new((MIN + 10.0, MIN + 10.0), (MAX - 10.0, MAX - 10.0));
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(-20.0, 0.0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(0.0, -20.0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(20.0, 0.0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(0.0, 20.0)), None);
    assert_eq!(r, Rect::new((MIN + 10.0, MIN + 10.0), (MAX - 10.0, MAX - 10.0)));
}

#[test]
fn edge_out_of_bounds() {
    let mut r = Rect::largest();
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(-1.0, 0.0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(0.0, -1.0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(1.0, 0.0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(0.0, 1.0)), None);
    assert_eq!(r, Rect::largest());
}

#[test]
fn limits_out_of_bounds() {
    let mut r = Rect::largest();
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(MIN, 0.0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(0.0, MIN)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(MAX, 0.0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(0.0, MAX)), None);
    assert_eq!(r, Rect::largest());
}

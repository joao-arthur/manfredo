use super::try_checked_translate_assign;
use crate::cartesian::{
    d1::point::point_i64::{MAX, MIN},
    d2::{point::point_i64::Point, rect::rect_i64::Rect},
};

#[test]
fn test() {
    let mut r = Rect::new((5, 9), (13, 37));
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(-10, -20)), Some(()));
    assert_eq!(r, Rect::new((-5, -11), (3, 17)));
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(6, -19)), Some(()));
    assert_eq!(r, Rect::new((1, -30), (9, -2)));
}

#[test]
fn to_bounds() {
    let mut r_min = Rect::new((MIN + 2, MIN + 5), (MAX, MAX));
    assert_eq!(try_checked_translate_assign(&mut r_min, &Point::new(-2, -5)), Some(()));
    assert_eq!(r_min, Rect::new((MIN, MIN), (MAX - 2, MAX - 5)));

    let mut r_max = Rect::new((MIN, MIN), (MAX - 2, MAX - 5));
    assert_eq!(try_checked_translate_assign(&mut r_max, &Point::new(2, 5)), Some(()));
    assert_eq!(r_max, Rect::new((MIN + 2, MIN + 5), (MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    let mut r = Rect::new((MIN + 10, MIN + 10), (MAX - 10, MAX - 10));
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(-20, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(0, -20)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(20, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(0, 20)), None);
    assert_eq!(r, Rect::new((MIN + 10, MIN + 10), (MAX - 10, MAX - 10)));
}

#[test]
fn limits_out_of_bounds() {
    let mut r = Rect::largest();
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(MIN, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(0, MIN)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(MAX, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(0, MAX)), None);
    assert_eq!(r, Rect::largest());
}

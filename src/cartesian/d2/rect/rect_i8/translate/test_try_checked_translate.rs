use super::try_checked_translate;
use crate::cartesian::{
    d1::point::point_i8::{MAX, MIN},
    d2::{point::point_i8::Point, rect::rect_i8::Rect},
};

#[test]
fn test() {
    assert_eq!(try_checked_translate(&Rect::new((5, 9), (13, 37)), &Point::new(-10, -20)), Some(Rect::new((-5, -11), (3, 17))));
    assert_eq!(try_checked_translate(&Rect::new((-5, -11), (3, 17)), &Point::new(6, -19)), Some(Rect::new((1, -30), (9, -2))));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_translate(&Rect::new((MIN + 2, MIN + 5), (MAX, MAX)), &Point::new(-2, -5)), Some(Rect::new((MIN, MIN), (MAX - 2, MAX - 5))));
    assert_eq!(try_checked_translate(&Rect::new((MIN, MIN), (MAX - 2, MAX - 5)), &Point::new(2, 5)), Some(Rect::new((MIN + 2, MIN + 5), (MAX, MAX))));
}

#[test]
fn out_of_bounds() {
    let r = Rect::new((MIN + 10, MIN + 10), (MAX - 10, MAX - 10));
    assert_eq!(try_checked_translate(&r, &Point::new(-20, 0)), None);
    assert_eq!(try_checked_translate(&r, &Point::new(0, -20)), None);
    assert_eq!(try_checked_translate(&r, &Point::new(20, 0)), None);
    assert_eq!(try_checked_translate(&r, &Point::new(0, 20)), None);
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(try_checked_translate(&r, &Point::new(MIN, 0)), None);
    assert_eq!(try_checked_translate(&r, &Point::new(0, MIN)), None);
    assert_eq!(try_checked_translate(&r, &Point::new(MAX, 0)), None);
    assert_eq!(try_checked_translate(&r, &Point::new(0, MAX)), None);
}

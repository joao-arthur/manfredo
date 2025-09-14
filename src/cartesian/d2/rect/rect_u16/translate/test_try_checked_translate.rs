use super::try_checked_translate;
use crate::cartesian::{point::point_i16::Point, rect::rect_u16::Rect};

const MAX: u16 = u16::MAX;

#[test]
fn test() {
    assert_eq!(try_checked_translate(&Rect::of(0, 0, 12, 15), &Point::of(5, 4)), Some(Rect::of(5, 4, 17, 19)));
    assert_eq!(try_checked_translate(&Rect::of(5, 4, 17, 19), &Point::of(-4, -2)), Some(Rect::of(1, 2, 13, 17)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_translate(&Rect::of(2, 5, MAX, MAX), &Point::of(-2, -5)), Some(Rect::of(0, 0, MAX - 2, MAX - 5)));
    assert_eq!(try_checked_translate(&Rect::of(0, 0, MAX - 2, MAX - 5), &Point::of(2, 5)), Some(Rect::of(2, 5, MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    let r = Rect::of(10, 10, MAX - 10, MAX - 10);
    assert_eq!(try_checked_translate(&r, &Point::of(-20, 0)), None);
    assert_eq!(try_checked_translate(&r, &Point::of(0, -20)), None);
    assert_eq!(try_checked_translate(&r, &Point::of(20, 0)), None);
    assert_eq!(try_checked_translate(&r, &Point::of(0, 20)), None);
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(try_checked_translate(&r, &Point::of(i16::MIN, 0)), None);
    assert_eq!(try_checked_translate(&r, &Point::of(0, i16::MIN)), None);
    assert_eq!(try_checked_translate(&r, &Point::of(i16::MAX, 0)), None);
    assert_eq!(try_checked_translate(&r, &Point::of(0, i16::MAX)), None);
}

use super::try_checked_add;
use crate::matrix::rect::{rect_i8::Rect as RectI, rect_u8::Rect};

const MAX: u8 = u8::MAX;

#[test]
fn test() {
    assert_eq!(try_checked_add(&Rect::of(0, 0, 12, 15), &RectI::of(5, 4, 3, 2)), Some(Rect::of(5, 4, 15, 17)));
    assert_eq!(try_checked_add(&Rect::of(5, 4, 15, 20), &RectI::of(-4, -3, -2, -1)), Some(Rect::of(1, 1, 13, 19)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&Rect::of(2, 5, MAX - 2, MAX - 5), &RectI::of(-2, -5, 2, 5)), Some(Rect::largest()));
    assert_eq!(try_checked_add(&Rect::of(2, 5, MAX, MAX), &RectI::of(-2, -5, 0, 0)), Some(Rect::largest()));
    assert_eq!(try_checked_add(&Rect::of(0, 0, MAX - 2, MAX - 5), &RectI::of(0, 0, 2, 5)), Some(Rect::largest()));
}

#[test]
fn out_of_bounds() {
    let r = Rect::of(10, 10, MAX - 10, MAX - 10);
    assert_eq!(try_checked_add(&r, &RectI::of(-20, 0, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI::of(0, -20, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI::of(0, 0, 20, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI::of(0, 0, 0, 20)), None);
}

#[test]
fn edge_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(try_checked_add(&r, &RectI::of(-1, 0, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI::of(0, -1, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI::of(0, 0, 1, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI::of(0, 0, 0, 1)), None);
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(try_checked_add(&r, &RectI::of(i8::MIN, 0, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI::of(0, i8::MIN, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI::of(0, 0, i8::MAX, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI::of(0, 0, 0, i8::MAX)), None);
}

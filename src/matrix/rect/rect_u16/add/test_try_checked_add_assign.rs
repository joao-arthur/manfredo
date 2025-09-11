use super::try_checked_add_assign;
use crate::matrix::rect::{rect_i16::Rect, rect_u16::RectU16};

const MAX: u16 = u16::MAX;

#[test]
fn test() {
    let mut r = RectU16::of(0, 0, 12, 12);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(5, 4, 3, 2)), Some(()));
    assert_eq!(r, RectU16::of(5, 4, 15, 14));
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(-4, -3, -2, -1)), Some(()));
    assert_eq!(r, RectU16::of(1, 1, 13, 13));
}

#[test]
fn to_bounds() {
    let mut r = RectU16::of(2, 5, MAX - 2, MAX - 5);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(-2, -5, 2, 5)), Some(()));
    assert_eq!(r, RectU16::largest());

    let mut r_min = RectU16::of(2, 5, MAX, MAX);
    assert_eq!(try_checked_add_assign(&mut r_min, &Rect::of(-2, -5, 0, 0)), Some(()));
    assert_eq!(r_min, RectU16::largest());

    let mut r_max = RectU16::of(0, 0, MAX - 2, MAX - 5);
    assert_eq!(try_checked_add_assign(&mut r_max, &Rect::of(0, 0, 2, 5)), Some(()));
    assert_eq!(r_max, RectU16::largest());
}

#[test]
fn out_of_bounds() {
    let mut r = RectU16::of(10, 10, MAX - 10, MAX - 10);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(-20, 0, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(0, -20, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(0, 0, 20, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(0, 0, 0, 20)), None);
    assert_eq!(r, RectU16::of(10, 10, MAX - 10, MAX - 10));
}

#[test]
fn edge_out_of_bounds() {
    let mut r = RectU16::largest();
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(-1, 0, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(0, -1, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(0, 0, 1, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(0, 0, 0, 1)), None);
    assert_eq!(r, RectU16::largest());
}

#[test]
fn limits_out_of_bounds() {
    let mut r = RectU16::largest();
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(i16::MIN, 0, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(0, i16::MIN, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(0, 0, i16::MAX, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(0, 0, 0, i16::MAX)), None);
    assert_eq!(r, RectU16::largest());
}

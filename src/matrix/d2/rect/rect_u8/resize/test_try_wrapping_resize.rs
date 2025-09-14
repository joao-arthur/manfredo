use super::try_wrapping_resize;
use crate::matrix::d2::rect::rect_u8::Rect;

const MAX: u8 = u8::MAX;

#[test]
fn odd() {
    assert_eq!(try_wrapping_resize(&Rect::of(5, 5, 15, 15), 9), Some(Rect::of(6, 6, 14, 14)));
    assert_eq!(try_wrapping_resize(&Rect::of(6, 6, 14, 14), 7), Some(Rect::of(7, 7, 13, 13)));
    assert_eq!(try_wrapping_resize(&Rect::of(7, 7, 13, 13), 5), Some(Rect::of(8, 8, 12, 12)));
    assert_eq!(try_wrapping_resize(&Rect::of(8, 8, 12, 12), 3), Some(Rect::of(9, 9, 11, 11)));
    assert_eq!(try_wrapping_resize(&Rect::of(9, 9, 11, 11), 9), Some(Rect::of(6, 6, 14, 14)));
}

#[test]
fn even() {
    assert_eq!(try_wrapping_resize(&Rect::of(5, 5, 14, 14), 10), Some(Rect::of(5, 5, 14, 14)));
    assert_eq!(try_wrapping_resize(&Rect::of(5, 5, 14, 14), 8), Some(Rect::of(6, 6, 13, 13)));
    assert_eq!(try_wrapping_resize(&Rect::of(6, 6, 13, 13), 6), Some(Rect::of(7, 7, 12, 12)));
    assert_eq!(try_wrapping_resize(&Rect::of(7, 7, 12, 12), 4), Some(Rect::of(8, 8, 11, 11)));
    assert_eq!(try_wrapping_resize(&Rect::of(8, 8, 11, 11), 8), Some(Rect::of(6, 6, 13, 13)));
}

#[test]
fn small_size() {
    let r = Rect::of(10, 10, 20, 20);
    assert_eq!(try_wrapping_resize(&r, 0), None);
    assert_eq!(try_wrapping_resize(&r, 1), None);
    assert_eq!(try_wrapping_resize(&r, 2), None);
}

#[test]
fn same_size() {
    assert_eq!(try_wrapping_resize(&Rect::of(0, 0, 2, 2), 3), Some(Rect::of(0, 0, 2, 2)));
    assert_eq!(try_wrapping_resize(&Rect::of(0, 0, 3, 3), 4), Some(Rect::of(0, 0, 3, 3)));
    assert_eq!(try_wrapping_resize(&Rect::of(MAX - 2, MAX - 2, MAX, MAX), 3), Some(Rect::of(MAX - 2, MAX - 2, MAX, MAX)));
    assert_eq!(try_wrapping_resize(&Rect::of(MAX - 3, MAX - 3, MAX, MAX), 4), Some(Rect::of(MAX - 3, MAX - 3, MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    assert_eq!(try_wrapping_resize(&Rect::of(0, 2, 2, 4), 5), Some(Rect::of(MAX, 1, 3, 5)));
    assert_eq!(try_wrapping_resize(&Rect::of(2, 0, 4, 2), 5), Some(Rect::of(1, MAX, 5, 3)));
    assert_eq!(try_wrapping_resize(&Rect::of(MAX - 2, MAX - 4, MAX, MAX - 2), 5), Some(Rect::of(MAX - 3, MAX - 5, 0, MAX - 1)));
    assert_eq!(try_wrapping_resize(&Rect::of(MAX - 4, MAX - 2, MAX - 2, MAX), 5), Some(Rect::of(MAX - 5, MAX - 3, MAX - 1, 0)));
}

#[test]
fn small_rect_limits_out_of_bounds() {
    assert_eq!(try_wrapping_resize(&Rect::of(0, 2, 2, 4), MAX), Some(Rect::of(MAX / 2 + 3, MAX / 2 + 5, MAX / 2 + 1, MAX / 2 + 3)));
    assert_eq!(try_wrapping_resize(&Rect::of(2, 0, 4, 2), MAX), Some(Rect::of(MAX / 2 + 5, MAX / 2 + 3, MAX / 2 + 3, MAX / 2 + 1)));
    assert_eq!(try_wrapping_resize(&Rect::of(MAX - 2, MAX - 4, MAX, MAX - 2), MAX), Some(Rect::of(MAX / 2, MAX / 2 - 2, MAX / 2 - 2, MAX / 2 - 4)));
    assert_eq!(try_wrapping_resize(&Rect::of(MAX - 4, MAX - 2, MAX - 2, MAX), MAX), Some(Rect::of(MAX / 2 - 2, MAX / 2, MAX / 2 - 4, MAX / 2 - 2)));
}

#[test]
fn big_rect_limits_out_of_bounds() {
    assert_eq!(try_wrapping_resize(&Rect::of(0, 0, MAX - 1, MAX - 1), MAX), Some(Rect::of(0, 0, MAX - 1, MAX - 1)));
    assert_eq!(try_wrapping_resize(&Rect::of(1, 1, MAX, MAX), MAX), Some(Rect::of(1, 1, MAX, MAX)));
    assert_eq!(try_wrapping_resize(&Rect::largest(), MAX), Some(Rect::of(0, 0, MAX - 1, MAX - 1)));
}

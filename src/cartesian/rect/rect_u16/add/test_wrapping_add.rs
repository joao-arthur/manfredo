use super::wrapping_add;
use crate::cartesian::rect::{rect_i16::RectI16, rect_u16::RectU16};

#[test]
fn test_wrapping_add() {
    assert_eq!(wrapping_add(&RectU16::of(0, 0, 12, 15), &RectI16::of(5, 4, 3, 2)), RectU16::of(5, 4, 15, 17));
    assert_eq!(wrapping_add(&RectU16::of(5, 4, 15, 20), &RectI16::of(-4, -3, -2, -1)), RectU16::of(1, 1, 13, 19));
}

#[test]
fn wrapping_add_small_rect_to_bounds() {
    assert_eq!(wrapping_add(&RectU16::of(2, 5, 12, 15), &RectI16::of(-2, -5, 9, 7)), RectU16::of(0, 0, 21, 22));
    assert_eq!(
        wrapping_add(&RectU16::of(u16::MAX - 12, u16::MAX - 15, u16::MAX - 2, u16::MAX - 5), &RectI16::of(-9, -7, 2, 5)),
        RectU16::of(u16::MAX - 21, u16::MAX - 22, u16::MAX, u16::MAX)
    );
}

#[test]
fn wrapping_add_big_rect_to_bounds() {
    assert_eq!(wrapping_add(&RectU16::of(2, 5, u16::MAX - 2, u16::MAX - 5), &RectI16::of(-2, -5, 2, 5)), RectU16::largest());
    assert_eq!(wrapping_add(&RectU16::of(2, 5, u16::MAX, u16::MAX), &RectI16::of(-2, -5, 0, 0)), RectU16::largest());
    assert_eq!(wrapping_add(&RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5), &RectI16::of(0, 0, 2, 5)), RectU16::largest());
}

#[test]
fn wrapping_add_small_rect_out_of_bounds() {
    assert_eq!(wrapping_add(&RectU16::of(10, 5, 20, 30), &RectI16::of(-20, -20, 0, 0)), RectU16::of(u16::MAX - 9, u16::MAX - 14, 20, 30));
    assert_eq!(
        wrapping_add(&RectU16::of(u16::MAX - 20, u16::MAX - 30, u16::MAX - 5, u16::MAX - 10), &RectI16::of(0, 0, 20, 20)),
        RectU16::of(u16::MAX - 20, u16::MAX - 30, 14, 9)
    );
}

#[test]
fn wrapping_add_big_rect_out_of_bounds() {
    assert_eq!(
        wrapping_add(&RectU16::of(10, 5, u16::MAX, u16::MAX), &RectI16::of(-20, -20, 0, 0)),
        RectU16::of(u16::MAX - 9, u16::MAX - 14, u16::MAX, u16::MAX)
    );
    assert_eq!(wrapping_add(&RectU16::of(0, 0, u16::MAX - 5, u16::MAX - 10), &RectI16::of(0, 0, 20, 20)), RectU16::of(0, 0, 14, 9));
}

#[test]
fn wrapping_add_small_rect_limits_out_of_bounds() {
    assert_eq!(
        wrapping_add(&RectU16::of(1, 1, 10, 10), &RectI16::min()),
        RectU16::of((i16::MAX as u16) + 2, (i16::MAX as u16) + 2, (i16::MAX as u16) + 11, (i16::MAX as u16) + 11)
    );
    assert_eq!(
        wrapping_add(&RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX - 1, u16::MAX - 1), &RectI16::max()),
        RectU16::of((i16::MAX as u16) - 11, (i16::MAX as u16) - 11, (i16::MAX as u16) - 2, (i16::MAX as u16) - 2)
    );
}

#[test]
fn wrapping_add_big_rect_limits_out_of_bounds() {
    assert_eq!(
        wrapping_add(&RectU16::largest(), &RectI16::min()),
        RectU16::of((i16::MAX as u16) + 1, (i16::MAX as u16) + 1, i16::MAX as u16, i16::MAX as u16)
    );
    assert_eq!(
        wrapping_add(&RectU16::largest(), &RectI16::max()),
        RectU16::of(i16::MAX as u16, i16::MAX as u16, (i16::MAX as u16) - 1, (i16::MAX as u16) - 1)
    );
    assert_eq!(
        wrapping_add(&RectU16::of(1, 1, u16::MAX, u16::MAX), &RectI16::min()),
        RectU16::of((i16::MAX as u16) + 2, (i16::MAX as u16) + 2, i16::MAX as u16, i16::MAX as u16)
    );
    assert_eq!(
        wrapping_add(&RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1), &RectI16::max()),
        RectU16::of(i16::MAX as u16, i16::MAX as u16, (i16::MAX as u16) - 2, (i16::MAX as u16) - 2)
    );
}

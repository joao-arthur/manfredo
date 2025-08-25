use super::wrapping_add;
use crate::cartesian::rect::rect_i16::RectI16;

#[test]
fn test_wrapping_add() {
    assert_eq!(wrapping_add(&RectI16::of(0, 0, 12, 15), &RectI16::of(5, 4, 3, 2)), RectI16::of(5, 4, 15, 17));
    assert_eq!(wrapping_add(&RectI16::of(5, 4, 15, 17), &RectI16::of(-14, -13, -12, -11)), RectI16::of(-9, -9, 3, 6));
}

#[test]
fn wrapping_add_small_rect_to_bounds() {
    assert_eq!(
        wrapping_add(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MIN + 12, i16::MIN + 15), &RectI16::of(-2, -5, 9, 7)),
        RectI16::of(i16::MIN, i16::MIN, i16::MIN + 21, i16::MIN + 22)
    );
    assert_eq!(
        wrapping_add(&RectI16::of(i16::MAX - 12, i16::MAX - 15, i16::MAX - 2, i16::MAX - 5), &RectI16::of(-9, -7, 2, 5)),
        RectI16::of(i16::MAX - 21, i16::MAX - 22, i16::MAX, i16::MAX)
    );
}

#[test]
fn wrapping_add_big_rect_to_bounds() {
    assert_eq!(wrapping_add(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX - 2, i16::MAX - 5), &RectI16::of(-2, -5, 2, 5)), RectI16::largest());
    assert_eq!(wrapping_add(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX), &RectI16::of(-2, -5, 0, 0)), RectI16::largest());
    assert_eq!(wrapping_add(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5), &RectI16::of(0, 0, 2, 5)), RectI16::largest());
}

#[test]
fn wrapping_add_small_rect_out_of_bounds() {
    assert_eq!(
        wrapping_add(&RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MIN + 20, i16::MIN + 30), &RectI16::of(-20, -20, 0, 0)),
        RectI16::of(i16::MAX - 9, i16::MAX - 14, i16::MIN + 20, i16::MIN + 30)
    );
    assert_eq!(
        wrapping_add(&RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX - 5, i16::MAX - 10), &RectI16::of(0, 0, 20, 20)),
        RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MIN + 14, i16::MIN + 9)
    );
}

#[test]
fn wrapping_add_big_rect_out_of_bounds() {
    assert_eq!(
        wrapping_add(&RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MAX, i16::MAX), &RectI16::of(-20, -20, 0, 0)),
        RectI16::of(i16::MAX - 9, i16::MAX - 14, i16::MAX, i16::MAX)
    );
    assert_eq!(
        wrapping_add(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 5, i16::MAX - 10), &RectI16::of(0, 0, 20, 20)),
        RectI16::of(i16::MIN, i16::MIN, i16::MIN + 14, i16::MIN + 9)
    );
}

#[test]
fn wrapping_add_small_rect_limits_out_of_bounds() {
    assert_eq!(wrapping_add(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10), &RectI16::min()), RectI16::of(1, 1, 10, 10));
    assert_eq!(wrapping_add(&RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1), &RectI16::max()), RectI16::of(-12, -12, -3, -3));
}

#[test]
fn wrapping_add_big_rect_limits_out_of_bounds() {
    assert_eq!(wrapping_add(&RectI16::largest(), &RectI16::min()), RectI16::of(0, 0, -1, -1));
    assert_eq!(wrapping_add(&RectI16::largest(), &RectI16::max()), RectI16::of(-1, -1, -2, -2));
    assert_eq!(wrapping_add(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX), &RectI16::min()), RectI16::of(1, 1, -1, -1));
    assert_eq!(wrapping_add(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1), &RectI16::max()), RectI16::of(-1, -1, -3, -3));
}

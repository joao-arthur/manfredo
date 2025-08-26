use super::saturating_add;
use crate::matrix::rect::rect_i16::RectI16;

#[test]
fn test_saturating_add() {
    assert_eq!(saturating_add(&RectI16::of(0, 0, 12, 15), &RectI16::of(5, 4, 3, 2)), RectI16::of(5, 4, 15, 17));
    assert_eq!(saturating_add(&RectI16::of(5, 4, 15, 17), &RectI16::of(-14, -13, -12, -11)), RectI16::of(-9, -9, 3, 6));
}

#[test]
fn saturating_add_to_bounds() {
    assert_eq!(saturating_add(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX - 2, i16::MAX - 5), &RectI16::of(-2, -5, 2, 5)), RectI16::largest());
    assert_eq!(saturating_add(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX), &RectI16::of(-2, -5, 0, 0)), RectI16::largest());
    assert_eq!(saturating_add(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5), &RectI16::of(0, 0, 2, 5)), RectI16::largest());
}

#[test]
fn saturating_add_out_of_bounds() {
    let r = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10);
    assert_eq!(saturating_add(&r, &RectI16::of(-20, 0, 0, 0)), RectI16::of(i16::MIN, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10));
    assert_eq!(saturating_add(&r, &RectI16::of(0, -20, 0, 0)), RectI16::of(i16::MIN + 10, i16::MIN, i16::MAX - 10, i16::MAX - 10));
    assert_eq!(saturating_add(&r, &RectI16::of(0, 0, 20, 0)), RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX, i16::MAX - 10));
    assert_eq!(saturating_add(&r, &RectI16::of(0, 0, 0, 20)), RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX));
}

#[test]
fn saturating_add_edge_out_of_bounds() {
    let r = RectI16::largest();
    assert_eq!(saturating_add(&r, &RectI16::of(-1, 0, 0, 0)), RectI16::largest());
    assert_eq!(saturating_add(&r, &RectI16::of(0, -1, 0, 0)), RectI16::largest());
    assert_eq!(saturating_add(&r, &RectI16::of(0, 0, 1, 0)), RectI16::largest());
    assert_eq!(saturating_add(&r, &RectI16::of(0, 0, 0, 1)), RectI16::largest());
}

#[test]
fn saturating_add_limits_out_of_bounds() {
    let r = RectI16::largest();
    assert_eq!(saturating_add(&r, &RectI16::of(i16::MIN, 0, 0, 0)), RectI16::largest());
    assert_eq!(saturating_add(&r, &RectI16::of(0, i16::MIN, 0, 0)), RectI16::largest());
    assert_eq!(saturating_add(&r, &RectI16::of(0, 0, i16::MAX, 0)), RectI16::largest());
    assert_eq!(saturating_add(&r, &RectI16::of(0, 0, 0, i16::MAX)), RectI16::largest());
}

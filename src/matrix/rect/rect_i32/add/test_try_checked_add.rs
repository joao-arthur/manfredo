use super::try_checked_add;
use crate::matrix::rect::rect_i32::RectI32;

#[test]
fn test_try_checked_add() {
    assert_eq!(try_checked_add(&RectI32::of(0, 0, 12, 15), &RectI32::of(5, 4, 3, 2)), Some(RectI32::of(5, 4, 15, 17)));
    assert_eq!(try_checked_add(&RectI32::of(5, 4, 15, 17), &RectI32::of(-14, -13, -12, -11)), Some(RectI32::of(-9, -9, 3, 6)));
}

#[test]
fn try_checked_add_to_bounds() {
    assert_eq!(try_checked_add(&RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX - 2, i32::MAX - 5), &RectI32::of(-2, -5, 2, 5)), Some(RectI32::largest()));
    assert_eq!(try_checked_add(&RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX), &RectI32::of(-2, -5, 0, 0)), Some(RectI32::largest()));
    assert_eq!(try_checked_add(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5), &RectI32::of(0, 0, 2, 5)), Some(RectI32::largest()));
}

#[test]
fn try_checked_add_out_of_bounds() {
    let r = RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 10, i32::MAX - 10);
    assert_eq!(try_checked_add(&r, &RectI32::of(-20, 0, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI32::of(0, -20, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI32::of(0, 0, 20, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI32::of(0, 0, 0, 20)), None);
}

#[test]
fn try_checked_add_edge_out_of_bounds() {
    let r = RectI32::largest();
    assert_eq!(try_checked_add(&r, &RectI32::of(-1, 0, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI32::of(0, -1, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI32::of(0, 0, 1, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI32::of(0, 0, 0, 1)), None);
}

#[test]
fn try_checked_add_limits_out_of_bounds() {
    let r = RectI32::largest();
    assert_eq!(try_checked_add(&r, &RectI32::of(i32::MIN, 0, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI32::of(0, i32::MIN, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI32::of(0, 0, i32::MAX, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI32::of(0, 0, 0, i32::MAX)), None);
}

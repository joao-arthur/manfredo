use super::try_checked_add_assign;
use crate::matrix::rect::rect_i64::RectI64;

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

#[test]
fn test() {
    let mut r = RectI64::of(-7, 9, -12, 15);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(5, 4, 3, 2)), Some(()));
    assert_eq!(r, RectI64::of(-2, 13, -9, 17));
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(9, -10, 11, -12)), Some(()));
    assert_eq!(r, RectI64::of(7, 3, 2, 5));
}

#[test]
fn to_bounds() {
    let mut r = RectI64::of(MIN + 2, MIN + 5, MAX - 2, MAX - 5);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(-2, -5, 2, 5)), Some(()));
    assert_eq!(r, RectI64::largest());

    let mut r_min = RectI64::of(MIN + 2, MIN + 5, MAX, MAX);
    assert_eq!(try_checked_add_assign(&mut r_min, &RectI64::of(-2, -5, 0, 0)), Some(()));
    assert_eq!(r_min, RectI64::largest());

    let mut r_max = RectI64::of(MIN, MIN, MAX - 2, MAX - 5);
    assert_eq!(try_checked_add_assign(&mut r_max, &RectI64::of(0, 0, 2, 5)), Some(()));
    assert_eq!(r_max, RectI64::largest());
}

#[test]
fn out_of_bounds() {
    let mut r = RectI64::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(-20, 0, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(0, -20, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(0, 0, 20, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(0, 0, 0, 20)), None);
    assert_eq!(r, RectI64::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10));
}

#[test]
fn edge_out_of_bounds() {
    let mut r = RectI64::largest();
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(-1, 0, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(0, -1, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(0, 0, 1, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(0, 0, 0, 1)), None);
    assert_eq!(r, RectI64::largest());
}

#[test]
fn limits_out_of_bounds() {
    let mut r = RectI64::largest();
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(MIN, 0, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(0, MIN, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(0, 0, MAX, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(0, 0, 0, MAX)), None);
    assert_eq!(r, RectI64::largest());
}

use super::wrapping_add_assign;
use crate::cartesian::rect::rect_i64::RectI64;

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

#[test]
fn test() {
    let mut r = RectI64::of(-7, 9, -12, 15);
    wrapping_add_assign(&mut r, &RectI64::of(5, 4, 3, 2));
    assert_eq!(r, RectI64::of(-2, 13, -9, 17));
    wrapping_add_assign(&mut r, &RectI64::of(9, -10, 11, -12));
    assert_eq!(r, RectI64::of(7, 3, 2, 5));
}

#[test]
fn to_bounds() {
    let mut r = RectI64::of(MIN + 2, MIN + 5, MAX - 2, MAX - 5);
    wrapping_add_assign(&mut r, &RectI64::of(-2, -5, 2, 5));
    assert_eq!(r, RectI64::largest());

    let mut r_min = RectI64::of(MIN + 2, MIN + 5, MAX, MAX);
    wrapping_add_assign(&mut r_min, &RectI64::of(-2, -5, 0, 0));
    assert_eq!(r_min, RectI64::largest());

    let mut r_max = RectI64::of(MIN, MIN, MAX - 2, MAX - 5);
    wrapping_add_assign(&mut r_max, &RectI64::of(0, 0, 2, 5));
    assert_eq!(r_max, RectI64::largest());
}

#[test]
fn out_of_bounds() {
    let mut r1 = RectI64::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    wrapping_add_assign(&mut r1, &RectI64::of(-20, 0, 0, 0));
    assert_eq!(r1, RectI64::of(MAX - 9, MIN + 10, MAX - 10, MAX - 10));

    let mut r2 = RectI64::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    wrapping_add_assign(&mut r2, &RectI64::of(0, -20, 0, 0));
    assert_eq!(r2, RectI64::of(MIN + 10, MAX - 9, MAX - 10, MAX - 10));

    let mut r3 = RectI64::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    wrapping_add_assign(&mut r3, &RectI64::of(0, 0, 20, 0));
    assert_eq!(r3, RectI64::of(MIN + 10, MIN + 10, MIN + 9, MAX - 10));

    let mut r4 = RectI64::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    wrapping_add_assign(&mut r4, &RectI64::of(0, 0, 0, 20));
    assert_eq!(r4, RectI64::of(MIN + 10, MIN + 10, MAX - 10, MIN + 9));
}

#[test]
fn edge_out_of_bounds() {
    let mut r1 = RectI64::largest();
    wrapping_add_assign(&mut r1, &RectI64::of(-1, 0, 0, 0));
    assert_eq!(r1, RectI64::of(MAX, MIN, MAX, MAX));

    let mut r2 = RectI64::largest();
    wrapping_add_assign(&mut r2, &RectI64::of(0, -1, 0, 0));
    assert_eq!(r2, RectI64::of(MIN, MAX, MAX, MAX));

    let mut r3 = RectI64::largest();
    wrapping_add_assign(&mut r3, &RectI64::of(0, 0, 1, 0));
    assert_eq!(r3, RectI64::of(MIN, MIN, MIN, MAX));

    let mut r4 = RectI64::largest();
    wrapping_add_assign(&mut r4, &RectI64::of(0, 0, 0, 1));
    assert_eq!(r4, RectI64::of(MIN, MIN, MAX, MIN));
}

#[test]
fn limits_out_of_bounds() {
    let mut r1 = RectI64::largest();
    wrapping_add_assign(&mut r1, &RectI64::of(MIN, 0, 0, 0));
    assert_eq!(r1, RectI64::of(0, MIN, MAX, MAX));

    let mut r2 = RectI64::largest();
    wrapping_add_assign(&mut r2, &RectI64::of(0, MIN, 0, 0));
    assert_eq!(r2, RectI64::of(MIN, 0, MAX, MAX));

    let mut r3 = RectI64::largest();
    wrapping_add_assign(&mut r3, &RectI64::of(0, 0, MAX, 0));
    assert_eq!(r3, RectI64::of(MIN, MIN, -2, MAX));

    let mut r4 = RectI64::largest();
    wrapping_add_assign(&mut r4, &RectI64::of(0, 0, 0, MAX));
    assert_eq!(r4, RectI64::of(MIN, MIN, MAX, -2));
}

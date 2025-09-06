use super::wrapping_add_assign;
use crate::matrix::rect::rect_i8::RectI8;

#[test]
fn test() {
    let mut r = RectI8::of(-7, 9, -12, 15);
    wrapping_add_assign(&mut r, &RectI8::of(5, 4, 3, 2));
    assert_eq!(r, RectI8::of(-2, 13, -9, 17));
    wrapping_add_assign(&mut r, &RectI8::of(9, -10, 11, -12));
    assert_eq!(r, RectI8::of(7, 3, 2, 5));
}

#[test]
fn to_bounds() {
    let mut r = RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX - 2, i8::MAX - 5);
    wrapping_add_assign(&mut r, &RectI8::of(-2, -5, 2, 5));
    assert_eq!(r, RectI8::largest());

    let mut r_min = RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX);
    wrapping_add_assign(&mut r_min, &RectI8::of(-2, -5, 0, 0));
    assert_eq!(r_min, RectI8::largest());

    let mut r_max = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5);
    wrapping_add_assign(&mut r_max, &RectI8::of(0, 0, 2, 5));
    assert_eq!(r_max, RectI8::largest());
}

#[test]
fn out_of_bounds() {
    let mut r1 = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10);
    wrapping_add_assign(&mut r1, &RectI8::of(-20, 0, 0, 0));
    assert_eq!(r1, RectI8::of(i8::MAX - 9, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10));

    let mut r2 = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10);
    wrapping_add_assign(&mut r2, &RectI8::of(0, -20, 0, 0));
    assert_eq!(r2, RectI8::of(i8::MIN + 10, i8::MAX - 9, i8::MAX - 10, i8::MAX - 10));

    let mut r3 = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10);
    wrapping_add_assign(&mut r3, &RectI8::of(0, 0, 20, 0));
    assert_eq!(r3, RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MIN + 9, i8::MAX - 10));

    let mut r4 = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10);
    wrapping_add_assign(&mut r4, &RectI8::of(0, 0, 0, 20));
    assert_eq!(r4, RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MIN + 9));
}

#[test]
fn edge_out_of_bounds() {
    let mut r1 = RectI8::largest();
    wrapping_add_assign(&mut r1, &RectI8::of(-1, 0, 0, 0));
    assert_eq!(r1, RectI8::of(i8::MAX, i8::MIN, i8::MAX, i8::MAX));

    let mut r2 = RectI8::largest();
    wrapping_add_assign(&mut r2, &RectI8::of(0, -1, 0, 0));
    assert_eq!(r2, RectI8::of(i8::MIN, i8::MAX, i8::MAX, i8::MAX));

    let mut r3 = RectI8::largest();
    wrapping_add_assign(&mut r3, &RectI8::of(0, 0, 1, 0));
    assert_eq!(r3, RectI8::of(i8::MIN, i8::MIN, i8::MIN, i8::MAX));

    let mut r4 = RectI8::largest();
    wrapping_add_assign(&mut r4, &RectI8::of(0, 0, 0, 1));
    assert_eq!(r4, RectI8::of(i8::MIN, i8::MIN, i8::MAX, i8::MIN));
}

#[test]
fn limits_out_of_bounds() {
    let mut r1 = RectI8::largest();
    wrapping_add_assign(&mut r1, &RectI8::of(i8::MIN, 0, 0, 0));
    assert_eq!(r1, RectI8::of(0, i8::MIN, i8::MAX, i8::MAX));

    let mut r2 = RectI8::largest();
    wrapping_add_assign(&mut r2, &RectI8::of(0, i8::MIN, 0, 0));
    assert_eq!(r2, RectI8::of(i8::MIN, 0, i8::MAX, i8::MAX));

    let mut r3 = RectI8::largest();
    wrapping_add_assign(&mut r3, &RectI8::of(0, 0, i8::MAX, 0));
    assert_eq!(r3, RectI8::of(i8::MIN, i8::MIN, -2, i8::MAX));

    let mut r4 = RectI8::largest();
    wrapping_add_assign(&mut r4, &RectI8::of(0, 0, 0, i8::MAX));
    assert_eq!(r4, RectI8::of(i8::MIN, i8::MIN, i8::MAX, -2));
}

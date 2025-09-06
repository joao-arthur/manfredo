use super::wrapping_add_assign;
use crate::cartesian::rect::rect_i8::RectI8;

#[test]
fn test() {
    let mut r = RectI8::of(0, 0, 12, 15);
    wrapping_add_assign(&mut r, &RectI8::of(5, 4, 3, 2));
    assert_eq!(r, RectI8::of(5, 4, 15, 17));
    wrapping_add_assign(&mut r, &RectI8::of(-14, -13, -12, -11));
    assert_eq!(r, RectI8::of(-9, -9, 3, 6));
}

#[test]
fn small_rect_to_bounds() {
    let mut min_r = RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MIN + 12, i8::MIN + 15);
    wrapping_add_assign(&mut min_r, &RectI8::of(-2, -5, 9, 7));
    assert_eq!(min_r, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 21, i8::MIN + 22));

    let mut max_r = RectI8::of(i8::MAX - 12, i8::MAX - 15, i8::MAX - 2, i8::MAX - 5);
    wrapping_add_assign(&mut max_r, &RectI8::of(-9, -7, 2, 5));
    assert_eq!(max_r, RectI8::of(i8::MAX - 21, i8::MAX - 22, i8::MAX, i8::MAX));
}

#[test]
fn big_rect_to_bounds() {
    let mut r = RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX - 2, i8::MAX - 5);
    wrapping_add_assign(&mut r, &RectI8::of(-2, -5, 2, 5));
    assert_eq!(r, RectI8::largest());

    let mut min_r = RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX);
    wrapping_add_assign(&mut min_r, &RectI8::of(-2, -5, 0, 0));
    assert_eq!(min_r, RectI8::largest());

    let mut max_r = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5);
    wrapping_add_assign(&mut max_r, &RectI8::of(0, 0, 2, 5));
    assert_eq!(max_r, RectI8::largest());
}

#[test]
fn small_rect_out_of_bounds() {
    let mut r_min = RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MIN + 20, i8::MIN + 30);
    wrapping_add_assign(&mut r_min, &RectI8::of(-20, -20, 0, 0));
    assert_eq!(r_min, RectI8::of(i8::MAX - 9, i8::MAX - 14, i8::MIN + 20, i8::MIN + 30));

    let mut r_max = RectI8::of(i8::MAX - 20, i8::MAX - 30, i8::MAX - 5, i8::MAX - 10);
    wrapping_add_assign(&mut r_max, &RectI8::of(0, 0, 20, 20));
    assert_eq!(r_max, RectI8::of(i8::MAX - 20, i8::MAX - 30, i8::MIN + 14, i8::MIN + 9));
}

#[test]
fn big_rect_out_of_bounds() {
    let mut r_min = RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MAX, i8::MAX);
    wrapping_add_assign(&mut r_min, &RectI8::of(-20, -20, 0, 0));
    assert_eq!(r_min, RectI8::of(i8::MAX - 9, i8::MAX - 14, i8::MAX, i8::MAX));

    let mut r_max = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 5, i8::MAX - 10);
    wrapping_add_assign(&mut r_max, &RectI8::of(0, 0, 20, 20));
    assert_eq!(r_max, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 14, i8::MIN + 9));
}

#[test]
fn small_rect_limits_out_of_bounds() {
    let mut r_min = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MIN + 10, i8::MIN + 10);
    wrapping_add_assign(&mut r_min, &RectI8::min());
    assert_eq!(r_min, RectI8::of(1, 1, 10, 10));

    let mut r_max = RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX - 1, i8::MAX - 1);
    wrapping_add_assign(&mut r_max, &RectI8::max());
    assert_eq!(r_max, RectI8::of(-12, -12, -3, -3));
}

#[test]
fn big_rect_limits_out_of_bounds() {
    let mut r1 = RectI8::largest();
    wrapping_add_assign(&mut r1, &RectI8::min());
    assert_eq!(r1, RectI8::of(0, 0, -1, -1));

    let mut r2 = RectI8::largest();
    wrapping_add_assign(&mut r2, &RectI8::max());
    assert_eq!(r2, RectI8::of(-1, -1, -2, -2));

    let mut r_min = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX);
    wrapping_add_assign(&mut r_min, &RectI8::min());
    assert_eq!(r_min, RectI8::of(1, 1, -1, -1));

    let mut r_max = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1);
    wrapping_add_assign(&mut r_max, &RectI8::max());
    assert_eq!(r_max, RectI8::of(-1, -1, -3, -3));
}

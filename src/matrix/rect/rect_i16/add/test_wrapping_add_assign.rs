use super::wrapping_add_assign;
use crate::matrix::rect::rect_i16::RectI16;

#[test]
fn test() {
    let mut r = RectI16::of(0, 0, 12, 15);
    wrapping_add_assign(&mut r, &RectI16::of(5, 4, 3, 2));
    assert_eq!(r, RectI16::of(5, 4, 15, 17));
    wrapping_add_assign(&mut r, &RectI16::of(-14, -13, -12, -11));
    assert_eq!(r, RectI16::of(-9, -9, 3, 6));
}

#[test]
fn small_rect_to_bounds() {
    let mut min_r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MIN + 12, i16::MIN + 15);
    wrapping_add_assign(&mut min_r, &RectI16::of(-2, -5, 9, 7));
    assert_eq!(min_r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 21, i16::MIN + 22));

    let mut max_r = RectI16::of(i16::MAX - 12, i16::MAX - 15, i16::MAX - 2, i16::MAX - 5);
    wrapping_add_assign(&mut max_r, &RectI16::of(-9, -7, 2, 5));
    assert_eq!(max_r, RectI16::of(i16::MAX - 21, i16::MAX - 22, i16::MAX, i16::MAX));
}

#[test]
fn big_rect_to_bounds() {
    let mut r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX - 2, i16::MAX - 5);
    wrapping_add_assign(&mut r, &RectI16::of(-2, -5, 2, 5));
    assert_eq!(r, RectI16::largest());

    let mut min_r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX);
    wrapping_add_assign(&mut min_r, &RectI16::of(-2, -5, 0, 0));
    assert_eq!(min_r, RectI16::largest());

    let mut max_r = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5);
    wrapping_add_assign(&mut max_r, &RectI16::of(0, 0, 2, 5));
    assert_eq!(max_r, RectI16::largest());
}

#[test]
fn small_rect_out_of_bounds() {
    let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MIN + 20, i16::MIN + 30);
    wrapping_add_assign(&mut r_min, &RectI16::of(-20, -20, 0, 0));
    assert_eq!(r_min, RectI16::of(i16::MAX - 9, i16::MAX - 14, i16::MIN + 20, i16::MIN + 30));

    let mut r_max = RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX - 5, i16::MAX - 10);
    wrapping_add_assign(&mut r_max, &RectI16::of(0, 0, 20, 20));
    assert_eq!(r_max, RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MIN + 14, i16::MIN + 9));
}

#[test]
fn big_rect_out_of_bounds() {
    let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MAX, i16::MAX);
    wrapping_add_assign(&mut r_min, &RectI16::of(-20, -20, 0, 0));
    assert_eq!(r_min, RectI16::of(i16::MAX - 9, i16::MAX - 14, i16::MAX, i16::MAX));

    let mut r_max = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 5, i16::MAX - 10);
    wrapping_add_assign(&mut r_max, &RectI16::of(0, 0, 20, 20));
    assert_eq!(r_max, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 14, i16::MIN + 9));
}

#[test]
fn small_rect_limits_out_of_bounds() {
    let mut r_min = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10);
    wrapping_add_assign(&mut r_min, &RectI16::min());
    assert_eq!(r_min, RectI16::of(1, 1, 10, 10));

    let mut r_max = RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1);
    wrapping_add_assign(&mut r_max, &RectI16::max());
    assert_eq!(r_max, RectI16::of(-12, -12, -3, -3));
}

#[test]
fn big_rect_limits_out_of_bounds() {
    let mut r1 = RectI16::largest();
    wrapping_add_assign(&mut r1, &RectI16::min());
    assert_eq!(r1, RectI16::of(0, 0, -1, -1));

    let mut r2 = RectI16::largest();
    wrapping_add_assign(&mut r2, &RectI16::max());
    assert_eq!(r2, RectI16::of(-1, -1, -2, -2));

    let mut r_min = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX);
    wrapping_add_assign(&mut r_min, &RectI16::min());
    assert_eq!(r_min, RectI16::of(1, 1, -1, -1));

    let mut r_max = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1);
    wrapping_add_assign(&mut r_max, &RectI16::max());
    assert_eq!(r_max, RectI16::of(-1, -1, -3, -3));
}

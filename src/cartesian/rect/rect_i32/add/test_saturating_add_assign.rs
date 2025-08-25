use super::saturating_add_assign;
use crate::cartesian::rect::rect_i32::RectI32;

#[test]
fn test_saturating_add_assign() {
    let mut r = RectI32::of(0, 0, 12, 15);
    saturating_add_assign(&mut r, &RectI32::of(5, 4, 3, 2));
    assert_eq!(r, RectI32::of(5, 4, 15, 17));
    saturating_add_assign(&mut r, &RectI32::of(-14, -13, -12, -11));
    assert_eq!(r, RectI32::of(-9, -9, 3, 6));
}

#[test]
fn saturating_add_assign_to_bounds() {
    let mut r = RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX - 2, i32::MAX - 5);
    saturating_add_assign(&mut r, &RectI32::of(-2, -5, 2, 5));
    assert_eq!(r, RectI32::largest());

    let mut r_min = RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX);
    saturating_add_assign(&mut r_min, &RectI32::of(-2, -5, 0, 0));
    assert_eq!(r_min, RectI32::largest());

    let mut r_max = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5);
    saturating_add_assign(&mut r_max, &RectI32::of(0, 0, 2, 5));
    assert_eq!(r_max, RectI32::largest());
}

#[test]
fn saturating_add_assign_edge_out_of_bounds() {
    let mut r = RectI32::largest();
    saturating_add_assign(&mut r, &RectI32::of(-1, 0, 0, 0));
    assert_eq!(r, RectI32::largest());
    saturating_add_assign(&mut r, &RectI32::of(0, -1, 0, 0));
    assert_eq!(r, RectI32::largest());
    saturating_add_assign(&mut r, &RectI32::of(0, 0, 1, 0));
    assert_eq!(r, RectI32::largest());
    saturating_add_assign(&mut r, &RectI32::of(0, 0, 0, 1));
    assert_eq!(r, RectI32::largest());
}

#[test]
fn saturating_add_assign_out_of_bounds() {
    let mut r1 = RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 10, i32::MAX - 10);
    saturating_add_assign(&mut r1, &RectI32::of(-20, 0, 0, 0));
    assert_eq!(r1, RectI32::of(i32::MIN, i32::MIN + 10, i32::MAX - 10, i32::MAX - 10));

    let mut r2 = RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 10, i32::MAX - 10);
    saturating_add_assign(&mut r2, &RectI32::of(0, -20, 0, 0));
    assert_eq!(r2, RectI32::of(i32::MIN + 10, i32::MIN, i32::MAX - 10, i32::MAX - 10));

    let mut r3 = RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 10, i32::MAX - 10);
    saturating_add_assign(&mut r3, &RectI32::of(0, 0, 20, 0));
    assert_eq!(r3, RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX, i32::MAX - 10));

    let mut r4 = RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 10, i32::MAX - 10);
    saturating_add_assign(&mut r4, &RectI32::of(0, 0, 0, 20));
    assert_eq!(r4, RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 10, i32::MAX));
}

#[test]
fn saturating_add_assign_limits_out_of_bounds() {
    let mut r = RectI32::largest();
    saturating_add_assign(&mut r, &RectI32::of(i32::MIN, 0, 0, 0));
    assert_eq!(r, RectI32::largest());
    saturating_add_assign(&mut r, &RectI32::of(0, i32::MIN, 0, 0));
    assert_eq!(r, RectI32::largest());
    saturating_add_assign(&mut r, &RectI32::of(0, 0, i32::MAX, 0));
    assert_eq!(r, RectI32::largest());
    saturating_add_assign(&mut r, &RectI32::of(0, 0, 0, i32::MAX));
    assert_eq!(r, RectI32::largest());
}

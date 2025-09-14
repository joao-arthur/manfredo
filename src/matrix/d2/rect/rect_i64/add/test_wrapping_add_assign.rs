use super::wrapping_add_assign;
use crate::matrix::d2::rect::rect_i64::Rect;

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

#[test]
fn test() {
    let mut r = Rect::of(-7, 9, -12, 15);
    wrapping_add_assign(&mut r, &Rect::of(5, 4, 3, 2));
    assert_eq!(r, Rect::of(-2, 13, -9, 17));
    wrapping_add_assign(&mut r, &Rect::of(9, -10, 11, -12));
    assert_eq!(r, Rect::of(7, 3, 2, 5));
}

#[test]
fn to_bounds() {
    let mut r = Rect::of(MIN + 2, MIN + 5, MAX - 2, MAX - 5);
    wrapping_add_assign(&mut r, &Rect::of(-2, -5, 2, 5));
    assert_eq!(r, Rect::largest());

    let mut r_min = Rect::of(MIN + 2, MIN + 5, MAX, MAX);
    wrapping_add_assign(&mut r_min, &Rect::of(-2, -5, 0, 0));
    assert_eq!(r_min, Rect::largest());

    let mut r_max = Rect::of(MIN, MIN, MAX - 2, MAX - 5);
    wrapping_add_assign(&mut r_max, &Rect::of(0, 0, 2, 5));
    assert_eq!(r_max, Rect::largest());
}

#[test]
fn out_of_bounds() {
    let mut r1 = Rect::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    wrapping_add_assign(&mut r1, &Rect::of(-20, 0, 0, 0));
    assert_eq!(r1, Rect::of(MAX - 9, MIN + 10, MAX - 10, MAX - 10));

    let mut r2 = Rect::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    wrapping_add_assign(&mut r2, &Rect::of(0, -20, 0, 0));
    assert_eq!(r2, Rect::of(MIN + 10, MAX - 9, MAX - 10, MAX - 10));

    let mut r3 = Rect::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    wrapping_add_assign(&mut r3, &Rect::of(0, 0, 20, 0));
    assert_eq!(r3, Rect::of(MIN + 10, MIN + 10, MIN + 9, MAX - 10));

    let mut r4 = Rect::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    wrapping_add_assign(&mut r4, &Rect::of(0, 0, 0, 20));
    assert_eq!(r4, Rect::of(MIN + 10, MIN + 10, MAX - 10, MIN + 9));
}

#[test]
fn edge_out_of_bounds() {
    let mut r1 = Rect::largest();
    wrapping_add_assign(&mut r1, &Rect::of(-1, 0, 0, 0));
    assert_eq!(r1, Rect::of(MAX, MIN, MAX, MAX));

    let mut r2 = Rect::largest();
    wrapping_add_assign(&mut r2, &Rect::of(0, -1, 0, 0));
    assert_eq!(r2, Rect::of(MIN, MAX, MAX, MAX));

    let mut r3 = Rect::largest();
    wrapping_add_assign(&mut r3, &Rect::of(0, 0, 1, 0));
    assert_eq!(r3, Rect::of(MIN, MIN, MIN, MAX));

    let mut r4 = Rect::largest();
    wrapping_add_assign(&mut r4, &Rect::of(0, 0, 0, 1));
    assert_eq!(r4, Rect::of(MIN, MIN, MAX, MIN));
}

#[test]
fn limits_out_of_bounds() {
    let mut r1 = Rect::largest();
    wrapping_add_assign(&mut r1, &Rect::of(MIN, 0, 0, 0));
    assert_eq!(r1, Rect::of(0, MIN, MAX, MAX));

    let mut r2 = Rect::largest();
    wrapping_add_assign(&mut r2, &Rect::of(0, MIN, 0, 0));
    assert_eq!(r2, Rect::of(MIN, 0, MAX, MAX));

    let mut r3 = Rect::largest();
    wrapping_add_assign(&mut r3, &Rect::of(0, 0, MAX, 0));
    assert_eq!(r3, Rect::of(MIN, MIN, -2, MAX));

    let mut r4 = Rect::largest();
    wrapping_add_assign(&mut r4, &Rect::of(0, 0, 0, MAX));
    assert_eq!(r4, Rect::of(MIN, MIN, MAX, -2));
}

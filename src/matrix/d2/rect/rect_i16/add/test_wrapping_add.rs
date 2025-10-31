use super::wrapping_add;
use crate::matrix::{
    d1::point::point_i16::{MAX, MIN},
    d2::rect::rect_i16::Rect,
};

#[test]
fn test() {
    assert_eq!(wrapping_add(&Rect::of((-7, 9), (-12, 15)), &Rect::of((5, 4), (3, 2))), Rect::of((-2, 13), (-9, 17)));
    assert_eq!(wrapping_add(&Rect::of((-2, 13), (-9, 17)), &Rect::of((9, -10), (11, -12))), Rect::of((7, 3), (2, 5)));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&Rect::of((MIN + 2, MIN + 5), (MAX - 2, MAX - 5)), &Rect::of((-2, -5), (2, 5))), Rect::largest());
    assert_eq!(wrapping_add(&Rect::of((MIN + 2, MIN + 5), (MAX, MAX)), &Rect::of((-2, -5), (0, 0))), Rect::largest());
    assert_eq!(wrapping_add(&Rect::of((MIN, MIN), (MAX - 2, MAX - 5)), &Rect::of((0, 0), (2, 5))), Rect::largest());
}

#[test]
fn out_of_bounds() {
    let r = Rect::of((MIN + 10, MIN + 10), (MAX - 10, MAX - 10));
    assert_eq!(wrapping_add(&r, &Rect::of((-20, 0), (0, 0))), Rect::of((MAX - 9, MIN + 10), (MAX - 10, MAX - 10)));
    assert_eq!(wrapping_add(&r, &Rect::of((0, -20), (0, 0))), Rect::of((MIN + 10, MAX - 9), (MAX - 10, MAX - 10)));
    assert_eq!(wrapping_add(&r, &Rect::of((0, 0), (20, 0))), Rect::of((MIN + 10, MIN + 10), (MIN + 9, MAX - 10)));
    assert_eq!(wrapping_add(&r, &Rect::of((0, 0), (0, 20))), Rect::of((MIN + 10, MIN + 10), (MAX - 10, MIN + 9)));
}

#[test]
fn edge_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(wrapping_add(&r, &Rect::of((-1, 0), (0, 0))), Rect::of((MAX, MIN), (MAX, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::of((0, -1), (0, 0))), Rect::of((MIN, MAX), (MAX, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::of((0, 0), (1, 0))), Rect::of((MIN, MIN), (MIN, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::of((0, 0), (0, 1))), Rect::of((MIN, MIN), (MAX, MIN)));
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(wrapping_add(&r, &Rect::of((MIN, 0), (0, 0))), Rect::of((0, MIN), (MAX, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::of((0, MIN), (0, 0))), Rect::of((MIN, 0), (MAX, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::of((0, 0), (MAX, 0))), Rect::of((MIN, MIN), (-2, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::of((0, 0), (0, MAX))), Rect::of((MIN, MIN), (MAX, -2)));
}

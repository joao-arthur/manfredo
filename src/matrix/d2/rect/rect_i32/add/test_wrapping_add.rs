use super::wrapping_add;
use crate::matrix::{
    d1::point::point_i32::{MAX, MIN},
    d2::rect::rect_i32::Rect,
};

#[test]
fn test() {
    assert_eq!(wrapping_add(&Rect::new((-7, 9), (-12, 15)), &Rect::new((5, 4), (3, 2))), Rect::new((-2, 13), (-9, 17)));
    assert_eq!(wrapping_add(&Rect::new((-2, 13), (-9, 17)), &Rect::new((9, -10), (11, -12))), Rect::new((7, 3), (2, 5)));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&Rect::new((MIN + 2, MIN + 5), (MAX - 2, MAX - 5)), &Rect::new((-2, -5), (2, 5))), Rect::largest());
    assert_eq!(wrapping_add(&Rect::new((MIN + 2, MIN + 5), (MAX, MAX)), &Rect::new((-2, -5), (0, 0))), Rect::largest());
    assert_eq!(wrapping_add(&Rect::new((MIN, MIN), (MAX - 2, MAX - 5)), &Rect::new((0, 0), (2, 5))), Rect::largest());
}

#[test]
fn out_of_bounds() {
    let r = Rect::new((MIN + 10, MIN + 10), (MAX - 10, MAX - 10));
    assert_eq!(wrapping_add(&r, &Rect::new((-20, 0), (0, 0))), Rect::new((MAX - 9, MIN + 10), (MAX - 10, MAX - 10)));
    assert_eq!(wrapping_add(&r, &Rect::new((0, -20), (0, 0))), Rect::new((MIN + 10, MAX - 9), (MAX - 10, MAX - 10)));
    assert_eq!(wrapping_add(&r, &Rect::new((0, 0), (20, 0))), Rect::new((MIN + 10, MIN + 10), (MIN + 9, MAX - 10)));
    assert_eq!(wrapping_add(&r, &Rect::new((0, 0), (0, 20))), Rect::new((MIN + 10, MIN + 10), (MAX - 10, MIN + 9)));
}

#[test]
fn edge_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(wrapping_add(&r, &Rect::new((-1, 0), (0, 0))), Rect::new((MAX, MIN), (MAX, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::new((0, -1), (0, 0))), Rect::new((MIN, MAX), (MAX, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::new((0, 0), (1, 0))), Rect::new((MIN, MIN), (MIN, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::new((0, 0), (0, 1))), Rect::new((MIN, MIN), (MAX, MIN)));
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(wrapping_add(&r, &Rect::new((MIN, 0), (0, 0))), Rect::new((0, MIN), (MAX, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::new((0, MIN), (0, 0))), Rect::new((MIN, 0), (MAX, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::new((0, 0), (MAX, 0))), Rect::new((MIN, MIN), (-2, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::new((0, 0), (0, MAX))), Rect::new((MIN, MIN), (MAX, -2)));
}

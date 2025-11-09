use super::wrapping_add;
use crate::cartesian::{
    d1::point::point_u32::MAX,
    d2::rect::{rect_i32::Rect as RectI, rect_u32::Rect},
};

#[test]
fn test() {
    assert_eq!(wrapping_add(&Rect::new((0, 0), (12, 15)), &RectI::new((5, 4), (3, 2))), Rect::new((5, 4), (15, 17)));
    assert_eq!(wrapping_add(&Rect::new((5, 4), (15, 20)), &RectI::new((-4, -3), (-2, -1))), Rect::new((1, 1), (13, 19)));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&Rect::new((2, 5), (MAX - 2, MAX - 5)), &RectI::new((-2, -5), (2, 5))), Rect::largest());
    assert_eq!(wrapping_add(&Rect::new((2, 5), (MAX, MAX)), &RectI::new((-2, -5), (0, 0))), Rect::largest());
    assert_eq!(wrapping_add(&Rect::new((0, 0), (MAX - 2, MAX - 5)), &RectI::new((0, 0), (2, 5))), Rect::largest());
}

#[test]
fn out_of_bounds() {
    let r = Rect::new((10, 10), (MAX - 10, MAX - 10));
    assert_eq!(wrapping_add(&r, &RectI::new((-20, 0), (0, 0))), Rect::new((MAX - 9, 10), (MAX - 10, MAX - 10)));
    assert_eq!(wrapping_add(&r, &RectI::new((0, -20), (0, 0))), Rect::new((10, MAX - 9), (MAX - 10, MAX - 10)));
    assert_eq!(wrapping_add(&r, &RectI::new((0, 0), (20, 0))), Rect::new((10, 10), (9, MAX - 10)));
    assert_eq!(wrapping_add(&r, &RectI::new((0, 0), (0, 20))), Rect::new((10, 10), (MAX - 10, 9)));
}

#[test]
fn edge_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(wrapping_add(&r, &RectI::new((-1, 0), (0, 0))), Rect::new((MAX, 0), (MAX, MAX)));
    assert_eq!(wrapping_add(&r, &RectI::new((0, -1), (0, 0))), Rect::new((0, MAX), (MAX, MAX)));
    assert_eq!(wrapping_add(&r, &RectI::new((0, 0), (1, 0))), Rect::new((0, 0), (0, MAX)));
    assert_eq!(wrapping_add(&r, &RectI::new((0, 0), (0, 1))), Rect::new((0, 0), (MAX, 0)));
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(wrapping_add(&r, &RectI::new((i32::MIN, 0), (0, 0))), Rect::new((MAX / 2 + 1, 0), (MAX, MAX)));
    assert_eq!(wrapping_add(&r, &RectI::new((0, i32::MIN), (0, 0))), Rect::new((0, MAX / 2 + 1), (MAX, MAX)));
    assert_eq!(wrapping_add(&r, &RectI::new((0, 0), (i32::MAX, 0))), Rect::new((0, 0), (MAX / 2 - 1, MAX)));
    assert_eq!(wrapping_add(&r, &RectI::new((0, 0), (0, i32::MAX))), Rect::new((0, 0), (MAX, MAX / 2 - 1)));
}

use super::try_checked_resize;
use crate::matrix::{
    d1::point::point_i32::{MAX, MIN},
    d2::rect::rect_i32::Rect,
};

#[test]
fn odd() {
    assert_eq!(try_checked_resize(&Rect::new((-5, -5), (5, 5)), 9), Some(Rect::new((-4, -4), (4, 4))));
    assert_eq!(try_checked_resize(&Rect::new((-4, -4), (4, 4)), 7), Some(Rect::new((-3, -3), (3, 3))));
    assert_eq!(try_checked_resize(&Rect::new((-3, -3), (3, 3)), 5), Some(Rect::new((-2, -2), (2, 2))));
    assert_eq!(try_checked_resize(&Rect::new((-2, -2), (2, 2)), 3), Some(Rect::new((-1, -1), (1, 1))));
    assert_eq!(try_checked_resize(&Rect::new((-1, -1), (1, 1)), 9), Some(Rect::new((-4, -4), (4, 4))));
}

#[test]
fn even() {
    assert_eq!(try_checked_resize(&Rect::new((-5, -5), (4, 4)), 10), Some(Rect::new((-5, -5), (4, 4))));
    assert_eq!(try_checked_resize(&Rect::new((-5, -5), (4, 4)), 8), Some(Rect::new((-4, -4), (3, 3))));
    assert_eq!(try_checked_resize(&Rect::new((-4, -4), (3, 3)), 6), Some(Rect::new((-3, -3), (2, 2))));
    assert_eq!(try_checked_resize(&Rect::new((-3, -3), (2, 2)), 4), Some(Rect::new((-2, -2), (1, 1))));
    assert_eq!(try_checked_resize(&Rect::new((-2, -2), (1, 1)), 8), Some(Rect::new((-4, -4), (3, 3))));
}

#[test]
fn small_size() {
    let r = Rect::new((10, 10), (20, 20));
    assert_eq!(try_checked_resize(&r, 0), None);
    assert_eq!(try_checked_resize(&r, 1), None);
    assert_eq!(try_checked_resize(&r, 2), None);
}

#[test]
fn same_size() {
    assert_eq!(try_checked_resize(&Rect::new((MIN, MIN), (MIN + 2, MIN + 2)), 3), Some(Rect::new((MIN, MIN), (MIN + 2, MIN + 2))));
    assert_eq!(try_checked_resize(&Rect::new((MIN, MIN), (MIN + 3, MIN + 3)), 4), Some(Rect::new((MIN, MIN), (MIN + 3, MIN + 3))));
    assert_eq!(try_checked_resize(&Rect::new((MAX - 2, MAX - 2), (MAX, MAX)), 3), Some(Rect::new((MAX - 2, MAX - 2), (MAX, MAX))));
    assert_eq!(try_checked_resize(&Rect::new((MAX - 3, MAX - 3), (MAX, MAX)), 4), Some(Rect::new((MAX - 3, MAX - 3), (MAX, MAX))));
}

#[test]
fn out_of_bounds() {
    assert_eq!(try_checked_resize(&Rect::new((MIN, MIN + 2), (MIN + 2, MIN + 4)), 5), None);
    assert_eq!(try_checked_resize(&Rect::new((MIN + 2, MIN), (MIN + 4, MIN + 2)), 5), None);
    assert_eq!(try_checked_resize(&Rect::new((MAX - 2, MAX - 4), (MAX, MAX - 2)), 5), None);
    assert_eq!(try_checked_resize(&Rect::new((MAX - 4, MAX - 2), (MAX - 2, MAX)), 5), None);
}

#[test]
fn small_rect_limits_out_of_bounds() {
    assert_eq!(try_checked_resize(&Rect::new((MIN, MIN + 2), (MIN + 2, MIN + 4)), u32::MAX), None);
    assert_eq!(try_checked_resize(&Rect::new((MIN + 2, MIN), (MIN + 4, MIN + 2)), u32::MAX), None);
    assert_eq!(try_checked_resize(&Rect::new((MAX - 2, MAX - 4), (MAX, MAX - 2)), u32::MAX), None);
    assert_eq!(try_checked_resize(&Rect::new((MAX - 4, MAX - 2), (MAX - 2, MAX)), u32::MAX), None);
}

#[test]
fn big_rect_limits_out_of_bounds() {
    assert_eq!(try_checked_resize(&Rect::new((MIN, MIN), (MAX - 1, MAX - 1)), u32::MAX), Some(Rect::new((MIN, MIN), (MAX - 1, MAX - 1))));
    assert_eq!(try_checked_resize(&Rect::new((MIN + 1, MIN + 1), (MAX, MAX)), u32::MAX), Some(Rect::new((MIN + 1, MIN + 1), (MAX, MAX))));
    assert_eq!(try_checked_resize(&Rect::largest(), u32::MAX), Some(Rect::new((MIN, MIN), (MAX - 1, MAX - 1))));
}

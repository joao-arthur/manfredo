use super::try_checked_resize_assign;
use crate::cartesian::d2::{
    point::point_f32::{MAX, MIN},
    rect::rect_f32::Rect,
};

#[test]
fn odd() {
    let mut r = Rect::of(-5.0, -5.0, 5.0, 5.0);
    assert_eq!(try_checked_resize_assign(&mut r, 9.0), Some(()));
    assert_eq!(r, Rect::of(-4.0, -4.0, 4.0, 4.0));
    assert_eq!(try_checked_resize_assign(&mut r, 7.0), Some(()));
    assert_eq!(r, Rect::of(-3.0, -3.0, 3.0, 3.0));
    assert_eq!(try_checked_resize_assign(&mut r, 5.0), Some(()));
    assert_eq!(r, Rect::of(-2.0, -2.0, 2.0, 2.0));
    assert_eq!(try_checked_resize_assign(&mut r, 3.0), Some(()));
    assert_eq!(r, Rect::of(-1.0, -1.0, 1.0, 1.0));
    assert_eq!(try_checked_resize_assign(&mut r, 9.0), Some(()));
    assert_eq!(r, Rect::of(-4.0, -4.0, 4.0, 4.0));
}

#[test]
fn even() {
    let mut r = Rect::of(-5.0, -5.0, 4.0, 4.0);
    assert_eq!(try_checked_resize_assign(&mut r, 10.0), Some(()));
    assert_eq!(r, Rect::of(-5.0, -5.0, 4.0, 4.0));
    assert_eq!(try_checked_resize_assign(&mut r, 8.0), Some(()));
    assert_eq!(r, Rect::of(-4.0, -4.0, 3.0, 3.0));
    assert_eq!(try_checked_resize_assign(&mut r, 6.0), Some(()));
    assert_eq!(r, Rect::of(-3.0, -3.0, 2.0, 2.0));
    assert_eq!(try_checked_resize_assign(&mut r, 4.0), Some(()));
    assert_eq!(r, Rect::of(-2.0, -2.0, 1.0, 1.0));
    assert_eq!(try_checked_resize_assign(&mut r, 8.0), Some(()));
    assert_eq!(r, Rect::of(-4.0, -4.0, 3.0, 3.0));
}

#[test]
fn small_size() {
    let mut r = Rect::of(10.0, 10.0, 20.0, 20.0);
    assert_eq!(try_checked_resize_assign(&mut r, 0.0), None);
    assert_eq!(try_checked_resize_assign(&mut r, 1.0), None);
    assert_eq!(try_checked_resize_assign(&mut r, 2.0), None);
    assert_eq!(try_checked_resize_assign(&mut r, MAX + 1.0), None);
    assert_eq!(try_checked_resize_assign(&mut r, MAX + 2.0), None);
    assert_eq!(try_checked_resize_assign(&mut r, MAX + 3.0), None);
    assert_eq!(r, Rect::of(10.0, 10.0, 20.0, 20.0));
}

#[test]
fn same_size() {
    let mut r_min_2 = Rect::of(MIN, MIN, MIN + 2.0, MIN + 2.0);
    assert_eq!(try_checked_resize_assign(&mut r_min_2, 3.0), Some(()));
    assert_eq!(r_min_2, Rect::of(MIN, MIN, MIN + 2.0, MIN + 2.0));

    let mut r_min_3 = Rect::of(MIN, MIN, MIN + 3.0, MIN + 3.0);
    assert_eq!(try_checked_resize_assign(&mut r_min_3, 4.0), Some(()));
    assert_eq!(r_min_3, Rect::of(MIN, MIN, MIN + 3.0, MIN + 3.0));

    let mut r_max_2 = Rect::of(MAX - 2.0, MAX - 2.0, MAX, MAX);
    assert_eq!(try_checked_resize_assign(&mut r_max_2, 3.0), Some(()));
    assert_eq!(r_max_2, Rect::of(MAX - 2.0, MAX - 2.0, MAX, MAX));

    let mut r_max_3 = Rect::of(MAX - 3.0, MAX - 3.0, MAX, MAX);
    assert_eq!(try_checked_resize_assign(&mut r_max_3, 4.0), Some(()));
    assert_eq!(r_max_3, Rect::of(MAX - 3.0, MAX - 3.0, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let mut r_min_x = Rect::of(MIN, MIN + 2.0, MIN + 2.0, MIN + 4.0);
    assert_eq!(try_checked_resize_assign(&mut r_min_x, 5.0), None);
    assert_eq!(r_min_x, Rect::of(MIN, MIN + 2.0, MIN + 2.0, MIN + 4.0));

    let mut r_min_y = Rect::of(MIN + 2.0, MIN, MIN + 4.0, MIN + 2.0);
    assert_eq!(try_checked_resize_assign(&mut r_min_y, 5.0), None);
    assert_eq!(r_min_y, Rect::of(MIN + 2.0, MIN, MIN + 4.0, MIN + 2.0));

    let mut r_max_x = Rect::of(MAX - 2.0, MAX - 4.0, MAX, MAX - 2.0);
    assert_eq!(try_checked_resize_assign(&mut r_max_x, 5.0), None);
    assert_eq!(r_max_x, Rect::of(MAX - 2.0, MAX - 4.0, MAX, MAX - 2.0));

    let mut r_max_y = Rect::of(MAX - 4.0, MAX - 2.0, MAX - 2.0, MAX);
    assert_eq!(try_checked_resize_assign(&mut r_max_y, 5.0), None);
    assert_eq!(r_max_y, Rect::of(MAX - 4.0, MAX - 2.0, MAX - 2.0, MAX));
}

#[test]
fn small_rect_limits_out_of_bounds() {
    let mut r_min_x = Rect::of(MIN, MIN + 2.0, MIN + 2.0, MIN + 4.0);
    assert_eq!(try_checked_resize_assign(&mut r_min_x, MAX), None);
    assert_eq!(r_min_x, Rect::of(MIN, MIN + 2.0, MIN + 2.0, MIN + 4.0));

    let mut r_min_y = Rect::of(MIN + 2.0, MIN, MIN + 4.0, MIN + 2.0);
    assert_eq!(try_checked_resize_assign(&mut r_min_y, MAX), None);
    assert_eq!(r_min_y, Rect::of(MIN + 2.0, MIN, MIN + 4.0, MIN + 2.0));

    let mut r_max_x = Rect::of(MAX - 2.0, MAX - 4.0, MAX, MAX - 2.0);
    assert_eq!(try_checked_resize_assign(&mut r_max_x, MAX), None);
    assert_eq!(r_max_x, Rect::of(MAX - 2.0, MAX - 4.0, MAX, MAX - 2.0));

    let mut r_max_y = Rect::of(MAX - 4.0, MAX - 2.0, MAX - 2.0, MAX);
    assert_eq!(try_checked_resize_assign(&mut r_max_y, MAX), None);
    assert_eq!(r_max_y, Rect::of(MAX - 4.0, MAX - 2.0, MAX - 2.0, MAX));
}

#[test]
fn big_rect_limits_out_of_bounds() {
    let mut r_min_1 = Rect::of(MIN, MIN, -2.0, -2.0);
    assert_eq!(try_checked_resize_assign(&mut r_min_1, MAX), Some(()));
    assert_eq!(r_min_1, Rect::of(MIN, MIN, -2.0, -2.0));

    let mut r_min_2 = Rect::of(MIN + 1.0, MIN + 1.0, -1.0, -1.0);
    assert_eq!(try_checked_resize_assign(&mut r_min_2, MAX), Some(()));
    assert_eq!(r_min_2, Rect::of(MIN + 1.0, MIN + 1.0, -1.0, -1.0));

    let mut r_min_3 = Rect::of(MIN + 2.0, MIN + 2.0, 0.0, 0.0);
    assert_eq!(try_checked_resize_assign(&mut r_min_3, MAX), Some(()));
    assert_eq!(r_min_3, Rect::of(MIN + 2.0, MIN + 2.0, 0.0, 0.0));

    let mut r_max_1 = Rect::of(0.0, 0.0, MAX - 1.0, MAX - 1.0);
    assert_eq!(try_checked_resize_assign(&mut r_max_1, MAX), Some(()));
    assert_eq!(r_max_1, Rect::of(0.0, 0.0, MAX - 1.0, MAX - 1.0));

    let mut r_max_2 = Rect::of(1.0, 1.0, MAX, MAX);
    assert_eq!(try_checked_resize_assign(&mut r_max_2, MAX), Some(()));
    assert_eq!(r_max_2, Rect::of(1.0, 1.0, MAX, MAX));
}

use super::try_wrapping_resize_assign;
use crate::matrix::rect::rect_u16::Rect;

const MAX: u16 = u16::MAX;

#[test]
fn odd() {
    let mut r = Rect::of(5, 5, 15, 15);
    assert_eq!(try_wrapping_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, Rect::of(6, 6, 14, 14));
    assert_eq!(try_wrapping_resize_assign(&mut r, 7), Some(()));
    assert_eq!(r, Rect::of(7, 7, 13, 13));
    assert_eq!(try_wrapping_resize_assign(&mut r, 5), Some(()));
    assert_eq!(r, Rect::of(8, 8, 12, 12));
    assert_eq!(try_wrapping_resize_assign(&mut r, 3), Some(()));
    assert_eq!(r, Rect::of(9, 9, 11, 11));
    assert_eq!(try_wrapping_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, Rect::of(6, 6, 14, 14));
}

#[test]
fn even() {
    let mut r = Rect::of(5, 5, 14, 14);
    assert_eq!(try_wrapping_resize_assign(&mut r, 10), Some(()));
    assert_eq!(r, Rect::of(5, 5, 14, 14));
    assert_eq!(try_wrapping_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, Rect::of(6, 6, 13, 13));
    assert_eq!(try_wrapping_resize_assign(&mut r, 6), Some(()));
    assert_eq!(r, Rect::of(7, 7, 12, 12));
    assert_eq!(try_wrapping_resize_assign(&mut r, 4), Some(()));
    assert_eq!(r, Rect::of(8, 8, 11, 11));
    assert_eq!(try_wrapping_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, Rect::of(6, 6, 13, 13));
}

#[test]
fn small_size() {
    let mut r = Rect::of(10, 10, 20, 20);
    assert_eq!(try_wrapping_resize_assign(&mut r, 0), None);
    assert_eq!(try_wrapping_resize_assign(&mut r, 1), None);
    assert_eq!(try_wrapping_resize_assign(&mut r, 2), None);
    assert_eq!(r, Rect::of(10, 10, 20, 20));
}

#[test]
fn same_size() {
    let mut r_min_2 = Rect::of(0, 0, 2, 2);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_2, 3), Some(()));
    assert_eq!(r_min_2, Rect::of(0, 0, 2, 2));

    let mut r_min_3 = Rect::of(0, 0, 3, 3);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_3, 4), Some(()));
    assert_eq!(r_min_3, Rect::of(0, 0, 3, 3));

    let mut r_max_2 = Rect::of(MAX - 2, MAX - 2, MAX, MAX);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_2, 3), Some(()));
    assert_eq!(r_max_2, Rect::of(MAX - 2, MAX - 2, MAX, MAX));

    let mut r_max_3 = Rect::of(MAX - 3, MAX - 3, MAX, MAX);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_3, 4), Some(()));
    assert_eq!(r_max_3, Rect::of(MAX - 3, MAX - 3, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let mut r_min_row = Rect::of(0, 2, 2, 4);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_row, 5), Some(()));
    assert_eq!(r_min_row, Rect::of(MAX, 1, 3, 5));

    let mut r_min_y = Rect::of(2, 0, 4, 2);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_y, 5), Some(()));
    assert_eq!(r_min_y, Rect::of(1, MAX, 5, 3));

    let mut r_max_x = Rect::of(MAX - 2, MAX - 4, MAX, MAX - 2);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_x, 5), Some(()));
    assert_eq!(r_max_x, Rect::of(MAX - 3, MAX - 5, 0, MAX - 1));

    let mut r_max_y = Rect::of(MAX - 4, MAX - 2, MAX - 2, MAX);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_y, 5), Some(()));
    assert_eq!(r_max_y, Rect::of(MAX - 5, MAX - 3, MAX - 1, 0));
}

#[test]
fn small_rect_limits_out_of_bounds() {
    let mut r_min_row = Rect::of(0, 2, 2, 4);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_row, MAX), Some(()));
    assert_eq!(r_min_row, Rect::of(MAX / 2 + 3, MAX / 2 + 5, MAX / 2 + 1, MAX / 2 + 3));

    let mut r_min_y = Rect::of(2, 0, 4, 2);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_y, MAX), Some(()));
    assert_eq!(r_min_y, Rect::of(MAX / 2 + 5, MAX / 2 + 3, MAX / 2 + 3, MAX / 2 + 1));

    let mut r_max_x = Rect::of(MAX - 2, MAX - 4, MAX, MAX - 2);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_x, MAX), Some(()));
    assert_eq!(r_max_x, Rect::of(MAX / 2, MAX / 2 - 2, MAX / 2 - 2, MAX / 2 - 4));

    let mut r_max_y = Rect::of(MAX - 4, MAX - 2, MAX - 2, MAX);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_y, MAX), Some(()));
    assert_eq!(r_max_y, Rect::of(MAX / 2 - 2, MAX / 2, MAX / 2 - 4, MAX / 2 - 2));
}

#[test]
fn big_rect_limits_out_of_bounds() {
    let mut r_odd_1 = Rect::of(0, 0, MAX - 1, MAX - 1);
    assert_eq!(try_wrapping_resize_assign(&mut r_odd_1, MAX), Some(()));
    assert_eq!(r_odd_1, Rect::of(0, 0, MAX - 1, MAX - 1));

    let mut r_odd_1 = Rect::of(1, 1, MAX, MAX);
    assert_eq!(try_wrapping_resize_assign(&mut r_odd_1, MAX), Some(()));
    assert_eq!(r_odd_1, Rect::of(1, 1, MAX, MAX));

    let mut r_even = Rect::largest();
    assert_eq!(try_wrapping_resize_assign(&mut r_even, MAX), Some(()));
    assert_eq!(r_even, Rect::of(0, 0, MAX - 1, MAX - 1));
}

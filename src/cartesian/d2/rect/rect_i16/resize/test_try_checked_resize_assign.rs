use super::try_checked_resize_assign;
use crate::cartesian::{
    d1::point::point_i16::{MAX, MIN},
    d2::rect::rect_i16::Rect,
};

#[test]
fn odd() {
    let mut r = Rect::new((-5, -5), (5, 5));
    assert_eq!(try_checked_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, Rect::new((-4, -4), (4, 4)));
    assert_eq!(try_checked_resize_assign(&mut r, 7), Some(()));
    assert_eq!(r, Rect::new((-3, -3), (3, 3)));
    assert_eq!(try_checked_resize_assign(&mut r, 5), Some(()));
    assert_eq!(r, Rect::new((-2, -2), (2, 2)));
    assert_eq!(try_checked_resize_assign(&mut r, 3), Some(()));
    assert_eq!(r, Rect::new((-1, -1), (1, 1)));
    assert_eq!(try_checked_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, Rect::new((-4, -4), (4, 4)));
}

#[test]
fn even() {
    let mut r = Rect::new((-5, -5), (4, 4));
    assert_eq!(try_checked_resize_assign(&mut r, 10), Some(()));
    assert_eq!(r, Rect::new((-5, -5), (4, 4)));
    assert_eq!(try_checked_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, Rect::new((-4, -4), (3, 3)));
    assert_eq!(try_checked_resize_assign(&mut r, 6), Some(()));
    assert_eq!(r, Rect::new((-3, -3), (2, 2)));
    assert_eq!(try_checked_resize_assign(&mut r, 4), Some(()));
    assert_eq!(r, Rect::new((-2, -2), (1, 1)));
    assert_eq!(try_checked_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, Rect::new((-4, -4), (3, 3)));
}

#[test]
fn small_size() {
    let mut r = Rect::new((10, 10), (20, 20));
    assert_eq!(try_checked_resize_assign(&mut r, 0), None);
    assert_eq!(try_checked_resize_assign(&mut r, 1), None);
    assert_eq!(try_checked_resize_assign(&mut r, 2), None);
    assert_eq!(r, Rect::new((10, 10), (20, 20)));
}

#[test]
fn same_size() {
    let mut r_min_2 = Rect::new((MIN, MIN), (MIN + 2, MIN + 2));
    assert_eq!(try_checked_resize_assign(&mut r_min_2, 3), Some(()));
    assert_eq!(r_min_2, Rect::new((MIN, MIN), (MIN + 2, MIN + 2)));

    let mut r_min_3 = Rect::new((MIN, MIN), (MIN + 3, MIN + 3));
    assert_eq!(try_checked_resize_assign(&mut r_min_3, 4), Some(()));
    assert_eq!(r_min_3, Rect::new((MIN, MIN), (MIN + 3, MIN + 3)));

    let mut r_max_2 = Rect::new((MAX - 2, MAX - 2), (MAX, MAX));
    assert_eq!(try_checked_resize_assign(&mut r_max_2, 3), Some(()));
    assert_eq!(r_max_2, Rect::new((MAX - 2, MAX - 2), (MAX, MAX)));

    let mut r_max_3 = Rect::new((MAX - 3, MAX - 3), (MAX, MAX));
    assert_eq!(try_checked_resize_assign(&mut r_max_3, 4), Some(()));
    assert_eq!(r_max_3, Rect::new((MAX - 3, MAX - 3), (MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    let mut r_min_x = Rect::new((MIN, MIN + 2), (MIN + 2, MIN + 4));
    assert_eq!(try_checked_resize_assign(&mut r_min_x, 5), None);
    assert_eq!(r_min_x, Rect::new((MIN, MIN + 2), (MIN + 2, MIN + 4)));

    let mut r_min_y = Rect::new((MIN + 2, MIN), (MIN + 4, MIN + 2));
    assert_eq!(try_checked_resize_assign(&mut r_min_y, 5), None);
    assert_eq!(r_min_y, Rect::new((MIN + 2, MIN), (MIN + 4, MIN + 2)));

    let mut r_max_x = Rect::new((MAX - 2, MAX - 4), (MAX, MAX - 2));
    assert_eq!(try_checked_resize_assign(&mut r_max_x, 5), None);
    assert_eq!(r_max_x, Rect::new((MAX - 2, MAX - 4), (MAX, MAX - 2)));

    let mut r_max_y = Rect::new((MAX - 4, MAX - 2), (MAX - 2, MAX));
    assert_eq!(try_checked_resize_assign(&mut r_max_y, 5), None);
    assert_eq!(r_max_y, Rect::new((MAX - 4, MAX - 2), (MAX - 2, MAX)));
}

#[test]
fn small_rect_limits_out_of_bounds() {
    let mut r_min_x = Rect::new((MIN, MIN + 2), (MIN + 2, MIN + 4));
    assert_eq!(try_checked_resize_assign(&mut r_min_x, u16::MAX), None);
    assert_eq!(r_min_x, Rect::new((MIN, MIN + 2), (MIN + 2, MIN + 4)));

    let mut r_min_y = Rect::new((MIN + 2, MIN), (MIN + 4, MIN + 2));
    assert_eq!(try_checked_resize_assign(&mut r_min_y, u16::MAX), None);
    assert_eq!(r_min_y, Rect::new((MIN + 2, MIN), (MIN + 4, MIN + 2)));

    let mut r_max_x = Rect::new((MAX - 2, MAX - 4), (MAX, MAX - 2));
    assert_eq!(try_checked_resize_assign(&mut r_max_x, u16::MAX), None);
    assert_eq!(r_max_x, Rect::new((MAX - 2, MAX - 4), (MAX, MAX - 2)));

    let mut r_max_y = Rect::new((MAX - 4, MAX - 2), (MAX - 2, MAX));
    assert_eq!(try_checked_resize_assign(&mut r_max_y, u16::MAX), None);
    assert_eq!(r_max_y, Rect::new((MAX - 4, MAX - 2), (MAX - 2, MAX)));
}

#[test]
fn big_rect_limits_out_of_bounds() {
    let mut r_odd_1 = Rect::new((MIN, MIN), (MAX - 1, MAX - 1));
    assert_eq!(try_checked_resize_assign(&mut r_odd_1, u16::MAX), Some(()));
    assert_eq!(r_odd_1, Rect::new((MIN, MIN), (MAX - 1, MAX - 1)));

    let mut r_odd_1 = Rect::new((MIN + 1, MIN + 1), (MAX, MAX));
    assert_eq!(try_checked_resize_assign(&mut r_odd_1, u16::MAX), Some(()));
    assert_eq!(r_odd_1, Rect::new((MIN + 1, MIN + 1), (MAX, MAX)));

    let mut r_even = Rect::largest();
    assert_eq!(try_checked_resize_assign(&mut r_even, u16::MAX), Some(()));
    assert_eq!(r_even, Rect::new((MIN, MIN), (MAX - 1, MAX - 1)));
}

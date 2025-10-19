use super::try_checked_inflate_assign;
use crate::cartesian::{d1::point::point_u32::MAX, d2::rect::rect_u32::Rect};

#[test]
fn min_bounds() {
    let mut r = Rect::of(7, 3, 9, 13);
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::of(6, 2, 10, 14));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::of(5, 1, 11, 15));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::of(4, 0, 12, 16));
}

#[test]
fn max_bounds() {
    let mut r = Rect::of(MAX - 33, MAX - 17, MAX - 5, MAX - 3);
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::of(MAX - 36, MAX - 20, MAX - 2, MAX));
}

#[test]
fn to_bounds() {
    let mut r = Rect::of(1, 1, MAX - 1, MAX - 1);
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::largest());

    let mut r_min_x = Rect::of(1, 10, MAX - 10, MAX - 10);
    assert_eq!(try_checked_inflate_assign(&mut r_min_x), Some(()));
    assert_eq!(r_min_x, Rect::of(0, 9, MAX - 9, MAX - 9));

    let mut r_min_y = Rect::of(10, 1, MAX - 10, MAX - 10);
    assert_eq!(try_checked_inflate_assign(&mut r_min_y), Some(()));
    assert_eq!(r_min_y, Rect::of(9, 0, MAX - 9, MAX - 9));

    let mut r_max_x = Rect::of(10, 10, MAX - 1, MAX - 10);
    assert_eq!(try_checked_inflate_assign(&mut r_max_x), Some(()));
    assert_eq!(r_max_x, Rect::of(9, 9, MAX, MAX - 9));

    let mut r_max_y = Rect::of(10, 10, MAX - 10, MAX - 1);
    assert_eq!(try_checked_inflate_assign(&mut r_max_y), Some(()));
    assert_eq!(r_max_y, Rect::of(9, 9, MAX - 9, MAX));
}

#[test]
fn out_of_bounds() {
    let mut r = Rect::largest();
    assert_eq!(try_checked_inflate_assign(&mut r), None);
    assert_eq!(r, Rect::largest());

    let mut r_min_x = Rect::of(0, 9, MAX - 9, MAX - 9);
    assert_eq!(try_checked_inflate_assign(&mut r_min_x), None);
    assert_eq!(r_min_x, Rect::of(0, 9, MAX - 9, MAX - 9));

    let mut r_min_y = Rect::of(9, 0, MAX - 9, MAX - 9);
    assert_eq!(try_checked_inflate_assign(&mut r_min_y), None);
    assert_eq!(r_min_y, Rect::of(9, 0, MAX - 9, MAX - 9));

    let mut r_max_x = Rect::of(9, 9, MAX, MAX - 9);
    assert_eq!(try_checked_inflate_assign(&mut r_max_x), None);
    assert_eq!(r_max_x, Rect::of(9, 9, MAX, MAX - 9));

    let mut r_max_y = Rect::of(9, 9, MAX - 9, MAX);
    assert_eq!(try_checked_inflate_assign(&mut r_max_y), None);
    assert_eq!(r_max_y, Rect::of(9, 9, MAX - 9, MAX));
}

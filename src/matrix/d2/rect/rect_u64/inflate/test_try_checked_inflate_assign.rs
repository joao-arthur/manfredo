use super::try_checked_inflate_assign;
use crate::matrix::{d1::point::point_u64::MAX, d2::rect::rect_u64::Rect};

#[test]
fn min_bounds() {
    let mut r = Rect::new((7, 3), (9, 13));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::new((6, 2), (10, 14)));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::new((5, 1), (11, 15)));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::new((4, 0), (12, 16)));
}

#[test]
fn max_bounds() {
    let mut r = Rect::new((MAX - 33, MAX - 17), (MAX - 5, MAX - 3));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::new((MAX - 34, MAX - 18), (MAX - 4, MAX - 2)));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::new((MAX - 35, MAX - 19), (MAX - 3, MAX - 1)));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::new((MAX - 36, MAX - 20), (MAX - 2, MAX)));
}

#[test]
fn to_bounds() {
    let mut r = Rect::new((1, 1), (MAX - 1, MAX - 1));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::largest());

    let mut r_min_row = Rect::new((1, 10), (MAX - 10, MAX - 10));
    assert_eq!(try_checked_inflate_assign(&mut r_min_row), Some(()));
    assert_eq!(r_min_row, Rect::new((0, 9), (MAX - 9, MAX - 9)));

    let mut r_min_y = Rect::new((10, 1), (MAX - 10, MAX - 10));
    assert_eq!(try_checked_inflate_assign(&mut r_min_y), Some(()));
    assert_eq!(r_min_y, Rect::new((9, 0), (MAX - 9, MAX - 9)));

    let mut r_max_x = Rect::new((10, 10), (MAX - 1, MAX - 10));
    assert_eq!(try_checked_inflate_assign(&mut r_max_x), Some(()));
    assert_eq!(r_max_x, Rect::new((9, 9), (MAX, MAX - 9)));

    let mut r_max_y = Rect::new((10, 10), (MAX - 10, MAX - 1));
    assert_eq!(try_checked_inflate_assign(&mut r_max_y), Some(()));
    assert_eq!(r_max_y, Rect::new((9, 9), (MAX - 9, MAX)));
}

#[test]
fn out_of_bounds() {
    let mut r = Rect::largest();
    assert_eq!(try_checked_inflate_assign(&mut r), None);
    assert_eq!(r, Rect::largest());

    let mut r_min_row = Rect::new((0, 9), (MAX - 9, MAX - 9));
    assert_eq!(try_checked_inflate_assign(&mut r_min_row), None);
    assert_eq!(r_min_row, Rect::new((0, 9), (MAX - 9, MAX - 9)));

    let mut r_min_y = Rect::new((9, 0), (MAX - 9, MAX - 9));
    assert_eq!(try_checked_inflate_assign(&mut r_min_y), None);
    assert_eq!(r_min_y, Rect::new((9, 0), (MAX - 9, MAX - 9)));

    let mut r_max_x = Rect::new((9, 9), (MAX, MAX - 9));
    assert_eq!(try_checked_inflate_assign(&mut r_max_x), None);
    assert_eq!(r_max_x, Rect::new((9, 9), (MAX, MAX - 9)));

    let mut r_max_y = Rect::new((9, 9), (MAX - 9, MAX));
    assert_eq!(try_checked_inflate_assign(&mut r_max_y), None);
    assert_eq!(r_max_y, Rect::new((9, 9), (MAX - 9, MAX)));
}

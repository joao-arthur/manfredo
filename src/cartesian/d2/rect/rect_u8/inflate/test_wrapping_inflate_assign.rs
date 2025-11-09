use super::wrapping_inflate_assign;
use crate::cartesian::{d1::point::point_u8::MAX, d2::rect::rect_u8::Rect};

#[test]
fn min_bounds() {
    let mut r = Rect::new((7, 3), (9, 13));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((6, 2), (10, 14)));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((5, 1), (11, 15)));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((4, 0), (12, 16)));
}

#[test]
fn max_bounds() {
    let mut r = Rect::new((MAX - 33, MAX - 17), (MAX - 5, MAX - 3));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MAX - 34, MAX - 18), (MAX - 4, MAX - 2)));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MAX - 35, MAX - 19), (MAX - 3, MAX - 1)));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MAX - 36, MAX - 20), (MAX - 2, MAX)));
}

#[test]
fn to_bounds() {
    let mut r = Rect::new((1, 1), (MAX - 1, MAX - 1));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, Rect::largest());

    let mut r_min_x = Rect::new((1, 10), (MAX - 10, MAX - 10));
    wrapping_inflate_assign(&mut r_min_x);
    assert_eq!(r_min_x, Rect::new((0, 9), (MAX - 9, MAX - 9)));

    let mut r_min_y = Rect::new((10, 1), (MAX - 10, MAX - 10));
    wrapping_inflate_assign(&mut r_min_y);
    assert_eq!(r_min_y, Rect::new((9, 0), (MAX - 9, MAX - 9)));

    let mut r_max_x = Rect::new((10, 10), (MAX - 1, MAX - 10));
    wrapping_inflate_assign(&mut r_max_x);
    assert_eq!(r_max_x, Rect::new((9, 9), (MAX, MAX - 9)));

    let mut r_max_y = Rect::new((10, 10), (MAX - 10, MAX - 1));
    wrapping_inflate_assign(&mut r_max_y);
    assert_eq!(r_max_y, Rect::new((9, 9), (MAX - 9, MAX)));
}

#[test]
fn out_of_bounds() {
    let mut r = Rect::largest();
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MAX, MAX), (0, 0)));

    let mut r_min_x = Rect::new((0, 10), (MAX - 10, MAX - 10));
    wrapping_inflate_assign(&mut r_min_x);
    assert_eq!(r_min_x, Rect::new((MAX, 9), (MAX - 9, MAX - 9)));

    let mut r_min_y = Rect::new((10, 0), (MAX - 10, MAX - 10));
    wrapping_inflate_assign(&mut r_min_y);
    assert_eq!(r_min_y, Rect::new((9, MAX), (MAX - 9, MAX - 9)));

    let mut r_max_x = Rect::new((10, 10), (MAX, MAX - 10));
    wrapping_inflate_assign(&mut r_max_x);
    assert_eq!(r_max_x, Rect::new((9, 9), (0, MAX - 9)));

    let mut r_max_y = Rect::new((10, 10), (MAX - 10, MAX));
    wrapping_inflate_assign(&mut r_max_y);
    assert_eq!(r_max_y, Rect::new((9, 9), (MAX - 9, 0)));
}

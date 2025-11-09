use super::wrapping_inflate_assign;
use crate::matrix::{
    d1::point::point_i16::{MAX, MIN},
    d2::rect::rect_i16::Rect,
};

#[test]
fn min_bounds() {
    let mut r = Rect::new((MIN + 7, MIN + 3), (MIN + 9, MIN + 13));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MIN + 6, MIN + 2), (MIN + 10, MIN + 14)));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MIN + 5, MIN + 1), (MIN + 11, MIN + 15)));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MIN + 4, MIN), (MIN + 12, MIN + 16)));
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
    let mut r = Rect::new((MIN + 1, MIN + 1), (MAX - 1, MAX - 1));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, Rect::largest());

    let mut r_min_row = Rect::new((MIN + 1, MIN + 10), (MAX - 10, MAX - 10));
    wrapping_inflate_assign(&mut r_min_row);
    assert_eq!(r_min_row, Rect::new((MIN, MIN + 9), (MAX - 9, MAX - 9)));

    let mut r_min_y = Rect::new((MIN + 10, MIN + 1), (MAX - 10, MAX - 10));
    wrapping_inflate_assign(&mut r_min_y);
    assert_eq!(r_min_y, Rect::new((MIN + 9, MIN), (MAX - 9, MAX - 9)));

    let mut r_max_x = Rect::new((MIN + 10, MIN + 10), (MAX - 1, MAX - 10));
    wrapping_inflate_assign(&mut r_max_x);
    assert_eq!(r_max_x, Rect::new((MIN + 9, MIN + 9), (MAX, MAX - 9)));

    let mut r_max_y = Rect::new((MIN + 10, MIN + 10), (MAX - 10, MAX - 1));
    wrapping_inflate_assign(&mut r_max_y);
    assert_eq!(r_max_y, Rect::new((MIN + 9, MIN + 9), (MAX - 9, MAX)));
}

#[test]
fn out_of_bounds() {
    let mut r = Rect::largest();
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MAX, MAX), (MIN, MIN)));

    let mut r_min_row = Rect::new((MIN, MIN + 10), (MAX - 10, MAX - 10));
    wrapping_inflate_assign(&mut r_min_row);
    assert_eq!(r_min_row, Rect::new((MAX, MIN + 9), (MAX - 9, MAX - 9)));

    let mut r_min_y = Rect::new((MIN + 10, MIN), (MAX - 10, MAX - 10));
    wrapping_inflate_assign(&mut r_min_y);
    assert_eq!(r_min_y, Rect::new((MIN + 9, MAX), (MAX - 9, MAX - 9)));

    let mut r_max_x = Rect::new((MIN + 10, MIN + 10), (MAX, MAX - 10));
    wrapping_inflate_assign(&mut r_max_x);
    assert_eq!(r_max_x, Rect::new((MIN + 9, MIN + 9), (MIN, MAX - 9)));

    let mut r_max_y = Rect::new((MIN + 10, MIN + 10), (MAX - 10, MAX));
    wrapping_inflate_assign(&mut r_max_y);
    assert_eq!(r_max_y, Rect::new((MIN + 9, MIN + 9), (MAX - 9, MIN)));
}

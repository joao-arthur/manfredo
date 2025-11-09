use super::try_saturating_inflate_assign;
use crate::cartesian::{d1::point::point_u8::MAX, d2::rect::rect_u8::Rect};

#[test]
fn min_bounds() {
    let mut r = Rect::new((7, 2), (17, 13));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::new((6, 1), (18, 14)));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::new((5, 0), (19, 15)));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::new((4, 0), (20, 17)));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::new((3, 0), (21, 19)));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::new((2, 0), (22, 21)));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::new((1, 0), (23, 23)));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::new((0, 0), (24, 25)));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::new((0, 0), (26, 27)));
}

#[test]
fn max_bounds() {
    let mut r = Rect::new((MAX - 33, MAX - 17), (MAX - 5, MAX - 3));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::new((MAX - 34, MAX - 18), (MAX - 4, MAX - 2)));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::new((MAX - 35, MAX - 19), (MAX - 3, MAX - 1)));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::new((MAX - 36, MAX - 20), (MAX - 2, MAX)));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::new((MAX - 37, MAX - 22), (MAX - 1, MAX)));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::new((MAX - 38, MAX - 24), (MAX, MAX)));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::new((MAX - 40, MAX - 26), (MAX, MAX)));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::new((MAX - 42, MAX - 28), (MAX, MAX)));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::new((MAX - 44, MAX - 30), (MAX, MAX)));
}

#[test]
fn to_bounds() {
    let mut r = Rect::new((1, 1), (MAX - 1, MAX - 1));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::largest());

    let mut r_min = Rect::new((0, 0), (MAX - 1, MAX - 1));
    assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
    assert_eq!(r_min, Rect::largest());

    let mut r_max = Rect::new((1, 1), (MAX, MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
    assert_eq!(r_max, Rect::largest());

    let mut r_min_x = Rect::new((1, 10), (20, 20));
    assert_eq!(try_saturating_inflate_assign(&mut r_min_x), Some(()));
    assert_eq!(r_min_x, Rect::new((0, 9), (21, 21)));

    let mut r_min_y = Rect::new((10, 1), (20, 20));
    assert_eq!(try_saturating_inflate_assign(&mut r_min_y), Some(()));
    assert_eq!(r_min_y, Rect::new((9, 0), (21, 21)));

    let mut r_max_x = Rect::new((10, 10), (MAX - 1, 20));
    assert_eq!(try_saturating_inflate_assign(&mut r_max_x), Some(()));
    assert_eq!(r_max_x, Rect::new((9, 9), (MAX, 21)));

    let mut r_max_y = Rect::new((10, 10), (20, MAX - 1));
    assert_eq!(try_saturating_inflate_assign(&mut r_max_y), Some(()));
    assert_eq!(r_max_y, Rect::new((9, 9), (21, MAX)));
}

#[test]
fn out_of_bounds() {
    let mut r = Rect::largest();
    assert_eq!(try_saturating_inflate_assign(&mut r), None);
    assert_eq!(r, Rect::largest());

    let mut r_x = Rect::new((0, 10), (MAX, 20));
    assert_eq!(try_saturating_inflate_assign(&mut r_x), None);
    assert_eq!(r_x, Rect::new((0, 10), (MAX, 20)));

    let mut r_y = Rect::new((10, 0), (20, MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r_y), None);
    assert_eq!(r_y, Rect::new((10, 0), (20, MAX)));
}

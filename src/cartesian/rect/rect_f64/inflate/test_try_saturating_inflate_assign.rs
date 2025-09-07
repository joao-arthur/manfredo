use super::try_saturating_inflate_assign;
use crate::cartesian::{
    point::point_f64::{MAX, MIN},
    rect::rect_f64::RectF64,
};

#[test]
fn min_bounds() {
    let mut r = RectF64::of(MIN + 7.0, MIN + 2.0, MIN + 17.0, MIN + 13.0);
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectF64::of(MIN + 6.0, MIN + 1.0, MIN + 18.0, MIN + 14.0));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectF64::of(MIN + 5.0, MIN, MIN + 19.0, MIN + 15.0));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectF64::of(MIN + 4.0, MIN, MIN + 20.0, MIN + 17.0));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectF64::of(MIN + 3.0, MIN, MIN + 21.0, MIN + 19.0));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectF64::of(MIN + 2.0, MIN, MIN + 22.0, MIN + 21.0));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectF64::of(MIN + 1.0, MIN, MIN + 23.0, MIN + 23.0));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectF64::of(MIN, MIN, MIN + 24.0, MIN + 25.0));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectF64::of(MIN, MIN, MIN + 26.0, MIN + 27.0));
}

#[test]
fn max_bounds() {
    let mut r = RectF64::of(MAX - 33.0, MAX - 17.0, MAX - 5.0, MAX - 3.0);
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectF64::of(MAX - 34.0, MAX - 18.0, MAX - 4.0, MAX - 2.0));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectF64::of(MAX - 35.0, MAX - 19.0, MAX - 3.0, MAX - 1.0));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectF64::of(MAX - 36.0, MAX - 20.0, MAX - 2.0, MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectF64::of(MAX - 37.0, MAX - 22.0, MAX - 1.0, MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectF64::of(MAX - 38.0, MAX - 24.0, MAX, MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectF64::of(MAX - 40.0, MAX - 26.0, MAX, MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectF64::of(MAX - 42.0, MAX - 28.0, MAX, MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectF64::of(MAX - 44.0, MAX - 30.0, MAX, MAX));
}

#[test]
fn to_bounds() {
    let mut r = RectF64::of(MIN + 1.0, MIN + 1.0, MAX - 1.0, MAX - 1.0);
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectF64::largest());

    let mut r_min = RectF64::of(MIN, MIN, MAX - 1.0, MAX - 1.0);
    assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
    assert_eq!(r_min, RectF64::largest());

    let mut r_max = RectF64::of(MIN + 1.0, MIN + 1.0, MAX, MAX);
    assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
    assert_eq!(r_max, RectF64::largest());

    let mut r_min_x = RectF64::of(MIN + 1.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    assert_eq!(try_saturating_inflate_assign(&mut r_min_x), Some(()));
    assert_eq!(r_min_x, RectF64::of(MIN, MIN + 9.0, MAX - 9.0, MAX - 9.0));

    let mut r_min_y = RectF64::of(MIN + 10.0, MIN + 1.0, MAX - 10.0, MAX - 10.0);
    assert_eq!(try_saturating_inflate_assign(&mut r_min_y), Some(()));
    assert_eq!(r_min_y, RectF64::of(MIN + 9.0, MIN, MAX - 9.0, MAX - 9.0));

    let mut r_max_x = RectF64::of(MIN + 10.0, MIN + 10.0, MAX - 1.0, MAX - 10.0);
    assert_eq!(try_saturating_inflate_assign(&mut r_max_x), Some(()));
    assert_eq!(r_max_x, RectF64::of(MIN + 9.0, MIN + 9.0, MAX, MAX - 9.0));

    let mut r_max_y = RectF64::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 1.0);
    assert_eq!(try_saturating_inflate_assign(&mut r_max_y), Some(()));
    assert_eq!(r_max_y, RectF64::of(MIN + 9.0, MIN + 9.0, MAX - 9.0, MAX));
}

#[test]
fn out_of_bounds() {
    let mut r = RectF64::largest();
    assert_eq!(try_saturating_inflate_assign(&mut r), None);
    assert_eq!(r, RectF64::largest());

    let mut r_x = RectF64::of(MIN, MIN + 10.0, MAX, MAX - 10.0);
    assert_eq!(try_saturating_inflate_assign(&mut r_x), None);
    assert_eq!(r_x, RectF64::of(MIN, MIN + 10.0, MAX, MAX - 10.0));

    let mut r_y = RectF64::of(MIN + 10.0, MIN, MAX - 10.0, MAX);
    assert_eq!(try_saturating_inflate_assign(&mut r_y), None);
    assert_eq!(r_y, RectF64::of(MIN + 10.0, MIN, MAX - 10.0, MAX));
}

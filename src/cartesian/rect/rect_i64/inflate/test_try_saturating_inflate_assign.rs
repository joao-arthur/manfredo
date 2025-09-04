use super::try_saturating_inflate_assign;
use crate::cartesian::rect::rect_i64::RectI64;

#[test]
fn try_saturating_inflate_assign_min_bounds() {
    let mut r = RectI64::of(i64::MIN + 7, i64::MIN + 2, i64::MIN + 17, i64::MIN + 13);
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::of(i64::MIN + 6, i64::MIN + 1, i64::MIN + 18, i64::MIN + 14));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::of(i64::MIN + 5, i64::MIN, i64::MIN + 19, i64::MIN + 15));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 20, i64::MIN + 17));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::of(i64::MIN + 3, i64::MIN, i64::MIN + 21, i64::MIN + 19));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::of(i64::MIN + 2, i64::MIN, i64::MIN + 22, i64::MIN + 21));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::of(i64::MIN + 1, i64::MIN, i64::MIN + 23, i64::MIN + 23));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 24, i64::MIN + 25));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 26, i64::MIN + 27));
}

#[test]
fn try_saturating_inflate_assign_max_bounds() {
    let mut r = RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3);
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::of(i64::MAX - 37, i64::MAX - 22, i64::MAX - 1, i64::MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::of(i64::MAX - 38, i64::MAX - 24, i64::MAX, i64::MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::of(i64::MAX - 40, i64::MAX - 26, i64::MAX, i64::MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::of(i64::MAX - 42, i64::MAX - 28, i64::MAX, i64::MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::of(i64::MAX - 44, i64::MAX - 30, i64::MAX, i64::MAX));
}

#[test]
fn try_saturating_inflate_assign_to_bounds() {
    let mut r_odd = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1);
    assert_eq!(try_saturating_inflate_assign(&mut r_odd), Some(()));
    assert_eq!(r_odd, RectI64::largest());

    let mut r_even_1 = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1);
    assert_eq!(try_saturating_inflate_assign(&mut r_even_1), Some(()));
    assert_eq!(r_even_1, RectI64::largest());

    let mut r_even_2 = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX);
    assert_eq!(try_saturating_inflate_assign(&mut r_even_2), Some(()));
    assert_eq!(r_even_2, RectI64::largest());
}

#[test]
fn try_saturating_inflate_assign_width_to_bounds() {
    let mut r_min = RectI64::of(i64::MIN + 1, i64::MIN + 10, i64::MIN + 20, i64::MIN + 20);
    assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
    assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN + 9, i64::MIN + 21, i64::MIN + 21));

    let mut r_max = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 1, i64::MIN + 20);
    assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
    assert_eq!(r_max, RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MAX, i64::MIN + 21));
}

#[test]
fn try_saturating_inflate_assign_height_to_bounds() {
    let mut r_min = RectI64::of(i64::MIN + 10, i64::MIN + 1, i64::MIN + 20, i64::MIN + 20);
    assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
    assert_eq!(r_min, RectI64::of(i64::MIN + 9, i64::MIN, i64::MIN + 21, i64::MIN + 21));

    let mut r_max = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MIN + 20, i64::MAX - 1);
    assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
    assert_eq!(r_max, RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MIN + 21, i64::MAX));
}

#[test]
fn try_saturating_inflate_assign_out_of_bounds() {
    let mut r = RectI64::largest();
    assert_eq!(try_saturating_inflate_assign(&mut r), None);
    assert_eq!(r, RectI64::largest());

    let mut r_x = RectI64::of(i64::MIN, i64::MIN + 10, i64::MAX, i64::MIN + 20);
    assert_eq!(try_saturating_inflate_assign(&mut r_x), None);
    assert_eq!(r_x, RectI64::of(i64::MIN, i64::MIN + 10, i64::MAX, i64::MIN + 20));

    let mut r_y = RectI64::of(i64::MIN + 10, i64::MIN, i64::MIN + 20, i64::MAX);
    assert_eq!(try_saturating_inflate_assign(&mut r_y), None);
    assert_eq!(r_y, RectI64::of(i64::MIN + 10, i64::MIN, i64::MIN + 20, i64::MAX));
}

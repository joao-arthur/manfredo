use super::try_saturating_resize_assign;
use crate::cartesian::{
    point::point_f32::{MAX, MIN},
    rect::rect_f32::RectF32,
};

#[test]
fn odd() {
    let mut r = RectF32::of(-5.0, -5.0, 5.0, 5.0);
    assert_eq!(try_saturating_resize_assign(&mut r, 9.0), Some(()));
    assert_eq!(r, RectF32::of(-4.0, -4.0, 4.0, 4.0));
    assert_eq!(try_saturating_resize_assign(&mut r, 7.0), Some(()));
    assert_eq!(r, RectF32::of(-3.0, -3.0, 3.0, 3.0));
    assert_eq!(try_saturating_resize_assign(&mut r, 5.0), Some(()));
    assert_eq!(r, RectF32::of(-2.0, -2.0, 2.0, 2.0));
    assert_eq!(try_saturating_resize_assign(&mut r, 3.0), Some(()));
    assert_eq!(r, RectF32::of(-1.0, -1.0, 1.0, 1.0));
    assert_eq!(try_saturating_resize_assign(&mut r, 9.0), Some(()));
    assert_eq!(r, RectF32::of(-4.0, -4.0, 4.0, 4.0));
}

#[test]
fn even() {
    let mut r = RectF32::of(-5.0, -5.0, 4.0, 4.0);
    assert_eq!(try_saturating_resize_assign(&mut r, 10.0), Some(()));
    assert_eq!(r, RectF32::of(-5.0, -5.0, 4.0, 4.0));
    assert_eq!(try_saturating_resize_assign(&mut r, 8.0), Some(()));
    assert_eq!(r, RectF32::of(-4.0, -4.0, 3.0, 3.0));
    assert_eq!(try_saturating_resize_assign(&mut r, 6.0), Some(()));
    assert_eq!(r, RectF32::of(-3.0, -3.0, 2.0, 2.0));
    assert_eq!(try_saturating_resize_assign(&mut r, 4.0), Some(()));
    assert_eq!(r, RectF32::of(-2.0, -2.0, 1.0, 1.0));
    assert_eq!(try_saturating_resize_assign(&mut r, 8.0), Some(()));
    assert_eq!(r, RectF32::of(-4.0, -4.0, 3.0, 3.0));
}

#[test]
fn small_size() {
    let mut r = RectF32::of(10.0, 10.0, 20.0, 20.0);
    assert_eq!(try_saturating_resize_assign(&mut r, 0.0), None);
    assert_eq!(try_saturating_resize_assign(&mut r, 1.0), None);
    assert_eq!(try_saturating_resize_assign(&mut r, 2.0), None);
    assert_eq!(try_saturating_resize_assign(&mut r, MAX + 1.0), None);
    assert_eq!(try_saturating_resize_assign(&mut r, MAX + 2.0), None);
    assert_eq!(try_saturating_resize_assign(&mut r, MAX + 3.0), None);
    assert_eq!(r, RectF32::of(10.0, 10.0, 20.0, 20.0));
}

#[test]
fn same_size() {
    let mut r_min_2 = RectF32::of(MIN, MIN, MIN + 2.0, MIN + 2.0);
    assert_eq!(try_saturating_resize_assign(&mut r_min_2, 3.0), Some(()));
    assert_eq!(r_min_2, RectF32::of(MIN, MIN, MIN + 2.0, MIN + 2.0));

    let mut r_min_3 = RectF32::of(MIN, MIN, MIN + 3.0, MIN + 3.0);
    assert_eq!(try_saturating_resize_assign(&mut r_min_3, 4.0), Some(()));
    assert_eq!(r_min_3, RectF32::of(MIN, MIN, MIN + 3.0, MIN + 3.0));

    let mut r_max_2 = RectF32::of(MAX - 2.0, MAX - 2.0, MAX, MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max_2, 3.0), Some(()));
    assert_eq!(r_max_2, RectF32::of(MAX - 2.0, MAX - 2.0, MAX, MAX));

    let mut r_max_3 = RectF32::of(MAX - 3.0, MAX - 3.0, MAX, MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max_3, 4.0), Some(()));
    assert_eq!(r_max_3, RectF32::of(MAX - 3.0, MAX - 3.0, MAX, MAX));
}

#[test]
fn bounds() {
    let mut r_min = RectF32::of(MIN, MIN, MIN + 2.0, MIN + 2.0);
    assert_eq!(try_saturating_resize_assign(&mut r_min, 11.0), Some(()));
    assert_eq!(r_min, RectF32::of(MIN, MIN, MIN + 10.0, MIN + 10.0));

    let mut r_max = RectF32::of(MAX - 2.0, MAX - 2.0, MAX, MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max, 11.0), Some(()));
    assert_eq!(r_max, RectF32::of(MAX - 10.0, MAX - 10.0, MAX, MAX));
}

//#[test]
//fn small_rect_limits() {
//    let mut r_min = RectF32::of(MIN, MIN, MIN + 2.0, MIN + 2.0);
//    assert_eq!(try_saturating_resize_assign(&mut r_min, MAX), Some(()));
//    assert_eq!(r_min, RectF32::of(MIN, MIN, MAX - 1.0, MAX - 1.0));
//
//    let mut r_max = RectF32::of(MAX - 2.0, MAX - 2.0, MAX, MAX);
//    assert_eq!(try_saturating_resize_assign(&mut r_max, MAX), Some(()));
//    assert_eq!(r_max, RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX));
//}

//#[test]
//fn big_rect_limits() {
//    let mut r_odd_1 = RectF32::of(MIN, MIN, MAX - 1.0, MAX - 1.0);
//    assert_eq!(try_saturating_resize_assign(&mut r_odd_1, MAX), Some(()));
//    assert_eq!(r_odd_1, RectF32::of(MIN, MIN, MAX - 1.0, MAX - 1.0));
//
//    let mut r_odd_1 = RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX);
//    assert_eq!(try_saturating_resize_assign(&mut r_odd_1, MAX), Some(()));
//    assert_eq!(r_odd_1, RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX));
//
//    let mut r_even = RectF32::largest();
//    assert_eq!(try_saturating_resize_assign(&mut r_even, MAX), Some(()));
//    assert_eq!(r_even, RectF32::of(MIN, MIN, MAX - 1.0, MAX - 1.0));
//}

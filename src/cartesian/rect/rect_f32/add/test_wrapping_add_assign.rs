use super::wrapping_add_assign;
use crate::cartesian::{
    point::point_f32::{MAX, MIN},
    rect::rect_f32::RectF32,
};

#[test]
fn test() {
    let mut r = RectF32::of(-7.0, 9.0, -12.0, 15.0);
    wrapping_add_assign(&mut r, &RectF32::of(5.0, 4.0, 3.0, 2.0));
    assert_eq!(r, RectF32::of(-2.0, 13.0, -9.0, 17.0));
    wrapping_add_assign(&mut r, &RectF32::of(9.0, -10.0, 11.0, -12.0));
    assert_eq!(r, RectF32::of(7.0, 3.0, 2.0, 5.0));
}

#[test]
fn big_rect_to_bounds() {
    let mut r = RectF32::of(MIN + 2.0, MIN + 5.0, MAX - 2.0, MAX - 5.0);
    wrapping_add_assign(&mut r, &RectF32::of(-2.0, -5.0, 2.0, 5.0));
    assert_eq!(r, RectF32::largest());

    let mut r_min = RectF32::of(MIN + 2.0, MIN + 5.0, MAX, MAX);
    wrapping_add_assign(&mut r_min, &RectF32::of(-2.0, -5.0, 0.0, 0.0));
    assert_eq!(r_min, RectF32::largest());

    let mut r_max = RectF32::of(MIN, MIN, MAX - 2.0, MAX - 5.0);
    wrapping_add_assign(&mut r_max, &RectF32::of(0.0, 0.0, 2.0, 5.0));
    assert_eq!(r_max, RectF32::largest());
}

#[test]
fn out_of_bounds() {
    let mut r1 = RectF32::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    wrapping_add_assign(&mut r1, &RectF32::of(-20.0, 0.0, 0.0, 0.0));
    assert_eq!(r1, RectF32::of(MAX - 9.0, MIN + 10.0, MAX - 10.0, MAX - 10.0));

    let mut r2 = RectF32::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    wrapping_add_assign(&mut r2, &RectF32::of(0.0, -20.0, 0.0, 0.0));
    assert_eq!(r2, RectF32::of(MIN + 10.0, MAX - 9.0, MAX - 10.0, MAX - 10.0));

    let mut r3 = RectF32::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    wrapping_add_assign(&mut r3, &RectF32::of(0.0, 0.0, 20.0, 0.0));
    assert_eq!(r3, RectF32::of(MIN + 10.0, MIN + 10.0, MIN + 9.0, MAX - 10.0));

    let mut r4 = RectF32::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    wrapping_add_assign(&mut r4, &RectF32::of(0.0, 0.0, 0.0, 20.0));
    assert_eq!(r4, RectF32::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MIN + 9.0));
}

// #[test]
// fn edge_out_of_bounds() {
//     let mut r1 = RectF32::largest();
//     wrapping_add_assign(&mut r1, &RectF32::of(-1.0, 0.0, 0.0, 0.0));
//     assert_eq!(r1, RectF32::of(MAX, MIN, MAX, MAX));
//
//     let mut r2 = RectF32::largest();
//     wrapping_add_assign(&mut r2, &RectF32::of(0.0, -1.0, 0.0, 0.0));
//     assert_eq!(r2, RectF32::of(MIN, MAX, MAX, MAX));
//
//     let mut r3 = RectF32::largest();
//     wrapping_add_assign(&mut r3, &RectF32::of(0.0, 0.0, 1.0, 0.0));
//     assert_eq!(r3, RectF32::of(MIN, MIN, MIN, MAX));
//
//     let mut r4 = RectF32::largest();
//     wrapping_add_assign(&mut r4, &RectF32::of(0.0, 0.0, 0.0, 1.0));
//     assert_eq!(r4, RectF32::of(MIN, MIN, MAX, MIN));
// }

#[test]
fn limits_out_of_bounds() {
    let mut r1 = RectF32::largest();
    wrapping_add_assign(&mut r1, &RectF32::of(MIN, 0.0, 0.0, 0.0));
    assert_eq!(r1, RectF32::of(0.0, MIN, MAX, MAX));

    let mut r2 = RectF32::largest();
    wrapping_add_assign(&mut r2, &RectF32::of(0.0, MIN, 0.0, 0.0));
    assert_eq!(r2, RectF32::of(MIN, 0.0, MAX, MAX));

    let mut r3 = RectF32::largest();
    wrapping_add_assign(&mut r3, &RectF32::of(0.0, 0.0, MAX, 0.0));
    assert_eq!(r3, RectF32::of(MIN, MIN, -2.0, MAX));

    let mut r4 = RectF32::largest();
    wrapping_add_assign(&mut r4, &RectF32::of(0.0, 0.0, 0.0, MAX));
    assert_eq!(r4, RectF32::of(MIN, MIN, MAX, -2.0));
}

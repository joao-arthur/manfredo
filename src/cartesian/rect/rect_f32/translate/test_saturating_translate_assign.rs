use super::saturating_translate_assign;
use crate::cartesian::{
    point::point_f32::{MAX, MIN, PointF32},
    rect::rect_f32::RectF32,
};

#[test]
fn test() {
    let mut r = RectF32::of(0.0, 0.0, 10.0, 10.0);
    saturating_translate_assign(&mut r, &PointF32::of(10.0, 20.0));
    assert_eq!(r, RectF32::of(10.0, 20.0, 20.0, 30.0));
    saturating_translate_assign(&mut r, &PointF32::of(-20.0, -15.0));
    assert_eq!(r, RectF32::of(-10.0, 5.0, 0.0, 15.0));
}

// #[test]
// fn to_bounds() {
//     let mut r_min = RectF32::of(MIN + 2.0, MIN + 5.0, MAX, MAX);
//     saturating_translate_assign(&mut r_min, &PointF32::of(-2.0, -5.0));
//     assert_eq!(r_min, RectF32::of(MIN, MIN, MAX - 2.0, MAX - 5.0));
//
//     let mut r_max = RectF32::of(MIN, MIN, MAX - 2.0, MAX - 5.0);
//     saturating_translate_assign(&mut r_max, &PointF32::of(2.0, 5.0));
//     assert_eq!(r_max, RectF32::of(MIN + 2.0, MIN + 5.0, MAX, MAX));
// }

//#[test]
//fn out_of_bounds() {
//    let mut r1 = RectF32::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
//    saturating_translate_assign(&mut r1, &PointF32::of(-20.0, 0.0));
//    assert_eq!(r1, RectF32::of(MIN, MIN + 10.0, MAX - 20.0, MAX - 10.0));
//
//    let mut r2 = RectF32::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
//    saturating_translate_assign(&mut r2, &PointF32::of(0.0, -20.0));
//    assert_eq!(r2, RectF32::of(MIN + 10.0, MIN, MAX - 10.0, MAX - 20.0));
//
//    let mut r3 = RectF32::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
//    saturating_translate_assign(&mut r3, &PointF32::of(20.0, 0.0));
//    assert_eq!(r3, RectF32::of(MIN + 20.0, MIN + 10.0, MAX, MAX - 10.0));
//
//    let mut r4 = RectF32::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
//    saturating_translate_assign(&mut r4, &PointF32::of(0.0, 20.0));
//    assert_eq!(r4, RectF32::of(MIN + 10.0, MIN + 20.0, MAX - 10.0, MAX));
//}

//#[test]
//fn limits_out_of_bounds() {
//    let mut r = RectF32::largest();
//    saturating_translate_assign(&mut r, &PointF32::of(MIN, 0.0));
//    assert_eq!(r, RectF32::largest());
//    saturating_translate_assign(&mut r, &PointF32::of(0.0, MIN));
//    assert_eq!(r, RectF32::largest());
//    saturating_translate_assign(&mut r, &PointF32::of(MAX, 0.0));
//    assert_eq!(r, RectF32::largest());
//    saturating_translate_assign(&mut r, &PointF32::of(0.0, MAX));
//    assert_eq!(r, RectF32::largest());
//}

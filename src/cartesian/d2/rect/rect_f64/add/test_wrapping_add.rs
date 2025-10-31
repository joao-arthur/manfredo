use super::wrapping_add;
use crate::cartesian::{
    d1::point::point_f64::{MAX, MIN},
    d2::rect::rect_f64::Rect,
};

#[test]
fn test() {
    assert_eq!(wrapping_add(&Rect::of((-7.0, 9.0), (-12.0, 15.0)), &Rect::of((5.0, 4.0), (3.0, 2.0))), Rect::of((-2.0, 13.0), (-9.0, 17.0)));
    assert_eq!(wrapping_add(&Rect::of((-2.0, 13.0), (-9.0, 17.0)), &Rect::of((9.0, -10.0), (11.0, -12.0))), Rect::of((7.0, 3.0), (2.0, 5.0)));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&Rect::of((MIN + 2.0, MIN + 5.0), (MAX - 2.0, MAX - 5.0)), &Rect::of((-2.0, -5.0), (2.0, 5.0))), Rect::largest());
    assert_eq!(wrapping_add(&Rect::of((MIN + 2.0, MIN + 5.0), (MAX, MAX)), &Rect::of((-2.0, -5.0), (0.0, 0.0))), Rect::largest());
    assert_eq!(wrapping_add(&Rect::of((MIN, MIN), (MAX - 2.0, MAX - 5.0)), &Rect::of((0.0, 0.0), (2.0, 5.0))), Rect::largest());
}

#[test]
fn out_of_bounds() {
    let r = Rect::of((MIN + 10.0, MIN + 10.0), (MAX - 10.0, MAX - 10.0));
    assert_eq!(wrapping_add(&r, &Rect::of((-20.0, 0.0), (0.0, 0.0))), Rect::of((MAX - 9.0, MIN + 10.0), (MAX - 10.0, MAX - 10.0)));
    assert_eq!(wrapping_add(&r, &Rect::of((0.0, -20.0), (0.0, 0.0))), Rect::of((MIN + 10.0, MAX - 9.0), (MAX - 10.0, MAX - 10.0)));
    assert_eq!(wrapping_add(&r, &Rect::of((0.0, 0.0), (20.0, 0.0))), Rect::of((MIN + 10.0, MIN + 10.0), (MIN + 9.0, MAX - 10.0)));
    assert_eq!(wrapping_add(&r, &Rect::of((0.0, 0.0), (0.0, 20.0))), Rect::of((MIN + 10.0, MIN + 10.0), (MAX - 10.0, MIN + 9.0)));

    let r_min = Rect::of((MIN, MIN), (MIN + 10.0, MIN + 10.0));
    assert_eq!(wrapping_add(&r_min, &Rect::of((-20.0, -20.0), (-20.0, -20.0))), Rect::of((MAX - 19.0, MAX - 19.0), (MAX - 9.0, MAX - 9.0)));

    let r_max = Rect::of((MAX, MAX), (MAX - 10.0, MAX - 10.0));
    assert_eq!(wrapping_add(&r_max, &Rect::of((20.0, 20.0), (20.0, 20.0))), Rect::of((MIN + 19.0, MIN + 19.0), (MIN + 9.0, MIN + 9.0)));
}

#[test]
fn edge_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(wrapping_add(&r, &Rect::of((-1.0, 0.0), (0.0, 0.0))), Rect::of((MAX, MIN), (MAX, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::of((0.0, -1.0), (0.0, 0.0))), Rect::of((MIN, MAX), (MAX, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::of((0.0, 0.0), (1.0, 0.0))), Rect::of((MIN, MIN), (MIN, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::of((0.0, 0.0), (0.0, 1.0))), Rect::of((MIN, MIN), (MAX, MIN)));
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(wrapping_add(&r, &Rect::of((MIN, 0.0), (0.0, 0.0))), Rect::of((0.0, MIN), (MAX, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::of((0.0, MIN), (0.0, 0.0))), Rect::of((MIN, 0.0), (MAX, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::of((0.0, 0.0), (MAX, 0.0))), Rect::of((MIN, MIN), (-2.0, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::of((0.0, 0.0), (0.0, MAX))), Rect::of((MIN, MIN), (MAX, -2.0)));
}

use super::wrapping_add;
use crate::cartesian::{
    d1::point::point_f64::{MAX, MIN},
    d2::rect::rect_f64::Rect,
};

#[test]
fn test() {
    assert_eq!(wrapping_add(&Rect::new((-7.0, 9.0), (-12.0, 15.0)), &Rect::new((5.0, 4.0), (3.0, 2.0))), Rect::new((-2.0, 13.0), (-9.0, 17.0)));
    assert_eq!(wrapping_add(&Rect::new((-2.0, 13.0), (-9.0, 17.0)), &Rect::new((9.0, -10.0), (11.0, -12.0))), Rect::new((7.0, 3.0), (2.0, 5.0)));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&Rect::new((MIN + 2.0, MIN + 5.0), (MAX - 2.0, MAX - 5.0)), &Rect::new((-2.0, -5.0), (2.0, 5.0))), Rect::largest());
    assert_eq!(wrapping_add(&Rect::new((MIN + 2.0, MIN + 5.0), (MAX, MAX)), &Rect::new((-2.0, -5.0), (0.0, 0.0))), Rect::largest());
    assert_eq!(wrapping_add(&Rect::new((MIN, MIN), (MAX - 2.0, MAX - 5.0)), &Rect::new((0.0, 0.0), (2.0, 5.0))), Rect::largest());
}

#[test]
fn out_of_bounds() {
    let r = Rect::new((MIN + 10.0, MIN + 10.0), (MAX - 10.0, MAX - 10.0));
    assert_eq!(wrapping_add(&r, &Rect::new((-20.0, 0.0), (0.0, 0.0))), Rect::new((MAX - 9.0, MIN + 10.0), (MAX - 10.0, MAX - 10.0)));
    assert_eq!(wrapping_add(&r, &Rect::new((0.0, -20.0), (0.0, 0.0))), Rect::new((MIN + 10.0, MAX - 9.0), (MAX - 10.0, MAX - 10.0)));
    assert_eq!(wrapping_add(&r, &Rect::new((0.0, 0.0), (20.0, 0.0))), Rect::new((MIN + 10.0, MIN + 10.0), (MIN + 9.0, MAX - 10.0)));
    assert_eq!(wrapping_add(&r, &Rect::new((0.0, 0.0), (0.0, 20.0))), Rect::new((MIN + 10.0, MIN + 10.0), (MAX - 10.0, MIN + 9.0)));

    let r_min = Rect::new((MIN, MIN), (MIN + 10.0, MIN + 10.0));
    assert_eq!(wrapping_add(&r_min, &Rect::new((-20.0, -20.0), (-20.0, -20.0))), Rect::new((MAX - 19.0, MAX - 19.0), (MAX - 9.0, MAX - 9.0)));

    let r_max = Rect::new((MAX, MAX), (MAX - 10.0, MAX - 10.0));
    assert_eq!(wrapping_add(&r_max, &Rect::new((20.0, 20.0), (20.0, 20.0))), Rect::new((MIN + 19.0, MIN + 19.0), (MIN + 9.0, MIN + 9.0)));
}

#[test]
fn edge_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(wrapping_add(&r, &Rect::new((-1.0, 0.0), (0.0, 0.0))), Rect::new((MAX, MIN), (MAX, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::new((0.0, -1.0), (0.0, 0.0))), Rect::new((MIN, MAX), (MAX, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::new((0.0, 0.0), (1.0, 0.0))), Rect::new((MIN, MIN), (MIN, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::new((0.0, 0.0), (0.0, 1.0))), Rect::new((MIN, MIN), (MAX, MIN)));
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(wrapping_add(&r, &Rect::new((MIN, 0.0), (0.0, 0.0))), Rect::new((0.0, MIN), (MAX, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::new((0.0, MIN), (0.0, 0.0))), Rect::new((MIN, 0.0), (MAX, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::new((0.0, 0.0), (MAX, 0.0))), Rect::new((MIN, MIN), (-2.0, MAX)));
    assert_eq!(wrapping_add(&r, &Rect::new((0.0, 0.0), (0.0, MAX))), Rect::new((MIN, MIN), (MAX, -2.0)));
}

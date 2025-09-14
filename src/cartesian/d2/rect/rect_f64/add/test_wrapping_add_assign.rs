use super::wrapping_add_assign;
use crate::cartesian::d2::{
    point::point_f64::{MAX, MIN},
    rect::rect_f64::Rect,
};

#[test]
fn test() {
    let mut r = Rect::of(-7.0, 9.0, -12.0, 15.0);
    wrapping_add_assign(&mut r, &Rect::of(5.0, 4.0, 3.0, 2.0));
    assert_eq!(r, Rect::of(-2.0, 13.0, -9.0, 17.0));
    wrapping_add_assign(&mut r, &Rect::of(9.0, -10.0, 11.0, -12.0));
    assert_eq!(r, Rect::of(7.0, 3.0, 2.0, 5.0));
}

#[test]
fn to_bounds() {
    let mut r = Rect::of(MIN + 2.0, MIN + 5.0, MAX - 2.0, MAX - 5.0);
    wrapping_add_assign(&mut r, &Rect::of(-2.0, -5.0, 2.0, 5.0));
    assert_eq!(r, Rect::largest());

    let mut r_min = Rect::of(MIN + 2.0, MIN + 5.0, MAX, MAX);
    wrapping_add_assign(&mut r_min, &Rect::of(-2.0, -5.0, 0.0, 0.0));
    assert_eq!(r_min, Rect::largest());

    let mut r_max = Rect::of(MIN, MIN, MAX - 2.0, MAX - 5.0);
    wrapping_add_assign(&mut r_max, &Rect::of(0.0, 0.0, 2.0, 5.0));
    assert_eq!(r_max, Rect::largest());
}

#[test]
fn out_of_bounds() {
    let mut r1 = Rect::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    wrapping_add_assign(&mut r1, &Rect::of(-20.0, 0.0, 0.0, 0.0));
    assert_eq!(r1, Rect::of(MAX - 9.0, MIN + 10.0, MAX - 10.0, MAX - 10.0));

    let mut r2 = Rect::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    wrapping_add_assign(&mut r2, &Rect::of(0.0, -20.0, 0.0, 0.0));
    assert_eq!(r2, Rect::of(MIN + 10.0, MAX - 9.0, MAX - 10.0, MAX - 10.0));

    let mut r3 = Rect::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    wrapping_add_assign(&mut r3, &Rect::of(0.0, 0.0, 20.0, 0.0));
    assert_eq!(r3, Rect::of(MIN + 10.0, MIN + 10.0, MIN + 9.0, MAX - 10.0));

    let mut r4 = Rect::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    wrapping_add_assign(&mut r4, &Rect::of(0.0, 0.0, 0.0, 20.0));
    assert_eq!(r4, Rect::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MIN + 9.0));

    let mut r_min = Rect::of(MIN, MIN, MIN + 10.0, MIN + 10.0);
    wrapping_add_assign(&mut r_min, &Rect::of(-20.0, -20.0, -20.0, -20.0));
    assert_eq!(r_min, Rect::of(MAX - 19.0, MAX - 19.0, MAX - 9.0, MAX - 9.0));

    let mut r_max = Rect::of(MAX, MAX, MAX - 10.0, MAX - 10.0);
    wrapping_add_assign(&mut r_max, &Rect::of(20.0, 20.0, 20.0, 20.0));
    assert_eq!(r_max, Rect::of(MIN + 19.0, MIN + 19.0, MIN + 9.0, MIN + 9.0));
}

#[test]
fn edge_out_of_bounds() {
    let mut r1 = Rect::largest();
    wrapping_add_assign(&mut r1, &Rect::of(-1.0, 0.0, 0.0, 0.0));
    assert_eq!(r1, Rect::of(MAX, MIN, MAX, MAX));

    let mut r2 = Rect::largest();
    wrapping_add_assign(&mut r2, &Rect::of(0.0, -1.0, 0.0, 0.0));
    assert_eq!(r2, Rect::of(MIN, MAX, MAX, MAX));

    let mut r3 = Rect::largest();
    wrapping_add_assign(&mut r3, &Rect::of(0.0, 0.0, 1.0, 0.0));
    assert_eq!(r3, Rect::of(MIN, MIN, MIN, MAX));

    let mut r4 = Rect::largest();
    wrapping_add_assign(&mut r4, &Rect::of(0.0, 0.0, 0.0, 1.0));
    assert_eq!(r4, Rect::of(MIN, MIN, MAX, MIN));
}

#[test]
fn limits_out_of_bounds() {
    let mut r1 = Rect::largest();
    wrapping_add_assign(&mut r1, &Rect::of(MIN, 0.0, 0.0, 0.0));
    assert_eq!(r1, Rect::of(0.0, MIN, MAX, MAX));

    let mut r2 = Rect::largest();
    wrapping_add_assign(&mut r2, &Rect::of(0.0, MIN, 0.0, 0.0));
    assert_eq!(r2, Rect::of(MIN, 0.0, MAX, MAX));

    let mut r3 = Rect::largest();
    wrapping_add_assign(&mut r3, &Rect::of(0.0, 0.0, MAX, 0.0));
    assert_eq!(r3, Rect::of(MIN, MIN, -2.0, MAX));

    let mut r4 = Rect::largest();
    wrapping_add_assign(&mut r4, &Rect::of(0.0, 0.0, 0.0, MAX));
    assert_eq!(r4, Rect::of(MIN, MIN, MAX, -2.0));
}

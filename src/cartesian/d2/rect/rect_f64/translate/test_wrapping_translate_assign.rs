use super::wrapping_translate_assign;
use crate::cartesian::{
    d1::point::point_f64::{MAX, MIN},
    d2::{point::point_f64::Point, rect::rect_f64::Rect},
};

#[test]
fn test() {
    let mut r = Rect::of(0.0, 0.0, 12.0, 15.0);
    wrapping_translate_assign(&mut r, &Point::of(5.0, 4.0));
    assert_eq!(r, Rect::of(5.0, 4.0, 17.0, 19.0));
    wrapping_translate_assign(&mut r, &Point::of(-4.0, -2.0));
    assert_eq!(r, Rect::of(1.0, 2.0, 13.0, 17.0));
}

#[test]
fn to_bounds() {
    let mut r_min = Rect::of(MIN + 2.0, MIN + 5.0, MAX, MAX);
    wrapping_translate_assign(&mut r_min, &Point::of(-2.0, -5.0));
    assert_eq!(r_min, Rect::of(MIN, MIN, MAX - 2.0, MAX - 5.0));

    let mut r_max = Rect::of(MIN, MIN, MAX - 2.0, MAX - 5.0);
    wrapping_translate_assign(&mut r_max, &Point::of(2.0, 5.0));
    assert_eq!(r_max, Rect::of(MIN + 2.0, MIN + 5.0, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let mut r1 = Rect::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    wrapping_translate_assign(&mut r1, &Point::of(-20.0, 0.0));
    assert_eq!(r1, Rect::of(MAX - 9.0, MIN + 10.0, MAX - 30.0, MAX - 10.0));

    let mut r2 = Rect::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    wrapping_translate_assign(&mut r2, &Point::of(0.0, -20.0));
    assert_eq!(r2, Rect::of(MIN + 10.0, MAX - 9.0, MAX - 10.0, MAX - 30.0));

    let mut r3 = Rect::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    wrapping_translate_assign(&mut r3, &Point::of(20.0, 0.0));
    assert_eq!(r3, Rect::of(MIN + 30.0, MIN + 10.0, MIN + 9.0, MAX - 10.0));

    let mut r4 = Rect::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    wrapping_translate_assign(&mut r4, &Point::of(0.0, 20.0));
    assert_eq!(r4, Rect::of(MIN + 10.0, MIN + 30.0, MAX - 10.0, MIN + 9.0));

    let mut r_min = Rect::of(MIN, MIN, MIN + 10.0, MIN + 10.0);
    wrapping_translate_assign(&mut r_min, &Point::of(-20.0, -20.0));
    assert_eq!(r_min, Rect::of(MAX - 19.0, MAX - 19.0, MAX - 9.0, MAX - 9.0));

    let mut r_max = Rect::of(MAX, MAX, MAX - 10.0, MAX - 10.0);
    wrapping_translate_assign(&mut r_max, &Point::of(20.0, 20.0));
    assert_eq!(r_max, Rect::of(MIN + 19.0, MIN + 19.0, MIN + 9.0, MIN + 9.0));
}

#[test]
fn edge_out_of_bounds() {
    let mut r1 = Rect::largest();
    wrapping_translate_assign(&mut r1, &Point::of(-1.0, 0.0));
    assert_eq!(r1, Rect::of(MAX, MIN, MAX - 1.0, MAX));

    let mut r2 = Rect::largest();
    wrapping_translate_assign(&mut r2, &Point::of(0.0, -1.0));
    assert_eq!(r2, Rect::of(MIN, MAX, MAX, MAX - 1.0));

    let mut r3 = Rect::largest();
    wrapping_translate_assign(&mut r3, &Point::of(1.0, 0.0));
    assert_eq!(r3, Rect::of(MIN + 1.0, MIN, MIN, MAX));

    let mut r4 = Rect::largest();
    wrapping_translate_assign(&mut r4, &Point::of(0.0, 1.0));
    assert_eq!(r4, Rect::of(MIN, MIN + 1.0, MAX, MIN));
}

#[test]
fn limits_out_of_bounds() {
    let mut r1 = Rect::largest();
    wrapping_translate_assign(&mut r1, &Point::of(MIN, 0.0));
    assert_eq!(r1, Rect::of(0.0, MIN, -1.0, MAX));

    let mut r2 = Rect::largest();
    wrapping_translate_assign(&mut r2, &Point::of(0.0, MIN));
    assert_eq!(r2, Rect::of(MIN, 0.0, MAX, -1.0));

    let mut r3 = Rect::largest();
    wrapping_translate_assign(&mut r3, &Point::of(MAX, 0.0));
    assert_eq!(r3, Rect::of(-1.0, MIN, -2.0, MAX));

    let mut r4 = Rect::largest();
    wrapping_translate_assign(&mut r4, &Point::of(0.0, MAX));
    assert_eq!(r4, Rect::of(MIN, -1.0, MAX, -2.0));
}

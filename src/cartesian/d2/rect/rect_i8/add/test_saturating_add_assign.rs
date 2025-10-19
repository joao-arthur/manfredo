use super::saturating_add_assign;
use crate::cartesian::{
    d1::point::point_i8::{MAX, MIN},
    d2::rect::rect_i8::Rect,
};

#[test]
fn test() {
    let mut r = Rect::of(-7, 9, -12, 15);
    saturating_add_assign(&mut r, &Rect::of(5, 4, 3, 2));
    assert_eq!(r, Rect::of(-2, 13, -9, 17));
    saturating_add_assign(&mut r, &Rect::of(9, -10, 11, -12));
    assert_eq!(r, Rect::of(7, 3, 2, 5));
}

#[test]
fn to_bounds() {
    let mut r = Rect::of(MIN + 2, MIN + 5, MAX - 2, MAX - 5);
    saturating_add_assign(&mut r, &Rect::of(-2, -5, 2, 5));
    assert_eq!(r, Rect::largest());

    let mut r_min = Rect::of(MIN + 2, MIN + 5, MAX, MAX);
    saturating_add_assign(&mut r_min, &Rect::of(-2, -5, 0, 0));
    assert_eq!(r_min, Rect::largest());

    let mut r_max = Rect::of(MIN, MIN, MAX - 2, MAX - 5);
    saturating_add_assign(&mut r_max, &Rect::of(0, 0, 2, 5));
    assert_eq!(r_max, Rect::largest());
}

#[test]
fn out_of_bounds() {
    let mut r1 = Rect::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    saturating_add_assign(&mut r1, &Rect::of(-20, 0, 0, 0));
    assert_eq!(r1, Rect::of(MIN, MIN + 10, MAX - 10, MAX - 10));

    let mut r2 = Rect::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    saturating_add_assign(&mut r2, &Rect::of(0, -20, 0, 0));
    assert_eq!(r2, Rect::of(MIN + 10, MIN, MAX - 10, MAX - 10));

    let mut r3 = Rect::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    saturating_add_assign(&mut r3, &Rect::of(0, 0, 20, 0));
    assert_eq!(r3, Rect::of(MIN + 10, MIN + 10, MAX, MAX - 10));

    let mut r4 = Rect::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    saturating_add_assign(&mut r4, &Rect::of(0, 0, 0, 20));
    assert_eq!(r4, Rect::of(MIN + 10, MIN + 10, MAX - 10, MAX));
}

#[test]
fn edge_out_of_bounds() {
    let mut r = Rect::largest();
    saturating_add_assign(&mut r, &Rect::of(-1, 0, 0, 0));
    assert_eq!(r, Rect::largest());
    saturating_add_assign(&mut r, &Rect::of(0, -1, 0, 0));
    assert_eq!(r, Rect::largest());
    saturating_add_assign(&mut r, &Rect::of(0, 0, 1, 0));
    assert_eq!(r, Rect::largest());
    saturating_add_assign(&mut r, &Rect::of(0, 0, 0, 1));
    assert_eq!(r, Rect::largest());
}

#[test]
fn limits_out_of_bounds() {
    let mut r = Rect::largest();
    saturating_add_assign(&mut r, &Rect::of(MIN, 0, 0, 0));
    assert_eq!(r, Rect::largest());
    saturating_add_assign(&mut r, &Rect::of(0, MIN, 0, 0));
    assert_eq!(r, Rect::largest());
    saturating_add_assign(&mut r, &Rect::of(0, 0, MAX, 0));
    assert_eq!(r, Rect::largest());
    saturating_add_assign(&mut r, &Rect::of(0, 0, 0, MAX));
    assert_eq!(r, Rect::largest());
}

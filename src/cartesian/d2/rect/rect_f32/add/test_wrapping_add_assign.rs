use super::wrapping_add_assign;
use crate::cartesian::{
    d1::point::point_f32::{MAX, MIN},
    d2::rect::rect_f32::Rect,
};

#[test]
fn test() {
    let mut r = Rect::new((-7.0, 9.0), (-12.0, 15.0));
    wrapping_add_assign(&mut r, &Rect::new((5.0, 4.0), (3.0, 2.0)));
    assert_eq!(r, Rect::new((-2.0, 13.0), (-9.0, 17.0)));
    wrapping_add_assign(&mut r, &Rect::new((9.0, -10.0), (11.0, -12.0)));
    assert_eq!(r, Rect::new((7.0, 3.0), (2.0, 5.0)));
}

#[test]
fn to_bounds() {
    let mut r = Rect::new((MIN + 2.0, MIN + 5.0), (MAX - 2.0, MAX - 5.0));
    wrapping_add_assign(&mut r, &Rect::new((-2.0, -5.0), (2.0, 5.0)));
    assert_eq!(r, Rect::largest());

    let mut r_min = Rect::new((MIN + 2.0, MIN + 5.0), (MAX, MAX));
    wrapping_add_assign(&mut r_min, &Rect::new((-2.0, -5.0), (0.0, 0.0)));
    assert_eq!(r_min, Rect::largest());

    let mut r_max = Rect::new((MIN, MIN), (MAX - 2.0, MAX - 5.0));
    wrapping_add_assign(&mut r_max, &Rect::new((0.0, 0.0), (2.0, 5.0)));
    assert_eq!(r_max, Rect::largest());
}

#[test]
fn out_of_bounds() {
    let mut r1 = Rect::new((MIN + 10.0, MIN + 10.0), (MAX - 10.0, MAX - 10.0));
    wrapping_add_assign(&mut r1, &Rect::new((-20.0, 0.0), (0.0, 0.0)));
    assert_eq!(r1, Rect::new((MAX - 9.0, MIN + 10.0), (MAX - 10.0, MAX - 10.0)));

    let mut r2 = Rect::new((MIN + 10.0, MIN + 10.0), (MAX - 10.0, MAX - 10.0));
    wrapping_add_assign(&mut r2, &Rect::new((0.0, -20.0), (0.0, 0.0)));
    assert_eq!(r2, Rect::new((MIN + 10.0, MAX - 9.0), (MAX - 10.0, MAX - 10.0)));

    let mut r3 = Rect::new((MIN + 10.0, MIN + 10.0), (MAX - 10.0, MAX - 10.0));
    wrapping_add_assign(&mut r3, &Rect::new((0.0, 0.0), (20.0, 0.0)));
    assert_eq!(r3, Rect::new((MIN + 10.0, MIN + 10.0), (MIN + 9.0, MAX - 10.0)));

    let mut r4 = Rect::new((MIN + 10.0, MIN + 10.0), (MAX - 10.0, MAX - 10.0));
    wrapping_add_assign(&mut r4, &Rect::new((0.0, 0.0), (0.0, 20.0)));
    assert_eq!(r4, Rect::new((MIN + 10.0, MIN + 10.0), (MAX - 10.0, MIN + 9.0)));

    let mut r_min = Rect::new((MIN, MIN), (MIN + 10.0, MIN + 10.0));
    wrapping_add_assign(&mut r_min, &Rect::new((-20.0, -20.0), (-20.0, -20.0)));
    assert_eq!(r_min, Rect::new((MAX - 19.0, MAX - 19.0), (MAX - 9.0, MAX - 9.0)));

    let mut r_max = Rect::new((MAX, MAX), (MAX - 10.0, MAX - 10.0));
    wrapping_add_assign(&mut r_max, &Rect::new((20.0, 20.0), (20.0, 20.0)));
    assert_eq!(r_max, Rect::new((MIN + 19.0, MIN + 19.0), (MIN + 9.0, MIN + 9.0)));
}

#[test]
fn edge_out_of_bounds() {
    let mut r1 = Rect::largest();
    wrapping_add_assign(&mut r1, &Rect::new((-1.0, 0.0), (0.0, 0.0)));
    assert_eq!(r1, Rect::new((MAX, MIN), (MAX, MAX)));

    let mut r2 = Rect::largest();
    wrapping_add_assign(&mut r2, &Rect::new((0.0, -1.0), (0.0, 0.0)));
    assert_eq!(r2, Rect::new((MIN, MAX), (MAX, MAX)));

    let mut r3 = Rect::largest();
    wrapping_add_assign(&mut r3, &Rect::new((0.0, 0.0), (1.0, 0.0)));
    assert_eq!(r3, Rect::new((MIN, MIN), (MIN, MAX)));

    let mut r4 = Rect::largest();
    wrapping_add_assign(&mut r4, &Rect::new((0.0, 0.0), (0.0, 1.0)));
    assert_eq!(r4, Rect::new((MIN, MIN), (MAX, MIN)));
}

#[test]
fn limits_out_of_bounds() {
    let mut r1 = Rect::largest();
    wrapping_add_assign(&mut r1, &Rect::new((MIN, 0.0), (0.0, 0.0)));
    assert_eq!(r1, Rect::new((0.0, MIN), (MAX, MAX)));

    let mut r2 = Rect::largest();
    wrapping_add_assign(&mut r2, &Rect::new((0.0, MIN), (0.0, 0.0)));
    assert_eq!(r2, Rect::new((MIN, 0.0), (MAX, MAX)));

    let mut r3 = Rect::largest();
    wrapping_add_assign(&mut r3, &Rect::new((0.0, 0.0), (MAX, 0.0)));
    assert_eq!(r3, Rect::new((MIN, MIN), (-2.0, MAX)));

    let mut r4 = Rect::largest();
    wrapping_add_assign(&mut r4, &Rect::new((0.0, 0.0), (0.0, MAX)));
    assert_eq!(r4, Rect::new((MIN, MIN), (MAX, -2.0)));
}

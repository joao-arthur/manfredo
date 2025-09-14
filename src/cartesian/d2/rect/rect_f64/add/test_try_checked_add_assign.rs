use super::try_checked_add_assign;
use crate::cartesian::{
    point::point_f64::{MAX, MIN},
    rect::rect_f64::Rect,
};

#[test]
fn test() {
    let mut r = Rect::of(-7.0, 9.0, -12.0, 15.0);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(5.0, 4.0, 3.0, 2.0)), Some(()));
    assert_eq!(r, Rect::of(-2.0, 13.0, -9.0, 17.0));
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(9.0, -10.0, 11.0, -12.0)), Some(()));
    assert_eq!(r, Rect::of(7.0, 3.0, 2.0, 5.0));
}

#[test]
fn to_bounds() {
    let mut r_min = Rect::of(MIN + 2.0, MIN + 5.0, MAX - 2.0, MAX - 5.0);
    assert_eq!(try_checked_add_assign(&mut r_min, &Rect::of(-2.0, -5.0, 2.0, 5.0)), Some(()));
    assert_eq!(r_min, Rect::largest());

    let mut r_min = Rect::of(MIN + 2.0, MIN + 5.0, MAX, MAX);
    assert_eq!(try_checked_add_assign(&mut r_min, &Rect::of(-2.0, -5.0, 0.0, 0.0)), Some(()));
    assert_eq!(r_min, Rect::largest());

    let mut r_max = Rect::of(MIN, MIN, MAX - 2.0, MAX - 5.0);
    assert_eq!(try_checked_add_assign(&mut r_max, &Rect::of(0.0, 0.0, 2.0, 5.0)), Some(()));
    assert_eq!(r_max, Rect::largest());
}

#[test]
fn out_of_bounds() {
    let mut r = Rect::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(-20.0, 0.0, 0.0, 0.0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(0.0, -20.0, 0.0, 0.0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(0.0, 0.0, 20.0, 0.0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(0.0, 0.0, 0.0, 20.0)), None);
    assert_eq!(r, Rect::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0));
}

#[test]
fn edge_out_of_bounds() {
    let mut r = Rect::largest();
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(-1.0, 0.0, 0.0, 0.0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(0.0, -1.0, 0.0, 0.0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(0.0, 0.0, 1.0, 0.0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(0.0, 0.0, 0.0, 1.0)), None);
    assert_eq!(r, Rect::largest());
}

#[test]
fn limits_out_of_bounds() {
    let mut r = Rect::largest();
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(MIN, 0.0, 0.0, 0.0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(0.0, MIN, 0.0, 0.0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(0.0, 0.0, MAX, 0.0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::of(0.0, 0.0, 0.0, MAX)), None);
    assert_eq!(r, Rect::largest());
}

use super::try_checked_add_assign;
use crate::matrix::{
    d1::point::point_i16::{MAX, MIN},
    d2::rect::rect_i16::Rect,
};

#[test]
fn test() {
    let mut r = Rect::new((-7, 9), (-12, 15));
    assert_eq!(try_checked_add_assign(&mut r, &Rect::new((5, 4), (3, 2))), Some(()));
    assert_eq!(r, Rect::new((-2, 13), (-9, 17)));
    assert_eq!(try_checked_add_assign(&mut r, &Rect::new((9, -10), (11, -12))), Some(()));
    assert_eq!(r, Rect::new((7, 3), (2, 5)));
}

#[test]
fn to_bounds() {
    let mut r = Rect::new((MIN + 2, MIN + 5), (MAX - 2, MAX - 5));
    assert_eq!(try_checked_add_assign(&mut r, &Rect::new((-2, -5), (2, 5))), Some(()));
    assert_eq!(r, Rect::largest());

    let mut r_min = Rect::new((MIN + 2, MIN + 5), (MAX, MAX));
    assert_eq!(try_checked_add_assign(&mut r_min, &Rect::new((-2, -5), (0, 0))), Some(()));
    assert_eq!(r_min, Rect::largest());

    let mut r_max = Rect::new((MIN, MIN), (MAX - 2, MAX - 5));
    assert_eq!(try_checked_add_assign(&mut r_max, &Rect::new((0, 0), (2, 5))), Some(()));
    assert_eq!(r_max, Rect::largest());
}

#[test]
fn out_of_bounds() {
    let mut r = Rect::new((MIN + 10, MIN + 10), (MAX - 10, MAX - 10));
    assert_eq!(try_checked_add_assign(&mut r, &Rect::new((-20, 0), (0, 0))), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::new((0, -20), (0, 0))), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::new((0, 0), (20, 0))), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::new((0, 0), (0, 20))), None);
    assert_eq!(r, Rect::new((MIN + 10, MIN + 10), (MAX - 10, MAX - 10)));
}

#[test]
fn edge_out_of_bounds() {
    let mut r = Rect::largest();
    assert_eq!(try_checked_add_assign(&mut r, &Rect::new((-1, 0), (0, 0))), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::new((0, -1), (0, 0))), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::new((0, 0), (1, 0))), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::new((0, 0), (0, 1))), None);
    assert_eq!(r, Rect::largest());
}

#[test]
fn limits_out_of_bounds() {
    let mut r = Rect::largest();
    assert_eq!(try_checked_add_assign(&mut r, &Rect::new((MIN, 0), (0, 0))), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::new((0, MIN), (0, 0))), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::new((0, 0), (MAX, 0))), None);
    assert_eq!(try_checked_add_assign(&mut r, &Rect::new((0, 0), (0, MAX))), None);
    assert_eq!(r, Rect::largest());
}

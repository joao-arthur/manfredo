use super::try_checked_add;
use crate::matrix::{
    d1::point::point_i64::{MAX, MIN},
    d2::rect::rect_i64::Rect,
};

#[test]
fn test() {
    assert_eq!(try_checked_add(&Rect::new((-7, 9), (-12, 15)), &Rect::new((5, 4), (3, 2))), Some(Rect::new((-2, 13), (-9, 17))));
    assert_eq!(try_checked_add(&Rect::new((-2, 13), (-9, 17)), &Rect::new((9, -10), (11, -12))), Some(Rect::new((7, 3), (2, 5))));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&Rect::new((MIN + 2, MIN + 5), (MAX - 2, MAX - 5)), &Rect::new((-2, -5), (2, 5))), Some(Rect::largest()));
    assert_eq!(try_checked_add(&Rect::new((MIN + 2, MIN + 5), (MAX, MAX)), &Rect::new((-2, -5), (0, 0))), Some(Rect::largest()));
    assert_eq!(try_checked_add(&Rect::new((MIN, MIN), (MAX - 2, MAX - 5)), &Rect::new((0, 0), (2, 5))), Some(Rect::largest()));
}

#[test]
fn out_of_bounds() {
    let r = Rect::new((MIN + 10, MIN + 10), (MAX - 10, MAX - 10));
    assert_eq!(try_checked_add(&r, &Rect::new((-20, 0), (0, 0))), None);
    assert_eq!(try_checked_add(&r, &Rect::new((0, -20), (0, 0))), None);
    assert_eq!(try_checked_add(&r, &Rect::new((0, 0), (20, 0))), None);
    assert_eq!(try_checked_add(&r, &Rect::new((0, 0), (0, 20))), None);
}

#[test]
fn edge_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(try_checked_add(&r, &Rect::new((-1, 0), (0, 0))), None);
    assert_eq!(try_checked_add(&r, &Rect::new((0, -1), (0, 0))), None);
    assert_eq!(try_checked_add(&r, &Rect::new((0, 0), (1, 0))), None);
    assert_eq!(try_checked_add(&r, &Rect::new((0, 0), (0, 1))), None);
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(try_checked_add(&r, &Rect::new((MIN, 0), (0, 0))), None);
    assert_eq!(try_checked_add(&r, &Rect::new((0, MIN), (0, 0))), None);
    assert_eq!(try_checked_add(&r, &Rect::new((0, 0), (MAX, 0))), None);
    assert_eq!(try_checked_add(&r, &Rect::new((0, 0), (0, MAX))), None);
}

use super::try_checked_add_assign;
use crate::matrix::{
    d1::point::point_u64::MAX,
    d2::rect::{rect_i64::Rect as RectI, rect_u64::Rect},
};

#[test]
fn test() {
    let mut r = Rect::new((0, 0), (12, 12));
    assert_eq!(try_checked_add_assign(&mut r, &RectI::new((5, 4), (3, 2))), Some(()));
    assert_eq!(r, Rect::new((5, 4), (15, 14)));
    assert_eq!(try_checked_add_assign(&mut r, &RectI::new((-4, -3), (-2, -1))), Some(()));
    assert_eq!(r, Rect::new((1, 1), (13, 13)));
}

#[test]
fn to_bounds() {
    let mut r = Rect::new((2, 5), (MAX - 2, MAX - 5));
    assert_eq!(try_checked_add_assign(&mut r, &RectI::new((-2, -5), (2, 5))), Some(()));
    assert_eq!(r, Rect::largest());

    let mut r_min = Rect::new((2, 5), (MAX, MAX));
    assert_eq!(try_checked_add_assign(&mut r_min, &RectI::new((-2, -5), (0, 0))), Some(()));
    assert_eq!(r_min, Rect::largest());

    let mut r_max = Rect::new((0, 0), (MAX - 2, MAX - 5));
    assert_eq!(try_checked_add_assign(&mut r_max, &RectI::new((0, 0), (2, 5))), Some(()));
    assert_eq!(r_max, Rect::largest());
}

#[test]
fn out_of_bounds() {
    let mut r = Rect::new((10, 10), (MAX - 10, MAX - 10));
    assert_eq!(try_checked_add_assign(&mut r, &RectI::new((-20, 0), (0, 0))), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI::new((0, -20), (0, 0))), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI::new((0, 0), (20, 0))), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI::new((0, 0), (0, 20))), None);
    assert_eq!(r, Rect::new((10, 10), (MAX - 10, MAX - 10)));
}

#[test]
fn edge_out_of_bounds() {
    let mut r = Rect::largest();
    assert_eq!(try_checked_add_assign(&mut r, &RectI::new((-1, 0), (0, 0))), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI::new((0, -1), (0, 0))), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI::new((0, 0), (1, 0))), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI::new((0, 0), (0, 1))), None);
    assert_eq!(r, Rect::largest());
}

#[test]
fn limits_out_of_bounds() {
    let mut r = Rect::largest();
    assert_eq!(try_checked_add_assign(&mut r, &RectI::new((i64::MIN, 0), (0, 0))), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI::new((0, i64::MIN), (0, 0))), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI::new((0, 0), (i64::MAX, 0))), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI::new((0, 0), (0, i64::MAX))), None);
    assert_eq!(r, Rect::largest());
}

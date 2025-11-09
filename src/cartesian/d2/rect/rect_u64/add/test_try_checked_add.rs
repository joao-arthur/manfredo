use super::try_checked_add;
use crate::cartesian::{
    d1::point::point_u64::MAX,
    d2::rect::{rect_i64::Rect as RectI, rect_u64::Rect},
};

#[test]
fn test() {
    assert_eq!(try_checked_add(&Rect::new((0, 0), (12, 10)), &RectI::new((5, 4), (3, 2))), Some(Rect::new((5, 4), (15, 12))));
    assert_eq!(try_checked_add(&Rect::new((5, 4), (15, 12)), &RectI::new((-4, -3), (-2, -1))), Some(Rect::new((1, 1), (13, 11))));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&Rect::new((2, 5), (MAX - 2, MAX - 5)), &RectI::new((-2, -5), (2, 5))), Some(Rect::largest()));
    assert_eq!(try_checked_add(&Rect::new((2, 5), (MAX, MAX)), &RectI::new((-2, -5), (0, 0))), Some(Rect::largest()));
    assert_eq!(try_checked_add(&Rect::new((0, 0), (MAX - 2, MAX - 5)), &RectI::new((0, 0), (2, 5))), Some(Rect::largest()));
}

#[test]
fn out_of_bounds() {
    let r = Rect::new((10, 10), (MAX - 10, MAX - 10));
    assert_eq!(try_checked_add(&r, &RectI::new((-20, 0), (0, 0))), None);
    assert_eq!(try_checked_add(&r, &RectI::new((0, -20), (0, 0))), None);
    assert_eq!(try_checked_add(&r, &RectI::new((0, 0), (20, 0))), None);
    assert_eq!(try_checked_add(&r, &RectI::new((0, 0), (0, 20))), None);
}

#[test]
fn edge_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(try_checked_add(&r, &RectI::new((-1, 0), (0, 0))), None);
    assert_eq!(try_checked_add(&r, &RectI::new((0, -1), (0, 0))), None);
    assert_eq!(try_checked_add(&r, &RectI::new((0, 0), (1, 0))), None);
    assert_eq!(try_checked_add(&r, &RectI::new((0, 0), (0, 1))), None);
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(try_checked_add(&r, &RectI::new((i64::MIN, 0), (0, 0))), None);
    assert_eq!(try_checked_add(&r, &RectI::new((0, i64::MIN), (0, 0))), None);
    assert_eq!(try_checked_add(&r, &RectI::new((0, 0), (i64::MAX, 0))), None);
    assert_eq!(try_checked_add(&r, &RectI::new((0, 0), (0, i64::MAX))), None);
}

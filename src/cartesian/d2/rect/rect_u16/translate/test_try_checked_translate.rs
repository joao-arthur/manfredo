use super::try_checked_translate;
use crate::cartesian::{
    d1::point::point_u16::MAX,
    d2::{point::point_i16::Point, rect::rect_u16::Rect},
};

#[test]
fn test() {
    assert_eq!(try_checked_translate(&Rect::new((0, 0), (12, 15)), &Point::new(5, 4)), Some(Rect::new((5, 4), (17, 19))));
    assert_eq!(try_checked_translate(&Rect::new((5, 4), (17, 19)), &Point::new(-4, -2)), Some(Rect::new((1, 2), (13, 17))));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_translate(&Rect::new((2, 5), (MAX, MAX)), &Point::new(-2, -5)), Some(Rect::new((0, 0), (MAX - 2, MAX - 5))));
    assert_eq!(try_checked_translate(&Rect::new((0, 0), (MAX - 2, MAX - 5)), &Point::new(2, 5)), Some(Rect::new((2, 5), (MAX, MAX))));
}

#[test]
fn out_of_bounds() {
    let r = Rect::new((10, 10), (MAX - 10, MAX - 10));
    assert_eq!(try_checked_translate(&r, &Point::new(-20, 0)), None);
    assert_eq!(try_checked_translate(&r, &Point::new(0, -20)), None);
    assert_eq!(try_checked_translate(&r, &Point::new(20, 0)), None);
    assert_eq!(try_checked_translate(&r, &Point::new(0, 20)), None);
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(try_checked_translate(&r, &Point::new(i16::MIN, 0)), None);
    assert_eq!(try_checked_translate(&r, &Point::new(0, i16::MIN)), None);
    assert_eq!(try_checked_translate(&r, &Point::new(i16::MAX, 0)), None);
    assert_eq!(try_checked_translate(&r, &Point::new(0, i16::MAX)), None);
}

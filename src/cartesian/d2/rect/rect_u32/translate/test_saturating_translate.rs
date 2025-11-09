use super::saturating_translate;
use crate::cartesian::{
    d1::point::point_u32::MAX,
    d2::{point::point_i32::Point, rect::rect_u32::Rect},
};

#[test]
fn test() {
    assert_eq!(saturating_translate(&Rect::new((0, 0), (12, 15)), &Point::new(5, 4)), Rect::new((5, 4), (17, 19)));
    assert_eq!(saturating_translate(&Rect::new((5, 4), (17, 19)), &Point::new(-4, -2)), Rect::new((1, 2), (13, 17)));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_translate(&Rect::new((2, 5), (MAX, MAX)), &Point::new(-2, -5)), Rect::new((0, 0), (MAX - 2, MAX - 5)));
    assert_eq!(saturating_translate(&Rect::new((0, 0), (MAX - 2, MAX - 5)), &Point::new(2, 5)), Rect::new((2, 5), (MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    let r = Rect::new((10, 10), (MAX - 10, MAX - 10));
    assert_eq!(saturating_translate(&r, &Point::new(-20, 0)), Rect::new((0, 10), (MAX - 20, MAX - 10)));
    assert_eq!(saturating_translate(&r, &Point::new(0, -20)), Rect::new((10, 0), (MAX - 10, MAX - 20)));
    assert_eq!(saturating_translate(&r, &Point::new(20, 0)), Rect::new((20, 10), (MAX, MAX - 10)));
    assert_eq!(saturating_translate(&r, &Point::new(0, 20)), Rect::new((10, 20), (MAX - 10, MAX)));
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(saturating_translate(&r, &Point::new(i32::MIN, 0)), Rect::largest());
    assert_eq!(saturating_translate(&r, &Point::new(0, i32::MIN)), Rect::largest());
    assert_eq!(saturating_translate(&r, &Point::new(i32::MAX, 0)), Rect::largest());
    assert_eq!(saturating_translate(&r, &Point::new(0, i32::MAX)), Rect::largest());
}

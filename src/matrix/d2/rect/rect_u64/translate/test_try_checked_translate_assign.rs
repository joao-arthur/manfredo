use super::try_checked_translate_assign;
use crate::matrix::{
    d1::point::point_u64::MAX,
    d2::{point::point_i64::Point, rect::rect_u64::Rect},
};

#[test]
fn test() {
    let mut r = Rect::new((0, 0), (12, 15));
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(5, 4)), Some(()));
    assert_eq!(r, Rect::new((5, 4), (17, 19)));
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(-4, -2)), Some(()));
    assert_eq!(r, Rect::new((1, 2), (13, 17)));
}

#[test]
fn to_bounds() {
    let mut r_min = Rect::new((2, 5), (MAX, MAX));
    assert_eq!(try_checked_translate_assign(&mut r_min, &Point::new(-2, -5)), Some(()));
    assert_eq!(r_min, Rect::new((0, 0), (MAX - 2, MAX - 5)));

    let mut r_max = Rect::new((0, 0), (MAX - 2, MAX - 5));
    assert_eq!(try_checked_translate_assign(&mut r_max, &Point::new(2, 5)), Some(()));
    assert_eq!(r_max, Rect::new((2, 5), (MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    let mut r = Rect::new((10, 10), (MAX - 10, MAX - 10));
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(-20, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(0, -20)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(20, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(0, 20)), None);
    assert_eq!(r, Rect::new((10, 10), (MAX - 10, MAX - 10)));
}

#[test]
fn limits_out_of_bounds() {
    let mut r = Rect::largest();
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(i64::MIN, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(0, i64::MIN)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(i64::MAX, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &Point::new(0, i64::MAX)), None);
    assert_eq!(r, Rect::largest());
}

use super::saturating_translate_assign;
use crate::cartesian::{
    d1::point::point_f64::{MAX, MIN},
    d2::{point::point_f64::Point, rect::rect_f64::Rect},
};

#[test]
fn test() {
    let mut r = Rect::new((0.0, 0.0), (10.0, 10.0));
    saturating_translate_assign(&mut r, &Point::new(10.0, 20.0));
    assert_eq!(r, Rect::new((10.0, 20.0), (20.0, 30.0)));
    saturating_translate_assign(&mut r, &Point::new(-20.0, -15.0));
    assert_eq!(r, Rect::new((-10.0, 5.0), (0.0, 15.0)));
}

#[test]
fn to_bounds() {
    let mut r_min_1 = Rect::new((MIN + 2.0, MIN + 5.0), (0.0, 0.0));
    saturating_translate_assign(&mut r_min_1, &Point::new(-2.0, -5.0));
    assert_eq!(r_min_1, Rect::new((MIN, MIN), (-2.0, -5.0)));

    let mut r_min_2 = Rect::new((MIN, MIN), (-2.0, -5.0));
    saturating_translate_assign(&mut r_min_2, &Point::new(2.0, 5.0));
    assert_eq!(r_min_2, Rect::new((MIN + 2.0, MIN + 5.0), (0.0, 0.0)));

    let mut r_max_1 = Rect::new((2.0, 5.0), (MAX, MAX));
    saturating_translate_assign(&mut r_max_1, &Point::new(-2.0, -5.0));
    assert_eq!(r_max_1, Rect::new((0.0, 0.0), (MAX - 2.0, MAX - 5.0)));

    let mut r_max_2 = Rect::new((0.0, 0.0), (MAX - 2.0, MAX - 5.0));
    saturating_translate_assign(&mut r_max_2, &Point::new(2.0, 5.0));
    assert_eq!(r_max_2, Rect::new((2.0, 5.0), (MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    let mut r_min_1 = Rect::new((MIN + 10.0, MIN + 10.0), (0.0, 0.0));
    saturating_translate_assign(&mut r_min_1, &Point::new(-20.0, 0.0));
    assert_eq!(r_min_1, Rect::new((MIN, MIN + 10.0), (-10.0, 0.0)));

    let mut r_min_2 = Rect::new((MIN + 10.0, MIN + 10.0), (0.0, 0.0));
    saturating_translate_assign(&mut r_min_2, &Point::new(0.0, -20.0));
    assert_eq!(r_min_2, Rect::new((MIN + 10.0, MIN), (0.0, -10.0)));

    let mut r_min_3 = Rect::new((MIN + 10.0, MIN + 10.0), (0.0, 0.0));
    saturating_translate_assign(&mut r_min_3, &Point::new(20.0, 0.0));
    assert_eq!(r_min_3, Rect::new((MIN + 30.0, MIN + 10.0), (20.0, 0.0)));

    let mut r_min_4 = Rect::new((MIN + 10.0, MIN + 10.0), (0.0, 0.0));
    saturating_translate_assign(&mut r_min_4, &Point::new(0.0, 20.0));
    assert_eq!(r_min_4, Rect::new((MIN + 10.0, MIN + 30.0), (0.0, 20.0)));

    let mut r_max_1 = Rect::new((0.0, 0.0), (MAX - 10.0, MAX - 10.0));
    saturating_translate_assign(&mut r_max_1, &Point::new(-20.0, 0.0));
    assert_eq!(r_max_1, Rect::new((-20.0, 0.0), (MAX - 30.0, MAX - 10.0)));

    let mut r_max_2 = Rect::new((0.0, 0.0), (MAX - 10.0, MAX - 10.0));
    saturating_translate_assign(&mut r_max_2, &Point::new(0.0, -20.0));
    assert_eq!(r_max_2, Rect::new((0.0, -20.0), (MAX - 10.0, MAX - 30.0)));

    let mut r_max_3 = Rect::new((0.0, 0.0), (MAX - 10.0, MAX - 10.0));
    saturating_translate_assign(&mut r_max_3, &Point::new(20.0, 0.0));
    assert_eq!(r_max_3, Rect::new((10.0, 0.0), (MAX, MAX - 10.0)));

    let mut r_max_4 = Rect::new((0.0, 0.0), (MAX - 10.0, MAX - 10.0));
    saturating_translate_assign(&mut r_max_4, &Point::new(0.0, 20.0));
    assert_eq!(r_max_4, Rect::new((0.0, 10.0), (MAX - 10.0, MAX)));
}

#[test]
fn limits_out_of_bounds() {
    let mut r1 = Rect::largest();
    saturating_translate_assign(&mut r1, &Point::new(MIN, 0.0));
    assert_eq!(r1, Rect::new((MIN, MIN), (-1.0, -1.0)));

    let mut r2 = Rect::largest();
    saturating_translate_assign(&mut r2, &Point::new(0.0, MIN));
    assert_eq!(r2, Rect::new((MIN, MIN), (-1.0, -1.0)));

    let mut r3 = Rect::largest();
    saturating_translate_assign(&mut r3, &Point::new(MAX, 0.0));
    assert_eq!(r3, Rect::new((-1.0, MIN), (MAX - 1.0, -1.0)));

    let mut r4 = Rect::largest();
    saturating_translate_assign(&mut r4, &Point::new(0.0, MAX));
    assert_eq!(r4, Rect::new((MIN, -1.0), (-1.0, MAX - 1.0)));
}

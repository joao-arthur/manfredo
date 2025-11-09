use super::checked_translate_assign;
use crate::cartesian::d2::{point::point_i64::Point, rect::rect_i64::Rect};

#[test]
fn test() {
    let mut r = Rect::new((5, 9), (13, 37));
    checked_translate_assign(&mut r, &Point::new(-10, -20));
    assert_eq!(r, Rect::new((-5, -11), (3, 17)));
    checked_translate_assign(&mut r, &Point::new(6, -19));
    assert_eq!(r, Rect::new((1, -30), (9, -2)));
}

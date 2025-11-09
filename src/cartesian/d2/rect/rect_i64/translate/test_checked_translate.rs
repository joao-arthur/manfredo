use super::checked_translate;
use crate::cartesian::d2::{point::point_i64::Point, rect::rect_i64::Rect};

#[test]
fn test() {
    assert_eq!(checked_translate(&Rect::new((5, 9), (13, 37)), &Point::new(-10, -20)), Rect::new((-5, -11), (3, 17)));
    assert_eq!(checked_translate(&Rect::new((-5, -11), (3, 17)), &Point::new(6, -19)), Rect::new((1, -30), (9, -2)));
}

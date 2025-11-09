use super::checked_translate;
use crate::cartesian::d2::{point::point_i64::Point, rect::rect_u64::Rect};

#[test]
fn test() {
    assert_eq!(checked_translate(&Rect::new((0, 0), (12, 15)), &Point::new(5, 4)), Rect::new((5, 4), (17, 19)));
    assert_eq!(checked_translate(&Rect::new((5, 4), (17, 19)), &Point::new(-4, -2)), Rect::new((1, 2), (13, 17)));
}

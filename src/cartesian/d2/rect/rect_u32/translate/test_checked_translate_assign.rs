use super::checked_translate_assign;
use crate::cartesian::d2::{point::point_i32::Point, rect::rect_u32::Rect};

#[test]
fn test() {
    let mut r = Rect::new((0, 0), (12, 15));
    checked_translate_assign(&mut r, &Point::new(5, 4));
    assert_eq!(r, Rect::new((5, 4), (17, 19)));
    checked_translate_assign(&mut r, &Point::new(-4, -2));
    assert_eq!(r, Rect::new((1, 2), (13, 17)));
}

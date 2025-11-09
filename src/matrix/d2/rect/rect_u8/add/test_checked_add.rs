use super::checked_add;
use crate::matrix::d2::rect::{rect_i8::Rect as RectI, rect_u8::Rect};

#[test]
fn test() {
    assert_eq!(checked_add(&Rect::new((0, 0), (12, 15)), &RectI::new((5, 4), (3, 2))), Rect::new((5, 4), (15, 17)));
    assert_eq!(checked_add(&Rect::new((5, 4), (15, 20)), &RectI::new((-4, -3), (-2, -1))), Rect::new((1, 1), (13, 19)));
}

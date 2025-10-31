use super::checked_add;
use crate::cartesian::d2::rect::{rect_i16::Rect as RectI, rect_u16::Rect};

#[test]
fn test() {
    assert_eq!(checked_add(&Rect::of((0, 0), (12, 15)), &RectI::of((5, 4), (3, 2))), Rect::of((5, 4), (15, 17)));
    assert_eq!(checked_add(&Rect::of((5, 4), (15, 20)), &RectI::of((-4, -3), (-2, -1))), Rect::of((1, 1), (13, 19)));
}

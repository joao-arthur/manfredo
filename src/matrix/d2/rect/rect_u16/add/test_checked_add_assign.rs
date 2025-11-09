use super::checked_add_assign;
use crate::matrix::d2::rect::{rect_i16::Rect as RectI, rect_u16::Rect};

#[test]
fn test() {
    let mut r = Rect::new((0, 0), (12, 10));
    checked_add_assign(&mut r, &RectI::new((5, 4), (3, 2)));
    assert_eq!(r, Rect::new((5, 4), (15, 12)));
    checked_add_assign(&mut r, &RectI::new((-4, -3), (-2, -1)));
    assert_eq!(r, Rect::new((1, 1), (13, 11)));
}

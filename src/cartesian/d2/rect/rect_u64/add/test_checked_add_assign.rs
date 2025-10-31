use super::checked_add_assign;
use crate::cartesian::d2::rect::{rect_i64::Rect as RectI, rect_u64::Rect};

#[test]
fn test() {
    let mut r = Rect::of((0, 0), (12, 10));
    checked_add_assign(&mut r, &RectI::of((5, 4), (3, 2)));
    assert_eq!(r, Rect::of((5, 4), (15, 12)));
    checked_add_assign(&mut r, &RectI::of((-4, -3), (-2, -1)));
    assert_eq!(r, Rect::of((1, 1), (13, 11)));
}

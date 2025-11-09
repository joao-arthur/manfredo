use super::checked_add;
use crate::cartesian::d2::rect::{rect_i64::Rect as RectI, rect_u64::Rect};

#[test]
fn test() {
    assert_eq!(checked_add(&Rect::new((0, 0), (12, 10)), &RectI::new((5, 4), (3, 2))), Rect::new((5, 4), (15, 12)));
    assert_eq!(checked_add(&Rect::new((5, 4), (15, 12)), &RectI::new((-4, -3), (-2, -1))), Rect::new((1, 1), (13, 11)));
}

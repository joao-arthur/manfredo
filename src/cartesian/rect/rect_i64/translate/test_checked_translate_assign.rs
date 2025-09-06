use super::checked_translate_assign;
use crate::cartesian::{point::point_i64::PointI64, rect::rect_i64::RectI64};

#[test]
fn test_checked_translate_assign() {
    let mut r = RectI64::of(5, 9, 13, 37);
    checked_translate_assign(&mut r, &PointI64::of(-10, -20));
    assert_eq!(r, RectI64::of(-5, -11, 3, 17));
    checked_translate_assign(&mut r, &PointI64::of(6, -19));
    assert_eq!(r, RectI64::of(1, -30, 9, -2));
}

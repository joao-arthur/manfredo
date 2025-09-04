use super::checked_translate_assign;
use crate::cartesian::{point::point_i64::PointI64, rect::rect_i64::RectI64};

#[test]
fn test_checked_translate_assign() {
    let mut r = RectI64::of(0, 0, 10, 10);
    checked_translate_assign(&mut r, &PointI64::of(10, 20));
    assert_eq!(r, RectI64::of(10, 20, 20, 30));
    checked_translate_assign(&mut r, &PointI64::of(-20, -15));
    assert_eq!(r, RectI64::of(-10, 5, 0, 15));
}

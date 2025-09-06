use super::checked_translate_assign;
use crate::matrix::{point::point_i64::PointI64, rect::rect_u64::RectU64};

#[test]
fn test() {
    let mut r = RectU64::of(0, 0, 12, 15);
    checked_translate_assign(&mut r, &PointI64::of(5, 4));
    assert_eq!(r, RectU64::of(5, 4, 17, 19));
    checked_translate_assign(&mut r, &PointI64::of(-4, -2));
    assert_eq!(r, RectU64::of(1, 2, 13, 17));
}

use super::checked_add;
use crate::matrix::d2::rect::rect_i64::Rect;

#[test]
fn test() {
    assert_eq!(checked_add(&Rect::new((-7, 9), (-12, 15)), &Rect::new((5, 4), (3, 2))), Rect::new((-2, 13), (-9, 17)));
    assert_eq!(checked_add(&Rect::new((-2, 13), (-9, 17)), &Rect::new((9, -10), (11, -12))), Rect::new((7, 3), (2, 5)));
}

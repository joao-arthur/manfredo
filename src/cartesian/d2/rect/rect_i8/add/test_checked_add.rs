use super::checked_add;
use crate::cartesian::d2::rect::rect_i8::Rect;

#[test]
fn test() {
    assert_eq!(checked_add(&Rect::of((-7, 9), (-12, 15)), &Rect::of((5, 4), (3, 2))), Rect::of((-2, 13), (-9, 17)));
    assert_eq!(checked_add(&Rect::of((-2, 13), (-9, 17)), &Rect::of((9, -10), (11, -12))), Rect::of((7, 3), (2, 5)));
}

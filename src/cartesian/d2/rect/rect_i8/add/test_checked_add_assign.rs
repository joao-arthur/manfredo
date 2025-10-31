use super::checked_add_assign;
use crate::cartesian::d2::rect::rect_i8::Rect;

#[test]
fn test() {
    let mut r = Rect::of((-7, 9), (-12, 15));
    checked_add_assign(&mut r, &Rect::of((5, 4), (3, 2)));
    assert_eq!(r, Rect::of((-2, 13), (-9, 17)));
    checked_add_assign(&mut r, &Rect::of((9, -10), (11, -12)));
    assert_eq!(r, Rect::of((7, 3), (2, 5)));
}

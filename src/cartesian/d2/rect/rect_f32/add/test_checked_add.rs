use super::checked_add;
use crate::cartesian::d2::rect::rect_f32::Rect;

#[test]
fn test() {
    assert_eq!(checked_add(&Rect::new((-7.0, 9.0), (-12.0, 15.0)), &Rect::new((5.0, 4.0), (3.0, 2.0))), Rect::new((-2.0, 13.0), (-9.0, 17.0)));
    assert_eq!(checked_add(&Rect::new((-2.0, 13.0), (-9.0, 17.0)), &Rect::new((9.0, -10.0), (11.0, -12.0))), Rect::new((7.0, 3.0), (2.0, 5.0)));
}

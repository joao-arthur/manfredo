use super::checked_add_assign;
use crate::cartesian::d2::rect::rect_f64::Rect;

#[test]
fn test() {
    let mut r = Rect::new((-7.0, 9.0), (-12.0, 15.0));
    checked_add_assign(&mut r, &Rect::new((5.0, 4.0), (3.0, 2.0)));
    assert_eq!(r, Rect::new((-2.0, 13.0), (-9.0, 17.0)));
    checked_add_assign(&mut r, &Rect::new((9.0, -10.0), (11.0, -12.0)));
    assert_eq!(r, Rect::new((7.0, 3.0), (2.0, 5.0)));
}

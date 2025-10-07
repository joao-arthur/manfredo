use super::delta_x;
use crate::cartesian::d2::rect::rect_u8::Rect;

const MAX: u8 = u8::MAX;

#[test]
fn test_delta_x() {
    assert_eq!(delta_x(&Rect::of(0, 0, 0, 0)), 0);
    assert_eq!(delta_x(&Rect::of(0, 0, MAX, 0)), MAX);
}

use super::delta_y;
use crate::cartesian::d2::rect::rect_i8::Rect;

const MIN: i8 = i8::MIN;
const MAX: i8 = i8::MAX;

#[test]
fn test_delta_y() {
    assert_eq!(delta_y(&Rect::of(MIN, 0, MAX, 0)), 0);
    assert_eq!(delta_y(&Rect::of(0, MIN, 0, MAX)), u8::MAX);
}

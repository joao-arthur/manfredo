use super::delta_y;
use crate::cartesian::d2::rect::rect_i16::Rect;

const MIN: i16 = i16::MIN;
const MAX: i16 = i16::MAX;

#[test]
fn test_delta_y() {
    assert_eq!(delta_y(&Rect::of(MIN, 0, MAX, 0)), 0);
    assert_eq!(delta_y(&Rect::of(0, MIN, 0, MAX)), u16::MAX);
}

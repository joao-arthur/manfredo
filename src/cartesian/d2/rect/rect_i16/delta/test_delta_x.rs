use super::delta_x;
use crate::cartesian::d2::rect::rect_i16::Rect;

const MIN: i16 = i16::MIN;
const MAX: i16 = i16::MAX;

#[test]
fn test_delta_x() {
    assert_eq!(delta_x(&Rect::of(0, MIN, 0, MAX)), 0);
    assert_eq!(delta_x(&Rect::of(MIN, 0, MAX, 0)), u16::MAX);
}

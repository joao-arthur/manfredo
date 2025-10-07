use super::delta_y;
use crate::cartesian::d2::rect::rect_u32::Rect;

const MAX: u32 = u32::MAX;

#[test]
fn test_delta_y() {
    assert_eq!(delta_y(&Rect::of(0, 0, 0, 0)), 0);
    assert_eq!(delta_y(&Rect::of(0, 0, 0, MAX)), MAX);
}

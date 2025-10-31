use super::delta_y;
use crate::cartesian::{d1::point::point_u16::MAX, d2::rect::rect_u16::Rect};

#[test]
fn test_delta_y() {
    assert_eq!(delta_y(&Rect::min()), 0);
    assert_eq!(delta_y(&Rect::of((0, 0), (0, MAX))), MAX);
}

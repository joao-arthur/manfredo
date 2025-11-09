use super::delta_y;
use crate::cartesian::{d1::point::point_u8::MAX, d2::rect::rect_u8::Rect};

#[test]
fn test_delta_y() {
    assert_eq!(delta_y(&Rect::min()), 0);
    assert_eq!(delta_y(&Rect::new((0, 0), (0, MAX))), MAX);
}

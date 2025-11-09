use super::delta_x;
use crate::cartesian::{d1::point::point_u8::MAX, d2::rect::rect_u8::Rect};

#[test]
fn test_delta_x() {
    assert_eq!(delta_x(&Rect::min()), 0);
    assert_eq!(delta_x(&Rect::new((0, 0), (MAX, 0))), MAX);
}

use super::delta_y;
use crate::cartesian::{
    d1::point::point_i16::{MAX, MIN},
    d2::rect::rect_i16::Rect,
};

#[test]
fn test_delta_y() {
    assert_eq!(delta_y(&Rect::new((MIN, 0), (MAX, 0))), 0);
    assert_eq!(delta_y(&Rect::new((0, MIN), (0, MAX))), u16::MAX);
}

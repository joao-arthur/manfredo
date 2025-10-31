use super::delta_y;
use crate::cartesian::{
    d1::point::point_i32::{MAX, MIN},
    d2::rect::rect_i32::Rect,
};

#[test]
fn test_delta_y() {
    assert_eq!(delta_y(&Rect::of((MIN, 0), (MAX, 0))), 0);
    assert_eq!(delta_y(&Rect::of((0, MIN), (0, MAX))), u32::MAX);
}

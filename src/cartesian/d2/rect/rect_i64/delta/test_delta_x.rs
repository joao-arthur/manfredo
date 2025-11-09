use super::delta_x;
use crate::cartesian::{
    d1::point::point_i64::{MAX, MIN},
    d2::rect::rect_i64::Rect,
};

#[test]
fn test_delta_x() {
    assert_eq!(delta_x(&Rect::new((0, MIN), (0, MAX))), 0);
    assert_eq!(delta_x(&Rect::new((MIN, 0), (MAX, 0))), u64::MAX);
}

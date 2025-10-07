use super::delta_x;
use crate::cartesian::d2::{point::point_f32::MAX, rect::rect_f32::Rect};

#[test]
fn test_delta_x() {
    assert_eq!(delta_x(&Rect::of(0.0, 0.0, 0.0, MAX)), 0.0);
    assert_eq!(delta_x(&Rect::of(0.0, 0.0, MAX, 0.0)), MAX);
    assert_eq!(delta_x(&Rect::of(0.0, -8_388_608.0, 0.0, 8_388_607.0)), 0.0);
    assert_eq!(delta_x(&Rect::of(-8_388_608.0, 0.0, 8_388_607.0, 0.0)), MAX);
}

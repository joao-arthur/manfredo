use super::delta_y;
use crate::cartesian::{d1::point::point_f32::MAX, d2::rect::rect_f32::Rect};

#[test]
fn test_delta_y() {
    assert_eq!(delta_y(&Rect::new((0.0, 0.0), (MAX, 0.0))), 0.0);
    assert_eq!(delta_y(&Rect::new((0.0, 0.0), (0.0, MAX))), MAX);
    assert_eq!(delta_y(&Rect::new((-8_388_608.0, 0.0), (8_388_607.0, 0.0))), 0.0);
    assert_eq!(delta_y(&Rect::new((0.0, -8_388_608.0), (0.0, 8_388_607.0))), MAX);
}

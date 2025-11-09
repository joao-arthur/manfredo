use super::checked_translate;
use crate::cartesian::d2::{point::point_f32::Point, rect::rect_f32::Rect};

#[test]
fn test() {
    assert_eq!(checked_translate(&Rect::new((0.0, 0.0), (10.0, 10.0)), &Point::new(10.0, 20.0)), Rect::new((10.0, 20.0), (20.0, 30.0)));
    assert_eq!(checked_translate(&Rect::new((10.0, 20.0), (20.0, 30.0)), &Point::new(-20.0, -15.0)), Rect::new((-10.0, 5.0), (0.0, 15.0)));
}

use super::saturating_inflate;
use crate::cartesian::{d1::point::point_u32::MAX, d2::rect::rect_u32::Rect};

#[test]
fn min_bounds() {
    assert_eq!(saturating_inflate(&Rect::new((7, 2), (17, 13))), Rect::new((6, 1), (18, 14)));
    assert_eq!(saturating_inflate(&Rect::new((6, 1), (18, 14))), Rect::new((5, 0), (19, 15)));
    assert_eq!(saturating_inflate(&Rect::new((5, 0), (19, 15))), Rect::new((4, 0), (20, 17)));
    assert_eq!(saturating_inflate(&Rect::new((4, 0), (20, 17))), Rect::new((3, 0), (21, 19)));
    assert_eq!(saturating_inflate(&Rect::new((3, 0), (21, 19))), Rect::new((2, 0), (22, 21)));
    assert_eq!(saturating_inflate(&Rect::new((2, 0), (22, 21))), Rect::new((1, 0), (23, 23)));
    assert_eq!(saturating_inflate(&Rect::new((1, 0), (23, 23))), Rect::new((0, 0), (24, 25)));
    assert_eq!(saturating_inflate(&Rect::new((0, 0), (24, 25))), Rect::new((0, 0), (26, 27)));
}

#[test]
fn max_bounds() {
    assert_eq!(saturating_inflate(&Rect::new((MAX - 33, MAX - 17), (MAX - 5, MAX - 3))), Rect::new((MAX - 34, MAX - 18), (MAX - 4, MAX - 2)));
    assert_eq!(saturating_inflate(&Rect::new((MAX - 34, MAX - 18), (MAX - 4, MAX - 2))), Rect::new((MAX - 35, MAX - 19), (MAX - 3, MAX - 1)));
    assert_eq!(saturating_inflate(&Rect::new((MAX - 35, MAX - 19), (MAX - 3, MAX - 1))), Rect::new((MAX - 36, MAX - 20), (MAX - 2, MAX)));
    assert_eq!(saturating_inflate(&Rect::new((MAX - 36, MAX - 20), (MAX - 2, MAX))), Rect::new((MAX - 37, MAX - 22), (MAX - 1, MAX)));
    assert_eq!(saturating_inflate(&Rect::new((MAX - 37, MAX - 22), (MAX - 1, MAX))), Rect::new((MAX - 38, MAX - 24), (MAX, MAX)));
    assert_eq!(saturating_inflate(&Rect::new((MAX - 38, MAX - 24), (MAX, MAX))), Rect::new((MAX - 40, MAX - 26), (MAX, MAX)));
    assert_eq!(saturating_inflate(&Rect::new((MAX - 40, MAX - 26), (MAX, MAX))), Rect::new((MAX - 42, MAX - 28), (MAX, MAX)));
    assert_eq!(saturating_inflate(&Rect::new((MAX - 42, MAX - 28), (MAX, MAX))), Rect::new((MAX - 44, MAX - 30), (MAX, MAX)));
}

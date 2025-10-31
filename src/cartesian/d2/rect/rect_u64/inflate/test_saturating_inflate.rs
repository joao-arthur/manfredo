use super::saturating_inflate;
use crate::cartesian::{d1::point::point_u64::MAX, d2::rect::rect_u64::Rect};

#[test]
fn min_bounds() {
    assert_eq!(saturating_inflate(&Rect::of((7, 2), (17, 13))), Rect::of((6, 1), (18, 14)));
    assert_eq!(saturating_inflate(&Rect::of((6, 1), (18, 14))), Rect::of((5, 0), (19, 15)));
    assert_eq!(saturating_inflate(&Rect::of((5, 0), (19, 15))), Rect::of((4, 0), (20, 17)));
    assert_eq!(saturating_inflate(&Rect::of((4, 0), (20, 17))), Rect::of((3, 0), (21, 19)));
    assert_eq!(saturating_inflate(&Rect::of((3, 0), (21, 19))), Rect::of((2, 0), (22, 21)));
    assert_eq!(saturating_inflate(&Rect::of((2, 0), (22, 21))), Rect::of((1, 0), (23, 23)));
    assert_eq!(saturating_inflate(&Rect::of((1, 0), (23, 23))), Rect::of((0, 0), (24, 25)));
    assert_eq!(saturating_inflate(&Rect::of((0, 0), (24, 25))), Rect::of((0, 0), (26, 27)));
}

#[test]
fn max_bounds() {
    assert_eq!(saturating_inflate(&Rect::of((MAX - 33, MAX - 17), (MAX - 5, MAX - 3))), Rect::of((MAX - 34, MAX - 18), (MAX - 4, MAX - 2)));
    assert_eq!(saturating_inflate(&Rect::of((MAX - 34, MAX - 18), (MAX - 4, MAX - 2))), Rect::of((MAX - 35, MAX - 19), (MAX - 3, MAX - 1)));
    assert_eq!(saturating_inflate(&Rect::of((MAX - 35, MAX - 19), (MAX - 3, MAX - 1))), Rect::of((MAX - 36, MAX - 20), (MAX - 2, MAX)));
    assert_eq!(saturating_inflate(&Rect::of((MAX - 36, MAX - 20), (MAX - 2, MAX))), Rect::of((MAX - 37, MAX - 22), (MAX - 1, MAX)));
    assert_eq!(saturating_inflate(&Rect::of((MAX - 37, MAX - 22), (MAX - 1, MAX))), Rect::of((MAX - 38, MAX - 24), (MAX, MAX)));
    assert_eq!(saturating_inflate(&Rect::of((MAX - 38, MAX - 24), (MAX, MAX))), Rect::of((MAX - 40, MAX - 26), (MAX, MAX)));
    assert_eq!(saturating_inflate(&Rect::of((MAX - 40, MAX - 26), (MAX, MAX))), Rect::of((MAX - 42, MAX - 28), (MAX, MAX)));
    assert_eq!(saturating_inflate(&Rect::of((MAX - 42, MAX - 28), (MAX, MAX))), Rect::of((MAX - 44, MAX - 30), (MAX, MAX)));
}

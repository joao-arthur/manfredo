use super::saturating_inflate;
use crate::cartesian::rect::rect_u32::RectU32;

#[test]
fn saturating_inflate_min_bounds() {
    assert_eq!(saturating_inflate(&RectU32::of(7, 2, 17, 13)), RectU32::of(6, 1, 18, 14));
    assert_eq!(saturating_inflate(&RectU32::of(6, 1, 18, 14)), RectU32::of(5, 0, 19, 15));
    assert_eq!(saturating_inflate(&RectU32::of(5, 0, 19, 15)), RectU32::of(4, 0, 20, 17));
    assert_eq!(saturating_inflate(&RectU32::of(4, 0, 20, 17)), RectU32::of(3, 0, 21, 19));
    assert_eq!(saturating_inflate(&RectU32::of(3, 0, 21, 19)), RectU32::of(2, 0, 22, 21));
    assert_eq!(saturating_inflate(&RectU32::of(2, 0, 22, 21)), RectU32::of(1, 0, 23, 23));
    assert_eq!(saturating_inflate(&RectU32::of(1, 0, 23, 23)), RectU32::of(0, 0, 24, 25));
    assert_eq!(saturating_inflate(&RectU32::of(0, 0, 24, 25)), RectU32::of(0, 0, 26, 27));
}

#[test]
fn saturating_inflate_max_bounds() {
    assert_eq!(
        saturating_inflate(&RectU32::of(u32::MAX - 33, u32::MAX - 17, u32::MAX - 5, u32::MAX - 3)),
        RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2)
    );
    assert_eq!(
        saturating_inflate(&RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2)),
        RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1)
    );
    assert_eq!(
        saturating_inflate(&RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1)),
        RectU32::of(u32::MAX - 36, u32::MAX - 20, u32::MAX - 2, u32::MAX)
    );
    assert_eq!(
        saturating_inflate(&RectU32::of(u32::MAX - 36, u32::MAX - 20, u32::MAX - 2, u32::MAX)),
        RectU32::of(u32::MAX - 37, u32::MAX - 22, u32::MAX - 1, u32::MAX)
    );
    assert_eq!(
        saturating_inflate(&RectU32::of(u32::MAX - 37, u32::MAX - 22, u32::MAX - 1, u32::MAX)),
        RectU32::of(u32::MAX - 38, u32::MAX - 24, u32::MAX, u32::MAX)
    );
    assert_eq!(
        saturating_inflate(&RectU32::of(u32::MAX - 38, u32::MAX - 24, u32::MAX, u32::MAX)),
        RectU32::of(u32::MAX - 40, u32::MAX - 26, u32::MAX, u32::MAX)
    );
    assert_eq!(
        saturating_inflate(&RectU32::of(u32::MAX - 40, u32::MAX - 26, u32::MAX, u32::MAX)),
        RectU32::of(u32::MAX - 42, u32::MAX - 28, u32::MAX, u32::MAX)
    );
    assert_eq!(
        saturating_inflate(&RectU32::of(u32::MAX - 42, u32::MAX - 28, u32::MAX, u32::MAX)),
        RectU32::of(u32::MAX - 44, u32::MAX - 30, u32::MAX, u32::MAX)
    );
}

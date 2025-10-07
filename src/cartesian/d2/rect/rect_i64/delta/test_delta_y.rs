use super::delta_y;
use crate::cartesian::d2::rect::rect_i64::Rect;

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

#[test]
fn test_delta_y() {
    assert_eq!(delta_y(&Rect::of(MIN, 0, MAX, 0)), 0);
    assert_eq!(delta_y(&Rect::of(0, MIN, 0, MAX)), u64::MAX);
}

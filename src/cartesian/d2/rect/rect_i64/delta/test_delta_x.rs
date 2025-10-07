use super::delta_x;
use crate::cartesian::d2::rect::rect_i64::Rect;

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

#[test]
fn test_delta_x() {
    assert_eq!(delta_x(&Rect::of(0, MIN, 0, MAX)), 0);
    assert_eq!(delta_x(&Rect::of(MIN, 0, MAX, 0)), u64::MAX);
}

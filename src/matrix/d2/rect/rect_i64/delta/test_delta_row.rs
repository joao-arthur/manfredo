use super::delta_row;
use crate::matrix::d2::rect::rect_i64::Rect;

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

#[test]
fn test_delta_row() {
    assert_eq!(delta_row(&Rect::of(0, MIN, 0, MAX)), 0);
    assert_eq!(delta_row(&Rect::of(MIN, 0, MAX, 0)), u64::MAX);
}

use super::checked_add_assign;
use crate::matrix::point::{point_i64::PointI64 as PointI, point_u64::PointU64};

#[test]
fn test() {
    let mut p = PointU64::min();
    checked_add_assign(&mut p, &PointI::of(10, 13));
    assert_eq!(p, PointU64::of(10, 13));
    checked_add_assign(&mut p, &PointI::of(-5, -3));
    assert_eq!(p, PointU64::of(5, 10));
}

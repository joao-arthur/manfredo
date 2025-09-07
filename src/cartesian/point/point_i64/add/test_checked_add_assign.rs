use super::checked_add_assign;
use crate::cartesian::point::point_i64::PointI64;

#[test]
fn test() {
    let mut p = PointI64::of(0, 0);
    checked_add_assign(&mut p, &PointI64::of(10, 13));
    assert_eq!(p, PointI64::of(10, 13));
    checked_add_assign(&mut p, &PointI64::of(-25, -30));
    assert_eq!(p, PointI64::of(-15, -17));
}

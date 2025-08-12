use crate::matrix::point::point_i64::PointI64;

use super::PointU64;

pub fn assign_add(p: &mut PointU64, delta: &PointI64) {
    let temp_row = i128::from(p.row) + i128::from(delta.row);
    let temp_col = i128::from(p.col) + i128::from(delta.col);
    p.row = temp_row.clamp(0, i128::from(u64::MAX)) as u64;
    p.col = temp_col.clamp(0, i128::from(u64::MAX)) as u64;
}

pub fn add(p: &PointU64, delta: &PointI64) -> PointU64 {
    let temp_row = i128::from(p.row) + i128::from(delta.row);
    let temp_col = i128::from(p.col) + i128::from(delta.col);
    let row = temp_row.clamp(0, i128::from(u64::MAX)) as u64;
    let col = temp_col.clamp(0, i128::from(u64::MAX)) as u64;
    PointU64 { row, col }
}

#[cfg(test)]
mod tests {
    use crate::matrix::point::{point_i64::PointI64, point_u64::PointU64};

    use super::{add, assign_add};

    #[test]
    fn test_assign_add() {
        let mut p = PointU64::of(0, 0);
        assign_add(&mut p, &PointI64::of(10, 13));
        assert_eq!(p, PointU64::of(10, 13));
        assign_add(&mut p, &PointI64::of(-5, -3));
        assert_eq!(p, PointU64::of(5, 10));
    }

    #[test]
    fn assign_add_to_bounds() {
        let mut p_min = PointU64::of(2, 5);
        assign_add(&mut p_min, &PointI64::of(-2, -5));
        assert_eq!(p_min, PointU64::min());

        let mut p_max = PointU64::of(u64::MAX - 2, u64::MAX - 5);
        assign_add(&mut p_max, &PointI64::of(2, 5));
        assert_eq!(p_max, PointU64::max());
    }

    #[test]
    fn assign_add_out_of_bounds() {
        let mut p_min = PointU64::of(2, 5);
        assign_add(&mut p_min, &PointI64::of(-10, -10));
        assert_eq!(p_min, PointU64::min());

        let mut p_max = PointU64::of(u64::MAX - 2, u64::MAX - 5);
        assign_add(&mut p_max, &PointI64::of(10, 10));
        assert_eq!(p_max, PointU64::max());
    }

    #[test]
    fn assign_add_limits_out_of_bounds() {
        let mut p_min = PointU64::of(1, 1);
        assign_add(&mut p_min, &PointI64::min());
        assert_eq!(p_min, PointU64::min());

        let mut p_max = PointU64::of(u64::MAX - 1, u64::MAX - 1);
        assign_add(&mut p_max, &PointI64::max());
        assert_eq!(p_max, PointU64::max());
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&PointU64::of(0, 0), &PointI64::of(10, 13)), PointU64::of(10, 13));
        assert_eq!(add(&PointU64::of(10, 10), &PointI64::of(-5, -3)), PointU64::of(5, 7));
    }

    #[test]
    fn add_to_bounds() {
        assert_eq!(add(&PointU64::of(2, 5), &PointI64::of(-2, -5)), PointU64::min());
        assert_eq!(add(&PointU64::of(u64::MAX - 2, u64::MAX - 5), &PointI64::of(2, 5)), PointU64::max());
    }

    #[test]
    fn add_out_of_bounds() {
        assert_eq!(add(&PointU64::of(2, 5), &PointI64::of(-10, -10)), PointU64::min());
        assert_eq!(add(&PointU64::of(u64::MAX - 2, u64::MAX - 5), &PointI64::of(10, 10)), PointU64::max());
    }

    #[test]
    fn add_limits_out_of_bounds() {
        assert_eq!(add(&PointU64::of(1, 1), &PointI64::min()), PointU64::min());
        assert_eq!(add(&PointU64::of(u64::MAX - 1, u64::MAX - 1), &PointI64::max()), PointU64::max());
    }
}

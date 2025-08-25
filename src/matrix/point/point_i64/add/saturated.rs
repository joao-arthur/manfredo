use crate::matrix::point::point_i64::PointI64;

pub fn saturating_add_assign(p: &mut PointI64, delta: &PointI64) {
    p.row = p.row.saturating_add(delta.row);
    p.col = p.col.saturating_add(delta.col);
}

pub fn saturating_add(p: &PointI64, delta: &PointI64) -> PointI64 {
    let row = p.row.saturating_add(delta.row);
    let col = p.col.saturating_add(delta.col);
    PointI64 { row, col }
}

#[cfg(test)]
mod tests {
    use super::{saturating_add, saturating_add_assign};
    use crate::matrix::point::point_i64::PointI64;

    #[test]
    fn test_saturating_add_assign() {
        let mut p = PointI64::of(0, 0);
        saturating_add_assign(&mut p, &PointI64::of(10, 13));
        assert_eq!(p, PointI64::of(10, 13));
        saturating_add_assign(&mut p, &PointI64::of(-5, -3));
        assert_eq!(p, PointI64::of(5, 10));
    }

    #[test]
    fn saturating_add_assign_to_bounds() {
        let mut p_min = PointI64::of(i64::MIN + 2, i64::MIN + 5);
        saturating_add_assign(&mut p_min, &PointI64::of(-2, -5));
        assert_eq!(p_min, PointI64::min());

        let mut p_max = PointI64::of(i64::MAX - 2, i64::MAX - 5);
        saturating_add_assign(&mut p_max, &PointI64::of(2, 5));
        assert_eq!(p_max, PointI64::max());
    }

    #[test]
    fn saturating_add_assign_out_of_bounds() {
        let mut p_min = PointI64::of(i64::MIN + 2, i64::MIN + 5);
        saturating_add_assign(&mut p_min, &PointI64::of(-10, -10));
        assert_eq!(p_min, PointI64::min());

        let mut p_max = PointI64::of(i64::MAX - 2, i64::MAX - 5);
        saturating_add_assign(&mut p_max, &PointI64::of(10, 10));
        assert_eq!(p_max, PointI64::max());
    }

    #[test]
    fn saturating_add_assign_limits_out_of_bounds() {
        let mut p_min = PointI64::of(i64::MIN + 1, i64::MIN + 1);
        saturating_add_assign(&mut p_min, &PointI64::min());
        assert_eq!(p_min, PointI64::min());

        let mut p_max = PointI64::of(i64::MAX - 1, i64::MAX - 1);
        saturating_add_assign(&mut p_max, &PointI64::max());
        assert_eq!(p_max, PointI64::max());
    }

    #[test]
    fn test_saturating_add() {
        assert_eq!(saturating_add(&PointI64::of(0, 0), &PointI64::of(10, 13)), PointI64::of(10, 13));
        assert_eq!(saturating_add(&PointI64::of(10, 10), &PointI64::of(-5, -3)), PointI64::of(5, 7));
    }

    #[test]
    fn saturating_add_to_bounds() {
        assert_eq!(saturating_add(&PointI64::of(i64::MIN + 2, i64::MIN + 5), &PointI64::of(-2, -5)), PointI64::min());
        assert_eq!(saturating_add(&PointI64::of(i64::MAX - 2, i64::MAX - 5), &PointI64::of(2, 5)), PointI64::max());
    }

    #[test]
    fn saturating_add_out_of_bounds() {
        assert_eq!(saturating_add(&PointI64::of(i64::MIN + 2, i64::MIN + 5), &PointI64::of(-10, -10)), PointI64::min());
        assert_eq!(saturating_add(&PointI64::of(i64::MAX - 2, i64::MAX - 5), &PointI64::of(10, 10)), PointI64::max());
    }

    #[test]
    fn saturating_add_limits_out_of_bounds() {
        assert_eq!(saturating_add(&PointI64::of(i64::MIN + 1, i64::MIN + 1), &PointI64::min()), PointI64::min());
        assert_eq!(saturating_add(&PointI64::of(i64::MAX - 1, i64::MAX - 1), &PointI64::max()), PointI64::max());
    }
}

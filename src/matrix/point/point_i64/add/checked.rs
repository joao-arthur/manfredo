use crate::matrix::point::point_i64::PointI64;

pub fn try_assign_add(p: &mut PointI64, delta: &PointI64) -> Option<()> {
    let row = p.row.checked_add(delta.row)?;
    let col = p.col.checked_add(delta.col)?;
    p.row = row;
    p.col = col;
    Some(())
}

pub fn try_add(p: &PointI64, delta: &PointI64) -> Option<PointI64> {
    let row = p.row.checked_add(delta.row)?;
    let col = p.col.checked_add(delta.col)?;
    Some(PointI64 { row, col })
}

pub fn assign_add(p: &mut PointI64, delta: &PointI64) {
    try_assign_add(p, delta).unwrap()
}

pub fn add(p: &PointI64, delta: &PointI64) -> PointI64 {
    try_add(p, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::matrix::point::point_i64::PointI64;

    use super::{add, assign_add, try_add, try_assign_add};

    #[test]
    fn test_try_assign_add() {
        let mut p = PointI64::of(0, 0);
        assert_eq!(try_assign_add(&mut p, &PointI64::of(10, 13)), Some(()));
        assert_eq!(p, PointI64::of(10, 13));
        assert_eq!(try_assign_add(&mut p, &PointI64::of(-25, -30)), Some(()));
        assert_eq!(p, PointI64::of(-15, -17));
    }

    #[test]
    fn try_assign_add_to_bounds() {
        let mut p_min = PointI64::of(i64::MIN + 2, i64::MIN + 5);
        assert_eq!(try_assign_add(&mut p_min, &PointI64::of(-2, -5)), Some(()));
        assert_eq!(p_min, PointI64::min());

        let mut p_max = PointI64::of(i64::MAX - 2, i64::MAX - 5);
        assert_eq!(try_assign_add(&mut p_max, &PointI64::of(2, 5)), Some(()));
        assert_eq!(p_max, PointI64::max());
    }

    #[test]
    fn try_assign_add_out_of_bounds() {
        let mut p_min = PointI64::of(i64::MIN + 2, i64::MIN + 5);
        assert_eq!(try_assign_add(&mut p_min, &PointI64::of(-10, 0)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointI64::of(0, -10)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointI64::of(-10, -10)), None);
        assert_eq!(p_min, PointI64::of(i64::MIN + 2, i64::MIN + 5));

        let mut p_max = PointI64::of(i64::MAX - 2, i64::MAX - 5);
        assert_eq!(try_assign_add(&mut p_max, &PointI64::of(10, 0)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointI64::of(0, 10)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointI64::of(10, 10)), None);
        assert_eq!(p_max, PointI64::of(i64::MAX - 2, i64::MAX - 5));
    }

    #[test]
    fn try_assign_add_limits_out_of_bounds() {
        let mut p_min = PointI64::of(i64::MIN + 1, i64::MIN + 1);
        assert_eq!(try_assign_add(&mut p_min, &PointI64::of(i64::MIN, 0)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointI64::of(0, i64::MIN)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointI64::min()), None);
        assert_eq!(p_min, PointI64::of(i64::MIN + 1, i64::MIN + 1));

        let mut p_max = PointI64::of(i64::MAX - 1, i64::MAX - 1);
        assert_eq!(try_assign_add(&mut p_max, &PointI64::of(i64::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointI64::of(0, i64::MAX)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointI64::max()), None);
        assert_eq!(p_max, PointI64::of(i64::MAX - 1, i64::MAX - 1));
    }

    #[test]
    fn test_try_add() {
        assert_eq!(try_add(&PointI64::of(0, 0), &PointI64::of(10, 13)), Some(PointI64::of(10, 13)));
        assert_eq!(try_add(&PointI64::of(10, 10), &PointI64::of(-5, -3)), Some(PointI64::of(5, 7)));
    }

    #[test]
    fn try_add_to_bounds() {
        assert_eq!(try_add(&PointI64::of(i64::MIN + 2, i64::MIN + 5), &PointI64::of(-2, -5)), Some(PointI64::min()));
        assert_eq!(try_add(&PointI64::of(i64::MAX - 2, i64::MAX - 5), &PointI64::of(2, 5)), Some(PointI64::max()));
    }

    #[test]
    fn try_add_out_of_bounds() {
        let p_min = PointI64::of(i64::MIN + 2, i64::MIN + 5);
        assert_eq!(try_add(&p_min, &PointI64::of(-10, 0)), None);
        assert_eq!(try_add(&p_min, &PointI64::of(0, -10)), None);
        assert_eq!(try_add(&p_min, &PointI64::of(-10, -10)), None);

        let m_max = PointI64::of(i64::MAX - 2, i64::MAX - 5);
        assert_eq!(try_add(&m_max, &PointI64::of(10, 0)), None);
        assert_eq!(try_add(&m_max, &PointI64::of(0, 10)), None);
        assert_eq!(try_add(&m_max, &PointI64::of(10, 10)), None);
    }

    #[test]
    fn try_add_limits_out_of_bounds() {
        let p_min = PointI64::of(i64::MIN + 1, i64::MIN + 1);
        assert_eq!(try_add(&p_min, &PointI64::of(i64::MIN, 0)), None);
        assert_eq!(try_add(&p_min, &PointI64::of(0, i64::MIN)), None);
        assert_eq!(try_add(&p_min, &PointI64::min()), None);

        let p_max = PointI64::of(i64::MAX - 1, i64::MAX - 1);
        assert_eq!(try_add(&p_max, &PointI64::of(i64::MAX, 0)), None);
        assert_eq!(try_add(&p_max, &PointI64::of(0, i64::MAX)), None);
        assert_eq!(try_add(&p_max, &PointI64::max()), None);
    }

    #[test]
    fn test_assign_add() {
        let mut p = PointI64::of(0, 0);
        assign_add(&mut p, &PointI64::of(10, 13));
        assert_eq!(p, PointI64::of(10, 13));
        assign_add(&mut p, &PointI64::of(-25, -30));
        assert_eq!(p, PointI64::of(-15, -17));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&PointI64::of(0, 0), &PointI64::of(10, 13)), PointI64::of(10, 13));
        assert_eq!(add(&PointI64::of(10, 13), &PointI64::of(-5, -3)), PointI64::of(5, 10));
    }
}

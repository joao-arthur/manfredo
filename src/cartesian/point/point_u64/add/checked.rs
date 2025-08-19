use crate::cartesian::point::{point_i64::PointI64, point_u64::PointU64};

pub fn try_assign_add(p: &mut PointU64, delta: &PointI64) -> Option<()> {
    let x = p.x.checked_add_signed(delta.x)?;
    let y = p.y.checked_add_signed(delta.y)?;
    p.x = x;
    p.y = y;
    Some(())
}

pub fn try_add(p: &PointU64, delta: &PointI64) -> Option<PointU64> {
    let x = p.x.checked_add_signed(delta.x)?;
    let y = p.y.checked_add_signed(delta.y)?;
    Some(PointU64 { x, y })
}

pub fn assign_add(p: &mut PointU64, delta: &PointI64) {
    try_assign_add(p, delta).unwrap()
}

pub fn add(p: &PointU64, delta: &PointI64) -> PointU64 {
    try_add(p, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::{point_i64::PointI64, point_u64::PointU64};

    use super::{add, assign_add, try_add, try_assign_add};

    #[test]
    fn test_try_assign_add() {
        let mut p = PointU64::min();
        assert_eq!(try_assign_add(&mut p, &PointI64::of(10, 13)), Some(()));
        assert_eq!(p, PointU64::of(10, 13));
        assert_eq!(try_assign_add(&mut p, &PointI64::of(-5, -3)), Some(()));
        assert_eq!(p, PointU64::of(5, 10));
    }

    #[test]
    fn try_assign_add_to_bounds() {
        let mut p_min = PointU64::of(2, 5);
        assert_eq!(try_assign_add(&mut p_min, &PointI64::of(-2, -5)), Some(()));
        assert_eq!(p_min, PointU64::min());

        let mut p_max = PointU64::of(u64::MAX - 2, u64::MAX - 5);
        assert_eq!(try_assign_add(&mut p_max, &PointI64::of(2, 5)), Some(()));
        assert_eq!(p_max, PointU64::max());
    }

    #[test]
    fn try_assign_add_out_of_bounds() {
        let mut p_min = PointU64::of(2, 5);
        assert_eq!(try_assign_add(&mut p_min, &PointI64::of(-10, 0)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointI64::of(0, -10)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointI64::of(-10, -10)), None);
        assert_eq!(p_min, PointU64::of(2, 5));

        let mut p_max = PointU64::of(u64::MAX - 2, u64::MAX - 5);
        assert_eq!(try_assign_add(&mut p_max, &PointI64::of(10, 0)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointI64::of(0, 10)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointI64::of(10, 10)), None);
        assert_eq!(p_max, PointU64::of(u64::MAX - 2, u64::MAX - 5));
    }

    #[test]
    fn try_assign_add_limits_out_of_bounds() {
        let mut p_min = PointU64::of(1, 1);
        assert_eq!(try_assign_add(&mut p_min, &PointI64::of(i64::MIN, 0)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointI64::of(0, i64::MIN)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointI64::min()), None);
        assert_eq!(p_min, PointU64::of(1, 1));

        let mut p_max = PointU64::of(u64::MAX - 1, u64::MAX - 1);
        assert_eq!(try_assign_add(&mut p_max, &PointI64::of(i64::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointI64::of(0, i64::MAX)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointI64::max()), None);
        assert_eq!(p_max, PointU64::of(u64::MAX - 1, u64::MAX - 1));
    }

    #[test]
    fn test_try_add() {
        assert_eq!(try_add(&PointU64::min(), &PointI64::of(10, 13)), Some(PointU64::of(10, 13)));
        assert_eq!(try_add(&PointU64::of(10, 10), &PointI64::of(-5, -3)), Some(PointU64::of(5, 7)));
    }

    #[test]
    fn try_add_to_bounds() {
        assert_eq!(try_add(&PointU64::of(2, 5), &PointI64::of(-2, -5)), Some(PointU64::min()));
        assert_eq!(try_add(&PointU64::of(u64::MAX - 2, u64::MAX - 5), &PointI64::of(2, 5)), Some(PointU64::max()));
    }

    #[test]
    fn try_add_out_of_bounds() {
        let p_min = PointU64::of(2, 5);
        assert_eq!(try_add(&p_min, &PointI64::of(-10, 0)), None);
        assert_eq!(try_add(&p_min, &PointI64::of(0, -10)), None);
        assert_eq!(try_add(&p_min, &PointI64::of(-10, -10)), None);

        let m_max = PointU64::of(u64::MAX - 2, u64::MAX - 5);
        assert_eq!(try_add(&m_max, &PointI64::of(10, 0)), None);
        assert_eq!(try_add(&m_max, &PointI64::of(0, 10)), None);
        assert_eq!(try_add(&m_max, &PointI64::of(10, 10)), None);
    }

    #[test]
    fn try_add_limits_out_of_bounds() {
        let p_min = PointU64::of(1, 1);
        assert_eq!(try_add(&p_min, &PointI64::of(i64::MIN, 0)), None);
        assert_eq!(try_add(&p_min, &PointI64::of(0, i64::MIN)), None);
        assert_eq!(try_add(&p_min, &PointI64::min()), None);

        let p_max = PointU64::of(u64::MAX - 1, u64::MAX - 1);
        assert_eq!(try_add(&p_max, &PointI64::of(i64::MAX, 0)), None);
        assert_eq!(try_add(&p_max, &PointI64::of(0, i64::MAX)), None);
        assert_eq!(try_add(&p_max, &PointI64::max()), None);
    }

    #[test]
    fn test_assign_add() {
        let mut p = PointU64::min();
        assign_add(&mut p, &PointI64::of(10, 13));
        assert_eq!(p, PointU64::of(10, 13));
        assign_add(&mut p, &PointI64::of(-5, -3));
        assert_eq!(p, PointU64::of(5, 10));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&PointU64::min(), &PointI64::of(10, 13)), PointU64::of(10, 13));
        assert_eq!(add(&PointU64::of(10, 13), &PointI64::of(-5, -3)), PointU64::of(5, 10));
    }
}

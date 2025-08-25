use crate::cartesian::point::{point_i32::PointI32, point_u32::PointU32};

pub fn try_checked_add_assign(p: &mut PointU32, delta: &PointI32) -> Option<()> {
    let x = p.x.checked_add_signed(delta.x)?;
    let y = p.y.checked_add_signed(delta.y)?;
    p.x = x;
    p.y = y;
    Some(())
}

pub fn try_checked_add(p: &PointU32, delta: &PointI32) -> Option<PointU32> {
    let x = p.x.checked_add_signed(delta.x)?;
    let y = p.y.checked_add_signed(delta.y)?;
    Some(PointU32 { x, y })
}

pub fn checked_add_assign(p: &mut PointU32, delta: &PointI32) {
    try_checked_add_assign(p, delta).unwrap()
}

pub fn checked_add(p: &PointU32, delta: &PointI32) -> PointU32 {
    try_checked_add(p, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{checked_add, checked_add_assign, try_checked_add, try_checked_add_assign};
    use crate::cartesian::point::{point_i32::PointI32, point_u32::PointU32};

    #[test]
    fn test_try_checked_add_assign() {
        let mut p = PointU32::min();
        assert_eq!(try_checked_add_assign(&mut p, &PointI32::of(10, 13)), Some(()));
        assert_eq!(p, PointU32::of(10, 13));
        assert_eq!(try_checked_add_assign(&mut p, &PointI32::of(-5, -3)), Some(()));
        assert_eq!(p, PointU32::of(5, 10));
    }

    #[test]
    fn try_checked_add_assign_to_bounds() {
        let mut p_min = PointU32::of(2, 5);
        assert_eq!(try_checked_add_assign(&mut p_min, &PointI32::of(-2, -5)), Some(()));
        assert_eq!(p_min, PointU32::min());

        let mut p_max = PointU32::of(u32::MAX - 2, u32::MAX - 5);
        assert_eq!(try_checked_add_assign(&mut p_max, &PointI32::of(2, 5)), Some(()));
        assert_eq!(p_max, PointU32::max());
    }

    #[test]
    fn try_checked_add_assign_out_of_bounds() {
        let mut p_min = PointU32::of(2, 5);
        assert_eq!(try_checked_add_assign(&mut p_min, &PointI32::of(-10, 0)), None);
        assert_eq!(try_checked_add_assign(&mut p_min, &PointI32::of(0, -10)), None);
        assert_eq!(try_checked_add_assign(&mut p_min, &PointI32::of(-10, -10)), None);
        assert_eq!(p_min, PointU32::of(2, 5));

        let mut p_max = PointU32::of(u32::MAX - 2, u32::MAX - 5);
        assert_eq!(try_checked_add_assign(&mut p_max, &PointI32::of(10, 0)), None);
        assert_eq!(try_checked_add_assign(&mut p_max, &PointI32::of(0, 10)), None);
        assert_eq!(try_checked_add_assign(&mut p_max, &PointI32::of(10, 10)), None);
        assert_eq!(p_max, PointU32::of(u32::MAX - 2, u32::MAX - 5));
    }

    #[test]
    fn try_checked_add_assign_limits_out_of_bounds() {
        let mut p_min = PointU32::of(1, 1);
        assert_eq!(try_checked_add_assign(&mut p_min, &PointI32::of(i32::MIN, 0)), None);
        assert_eq!(try_checked_add_assign(&mut p_min, &PointI32::of(0, i32::MIN)), None);
        assert_eq!(try_checked_add_assign(&mut p_min, &PointI32::min()), None);
        assert_eq!(p_min, PointU32::of(1, 1));

        let mut p_max = PointU32::of(u32::MAX - 1, u32::MAX - 1);
        assert_eq!(try_checked_add_assign(&mut p_max, &PointI32::of(i32::MAX, 0)), None);
        assert_eq!(try_checked_add_assign(&mut p_max, &PointI32::of(0, i32::MAX)), None);
        assert_eq!(try_checked_add_assign(&mut p_max, &PointI32::max()), None);
        assert_eq!(p_max, PointU32::of(u32::MAX - 1, u32::MAX - 1));
    }

    #[test]
    fn test_try_checked_add() {
        assert_eq!(try_checked_add(&PointU32::min(), &PointI32::of(10, 13)), Some(PointU32::of(10, 13)));
        assert_eq!(try_checked_add(&PointU32::of(10, 10), &PointI32::of(-5, -3)), Some(PointU32::of(5, 7)));
    }

    #[test]
    fn try_checked_add_to_bounds() {
        assert_eq!(try_checked_add(&PointU32::of(2, 5), &PointI32::of(-2, -5)), Some(PointU32::min()));
        assert_eq!(try_checked_add(&PointU32::of(u32::MAX - 2, u32::MAX - 5), &PointI32::of(2, 5)), Some(PointU32::max()));
    }

    #[test]
    fn try_checked_add_out_of_bounds() {
        let p_min = PointU32::of(2, 5);
        assert_eq!(try_checked_add(&p_min, &PointI32::of(-10, 0)), None);
        assert_eq!(try_checked_add(&p_min, &PointI32::of(0, -10)), None);
        assert_eq!(try_checked_add(&p_min, &PointI32::of(-10, -10)), None);

        let m_max = PointU32::of(u32::MAX - 2, u32::MAX - 5);
        assert_eq!(try_checked_add(&m_max, &PointI32::of(10, 0)), None);
        assert_eq!(try_checked_add(&m_max, &PointI32::of(0, 10)), None);
        assert_eq!(try_checked_add(&m_max, &PointI32::of(10, 10)), None);
    }

    #[test]
    fn try_checked_add_limits_out_of_bounds() {
        let p_min = PointU32::of(1, 1);
        assert_eq!(try_checked_add(&p_min, &PointI32::of(i32::MIN, 0)), None);
        assert_eq!(try_checked_add(&p_min, &PointI32::of(0, i32::MIN)), None);
        assert_eq!(try_checked_add(&p_min, &PointI32::min()), None);

        let p_max = PointU32::of(u32::MAX - 1, u32::MAX - 1);
        assert_eq!(try_checked_add(&p_max, &PointI32::of(i32::MAX, 0)), None);
        assert_eq!(try_checked_add(&p_max, &PointI32::of(0, i32::MAX)), None);
        assert_eq!(try_checked_add(&p_max, &PointI32::max()), None);
    }

    #[test]
    fn test_checked_add_assign() {
        let mut p = PointU32::min();
        checked_add_assign(&mut p, &PointI32::of(10, 13));
        assert_eq!(p, PointU32::of(10, 13));
        checked_add_assign(&mut p, &PointI32::of(-5, -3));
        assert_eq!(p, PointU32::of(5, 10));
    }

    #[test]
    fn test_checked_add() {
        assert_eq!(checked_add(&PointU32::min(), &PointI32::of(10, 13)), PointU32::of(10, 13));
        assert_eq!(checked_add(&PointU32::of(10, 13), &PointI32::of(-5, -3)), PointU32::of(5, 10));
    }
}

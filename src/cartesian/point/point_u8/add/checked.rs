use crate::cartesian::point::{point_i8::PointI8, point_u8::PointU8};

pub fn try_assign_add(p: &mut PointU8, delta: &PointI8) -> Option<()> {
    let x = p.x.checked_add_signed(delta.x)?;
    let y = p.y.checked_add_signed(delta.y)?;
    p.x = x;
    p.y = y;
    Some(())
}

pub fn try_add(p: &PointU8, delta: &PointI8) -> Option<PointU8> {
    let x = p.x.checked_add_signed(delta.x)?;
    let y = p.y.checked_add_signed(delta.y)?;
    Some(PointU8 { x, y })
}

pub fn assign_add(p: &mut PointU8, delta: &PointI8) {
    try_assign_add(p, delta).unwrap()
}

pub fn add(p: &PointU8, delta: &PointI8) -> PointU8 {
    try_add(p, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::{point_i8::PointI8, point_u8::PointU8};

    use super::{add, assign_add, try_add, try_assign_add};

    #[test]
    fn test_try_assign_add() {
        let mut p = PointU8::min();
        assert_eq!(try_assign_add(&mut p, &PointI8::of(10, 13)), Some(()));
        assert_eq!(p, PointU8::of(10, 13));
        assert_eq!(try_assign_add(&mut p, &PointI8::of(-5, -3)), Some(()));
        assert_eq!(p, PointU8::of(5, 10));
    }

    #[test]
    fn try_assign_add_to_bounds() {
        let mut p_min = PointU8::of(2, 5);
        assert_eq!(try_assign_add(&mut p_min, &PointI8::of(-2, -5)), Some(()));
        assert_eq!(p_min, PointU8::min());

        let mut p_max = PointU8::of(u8::MAX - 2, u8::MAX - 5);
        assert_eq!(try_assign_add(&mut p_max, &PointI8::of(2, 5)), Some(()));
        assert_eq!(p_max, PointU8::max());
    }

    #[test]
    fn try_assign_add_out_of_bounds() {
        let mut p_min = PointU8::of(2, 5);
        assert_eq!(try_assign_add(&mut p_min, &PointI8::of(-10, 0)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointI8::of(0, -10)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointI8::of(-10, -10)), None);
        assert_eq!(p_min, PointU8::of(2, 5));

        let mut p_max = PointU8::of(u8::MAX - 2, u8::MAX - 5);
        assert_eq!(try_assign_add(&mut p_max, &PointI8::of(10, 0)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointI8::of(0, 10)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointI8::of(10, 10)), None);
        assert_eq!(p_max, PointU8::of(u8::MAX - 2, u8::MAX - 5));
    }

    #[test]
    fn try_assign_add_limits_out_of_bounds() {
        let mut p_min = PointU8::of(1, 1);
        assert_eq!(try_assign_add(&mut p_min, &PointI8::of(i8::MIN, 0)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointI8::of(0, i8::MIN)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointI8::min()), None);
        assert_eq!(p_min, PointU8::of(1, 1));

        let mut p_max = PointU8::of(u8::MAX - 1, u8::MAX - 1);
        assert_eq!(try_assign_add(&mut p_max, &PointI8::of(i8::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointI8::of(0, i8::MAX)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointI8::max()), None);
        assert_eq!(p_max, PointU8::of(u8::MAX - 1, u8::MAX - 1));
    }

    #[test]
    fn test_try_add() {
        assert_eq!(try_add(&PointU8::min(), &PointI8::of(10, 13)), Some(PointU8::of(10, 13)));
        assert_eq!(try_add(&PointU8::of(10, 10), &PointI8::of(-5, -3)), Some(PointU8::of(5, 7)));
    }

    #[test]
    fn try_add_to_bounds() {
        assert_eq!(try_add(&PointU8::of(2, 5), &PointI8::of(-2, -5)), Some(PointU8::min()));
        assert_eq!(try_add(&PointU8::of(u8::MAX - 2, u8::MAX - 5), &PointI8::of(2, 5)), Some(PointU8::max()));
    }

    #[test]
    fn try_add_out_of_bounds() {
        let p_min = PointU8::of(2, 5);
        assert_eq!(try_add(&p_min, &PointI8::of(-10, 0)), None);
        assert_eq!(try_add(&p_min, &PointI8::of(0, -10)), None);
        assert_eq!(try_add(&p_min, &PointI8::of(-10, -10)), None);

        let m_max = PointU8::of(u8::MAX - 2, u8::MAX - 5);
        assert_eq!(try_add(&m_max, &PointI8::of(10, 0)), None);
        assert_eq!(try_add(&m_max, &PointI8::of(0, 10)), None);
        assert_eq!(try_add(&m_max, &PointI8::of(10, 10)), None);
    }

    #[test]
    fn try_add_limits_out_of_bounds() {
        let p_min = PointU8::of(1, 1);
        assert_eq!(try_add(&p_min, &PointI8::of(i8::MIN, 0)), None);
        assert_eq!(try_add(&p_min, &PointI8::of(0, i8::MIN)), None);
        assert_eq!(try_add(&p_min, &PointI8::min()), None);

        let p_max = PointU8::of(u8::MAX - 1, u8::MAX - 1);
        assert_eq!(try_add(&p_max, &PointI8::of(i8::MAX, 0)), None);
        assert_eq!(try_add(&p_max, &PointI8::of(0, i8::MAX)), None);
        assert_eq!(try_add(&p_max, &PointI8::max()), None);
    }

    #[test]
    fn test_assign_add() {
        let mut p = PointU8::min();
        assign_add(&mut p, &PointI8::of(10, 13));
        assert_eq!(p, PointU8::of(10, 13));
        assign_add(&mut p, &PointI8::of(-5, -3));
        assert_eq!(p, PointU8::of(5, 10));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&PointU8::min(), &PointI8::of(10, 13)), PointU8::of(10, 13));
        assert_eq!(add(&PointU8::of(10, 13), &PointI8::of(-5, -3)), PointU8::of(5, 10));
    }
}

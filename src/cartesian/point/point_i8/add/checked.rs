use crate::cartesian::point::point_i8::PointI8;

pub fn try_assign_add(p: &mut PointI8, delta: &PointI8) -> Option<()> {
    let x = p.x.checked_add(delta.x)?;
    let y = p.y.checked_add(delta.y)?;
    p.x = x;
    p.y = y;
    Some(())
}

pub fn try_add(p: &PointI8, delta: &PointI8) -> Option<PointI8> {
    let x = p.x.checked_add(delta.x)?;
    let y = p.y.checked_add(delta.y)?;
    Some(PointI8 { x, y })
}

pub fn assign_add(p: &mut PointI8, delta: &PointI8) {
    try_assign_add(p, delta).unwrap()
}

pub fn add(p: &PointI8, delta: &PointI8) -> PointI8 {
    try_add(p, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{add, assign_add, try_add, try_assign_add};
    use crate::cartesian::point::point_i8::PointI8;

    #[test]
    fn test_try_assign_add() {
        let mut p = PointI8::of(0, 0);
        assert_eq!(try_assign_add(&mut p, &PointI8::of(10, 13)), Some(()));
        assert_eq!(p, PointI8::of(10, 13));
        assert_eq!(try_assign_add(&mut p, &PointI8::of(-25, -30)), Some(()));
        assert_eq!(p, PointI8::of(-15, -17));
    }

    #[test]
    fn try_assign_add_to_bounds() {
        let mut p_min = PointI8::of(i8::MIN + 2, i8::MIN + 5);
        assert_eq!(try_assign_add(&mut p_min, &PointI8::of(-2, -5)), Some(()));
        assert_eq!(p_min, PointI8::min());

        let mut p_max = PointI8::of(i8::MAX - 2, i8::MAX - 5);
        assert_eq!(try_assign_add(&mut p_max, &PointI8::of(2, 5)), Some(()));
        assert_eq!(p_max, PointI8::max());
    }

    #[test]
    fn try_assign_add_out_of_bounds() {
        let mut p_min = PointI8::of(i8::MIN + 2, i8::MIN + 5);
        assert_eq!(try_assign_add(&mut p_min, &PointI8::of(-10, 0)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointI8::of(0, -10)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointI8::of(-10, -10)), None);
        assert_eq!(p_min, PointI8::of(i8::MIN + 2, i8::MIN + 5));

        let mut p_max = PointI8::of(i8::MAX - 2, i8::MAX - 5);
        assert_eq!(try_assign_add(&mut p_max, &PointI8::of(10, 0)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointI8::of(0, 10)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointI8::of(10, 10)), None);
        assert_eq!(p_max, PointI8::of(i8::MAX - 2, i8::MAX - 5));
    }

    #[test]
    fn try_assign_add_limits_out_of_bounds() {
        let mut p_min = PointI8::of(i8::MIN + 1, i8::MIN + 1);
        assert_eq!(try_assign_add(&mut p_min, &PointI8::of(i8::MIN, 0)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointI8::of(0, i8::MIN)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointI8::min()), None);
        assert_eq!(p_min, PointI8::of(i8::MIN + 1, i8::MIN + 1));

        let mut p_max = PointI8::of(i8::MAX - 1, i8::MAX - 1);
        assert_eq!(try_assign_add(&mut p_max, &PointI8::of(i8::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointI8::of(0, i8::MAX)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointI8::max()), None);
        assert_eq!(p_max, PointI8::of(i8::MAX - 1, i8::MAX - 1));
    }

    #[test]
    fn test_try_add() {
        assert_eq!(try_add(&PointI8::of(0, 0), &PointI8::of(10, 13)), Some(PointI8::of(10, 13)));
        assert_eq!(try_add(&PointI8::of(10, 10), &PointI8::of(-5, -3)), Some(PointI8::of(5, 7)));
    }

    #[test]
    fn try_add_to_bounds() {
        assert_eq!(try_add(&PointI8::of(i8::MIN + 2, i8::MIN + 5), &PointI8::of(-2, -5)), Some(PointI8::min()));
        assert_eq!(try_add(&PointI8::of(i8::MAX - 2, i8::MAX - 5), &PointI8::of(2, 5)), Some(PointI8::max()));
    }

    #[test]
    fn try_add_out_of_bounds() {
        let p_min = PointI8::of(i8::MIN + 2, i8::MIN + 5);
        assert_eq!(try_add(&p_min, &PointI8::of(-10, 0)), None);
        assert_eq!(try_add(&p_min, &PointI8::of(0, -10)), None);
        assert_eq!(try_add(&p_min, &PointI8::of(-10, -10)), None);

        let m_max = PointI8::of(i8::MAX - 2, i8::MAX - 5);
        assert_eq!(try_add(&m_max, &PointI8::of(10, 0)), None);
        assert_eq!(try_add(&m_max, &PointI8::of(0, 10)), None);
        assert_eq!(try_add(&m_max, &PointI8::of(10, 10)), None);
    }

    #[test]
    fn try_add_limits_out_of_bounds() {
        let p_min = PointI8::of(i8::MIN + 1, i8::MIN + 1);
        assert_eq!(try_add(&p_min, &PointI8::of(i8::MIN, 0)), None);
        assert_eq!(try_add(&p_min, &PointI8::of(0, i8::MIN)), None);
        assert_eq!(try_add(&p_min, &PointI8::min()), None);

        let p_max = PointI8::of(i8::MAX - 1, i8::MAX - 1);
        assert_eq!(try_add(&p_max, &PointI8::of(i8::MAX, 0)), None);
        assert_eq!(try_add(&p_max, &PointI8::of(0, i8::MAX)), None);
        assert_eq!(try_add(&p_max, &PointI8::max()), None);
    }

    #[test]
    fn test_assign_add() {
        let mut p = PointI8::of(0, 0);
        assign_add(&mut p, &PointI8::of(10, 13));
        assert_eq!(p, PointI8::of(10, 13));
        assign_add(&mut p, &PointI8::of(-25, -30));
        assert_eq!(p, PointI8::of(-15, -17));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&PointI8::of(0, 0), &PointI8::of(10, 13)), PointI8::of(10, 13));
        assert_eq!(add(&PointI8::of(10, 13), &PointI8::of(-5, -3)), PointI8::of(5, 10));
    }
}

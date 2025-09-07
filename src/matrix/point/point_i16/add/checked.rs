#[cfg(test)]
mod tests {
    use super::super::{checked_add, checked_add_assign, try_checked_add, try_checked_add_assign};
    use crate::matrix::point::point_i16::PointI16;

    const MIN: i16 = i16::MIN;
    const MAX: i16 = i16::MAX;

    #[test]
    fn test_try_checked_add_assign() {
        let mut p = PointI16::of(0, 0);
        assert_eq!(try_checked_add_assign(&mut p, &PointI16::of(10, 13)), Some(()));
        assert_eq!(p, PointI16::of(10, 13));
        assert_eq!(try_checked_add_assign(&mut p, &PointI16::of(-25, -30)), Some(()));
        assert_eq!(p, PointI16::of(-15, -17));
    }

    #[test]
    fn try_checked_add_assign_to_bounds() {
        let mut p_min = PointI16::of(MIN + 2, MIN + 5);
        assert_eq!(try_checked_add_assign(&mut p_min, &PointI16::of(-2, -5)), Some(()));
        assert_eq!(p_min, PointI16::min());

        let mut p_max = PointI16::of(MAX - 2, MAX - 5);
        assert_eq!(try_checked_add_assign(&mut p_max, &PointI16::of(2, 5)), Some(()));
        assert_eq!(p_max, PointI16::max());
    }

    #[test]
    fn try_checked_add_assign_out_of_bounds() {
        let mut p_min = PointI16::of(MIN + 2, MIN + 5);
        assert_eq!(try_checked_add_assign(&mut p_min, &PointI16::of(-10, 0)), None);
        assert_eq!(try_checked_add_assign(&mut p_min, &PointI16::of(0, -10)), None);
        assert_eq!(try_checked_add_assign(&mut p_min, &PointI16::of(-10, -10)), None);
        assert_eq!(p_min, PointI16::of(MIN + 2, MIN + 5));

        let mut p_max = PointI16::of(MAX - 2, MAX - 5);
        assert_eq!(try_checked_add_assign(&mut p_max, &PointI16::of(10, 0)), None);
        assert_eq!(try_checked_add_assign(&mut p_max, &PointI16::of(0, 10)), None);
        assert_eq!(try_checked_add_assign(&mut p_max, &PointI16::of(10, 10)), None);
        assert_eq!(p_max, PointI16::of(MAX - 2, MAX - 5));
    }

    #[test]
    fn try_checked_add_assign_limits_out_of_bounds() {
        let mut p_min = PointI16::of(MIN + 1, MIN + 1);
        assert_eq!(try_checked_add_assign(&mut p_min, &PointI16::of(MIN, 0)), None);
        assert_eq!(try_checked_add_assign(&mut p_min, &PointI16::of(0, MIN)), None);
        assert_eq!(try_checked_add_assign(&mut p_min, &PointI16::min()), None);
        assert_eq!(p_min, PointI16::of(MIN + 1, MIN + 1));

        let mut p_max = PointI16::of(MAX - 1, MAX - 1);
        assert_eq!(try_checked_add_assign(&mut p_max, &PointI16::of(MAX, 0)), None);
        assert_eq!(try_checked_add_assign(&mut p_max, &PointI16::of(0, MAX)), None);
        assert_eq!(try_checked_add_assign(&mut p_max, &PointI16::max()), None);
        assert_eq!(p_max, PointI16::of(MAX - 1, MAX - 1));
    }

    #[test]
    fn test_try_checked_add() {
        assert_eq!(try_checked_add(&PointI16::of(0, 0), &PointI16::of(10, 13)), Some(PointI16::of(10, 13)));
        assert_eq!(try_checked_add(&PointI16::of(10, 10), &PointI16::of(-5, -3)), Some(PointI16::of(5, 7)));
    }

    #[test]
    fn try_checked_add_to_bounds() {
        assert_eq!(try_checked_add(&PointI16::of(MIN + 2, MIN + 5), &PointI16::of(-2, -5)), Some(PointI16::min()));
        assert_eq!(try_checked_add(&PointI16::of(MAX - 2, MAX - 5), &PointI16::of(2, 5)), Some(PointI16::max()));
    }

    #[test]
    fn try_checked_add_out_of_bounds() {
        let p_min = PointI16::of(MIN + 2, MIN + 5);
        assert_eq!(try_checked_add(&p_min, &PointI16::of(-10, 0)), None);
        assert_eq!(try_checked_add(&p_min, &PointI16::of(0, -10)), None);
        assert_eq!(try_checked_add(&p_min, &PointI16::of(-10, -10)), None);

        let m_max = PointI16::of(MAX - 2, MAX - 5);
        assert_eq!(try_checked_add(&m_max, &PointI16::of(10, 0)), None);
        assert_eq!(try_checked_add(&m_max, &PointI16::of(0, 10)), None);
        assert_eq!(try_checked_add(&m_max, &PointI16::of(10, 10)), None);
    }

    #[test]
    fn try_checked_add_limits_out_of_bounds() {
        let p_min = PointI16::of(MIN + 1, MIN + 1);
        assert_eq!(try_checked_add(&p_min, &PointI16::of(MIN, 0)), None);
        assert_eq!(try_checked_add(&p_min, &PointI16::of(0, MIN)), None);
        assert_eq!(try_checked_add(&p_min, &PointI16::min()), None);

        let p_max = PointI16::of(MAX - 1, MAX - 1);
        assert_eq!(try_checked_add(&p_max, &PointI16::of(MAX, 0)), None);
        assert_eq!(try_checked_add(&p_max, &PointI16::of(0, MAX)), None);
        assert_eq!(try_checked_add(&p_max, &PointI16::max()), None);
    }

    #[test]
    fn test_checked_add_assign() {
        let mut p = PointI16::of(0, 0);
        checked_add_assign(&mut p, &PointI16::of(10, 13));
        assert_eq!(p, PointI16::of(10, 13));
        checked_add_assign(&mut p, &PointI16::of(-25, -30));
        assert_eq!(p, PointI16::of(-15, -17));
    }

    #[test]
    fn test_checked_add() {
        assert_eq!(checked_add(&PointI16::of(0, 0), &PointI16::of(10, 13)), PointI16::of(10, 13));
        assert_eq!(checked_add(&PointI16::of(10, 13), &PointI16::of(-5, -3)), PointI16::of(5, 10));
    }
}

use crate::cartesian::point::point_f32::{MAX, MIN, PointF32};

pub fn try_checked_add_assign(p: &mut PointF32, delta: &PointF32) -> Option<()> {
    let x = p.x + delta.x;
    let y = p.y + delta.y;
    if x < MIN || x > MAX || y < MIN || y > MAX {
        return None;
    }
    p.x = x;
    p.y = y;
    Some(())
}

pub fn try_checked_add(p: &PointF32, delta: &PointF32) -> Option<PointF32> {
    let x = p.x + delta.x;
    let y = p.y + delta.y;
    if x < MIN || x > MAX || y < MIN || y > MAX {
        return None;
    }
    Some(PointF32 { x, y })
}

pub fn checked_add_assign(p: &mut PointF32, delta: &PointF32) {
    try_checked_add_assign(p, delta).unwrap()
}

pub fn checked_add(p: &PointF32, delta: &PointF32) -> PointF32 {
    try_checked_add(p, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{checked_add, checked_add_assign, try_checked_add, try_checked_add_assign};
    use crate::cartesian::point::point_f32::{MAX, MIN, PointF32};

    #[test]
    fn test_try_checked_add_assign() {
        let mut p = PointF32::of(0.0, 0.0);
        assert_eq!(try_checked_add_assign(&mut p, &PointF32::of(10.0, 13.0)), Some(()));
        assert_eq!(p, PointF32::of(10.0, 13.0));
        assert_eq!(try_checked_add_assign(&mut p, &PointF32::of(-25.0, -30.0)), Some(()));
        assert_eq!(p, PointF32::of(-15.0, -17.0));
    }

    #[test]
    fn try_checked_add_assign_to_bounds() {
        let mut p_min = PointF32::of(MIN + 2.0, MIN + 5.0);
        assert_eq!(try_checked_add_assign(&mut p_min, &PointF32::of(-2.0, -5.0)), Some(()));
        assert_eq!(p_min, PointF32::min());

        let mut p_max = PointF32::of(MAX - 2.0, MAX - 5.0);
        assert_eq!(try_checked_add_assign(&mut p_max, &PointF32::of(2.0, 5.0)), Some(()));
        assert_eq!(p_max, PointF32::max());
    }

    #[test]
    fn try_checked_add_assign_out_of_bounds() {
        let mut p_min = PointF32::of(MIN + 2.0, MIN + 5.0);
        assert_eq!(try_checked_add_assign(&mut p_min, &PointF32::of(-10.0, 0.0)), None);
        assert_eq!(try_checked_add_assign(&mut p_min, &PointF32::of(0.0, -10.0)), None);
        assert_eq!(try_checked_add_assign(&mut p_min, &PointF32::of(-10.0, -10.0)), None);
        assert_eq!(p_min, PointF32::of(MIN + 2.0, MIN + 5.0));

        let mut p_max = PointF32::of(MAX - 2.0, MAX - 5.0);
        assert_eq!(try_checked_add_assign(&mut p_max, &PointF32::of(10.0, 0.0)), None);
        assert_eq!(try_checked_add_assign(&mut p_max, &PointF32::of(0.0, 10.0)), None);
        assert_eq!(try_checked_add_assign(&mut p_max, &PointF32::of(10.0, 10.0)), None);
        assert_eq!(p_max, PointF32::of(MAX - 2.0, MAX - 5.0));
    }

    #[test]
    fn try_checked_add_assign_limits_out_of_bounds() {
        let mut p_min = PointF32::of(MIN + 1.0, MIN + 1.0);
        assert_eq!(try_checked_add_assign(&mut p_min, &PointF32::of(MIN, 0.0)), None);
        assert_eq!(try_checked_add_assign(&mut p_min, &PointF32::of(0.0, MIN)), None);
        assert_eq!(try_checked_add_assign(&mut p_min, &PointF32::min()), None);
        assert_eq!(p_min, PointF32::of(MIN + 1.0, MIN + 1.0));

        let mut p_max = PointF32::of(MAX - 1.0, MAX - 1.0);
        assert_eq!(try_checked_add_assign(&mut p_max, &PointF32::of(MAX, 0.0)), None);
        assert_eq!(try_checked_add_assign(&mut p_max, &PointF32::of(0.0, MAX)), None);
        assert_eq!(try_checked_add_assign(&mut p_max, &PointF32::max()), None);
        assert_eq!(p_max, PointF32::of(MAX - 1.0, MAX - 1.0));
    }

    #[test]
    fn test_try_checked_add() {
        assert_eq!(try_checked_add(&PointF32::of(0.0, 0.0), &PointF32::of(10.0, 13.0)), Some(PointF32::of(10.0, 13.0)));
        assert_eq!(try_checked_add(&PointF32::of(10.0, 10.0), &PointF32::of(-5.0, -3.0)), Some(PointF32::of(5.0, 7.0)));
    }

    #[test]
    fn try_checked_add_to_bounds() {
        assert_eq!(try_checked_add(&PointF32::of(MIN + 2.0, MIN + 5.0), &PointF32::of(-2.0, -5.0)), Some(PointF32::min()));
        assert_eq!(try_checked_add(&PointF32::of(MAX - 2.0, MAX - 5.0), &PointF32::of(2.0, 5.0)), Some(PointF32::max()));
    }

    #[test]
    fn try_checked_add_out_of_bounds() {
        let p_min = PointF32::of(MIN + 2.0, MIN + 5.0);
        assert_eq!(try_checked_add(&p_min, &PointF32::of(-10.0, 0.0)), None);
        assert_eq!(try_checked_add(&p_min, &PointF32::of(0.0, -10.0)), None);
        assert_eq!(try_checked_add(&p_min, &PointF32::of(-10.0, -10.0)), None);

        let m_max = PointF32::of(MAX - 2.0, MAX - 5.0);
        assert_eq!(try_checked_add(&m_max, &PointF32::of(10.0, 0.0)), None);
        assert_eq!(try_checked_add(&m_max, &PointF32::of(0.0, 10.0)), None);
        assert_eq!(try_checked_add(&m_max, &PointF32::of(10.0, 10.0)), None);
    }

    #[test]
    fn try_checked_add_limits_out_of_bounds() {
        let p_min = PointF32::of(MIN + 1.0, MIN + 1.0);
        assert_eq!(try_checked_add(&p_min, &PointF32::of(MIN, 0.0)), None);
        assert_eq!(try_checked_add(&p_min, &PointF32::of(0.0, MIN)), None);
        assert_eq!(try_checked_add(&p_min, &PointF32::min()), None);

        let p_max = PointF32::of(MAX - 1.0, MAX - 1.0);
        assert_eq!(try_checked_add(&p_max, &PointF32::of(MAX, 0.0)), None);
        assert_eq!(try_checked_add(&p_max, &PointF32::of(0.0, MAX)), None);
        assert_eq!(try_checked_add(&p_max, &PointF32::max()), None);
    }

    #[test]
    fn test_checked_add_assign() {
        let mut p = PointF32::of(0.0, 0.0);
        checked_add_assign(&mut p, &PointF32::of(10.0, 13.0));
        assert_eq!(p, PointF32::of(10.0, 13.0));
        checked_add_assign(&mut p, &PointF32::of(-25.0, -30.0));
        assert_eq!(p, PointF32::of(-15.0, -17.0));
    }

    #[test]
    fn test_checked_add() {
        assert_eq!(checked_add(&PointF32::of(0.0, 0.0), &PointF32::of(10.0, 13.0)), PointF32::of(10.0, 13.0));
        assert_eq!(checked_add(&PointF32::of(10.0, 13.0), &PointF32::of(-5.0, -3.0)), PointF32::of(5.0, 10.0));
    }
}

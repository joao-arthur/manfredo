use super::{MAX, MIN, PointF64};

pub fn try_assign_add(p: &mut PointF64, delta: &PointF64) -> Option<()> {
    let x = p.x + delta.x;
    let y = p.y + delta.y;
    if x < MIN || x > MAX || y < MIN || y > MAX {
        return None;
    }
    p.x = x;
    p.y = y;
    Some(())
}

pub fn try_add(p: &PointF64, delta: &PointF64) -> Option<PointF64> {
    let x = p.x + delta.x;
    let y = p.y + delta.y;
    if x < MIN || x > MAX || y < MIN || y > MAX {
        return None;
    }
    Some(PointF64 { x, y })
}

pub fn assign_add(p: &mut PointF64, delta: &PointF64) {
    try_assign_add(p, delta).unwrap()
}

pub fn add(p: &PointF64, delta: &PointF64) -> PointF64 {
    try_add(p, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_f64::{MAX, MIN, PointF64};

    use super::{add, assign_add, try_add, try_assign_add};

    #[test]
    fn test_try_assign_add() {
        let mut p = PointF64::of(0.0, 0.0);
        assert_eq!(try_assign_add(&mut p, &PointF64::of(10.0, 13.0)), Some(()));
        assert_eq!(p, PointF64::of(10.0, 13.0));
        assert_eq!(try_assign_add(&mut p, &PointF64::of(-25.0, -30.0)), Some(()));
        assert_eq!(p, PointF64::of(-15.0, -17.0));
    }

    #[test]
    fn try_assign_add_to_bounds() {
        let mut p_min = PointF64::of(MIN + 2.0, MIN + 5.0);
        assert_eq!(try_assign_add(&mut p_min, &PointF64::of(-2.0, -5.0)), Some(()));
        assert_eq!(p_min, PointF64::min());

        let mut p_max = PointF64::of(MAX - 2.0, MAX - 5.0);
        assert_eq!(try_assign_add(&mut p_max, &PointF64::of(2.0, 5.0)), Some(()));
        assert_eq!(p_max, PointF64::max());
    }

    #[test]
    fn try_assign_add_beyond_bounds() {
        let mut p_min = PointF64::of(MIN + 2.0, MIN + 5.0);
        assert_eq!(try_assign_add(&mut p_min, &PointF64::of(-10.0, 0.0)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointF64::of(0.0, -10.0)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointF64::of(-10.0, -10.0)), None);
        assert_eq!(p_min, PointF64::of(MIN + 2.0, MIN + 5.0));

        let mut p_max = PointF64::of(MAX - 2.0, MAX - 5.0);
        assert_eq!(try_assign_add(&mut p_max, &PointF64::of(10.0, 0.0)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointF64::of(0.0, 10.0)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointF64::of(10.0, 10.0)), None);
        assert_eq!(p_max, PointF64::of(MAX - 2.0, MAX - 5.0));
    }

    #[test]
    fn try_assign_add_limits() {
        let mut p_min = PointF64::of(MIN + 1.0, MIN + 1.0);
        assert_eq!(try_assign_add(&mut p_min, &PointF64::of(MIN, 0.0)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointF64::of(0.0, MIN)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointF64::min()), None);
        assert_eq!(p_min, PointF64::of(MIN + 1.0, MIN + 1.0));

        let mut p_max = PointF64::of(MAX - 1.0, MAX - 1.0);
        assert_eq!(try_assign_add(&mut p_max, &PointF64::of(MAX, 0.0)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointF64::of(0.0, MAX)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointF64::max()), None);
        assert_eq!(p_max, PointF64::of(MAX - 1.0, MAX - 1.0));
    }

    #[test]
    fn test_try_add() {
        assert_eq!(try_add(&PointF64::of(0.0, 0.0), &PointF64::of(10.0, 13.0)), Some(PointF64::of(10.0, 13.0)));
        assert_eq!(try_add(&PointF64::of(10.0, 10.0), &PointF64::of(-5.0, -3.0)), Some(PointF64::of(5.0, 7.0)));
    }

    #[test]
    fn try_add_to_bounds() {
        assert_eq!(try_add(&PointF64::of(MIN + 2.0, MIN + 5.0), &PointF64::of(-2.0, -5.0)), Some(PointF64::min()));
        assert_eq!(try_add(&PointF64::of(MAX - 2.0, MAX - 5.0), &PointF64::of(2.0, 5.0)), Some(PointF64::max()));
    }

    #[test]
    fn try_add_beyond_bounds() {
        let p_min = PointF64::of(MIN + 2.0, MIN + 5.0);
        assert_eq!(try_add(&p_min, &PointF64::of(-10.0, 0.0)), None);
        assert_eq!(try_add(&p_min, &PointF64::of(0.0, -10.0)), None);
        assert_eq!(try_add(&p_min, &PointF64::of(-10.0, -10.0)), None);

        let m_max = PointF64::of(MAX - 2.0, MAX - 5.0);
        assert_eq!(try_add(&m_max, &PointF64::of(10.0, 0.0)), None);
        assert_eq!(try_add(&m_max, &PointF64::of(0.0, 10.0)), None);
        assert_eq!(try_add(&m_max, &PointF64::of(10.0, 10.0)), None);
    }

    #[test]
    fn try_add_limits() {
        let p_min = PointF64::of(MIN + 1.0, MIN + 1.0);
        assert_eq!(try_add(&p_min, &PointF64::of(MIN, 0.0)), None);
        assert_eq!(try_add(&p_min, &PointF64::of(0.0, MIN)), None);
        assert_eq!(try_add(&p_min, &PointF64::min()), None);

        let p_max = PointF64::of(MAX - 1.0, MAX - 1.0);
        assert_eq!(try_add(&p_max, &PointF64::of(MAX, 0.0)), None);
        assert_eq!(try_add(&p_max, &PointF64::of(0.0, MAX)), None);
        assert_eq!(try_add(&p_max, &PointF64::max()), None);
    }

    #[test]
    fn test_assign_add() {
        let mut p = PointF64::of(0.0, 0.0);
        assign_add(&mut p, &PointF64::of(10.0, 13.0));
        assert_eq!(p, PointF64::of(10.0, 13.0));
        assign_add(&mut p, &PointF64::of(-25.0, -30.0));
        assert_eq!(p, PointF64::of(-15.0, -17.0));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&PointF64::of(0.0, 0.0), &PointF64::of(10.0, 13.0)), PointF64::of(10.0, 13.0));
        assert_eq!(add(&PointF64::of(10.0, 13.0), &PointF64::of(-5.0, -3.0)), PointF64::of(5.0, 10.0));
    }
}

use super::PointI64;

pub fn assign_add(p: &mut PointI64, delta: &PointI64) {
    p.x = p.x.saturating_add(delta.x);
    p.y = p.y.saturating_add(delta.y);
}

pub fn add(p: &PointI64, delta: &PointI64) -> PointI64 {
    let x = p.x.saturating_add(delta.x);
    let y = p.y.saturating_add(delta.y);
    PointI64 { x, y }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_i64::PointI64;

    use super::{add, assign_add};

    #[test]
    fn test_assign_add() {
        let mut p = PointI64::of(0, 0);
        assign_add(&mut p, &PointI64::of(10, 13));
        assert_eq!(p, PointI64::of(10, 13));
        assign_add(&mut p, &PointI64::of(-5, -3));
        assert_eq!(p, PointI64::of(5, 10));
    }

    #[test]
    fn assign_add_to_bounds() {
        let mut p_min = PointI64::of(i64::MIN + 2, i64::MIN + 5);
        assign_add(&mut p_min, &PointI64::of(-2, -5));
        assert_eq!(p_min, PointI64::min());

        let mut p_max = PointI64::of(i64::MAX - 2, i64::MAX - 5);
        assign_add(&mut p_max, &PointI64::of(2, 5));
        assert_eq!(p_max, PointI64::max());
    }

    #[test]
    fn assign_add_beyond_bounds() {
        let mut p_min = PointI64::of(i64::MIN + 2, i64::MIN + 5);
        assign_add(&mut p_min, &PointI64::of(-10, -10));
        assert_eq!(p_min, PointI64::min());

        let mut p_max = PointI64::of(i64::MAX - 2, i64::MAX - 5);
        assign_add(&mut p_max, &PointI64::of(10, 10));
        assert_eq!(p_max, PointI64::max());
    }

    #[test]
    fn assign_add_limits() {
        let mut p_min = PointI64::of(i64::MIN + 1, i64::MIN + 1);
        assign_add(&mut p_min, &PointI64::min());
        assert_eq!(p_min, PointI64::min());

        let mut p_max = PointI64::of(i64::MAX - 1, i64::MAX - 1);
        assign_add(&mut p_max, &PointI64::max());
        assert_eq!(p_max, PointI64::max());
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&PointI64::of(0, 0), &PointI64::of(10, 13)), PointI64::of(10, 13));
        assert_eq!(add(&PointI64::of(10, 10), &PointI64::of(-5, -3)), PointI64::of(5, 7));
    }

    #[test]
    fn add_to_bounds() {
        assert_eq!(add(&PointI64::of(i64::MIN + 2, i64::MIN + 5), &PointI64::of(-2, -5)), PointI64::min());
        assert_eq!(add(&PointI64::of(i64::MAX - 2, i64::MAX - 5), &PointI64::of(2, 5)), PointI64::max());
    }

    #[test]
    fn add_beyond_bounds() {
        assert_eq!(add(&PointI64::of(i64::MIN + 2, i64::MIN + 5), &PointI64::of(-10, -10)), PointI64::min());
        assert_eq!(add(&PointI64::of(i64::MAX - 2, i64::MAX - 5), &PointI64::of(10, 10)), PointI64::max());
    }

    #[test]
    fn add_limits() {
        assert_eq!(add(&PointI64::of(i64::MIN + 1, i64::MIN + 1), &PointI64::min()), PointI64::min());
        assert_eq!(add(&PointI64::of(i64::MAX - 1, i64::MAX - 1), &PointI64::max()), PointI64::max());
    }
}

use crate::cartesian::point::point_i16::PointI16;

pub fn saturating_add_assign(p: &mut PointI16, delta: &PointI16) {
    p.x = p.x.saturating_add(delta.x);
    p.y = p.y.saturating_add(delta.y);
}

pub fn saturating_add(p: &PointI16, delta: &PointI16) -> PointI16 {
    let x = p.x.saturating_add(delta.x);
    let y = p.y.saturating_add(delta.y);
    PointI16 { x, y }
}

#[cfg(test)]
mod tests {
    use super::{saturating_add, saturating_add_assign};
    use crate::cartesian::point::point_i16::PointI16;

    #[test]
    fn test_saturating_add_assign() {
        let mut p = PointI16::of(0, 0);
        saturating_add_assign(&mut p, &PointI16::of(10, 13));
        assert_eq!(p, PointI16::of(10, 13));
        saturating_add_assign(&mut p, &PointI16::of(-5, -3));
        assert_eq!(p, PointI16::of(5, 10));
    }

    #[test]
    fn saturating_add_assign_to_bounds() {
        let mut p_min = PointI16::of(i16::MIN + 2, i16::MIN + 5);
        saturating_add_assign(&mut p_min, &PointI16::of(-2, -5));
        assert_eq!(p_min, PointI16::min());

        let mut p_max = PointI16::of(i16::MAX - 2, i16::MAX - 5);
        saturating_add_assign(&mut p_max, &PointI16::of(2, 5));
        assert_eq!(p_max, PointI16::max());
    }

    #[test]
    fn saturating_add_assign_out_of_bounds() {
        let mut p_min = PointI16::of(i16::MIN + 2, i16::MIN + 5);
        saturating_add_assign(&mut p_min, &PointI16::of(-10, -10));
        assert_eq!(p_min, PointI16::min());

        let mut p_max = PointI16::of(i16::MAX - 2, i16::MAX - 5);
        saturating_add_assign(&mut p_max, &PointI16::of(10, 10));
        assert_eq!(p_max, PointI16::max());
    }

    #[test]
    fn saturating_add_assign_limits_out_of_bounds() {
        let mut p_min = PointI16::of(i16::MIN + 1, i16::MIN + 1);
        saturating_add_assign(&mut p_min, &PointI16::min());
        assert_eq!(p_min, PointI16::min());

        let mut p_max = PointI16::of(i16::MAX - 1, i16::MAX - 1);
        saturating_add_assign(&mut p_max, &PointI16::max());
        assert_eq!(p_max, PointI16::max());
    }

    #[test]
    fn test_saturating_add() {
        assert_eq!(saturating_add(&PointI16::of(0, 0), &PointI16::of(10, 13)), PointI16::of(10, 13));
        assert_eq!(saturating_add(&PointI16::of(10, 10), &PointI16::of(-5, -3)), PointI16::of(5, 7));
    }

    #[test]
    fn saturating_add_to_bounds() {
        assert_eq!(saturating_add(&PointI16::of(i16::MIN + 2, i16::MIN + 5), &PointI16::of(-2, -5)), PointI16::min());
        assert_eq!(saturating_add(&PointI16::of(i16::MAX - 2, i16::MAX - 5), &PointI16::of(2, 5)), PointI16::max());
    }

    #[test]
    fn saturating_add_out_of_bounds() {
        assert_eq!(saturating_add(&PointI16::of(i16::MIN + 2, i16::MIN + 5), &PointI16::of(-10, -10)), PointI16::min());
        assert_eq!(saturating_add(&PointI16::of(i16::MAX - 2, i16::MAX - 5), &PointI16::of(10, 10)), PointI16::max());
    }

    #[test]
    fn saturating_add_limits_out_of_bounds() {
        assert_eq!(saturating_add(&PointI16::of(i16::MIN + 1, i16::MIN + 1), &PointI16::min()), PointI16::min());
        assert_eq!(saturating_add(&PointI16::of(i16::MAX - 1, i16::MAX - 1), &PointI16::max()), PointI16::max());
    }
}

#[cfg(test)]
mod tests {
    use super::super::{saturating_add, saturating_add_assign};
    use crate::matrix::point::{point_i16::PointI16, point_u16::PointU16};

    #[test]
    fn test_saturating_add_assign() {
        let mut p = PointU16::min();
        saturating_add_assign(&mut p, &PointI16::of(10, 13));
        assert_eq!(p, PointU16::of(10, 13));
        saturating_add_assign(&mut p, &PointI16::of(-5, -3));
        assert_eq!(p, PointU16::of(5, 10));
    }

    #[test]
    fn saturating_add_assign_to_bounds() {
        let mut p_min = PointU16::of(2, 5);
        saturating_add_assign(&mut p_min, &PointI16::of(-2, -5));
        assert_eq!(p_min, PointU16::min());

        let mut p_max = PointU16::of(u16::MAX - 2, u16::MAX - 5);
        saturating_add_assign(&mut p_max, &PointI16::of(2, 5));
        assert_eq!(p_max, PointU16::max());
    }

    #[test]
    fn saturating_add_assign_out_of_bounds() {
        let mut p_min = PointU16::of(2, 5);
        saturating_add_assign(&mut p_min, &PointI16::of(-10, -10));
        assert_eq!(p_min, PointU16::min());

        let mut p_max = PointU16::of(u16::MAX - 2, u16::MAX - 5);
        saturating_add_assign(&mut p_max, &PointI16::of(10, 10));
        assert_eq!(p_max, PointU16::max());
    }

    #[test]
    fn saturating_add_assign_limits_out_of_bounds() {
        let mut p_min = PointU16::of(1, 1);
        saturating_add_assign(&mut p_min, &PointI16::min());
        assert_eq!(p_min, PointU16::min());

        let mut p_max = PointU16::of(u16::MAX - 1, u16::MAX - 1);
        saturating_add_assign(&mut p_max, &PointI16::max());
        assert_eq!(p_max, PointU16::max());
    }

    #[test]
    fn test_saturating_add() {
        assert_eq!(saturating_add(&PointU16::min(), &PointI16::of(10, 13)), PointU16::of(10, 13));
        assert_eq!(saturating_add(&PointU16::of(10, 10), &PointI16::of(-5, -3)), PointU16::of(5, 7));
    }

    #[test]
    fn saturating_add_to_bounds() {
        assert_eq!(saturating_add(&PointU16::of(2, 5), &PointI16::of(-2, -5)), PointU16::min());
        assert_eq!(saturating_add(&PointU16::of(u16::MAX - 2, u16::MAX - 5), &PointI16::of(2, 5)), PointU16::max());
    }

    #[test]
    fn saturating_add_out_of_bounds() {
        assert_eq!(saturating_add(&PointU16::of(2, 5), &PointI16::of(-10, -10)), PointU16::min());
        assert_eq!(saturating_add(&PointU16::of(u16::MAX - 2, u16::MAX - 5), &PointI16::of(10, 10)), PointU16::max());
    }

    #[test]
    fn saturating_add_limits_out_of_bounds() {
        assert_eq!(saturating_add(&PointU16::of(1, 1), &PointI16::min()), PointU16::min());
        assert_eq!(saturating_add(&PointU16::of(u16::MAX - 1, u16::MAX - 1), &PointI16::max()), PointU16::max());
    }
}

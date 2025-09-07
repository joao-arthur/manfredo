#[cfg(test)]
mod tests {
    use super::super::{wrapping_add, wrapping_add_assign};
    use crate::cartesian::point::{point_i16::PointI16, point_u16::PointU16};

    const MAX: u16 = u16::MAX;

    #[test]
    fn test_wrapping_add_assign() {
        let mut p = PointU16::min();
        wrapping_add_assign(&mut p, &PointI16::of(10, 13));
        assert_eq!(p, PointU16::of(10, 13));
        wrapping_add_assign(&mut p, &PointI16::of(-5, -3));
        assert_eq!(p, PointU16::of(5, 10));
    }

    #[test]
    fn wrapping_add_assign_to_bounds() {
        let mut p_min = PointU16::of(2, 5);
        wrapping_add_assign(&mut p_min, &PointI16::of(-2, -5));
        assert_eq!(p_min, PointU16::min());

        let mut m_max = PointU16::of(MAX - 2, MAX - 5);
        wrapping_add_assign(&mut m_max, &PointI16::of(2, 5));
        assert_eq!(m_max, PointU16::max());
    }

    #[test]
    fn wrapping_add_assign_out_of_bounds() {
        let mut p_min = PointU16::of(2, 5);
        wrapping_add_assign(&mut p_min, &PointI16::of(-10, -10));
        assert_eq!(p_min, PointU16::of(MAX - 7, MAX - 4));

        let mut m_max = PointU16::of(MAX - 2, MAX - 5);
        wrapping_add_assign(&mut m_max, &PointI16::of(10, 10));
        assert_eq!(m_max, PointU16::of(7, 4));
    }

    #[test]
    fn wrapping_add_assign_limits_out_of_bounds() {
        let mut p_min = PointU16::of(1, 1);
        wrapping_add_assign(&mut p_min, &PointI16::min());
        assert_eq!(p_min, PointU16::of(32769, 32769));

        let mut m_max = PointU16::of(MAX - 1, MAX - 1);
        wrapping_add_assign(&mut m_max, &PointI16::max());
        assert_eq!(m_max, PointU16::of(32765, 32765));
    }

    #[test]
    fn test_wrapping_add() {
        assert_eq!(wrapping_add(&PointU16::min(), &PointI16::of(10, 13)), PointU16::of(10, 13));
        assert_eq!(wrapping_add(&PointU16::of(10, 10), &PointI16::of(-5, -3)), PointU16::of(5, 7));
    }

    #[test]
    fn wrapping_add_to_bounds() {
        assert_eq!(wrapping_add(&PointU16::of(2, 5), &PointI16::of(-2, -5)), PointU16::min());
        assert_eq!(wrapping_add(&PointU16::of(MAX - 2, MAX - 5), &PointI16::of(2, 5)), PointU16::max());
    }

    #[test]
    fn wrapping_add_out_of_bounds() {
        assert_eq!(wrapping_add(&PointU16::of(2, 5), &PointI16::of(-10, -10)), PointU16::of(MAX - 7, MAX - 4));
        assert_eq!(wrapping_add(&PointU16::of(MAX - 2, MAX - 5), &PointI16::of(10, 10)), PointU16::of(7, 4));
    }

    #[test]
    fn wrapping_add_limits_out_of_bounds() {
        assert_eq!(wrapping_add(&PointU16::of(1, 1), &PointI16::min()), PointU16::of(32769, 32769));
        assert_eq!(wrapping_add(&PointU16::of(MAX - 1, MAX - 1), &PointI16::max()), PointU16::of(32765, 32765));
    }
}

#[cfg(test)]
mod tests {
    use super::super::{wrapping_add, wrapping_add_assign};
    use crate::matrix::point::{point_i64::PointI64, point_u64::PointU64};

    #[test]
    fn test_wrapping_add_assign() {
        let mut p = PointU64::min();
        wrapping_add_assign(&mut p, &PointI64::of(10, 13));
        assert_eq!(p, PointU64::of(10, 13));
        wrapping_add_assign(&mut p, &PointI64::of(-5, -3));
        assert_eq!(p, PointU64::of(5, 10));
    }

    #[test]
    fn wrapping_add_assign_to_bounds() {
        let mut p_min = PointU64::of(2, 5);
        wrapping_add_assign(&mut p_min, &PointI64::of(-2, -5));
        assert_eq!(p_min, PointU64::min());

        let mut m_max = PointU64::of(u64::MAX - 2, u64::MAX - 5);
        wrapping_add_assign(&mut m_max, &PointI64::of(2, 5));
        assert_eq!(m_max, PointU64::max());
    }

    #[test]
    fn wrapping_add_assign_out_of_bounds() {
        let mut p_min = PointU64::of(2, 5);
        wrapping_add_assign(&mut p_min, &PointI64::of(-10, -10));
        assert_eq!(p_min, PointU64::of(u64::MAX - 7, u64::MAX - 4));

        let mut m_max = PointU64::of(u64::MAX - 2, u64::MAX - 5);
        wrapping_add_assign(&mut m_max, &PointI64::of(10, 10));
        assert_eq!(m_max, PointU64::of(7, 4));
    }

    #[test]
    fn wrapping_add_assign_limits_out_of_bounds() {
        let mut p_min = PointU64::of(1, 1);
        wrapping_add_assign(&mut p_min, &PointI64::min());
        assert_eq!(p_min, PointU64::of(9223372036854775809, 9223372036854775809));

        let mut m_max = PointU64::of(u64::MAX - 1, u64::MAX - 1);
        wrapping_add_assign(&mut m_max, &PointI64::max());
        assert_eq!(m_max, PointU64::of(9223372036854775805, 9223372036854775805));
    }

    #[test]
    fn test_wrapping_add() {
        assert_eq!(wrapping_add(&PointU64::min(), &PointI64::of(10, 13)), PointU64::of(10, 13));
        assert_eq!(wrapping_add(&PointU64::of(10, 10), &PointI64::of(-5, -3)), PointU64::of(5, 7));
    }

    #[test]
    fn wrapping_add_to_bounds() {
        assert_eq!(wrapping_add(&PointU64::of(2, 5), &PointI64::of(-2, -5)), PointU64::min());
        assert_eq!(wrapping_add(&PointU64::of(u64::MAX - 2, u64::MAX - 5), &PointI64::of(2, 5)), PointU64::max());
    }

    #[test]
    fn wrapping_add_out_of_bounds() {
        assert_eq!(wrapping_add(&PointU64::of(2, 5), &PointI64::of(-10, -10)), PointU64::of(u64::MAX - 7, u64::MAX - 4));
        assert_eq!(wrapping_add(&PointU64::of(u64::MAX - 2, u64::MAX - 5), &PointI64::of(10, 10)), PointU64::of(7, 4));
    }

    #[test]
    fn wrapping_add_limits_out_of_bounds() {
        assert_eq!(wrapping_add(&PointU64::of(1, 1), &PointI64::min()), PointU64::of(9223372036854775809, 9223372036854775809));
        assert_eq!(wrapping_add(&PointU64::of(u64::MAX - 1, u64::MAX - 1), &PointI64::max()), PointU64::of(9223372036854775805, 9223372036854775805));
    }
}

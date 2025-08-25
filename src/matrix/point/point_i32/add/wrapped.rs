#[cfg(test)]
mod tests {
    use super::super::{wrapping_add, wrapping_add_assign};
    use crate::matrix::point::point_i32::PointI32;

    #[test]
    fn test_wrapping_add_assign() {
        let mut p = PointI32::of(0, 0);
        wrapping_add_assign(&mut p, &PointI32::of(10, 13));
        assert_eq!(p, PointI32::of(10, 13));
        wrapping_add_assign(&mut p, &PointI32::of(-5, -3));
        assert_eq!(p, PointI32::of(5, 10));
    }

    #[test]
    fn wrapping_add_assign_to_bounds() {
        let mut p_min = PointI32::of(i32::MIN + 2, i32::MIN + 5);
        wrapping_add_assign(&mut p_min, &PointI32::of(-2, -5));
        assert_eq!(p_min, PointI32::min());

        let mut m_max = PointI32::of(i32::MAX - 2, i32::MAX - 5);
        wrapping_add_assign(&mut m_max, &PointI32::of(2, 5));
        assert_eq!(m_max, PointI32::max());
    }

    #[test]
    fn wrapping_add_assign_out_of_bounds() {
        let mut p_min = PointI32::of(i32::MIN + 2, i32::MIN + 5);
        wrapping_add_assign(&mut p_min, &PointI32::of(-10, -10));
        assert_eq!(p_min, PointI32::of(i32::MAX - 7, i32::MAX - 4));

        let mut m_max = PointI32::of(i32::MAX - 2, i32::MAX - 5);
        wrapping_add_assign(&mut m_max, &PointI32::of(10, 10));
        assert_eq!(m_max, PointI32::of(i32::MIN + 7, i32::MIN + 4));
    }

    #[test]
    fn wrapping_add_assign_limits_out_of_bounds() {
        let mut p_min = PointI32::of(i32::MIN + 1, i32::MIN + 1);
        wrapping_add_assign(&mut p_min, &PointI32::min());
        assert_eq!(p_min, PointI32::of(1, 1));

        let mut m_max = PointI32::of(i32::MAX - 1, i32::MAX - 1);
        wrapping_add_assign(&mut m_max, &PointI32::max());
        assert_eq!(m_max, PointI32::of(-3, -3));
    }

    #[test]
    fn test_wrapping_add() {
        assert_eq!(wrapping_add(&PointI32::of(0, 0), &PointI32::of(10, 13)), PointI32::of(10, 13));
        assert_eq!(wrapping_add(&PointI32::of(10, 10), &PointI32::of(-5, -3)), PointI32::of(5, 7));
    }

    #[test]
    fn wrapping_add_to_bounds() {
        assert_eq!(wrapping_add(&PointI32::of(i32::MIN + 2, i32::MIN + 5), &PointI32::of(-2, -5)), PointI32::min());
        assert_eq!(wrapping_add(&PointI32::of(i32::MAX - 2, i32::MAX - 5), &PointI32::of(2, 5)), PointI32::max());
    }

    #[test]
    fn wrapping_add_out_of_bounds() {
        assert_eq!(wrapping_add(&PointI32::of(i32::MIN + 2, i32::MIN + 5), &PointI32::of(-10, -10)), PointI32::of(i32::MAX - 7, i32::MAX - 4));
        assert_eq!(wrapping_add(&PointI32::of(i32::MAX - 2, i32::MAX - 5), &PointI32::of(10, 10)), PointI32::of(i32::MIN + 7, i32::MIN + 4));
    }

    #[test]
    fn wrapping_add_limits_out_of_bounds() {
        assert_eq!(wrapping_add(&PointI32::of(i32::MIN + 1, i32::MIN + 1), &PointI32::min()), PointI32::of(1, 1));
        assert_eq!(wrapping_add(&PointI32::of(i32::MAX - 1, i32::MAX - 1), &PointI32::max()), PointI32::of(-3, -3));
    }
}

#[cfg(test)]
mod tests {
    use super::super::{wrapping_add, wrapping_add_assign};
    use crate::matrix::point::{point_i8::PointI8, point_u8::PointU8};

    const MAX: u8 = u8::MAX;

    #[test]
    fn test_wrapping_add_assign() {
        let mut p = PointU8::min();
        wrapping_add_assign(&mut p, &PointI8::of(10, 13));
        assert_eq!(p, PointU8::of(10, 13));
        wrapping_add_assign(&mut p, &PointI8::of(-5, -3));
        assert_eq!(p, PointU8::of(5, 10));
    }

    #[test]
    fn wrapping_add_assign_to_bounds() {
        let mut p_min = PointU8::of(2, 5);
        wrapping_add_assign(&mut p_min, &PointI8::of(-2, -5));
        assert_eq!(p_min, PointU8::min());

        let mut m_max = PointU8::of(MAX - 2, MAX - 5);
        wrapping_add_assign(&mut m_max, &PointI8::of(2, 5));
        assert_eq!(m_max, PointU8::max());
    }

    #[test]
    fn wrapping_add_assign_out_of_bounds() {
        let mut p_min = PointU8::of(2, 5);
        wrapping_add_assign(&mut p_min, &PointI8::of(-10, -10));
        assert_eq!(p_min, PointU8::of(MAX - 7, MAX - 4));

        let mut m_max = PointU8::of(MAX - 2, MAX - 5);
        wrapping_add_assign(&mut m_max, &PointI8::of(10, 10));
        assert_eq!(m_max, PointU8::of(7, 4));
    }

    #[test]
    fn wrapping_add_assign_limits_out_of_bounds() {
        let mut p_min = PointU8::of(1, 1);
        wrapping_add_assign(&mut p_min, &PointI8::min());
        assert_eq!(p_min, PointU8::of(129, 129));

        let mut m_max = PointU8::of(MAX - 1, MAX - 1);
        wrapping_add_assign(&mut m_max, &PointI8::max());
        assert_eq!(m_max, PointU8::of(125, 125));
    }

    #[test]
    fn test_wrapping_add() {
        assert_eq!(wrapping_add(&PointU8::min(), &PointI8::of(10, 13)), PointU8::of(10, 13));
        assert_eq!(wrapping_add(&PointU8::of(10, 10), &PointI8::of(-5, -3)), PointU8::of(5, 7));
    }

    #[test]
    fn wrapping_add_to_bounds() {
        assert_eq!(wrapping_add(&PointU8::of(2, 5), &PointI8::of(-2, -5)), PointU8::min());
        assert_eq!(wrapping_add(&PointU8::of(MAX - 2, MAX - 5), &PointI8::of(2, 5)), PointU8::max());
    }

    #[test]
    fn wrapping_add_out_of_bounds() {
        assert_eq!(wrapping_add(&PointU8::of(2, 5), &PointI8::of(-10, -10)), PointU8::of(MAX - 7, MAX - 4));
        assert_eq!(wrapping_add(&PointU8::of(MAX - 2, MAX - 5), &PointI8::of(10, 10)), PointU8::of(7, 4));
    }

    #[test]
    fn wrapping_add_limits_out_of_bounds() {
        assert_eq!(wrapping_add(&PointU8::of(1, 1), &PointI8::min()), PointU8::of(129, 129));
        assert_eq!(wrapping_add(&PointU8::of(MAX - 1, MAX - 1), &PointI8::max()), PointU8::of(125, 125));
    }
}

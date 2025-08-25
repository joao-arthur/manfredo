use crate::matrix::point::point_i8::PointI8;

pub fn wrapping_add_assign(p: &mut PointI8, delta: &PointI8) {
    p.row = p.row.wrapping_add(delta.row);
    p.col = p.col.wrapping_add(delta.col);
}

pub fn wrapping_add(p: &PointI8, delta: &PointI8) -> PointI8 {
    let row = p.row.wrapping_add(delta.row);
    let col = p.col.wrapping_add(delta.col);
    PointI8 { row, col }
}

#[cfg(test)]
mod tests {
    use super::{wrapping_add, wrapping_add_assign};
    use crate::matrix::point::point_i8::PointI8;

    #[test]
    fn test_wrapping_add_assign() {
        let mut p = PointI8::of(0, 0);
        wrapping_add_assign(&mut p, &PointI8::of(10, 13));
        assert_eq!(p, PointI8::of(10, 13));
        wrapping_add_assign(&mut p, &PointI8::of(-5, -3));
        assert_eq!(p, PointI8::of(5, 10));
    }

    #[test]
    fn wrapping_add_assign_to_bounds() {
        let mut p_min = PointI8::of(i8::MIN + 2, i8::MIN + 5);
        wrapping_add_assign(&mut p_min, &PointI8::of(-2, -5));
        assert_eq!(p_min, PointI8::min());

        let mut m_max = PointI8::of(i8::MAX - 2, i8::MAX - 5);
        wrapping_add_assign(&mut m_max, &PointI8::of(2, 5));
        assert_eq!(m_max, PointI8::max());
    }

    #[test]
    fn wrapping_add_assign_out_of_bounds() {
        let mut p_min = PointI8::of(i8::MIN + 2, i8::MIN + 5);
        wrapping_add_assign(&mut p_min, &PointI8::of(-10, -10));
        assert_eq!(p_min, PointI8::of(i8::MAX - 7, i8::MAX - 4));

        let mut m_max = PointI8::of(i8::MAX - 2, i8::MAX - 5);
        wrapping_add_assign(&mut m_max, &PointI8::of(10, 10));
        assert_eq!(m_max, PointI8::of(i8::MIN + 7, i8::MIN + 4));
    }

    #[test]
    fn wrapping_add_assign_limits_out_of_bounds() {
        let mut p_min = PointI8::of(i8::MIN + 1, i8::MIN + 1);
        wrapping_add_assign(&mut p_min, &PointI8::min());
        assert_eq!(p_min, PointI8::of(1, 1));

        let mut m_max = PointI8::of(i8::MAX - 1, i8::MAX - 1);
        wrapping_add_assign(&mut m_max, &PointI8::max());
        assert_eq!(m_max, PointI8::of(-3, -3));
    }

    #[test]
    fn test_wrapping_add() {
        assert_eq!(wrapping_add(&PointI8::of(0, 0), &PointI8::of(10, 13)), PointI8::of(10, 13));
        assert_eq!(wrapping_add(&PointI8::of(10, 10), &PointI8::of(-5, -3)), PointI8::of(5, 7));
    }

    #[test]
    fn wrapping_add_to_bounds() {
        assert_eq!(wrapping_add(&PointI8::of(i8::MIN + 2, i8::MIN + 5), &PointI8::of(-2, -5)), PointI8::min());
        assert_eq!(wrapping_add(&PointI8::of(i8::MAX - 2, i8::MAX - 5), &PointI8::of(2, 5)), PointI8::max());
    }

    #[test]
    fn wrapping_add_out_of_bounds() {
        assert_eq!(wrapping_add(&PointI8::of(i8::MIN + 2, i8::MIN + 5), &PointI8::of(-10, -10)), PointI8::of(i8::MAX - 7, i8::MAX - 4));
        assert_eq!(wrapping_add(&PointI8::of(i8::MAX - 2, i8::MAX - 5), &PointI8::of(10, 10)), PointI8::of(i8::MIN + 7, i8::MIN + 4));
    }

    #[test]
    fn wrapping_add_limits_out_of_bounds() {
        assert_eq!(wrapping_add(&PointI8::of(i8::MIN + 1, i8::MIN + 1), &PointI8::min()), PointI8::of(1, 1));
        assert_eq!(wrapping_add(&PointI8::of(i8::MAX - 1, i8::MAX - 1), &PointI8::max()), PointI8::of(-3, -3));
    }
}

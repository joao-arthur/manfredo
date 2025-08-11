use super::PointI8;

pub fn assign_add(p: &mut PointI8, delta: &PointI8) {
    p.x = p.x.wrapping_add(delta.x);
    p.y = p.y.wrapping_add(delta.y);
}

pub fn add(p: &PointI8, delta: &PointI8) -> PointI8 {
    let x = p.x.wrapping_add(delta.x);
    let y = p.y.wrapping_add(delta.y);
    PointI8 { x, y }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_i8::PointI8;

    use super::{add, assign_add};

    #[test]
    fn test_assign_add() {
        let mut p = PointI8::of(0, 0);
        assign_add(&mut p, &PointI8::of(10, 13));
        assert_eq!(p, PointI8::of(10, 13));
        assign_add(&mut p, &PointI8::of(-5, -3));
        assert_eq!(p, PointI8::of(5, 10));
    }

    #[test]
    fn assign_add_to_bounds() {
        let mut p_min = PointI8::of(i8::MIN + 2, i8::MIN + 5);
        assign_add(&mut p_min, &PointI8::of(-2, -5));
        assert_eq!(p_min, PointI8::min());

        let mut m_max = PointI8::of(i8::MAX - 2, i8::MAX - 5);
        assign_add(&mut m_max, &PointI8::of(2, 5));
        assert_eq!(m_max, PointI8::max());
    }

    #[test]
    fn assign_add_out_of_bounds() {
        let mut p_min = PointI8::of(i8::MIN + 2, i8::MIN + 5);
        assign_add(&mut p_min, &PointI8::of(-10, -10));
        assert_eq!(p_min, PointI8::of(i8::MAX - 7, i8::MAX - 4));

        let mut m_max = PointI8::of(i8::MAX - 2, i8::MAX - 5);
        assign_add(&mut m_max, &PointI8::of(10, 10));
        assert_eq!(m_max, PointI8::of(i8::MIN + 7, i8::MIN + 4));
    }

    #[test]
    fn assign_add_limits_out_of_bounds_out_of_bounds() {
        let mut p_min = PointI8::of(i8::MIN + 1, i8::MIN + 1);
        assign_add(&mut p_min, &PointI8::min());
        assert_eq!(p_min, PointI8::of(1, 1));

        let mut m_max = PointI8::of(i8::MAX - 1, i8::MAX - 1);
        assign_add(&mut m_max, &PointI8::max());
        assert_eq!(m_max, PointI8::of(-3, -3));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&PointI8::of(0, 0), &PointI8::of(10, 13)), PointI8::of(10, 13));
        assert_eq!(add(&PointI8::of(10, 10), &PointI8::of(-5, -3)), PointI8::of(5, 7));
    }

    #[test]
    fn add_to_bounds() {
        assert_eq!(add(&PointI8::of(i8::MIN + 2, i8::MIN + 5), &PointI8::of(-2, -5)), PointI8::min());
        assert_eq!(add(&PointI8::of(i8::MAX - 2, i8::MAX - 5), &PointI8::of(2, 5)), PointI8::max());
    }

    #[test]
    fn add_out_of_bounds() {
        assert_eq!(add(&PointI8::of(i8::MIN + 2, i8::MIN + 5), &PointI8::of(-10, -10)), PointI8::of(i8::MAX - 7, i8::MAX - 4));
        assert_eq!(add(&PointI8::of(i8::MAX - 2, i8::MAX - 5), &PointI8::of(10, 10)), PointI8::of(i8::MIN + 7, i8::MIN + 4));
    }

    #[test]
    fn add_limits_out_of_bounds() {
        assert_eq!(add(&PointI8::of(i8::MIN + 1, i8::MIN + 1), &PointI8::min()), PointI8::of(1, 1));
        assert_eq!(add(&PointI8::of(i8::MAX - 1, i8::MAX - 1), &PointI8::max()), PointI8::of(-3, -3));
    }
}

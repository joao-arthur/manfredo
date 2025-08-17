use crate::cartesian::point::point_i32::PointI32;

pub fn assign_add(p: &mut PointI32, delta: &PointI32) {
    p.x = p.x.wrapping_add(delta.x);
    p.y = p.y.wrapping_add(delta.y);
}

pub fn add(p: &PointI32, delta: &PointI32) -> PointI32 {
    let x = p.x.wrapping_add(delta.x);
    let y = p.y.wrapping_add(delta.y);
    PointI32 { x, y }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_i32::PointI32;

    use super::{add, assign_add};

    #[test]
    fn test_assign_add() {
        let mut p = PointI32::of(0, 0);
        assign_add(&mut p, &PointI32::of(10, 13));
        assert_eq!(p, PointI32::of(10, 13));
        assign_add(&mut p, &PointI32::of(-5, -3));
        assert_eq!(p, PointI32::of(5, 10));
    }

    #[test]
    fn assign_add_to_bounds() {
        let mut p_min = PointI32::of(i32::MIN + 2, i32::MIN + 5);
        assign_add(&mut p_min, &PointI32::of(-2, -5));
        assert_eq!(p_min, PointI32::min());

        let mut m_max = PointI32::of(i32::MAX - 2, i32::MAX - 5);
        assign_add(&mut m_max, &PointI32::of(2, 5));
        assert_eq!(m_max, PointI32::max());
    }

    #[test]
    fn assign_add_out_of_bounds() {
        let mut p_min = PointI32::of(i32::MIN + 2, i32::MIN + 5);
        assign_add(&mut p_min, &PointI32::of(-10, -10));
        assert_eq!(p_min, PointI32::of(i32::MAX - 7, i32::MAX - 4));

        let mut m_max = PointI32::of(i32::MAX - 2, i32::MAX - 5);
        assign_add(&mut m_max, &PointI32::of(10, 10));
        assert_eq!(m_max, PointI32::of(i32::MIN + 7, i32::MIN + 4));
    }

    #[test]
    fn assign_add_limits_out_of_bounds() {
        let mut p_min = PointI32::of(i32::MIN + 1, i32::MIN + 1);
        assign_add(&mut p_min, &PointI32::min());
        assert_eq!(p_min, PointI32::of(1, 1));

        let mut m_max = PointI32::of(i32::MAX - 1, i32::MAX - 1);
        assign_add(&mut m_max, &PointI32::max());
        assert_eq!(m_max, PointI32::of(-3, -3));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&PointI32::of(0, 0), &PointI32::of(10, 13)), PointI32::of(10, 13));
        assert_eq!(add(&PointI32::of(10, 10), &PointI32::of(-5, -3)), PointI32::of(5, 7));
    }

    #[test]
    fn add_to_bounds() {
        assert_eq!(add(&PointI32::of(i32::MIN + 2, i32::MIN + 5), &PointI32::of(-2, -5)), PointI32::min());
        assert_eq!(add(&PointI32::of(i32::MAX - 2, i32::MAX - 5), &PointI32::of(2, 5)), PointI32::max());
    }

    #[test]
    fn add_out_of_bounds() {
        assert_eq!(add(&PointI32::of(i32::MIN + 2, i32::MIN + 5), &PointI32::of(-10, -10)), PointI32::of(i32::MAX - 7, i32::MAX - 4));
        assert_eq!(add(&PointI32::of(i32::MAX - 2, i32::MAX - 5), &PointI32::of(10, 10)), PointI32::of(i32::MIN + 7, i32::MIN + 4));
    }

    #[test]
    fn add_limits_out_of_bounds() {
        assert_eq!(add(&PointI32::of(i32::MIN + 1, i32::MIN + 1), &PointI32::min()), PointI32::of(1, 1));
        assert_eq!(add(&PointI32::of(i32::MAX - 1, i32::MAX - 1), &PointI32::max()), PointI32::of(-3, -3));
    }
}

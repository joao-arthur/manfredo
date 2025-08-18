use crate::cartesian::point::{point_i32::PointI32, point_u32::PointU32};

pub fn assign_add(p: &mut PointU32, delta: &PointI32) {
    p.x = p.x.saturating_add_signed(delta.x);
    p.y = p.y.saturating_add_signed(delta.y);
}

pub fn add(p: &PointU32, delta: &PointI32) -> PointU32 {
    let x = p.x.saturating_add_signed(delta.x);
    let y = p.y.saturating_add_signed(delta.y);
    PointU32 { x, y }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::{point_i32::PointI32, point_u32::PointU32};

    use super::{add, assign_add};

    #[test]
    fn test_assign_add() {
        let mut p = PointU32::of(0, 0);
        assign_add(&mut p, &PointI32::of(10, 13));
        assert_eq!(p, PointU32::of(10, 13));
        assign_add(&mut p, &PointI32::of(-5, -3));
        assert_eq!(p, PointU32::of(5, 10));
    }

    #[test]
    fn assign_add_to_bounds() {
        let mut p_min = PointU32::of(2, 5);
        assign_add(&mut p_min, &PointI32::of(-2, -5));
        assert_eq!(p_min, PointU32::min());

        let mut p_max = PointU32::of(u32::MAX - 2, u32::MAX - 5);
        assign_add(&mut p_max, &PointI32::of(2, 5));
        assert_eq!(p_max, PointU32::max());
    }

    #[test]
    fn assign_add_out_of_bounds() {
        let mut p_min = PointU32::of(2, 5);
        assign_add(&mut p_min, &PointI32::of(-10, -10));
        assert_eq!(p_min, PointU32::min());

        let mut p_max = PointU32::of(u32::MAX - 2, u32::MAX - 5);
        assign_add(&mut p_max, &PointI32::of(10, 10));
        assert_eq!(p_max, PointU32::max());
    }

    #[test]
    fn assign_add_limits_out_of_bounds() {
        let mut p_min = PointU32::of(1, 1);
        assign_add(&mut p_min, &PointI32::min());
        assert_eq!(p_min, PointU32::min());

        let mut p_max = PointU32::of(u32::MAX - 1, u32::MAX - 1);
        assign_add(&mut p_max, &PointI32::max());
        assert_eq!(p_max, PointU32::max());
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&PointU32::of(0, 0), &PointI32::of(10, 13)), PointU32::of(10, 13));
        assert_eq!(add(&PointU32::of(10, 10), &PointI32::of(-5, -3)), PointU32::of(5, 7));
    }

    #[test]
    fn add_to_bounds() {
        assert_eq!(add(&PointU32::of(2, 5), &PointI32::of(-2, -5)), PointU32::min());
        assert_eq!(add(&PointU32::of(u32::MAX - 2, u32::MAX - 5), &PointI32::of(2, 5)), PointU32::max());
    }

    #[test]
    fn add_out_of_bounds() {
        assert_eq!(add(&PointU32::of(2, 5), &PointI32::of(-10, -10)), PointU32::min());
        assert_eq!(add(&PointU32::of(u32::MAX - 2, u32::MAX - 5), &PointI32::of(10, 10)), PointU32::max());
    }

    #[test]
    fn add_limits_out_of_bounds() {
        assert_eq!(add(&PointU32::of(1, 1), &PointI32::min()), PointU32::min());
        assert_eq!(add(&PointU32::of(u32::MAX - 1, u32::MAX - 1), &PointI32::max()), PointU32::max());
    }
}

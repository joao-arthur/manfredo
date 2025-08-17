use crate::matrix::point::point_i32::PointI32;

pub fn assign_add(p: &mut PointI32, delta: &PointI32) {
    p.row = p.row.saturating_add(delta.row);
    p.col = p.col.saturating_add(delta.col);
}

pub fn add(p: &PointI32, delta: &PointI32) -> PointI32 {
    let row = p.row.saturating_add(delta.row);
    let col = p.col.saturating_add(delta.col);
    PointI32 { row, col }
}

#[cfg(test)]
mod tests {
    use crate::matrix::point::point_i32::PointI32;

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

        let mut p_max = PointI32::of(i32::MAX - 2, i32::MAX - 5);
        assign_add(&mut p_max, &PointI32::of(2, 5));
        assert_eq!(p_max, PointI32::max());
    }

    #[test]
    fn assign_add_out_of_bounds() {
        let mut p_min = PointI32::of(i32::MIN + 2, i32::MIN + 5);
        assign_add(&mut p_min, &PointI32::of(-10, -10));
        assert_eq!(p_min, PointI32::min());

        let mut p_max = PointI32::of(i32::MAX - 2, i32::MAX - 5);
        assign_add(&mut p_max, &PointI32::of(10, 10));
        assert_eq!(p_max, PointI32::max());
    }

    #[test]
    fn assign_add_limits_out_of_bounds() {
        let mut p_min = PointI32::of(i32::MIN + 1, i32::MIN + 1);
        assign_add(&mut p_min, &PointI32::min());
        assert_eq!(p_min, PointI32::min());

        let mut p_max = PointI32::of(i32::MAX - 1, i32::MAX - 1);
        assign_add(&mut p_max, &PointI32::max());
        assert_eq!(p_max, PointI32::max());
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
        assert_eq!(add(&PointI32::of(i32::MIN + 2, i32::MIN + 5), &PointI32::of(-10, -10)), PointI32::min());
        assert_eq!(add(&PointI32::of(i32::MAX - 2, i32::MAX - 5), &PointI32::of(10, 10)), PointI32::max());
    }

    #[test]
    fn add_limits_out_of_bounds() {
        assert_eq!(add(&PointI32::of(i32::MIN + 1, i32::MIN + 1), &PointI32::min()), PointI32::min());
        assert_eq!(add(&PointI32::of(i32::MAX - 1, i32::MAX - 1), &PointI32::max()), PointI32::max());
    }
}

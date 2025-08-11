use super::PointI16;

pub fn assign_add(p: &mut PointI16, delta: &PointI16) {
    p.row = p.row.saturating_add(delta.row);
    p.col = p.col.saturating_add(delta.col);
}

pub fn add(p: &PointI16, delta: &PointI16) -> PointI16 {
    let row = p.row.saturating_add(delta.row);
    let col = p.col.saturating_add(delta.col);
    PointI16 { row, col }
}

#[cfg(test)]
mod tests {
    use crate::matrix::point::point_i16::PointI16;

    use super::{add, assign_add};

    #[test]
    fn test_assign_add() {
        let mut p = PointI16::of(0, 0);
        assign_add(&mut p, &PointI16::of(10, 13));
        assert_eq!(p, PointI16::of(10, 13));
        assign_add(&mut p, &PointI16::of(-5, -3));
        assert_eq!(p, PointI16::of(5, 10));
    }

    #[test]
    fn assign_add_to_bounds() {
        let mut p_min = PointI16::of(i16::MIN + 2, i16::MIN + 5);
        assign_add(&mut p_min, &PointI16::of(-2, -5));
        assert_eq!(p_min, PointI16::min());

        let mut p_max = PointI16::of(i16::MAX - 2, i16::MAX - 5);
        assign_add(&mut p_max, &PointI16::of(2, 5));
        assert_eq!(p_max, PointI16::max());
    }

    #[test]
    fn assign_add_out_of_bounds() {
        let mut p_min = PointI16::of(i16::MIN + 2, i16::MIN + 5);
        assign_add(&mut p_min, &PointI16::of(-10, -10));
        assert_eq!(p_min, PointI16::min());

        let mut p_max = PointI16::of(i16::MAX - 2, i16::MAX - 5);
        assign_add(&mut p_max, &PointI16::of(10, 10));
        assert_eq!(p_max, PointI16::max());
    }

    #[test]
    fn assign_add_limits_out_of_bounds_out_of_bounds() {
        let mut p_min = PointI16::of(i16::MIN + 1, i16::MIN + 1);
        assign_add(&mut p_min, &PointI16::min());
        assert_eq!(p_min, PointI16::min());

        let mut p_max = PointI16::of(i16::MAX - 1, i16::MAX - 1);
        assign_add(&mut p_max, &PointI16::max());
        assert_eq!(p_max, PointI16::max());
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&PointI16::of(0, 0), &PointI16::of(10, 13)), PointI16::of(10, 13));
        assert_eq!(add(&PointI16::of(10, 10), &PointI16::of(-5, -3)), PointI16::of(5, 7));
    }

    #[test]
    fn add_to_bounds() {
        assert_eq!(add(&PointI16::of(i16::MIN + 2, i16::MIN + 5), &PointI16::of(-2, -5)), PointI16::min());
        assert_eq!(add(&PointI16::of(i16::MAX - 2, i16::MAX - 5), &PointI16::of(2, 5)), PointI16::max());
    }

    #[test]
    fn add_out_of_bounds() {
        assert_eq!(add(&PointI16::of(i16::MIN + 2, i16::MIN + 5), &PointI16::of(-10, -10)), PointI16::min());
        assert_eq!(add(&PointI16::of(i16::MAX - 2, i16::MAX - 5), &PointI16::of(10, 10)), PointI16::max());
    }

    #[test]
    fn add_limits_out_of_bounds() {
        assert_eq!(add(&PointI16::of(i16::MIN + 1, i16::MIN + 1), &PointI16::min()), PointI16::min());
        assert_eq!(add(&PointI16::of(i16::MAX - 1, i16::MAX - 1), &PointI16::max()), PointI16::max());
    }
}

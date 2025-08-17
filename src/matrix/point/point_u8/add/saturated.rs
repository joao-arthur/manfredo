use crate::matrix::point::{point_i8::PointI8, point_u8::PointU8};

pub fn assign_add(p: &mut PointU8, delta: &PointI8) {
    let temp_row = i16::from(p.row) + i16::from(delta.row);
    let temp_col = i16::from(p.col) + i16::from(delta.col);
    p.row = temp_row.clamp(0, i16::from(u8::MAX)) as u8;
    p.col = temp_col.clamp(0, i16::from(u8::MAX)) as u8;
}

pub fn add(p: &PointU8, delta: &PointI8) -> PointU8 {
    let temp_row = i16::from(p.row) + i16::from(delta.row);
    let temp_col = i16::from(p.col) + i16::from(delta.col);
    let row = temp_row.clamp(0, i16::from(u8::MAX)) as u8;
    let col = temp_col.clamp(0, i16::from(u8::MAX)) as u8;
    PointU8 { row, col }
}

#[cfg(test)]
mod tests {
    use crate::matrix::point::{point_i8::PointI8, point_u8::PointU8};

    use super::{add, assign_add};

    #[test]
    fn test_assign_add() {
        let mut p = PointU8::of(0, 0);
        assign_add(&mut p, &PointI8::of(10, 13));
        assert_eq!(p, PointU8::of(10, 13));
        assign_add(&mut p, &PointI8::of(-5, -3));
        assert_eq!(p, PointU8::of(5, 10));
    }

    #[test]
    fn assign_add_to_bounds() {
        let mut p_min = PointU8::of(2, 5);
        assign_add(&mut p_min, &PointI8::of(-2, -5));
        assert_eq!(p_min, PointU8::min());

        let mut p_max = PointU8::of(u8::MAX - 2, u8::MAX - 5);
        assign_add(&mut p_max, &PointI8::of(2, 5));
        assert_eq!(p_max, PointU8::max());
    }

    #[test]
    fn assign_add_out_of_bounds() {
        let mut p_min = PointU8::of(2, 5);
        assign_add(&mut p_min, &PointI8::of(-10, -10));
        assert_eq!(p_min, PointU8::min());

        let mut p_max = PointU8::of(u8::MAX - 2, u8::MAX - 5);
        assign_add(&mut p_max, &PointI8::of(10, 10));
        assert_eq!(p_max, PointU8::max());
    }

    #[test]
    fn assign_add_limits_out_of_bounds() {
        let mut p_min = PointU8::of(1, 1);
        assign_add(&mut p_min, &PointI8::min());
        assert_eq!(p_min, PointU8::min());

        let mut p_max = PointU8::of(u8::MAX - 1, u8::MAX - 1);
        assign_add(&mut p_max, &PointI8::max());
        assert_eq!(p_max, PointU8::max());
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&PointU8::of(0, 0), &PointI8::of(10, 13)), PointU8::of(10, 13));
        assert_eq!(add(&PointU8::of(10, 10), &PointI8::of(-5, -3)), PointU8::of(5, 7));
    }

    #[test]
    fn add_to_bounds() {
        assert_eq!(add(&PointU8::of(2, 5), &PointI8::of(-2, -5)), PointU8::min());
        assert_eq!(add(&PointU8::of(u8::MAX - 2, u8::MAX - 5), &PointI8::of(2, 5)), PointU8::max());
    }

    #[test]
    fn add_out_of_bounds() {
        assert_eq!(add(&PointU8::of(2, 5), &PointI8::of(-10, -10)), PointU8::min());
        assert_eq!(add(&PointU8::of(u8::MAX - 2, u8::MAX - 5), &PointI8::of(10, 10)), PointU8::max());
    }

    #[test]
    fn add_limits_out_of_bounds() {
        assert_eq!(add(&PointU8::of(1, 1), &PointI8::min()), PointU8::min());
        assert_eq!(add(&PointU8::of(u8::MAX - 1, u8::MAX - 1), &PointI8::max()), PointU8::max());
    }
}

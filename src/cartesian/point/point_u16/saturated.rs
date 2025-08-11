use crate::cartesian::point::point_i16::PointI16;

use super::PointU16;

pub fn assign_add(p: &mut PointU16, delta: &PointI16) {
    let temp_x = i32::from(p.x) + i32::from(delta.x);
    let temp_y = i32::from(p.y) + i32::from(delta.y);
    p.x = temp_x.clamp(0, i32::from(u16::MAX)) as u16;
    p.y = temp_y.clamp(0, i32::from(u16::MAX)) as u16;
}

pub fn add(p: &PointU16, delta: &PointI16) -> PointU16 {
    let temp_x = i32::from(p.x) + i32::from(delta.x);
    let temp_y = i32::from(p.y) + i32::from(delta.y);
    let x = temp_x.clamp(0, i32::from(u16::MAX)) as u16;
    let y = temp_y.clamp(0, i32::from(u16::MAX)) as u16;
    PointU16 { x, y }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::{point_i16::PointI16, point_u16::PointU16};

    use super::{add, assign_add};

    #[test]
    fn test_assign_add() {
        let mut p = PointU16::of(0, 0);
        assign_add(&mut p, &PointI16::of(10, 13));
        assert_eq!(p, PointU16::of(10, 13));
        assign_add(&mut p, &PointI16::of(-5, -3));
        assert_eq!(p, PointU16::of(5, 10));
    }

    #[test]
    fn assign_add_to_bounds() {
        let mut p_min = PointU16::of(2, 5);
        assign_add(&mut p_min, &PointI16::of(-2, -5));
        assert_eq!(p_min, PointU16::min());

        let mut p_max = PointU16::of(u16::MAX - 2, u16::MAX - 5);
        assign_add(&mut p_max, &PointI16::of(2, 5));
        assert_eq!(p_max, PointU16::max());
    }

    #[test]
    fn assign_add_out_of_bounds() {
        let mut p_min = PointU16::of(2, 5);
        assign_add(&mut p_min, &PointI16::of(-10, -10));
        assert_eq!(p_min, PointU16::min());

        let mut p_max = PointU16::of(u16::MAX - 2, u16::MAX - 5);
        assign_add(&mut p_max, &PointI16::of(10, 10));
        assert_eq!(p_max, PointU16::max());
    }

    #[test]
    fn assign_add_limits_out_of_bounds_out_of_bounds() {
        let mut p_min = PointU16::of(1, 1);
        assign_add(&mut p_min, &PointI16::min());
        assert_eq!(p_min, PointU16::min());

        let mut p_max = PointU16::of(u16::MAX - 1, u16::MAX - 1);
        assign_add(&mut p_max, &PointI16::max());
        assert_eq!(p_max, PointU16::max());
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&PointU16::of(0, 0), &PointI16::of(10, 13)), PointU16::of(10, 13));
        assert_eq!(add(&PointU16::of(10, 10), &PointI16::of(-5, -3)), PointU16::of(5, 7));
    }

    #[test]
    fn add_to_bounds() {
        assert_eq!(add(&PointU16::of(2, 5), &PointI16::of(-2, -5)), PointU16::min());
        assert_eq!(add(&PointU16::of(u16::MAX - 2, u16::MAX - 5), &PointI16::of(2, 5)), PointU16::max());
    }

    #[test]
    fn add_out_of_bounds() {
        assert_eq!(add(&PointU16::of(2, 5), &PointI16::of(-10, -10)), PointU16::min());
        assert_eq!(add(&PointU16::of(u16::MAX - 2, u16::MAX - 5), &PointI16::of(10, 10)), PointU16::max());
    }

    #[test]
    fn add_limits_out_of_bounds() {
        assert_eq!(add(&PointU16::of(1, 1), &PointI16::min()), PointU16::min());
        assert_eq!(add(&PointU16::of(u16::MAX - 1, u16::MAX - 1), &PointI16::max()), PointU16::max());
    }
}

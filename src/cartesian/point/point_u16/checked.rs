use crate::cartesian::point::point_i16::PointI16;

use super::PointU16;

pub fn try_assign_add(p: &mut PointU16, delta: &PointI16) -> Option<()> {
    let x = u16::try_from(i32::from(p.x) + i32::from(delta.x)).ok()?;
    let y = u16::try_from(i32::from(p.y) + i32::from(delta.y)).ok()?;
    p.x = x;
    p.y = y;
    Some(())
}

pub fn try_add(p: &PointU16, delta: &PointI16) -> Option<PointU16> {
    let x = u16::try_from(i32::from(p.x) + i32::from(delta.x)).ok()?;
    let y = u16::try_from(i32::from(p.y) + i32::from(delta.y)).ok()?;
    Some(PointU16 { x, y })
}

pub fn assign_add(p: &mut PointU16, delta: &PointI16) {
    try_assign_add(p, delta).unwrap()
}

pub fn add(p: &PointU16, delta: &PointI16) -> PointU16 {
    try_add(p, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::{point_i16::PointI16, point_u16::PointU16};

    use super::{add, assign_add, try_add, try_assign_add};

    #[test]
    fn test_try_assign_add() {
        let mut p = PointU16::of(0, 0);
        assert_eq!(try_assign_add(&mut p, &PointI16::of(10, 13)), Some(()));
        assert_eq!(p, PointU16::of(10, 13));
        assert_eq!(try_assign_add(&mut p, &PointI16::of(-5, -3)), Some(()));
        assert_eq!(p, PointU16::of(5, 10));
    }

    #[test]
    fn try_assign_add_to_bounds() {
        let mut p_min = PointU16::of(2, 5);
        assert_eq!(try_assign_add(&mut p_min, &PointI16::of(-2, -5)), Some(()));
        assert_eq!(p_min, PointU16::min());

        let mut p_max = PointU16::of(u16::MAX - 2, u16::MAX - 5);
        assert_eq!(try_assign_add(&mut p_max, &PointI16::of(2, 5)), Some(()));
        assert_eq!(p_max, PointU16::max());
    }

    #[test]
    fn try_assign_add_out_of_bounds() {
        let mut p_min = PointU16::of(2, 5);
        assert_eq!(try_assign_add(&mut p_min, &PointI16::of(-10, 0)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointI16::of(0, -10)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointI16::of(-10, -10)), None);
        assert_eq!(p_min, PointU16::of(2, 5));

        let mut p_max = PointU16::of(u16::MAX - 2, u16::MAX - 5);
        assert_eq!(try_assign_add(&mut p_max, &PointI16::of(10, 0)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointI16::of(0, 10)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointI16::of(10, 10)), None);
        assert_eq!(p_max, PointU16::of(u16::MAX - 2, u16::MAX - 5));
    }

    #[test]
    fn try_assign_add_limits_out_of_bounds_out_of_bounds() {
        let mut p_min = PointU16::of(1, 1);
        assert_eq!(try_assign_add(&mut p_min, &PointI16::of(i16::MIN, 0)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointI16::of(0, i16::MIN)), None);
        assert_eq!(try_assign_add(&mut p_min, &PointI16::min()), None);
        assert_eq!(p_min, PointU16::of(1, 1));

        let mut p_max = PointU16::of(u16::MAX - 1, u16::MAX - 1);
        assert_eq!(try_assign_add(&mut p_max, &PointI16::of(i16::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointI16::of(0, i16::MAX)), None);
        assert_eq!(try_assign_add(&mut p_max, &PointI16::max()), None);
        assert_eq!(p_max, PointU16::of(u16::MAX - 1, u16::MAX - 1));
    }

    #[test]
    fn test_try_add() {
        assert_eq!(try_add(&PointU16::of(0, 0), &PointI16::of(10, 13)), Some(PointU16::of(10, 13)));
        assert_eq!(try_add(&PointU16::of(10, 10), &PointI16::of(-5, -3)), Some(PointU16::of(5, 7)));
    }

    #[test]
    fn try_add_to_bounds() {
        assert_eq!(try_add(&PointU16::of(2, 5), &PointI16::of(-2, -5)), Some(PointU16::min()));
        assert_eq!(try_add(&PointU16::of(u16::MAX - 2, u16::MAX - 5), &PointI16::of(2, 5)), Some(PointU16::max()));
    }

    #[test]
    fn try_add_out_of_bounds() {
        let p_min = PointU16::of(2, 5);
        assert_eq!(try_add(&p_min, &PointI16::of(-10, 0)), None);
        assert_eq!(try_add(&p_min, &PointI16::of(0, -10)), None);
        assert_eq!(try_add(&p_min, &PointI16::of(-10, -10)), None);

        let m_max = PointU16::of(u16::MAX - 2, u16::MAX - 5);
        assert_eq!(try_add(&m_max, &PointI16::of(10, 0)), None);
        assert_eq!(try_add(&m_max, &PointI16::of(0, 10)), None);
        assert_eq!(try_add(&m_max, &PointI16::of(10, 10)), None);
    }

    #[test]
    fn try_add_limits_out_of_bounds_out_of_bounds() {
        let p_min = PointU16::of(1, 1);
        assert_eq!(try_add(&p_min, &PointI16::of(i16::MIN, 0)), None);
        assert_eq!(try_add(&p_min, &PointI16::of(0, i16::MIN)), None);
        assert_eq!(try_add(&p_min, &PointI16::min()), None);

        let p_max = PointU16::of(u16::MAX - 1, u16::MAX - 1);
        assert_eq!(try_add(&p_max, &PointI16::of(i16::MAX, 0)), None);
        assert_eq!(try_add(&p_max, &PointI16::of(0, i16::MAX)), None);
        assert_eq!(try_add(&p_max, &PointI16::max()), None);
    }

    #[test]
    fn test_assign_add() {
        let mut p = PointU16::of(0, 0);
        assign_add(&mut p, &PointI16::of(10, 13));
        assert_eq!(p, PointU16::of(10, 13));
        assign_add(&mut p, &PointI16::of(-5, -3));
        assert_eq!(p, PointU16::of(5, 10));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&PointU16::of(0, 0), &PointI16::of(10, 13)), PointU16::of(10, 13));
        assert_eq!(add(&PointU16::of(10, 13), &PointI16::of(-5, -3)), PointU16::of(5, 10));
    }
}

use super::PointU8;
use crate::cartesian::point::point_i8::PointI8;

pub fn assign_add(p: &mut PointU8, delta: &PointI8) {
    p.x = p.x.wrapping_add(delta.x as i8 as u8);
    p.y = p.y.wrapping_add(delta.y as i8 as u8);
}

pub fn add(p: &PointU8, delta: &PointI8) -> PointU8 {
    PointU8::of(p.x.wrapping_add(delta.x as i8 as u8), p.y.wrapping_add(delta.y as i8 as u8))
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_i8::PointI8;

    use super::{PointU8, add, assign_add};

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

        let mut m_max = PointU8::of(u8::MAX - 2, u8::MAX - 5);
        assign_add(&mut m_max, &PointI8::of(2, 5));
        assert_eq!(m_max, PointU8::max());
    }

    #[test]
    fn assign_add_beyond_bounds() {
        let mut p_min = PointU8::of(2, 5);
        assign_add(&mut p_min, &PointI8::of(-10, -10));
        assert_eq!(p_min, PointU8::of(u8::MAX - 7, u8::MAX - 4));
 
        let mut m_max = PointU8::of(u8::MAX - 2, u8::MAX - 5);
        assign_add(&mut m_max, &PointI8::of(10, 10));
        assert_eq!(m_max, PointU8::of(7, 4));
    }

    #[test]
    fn assign_add_limits() {
        let mut p_min = PointU8::of(1, 1);
        assign_add(&mut p_min, &PointI8::min());
        assert_eq!(p_min, PointU8::of(129, 129));

        let mut m_max = PointU8::of(u8::MAX - 1, u8::MAX - 1);
        assign_add(&mut m_max, &PointI8::max());
        assert_eq!(m_max, PointU8::of(125, 125));
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
    fn add_beyond_bounds() {
        assert_eq!(add(&PointU8::of(2, 5), &PointI8::of(-10, -10)), PointU8::of(u8::MAX - 7, u8::MAX - 4));
        assert_eq!(add(&PointU8::of(u8::MAX - 2, u8::MAX - 5), &PointI8::of(10, 10)), PointU8::of(7, 4));
    }

    #[test]
    fn add_limits() {
        assert_eq!(add(&PointU8::of(1, 1), &PointI8::min()), PointU8::of(129, 129));
        assert_eq!(add(&PointU8::of(u8::MAX - 1, u8::MAX - 1), &PointI8::max()), PointU8::of(125, 125));
    }
}

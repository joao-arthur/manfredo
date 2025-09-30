use crate::cartesian::d2::{point::point_u32, rect::rect_u32::Rect};

pub fn delta_x(r: &Rect) -> u32 {
    point_u32::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &Rect) -> u32 {
    point_u32::delta_y(&r.min, &r.max)
}

pub fn max_delta(r: &Rect) -> u32 {
    std::cmp::max(delta_x(r), delta_y(r))
}

#[cfg(test)]
mod tests {
    use super::{delta_x, delta_y, max_delta};
    use crate::cartesian::d2::{point::point_u32, rect::rect_u32::Rect};

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Rect::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_x(&Rect::of(0, 0, u32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&Rect::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_y(&Rect::of(0, 0, 0, u32::MAX)), u32::MAX);
    }

    #[test]
    fn test_max_delta() {
        assert_eq!(max_delta(&Rect::of(0, 5, 10, 10)), 10);
        assert_eq!(max_delta(&Rect::of(5, 0, 9, 9)), 9);
    }

    #[test]
    fn max_delta_0() {
        assert_eq!(max_delta(&Rect::of(0, 0, 0, 0)), 0);
        assert_eq!(max_delta(&Rect::of(1, 1, 1, 1)), 0);
        assert_eq!(max_delta(&Rect::of(5, 10, 5, 10)), 0);
    }

    #[test]
    fn max_delta_1() {
        assert_eq!(max_delta(&Rect::of(0, 0, 1, 1)), 1);
        assert_eq!(max_delta(&Rect::of(5, 5, 6, 6)), 1);
        assert_eq!(max_delta(&Rect::of(0, 0, 0, 1)), 1);
        assert_eq!(max_delta(&Rect::of(5, 9, 5, 10)), 1);
    }

    #[test]
    fn max_delta_bounds() {
        assert_eq!(max_delta(&Rect::of(0, 0, u32::MAX, u32::MAX - 1)), u32::MAX);
        assert_eq!(max_delta(&Rect::of(0, 0, u32::MAX - 1, u32::MAX)), u32::MAX);
    }
}

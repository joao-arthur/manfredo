use crate::matrix::d2::{point::point_u16, rect::rect_u16::Rect};

pub fn delta_row(r: &Rect) -> u16 {
    point_u16::delta_row(&r.min, &r.max)
}

pub fn delta_col(r: &Rect) -> u16 {
    point_u16::delta_col(&r.min, &r.max)
}

pub fn max_delta(r: &Rect) -> u16 {
    std::cmp::max(delta_row(r), delta_col(r))
}

#[cfg(test)]
mod tests {
    use super::{delta_col, delta_row, max_delta};
    use crate::matrix::d2::rect::rect_u16::Rect;

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&Rect::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_row(&Rect::of(0, 0, u16::MAX, 0)), u16::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&Rect::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_col(&Rect::of(0, 0, 0, u16::MAX)), u16::MAX);
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
        assert_eq!(max_delta(&Rect::of(0, 0, u16::MAX, u16::MAX - 1)), u16::MAX);
        assert_eq!(max_delta(&Rect::of(0, 0, u16::MAX - 1, u16::MAX)), u16::MAX);
    }
}

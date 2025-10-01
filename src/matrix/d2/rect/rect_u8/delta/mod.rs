use crate::matrix::d2::{point::point_u8, rect::rect_u8::Rect};

pub fn delta_row(r: &Rect) -> u8 {
    point_u8::delta_row(&r.min, &r.max)
}

pub fn delta_col(r: &Rect) -> u8 {
    point_u8::delta_col(&r.min, &r.max)
}

pub fn delta_max(r: &Rect) -> u8 {
    std::cmp::max(delta_row(r), delta_col(r))
}

#[cfg(test)]
mod tests {
    use super::{delta_col, delta_max, delta_row};
    use crate::matrix::d2::rect::rect_u8::Rect;

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&Rect::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_row(&Rect::of(0, 0, u8::MAX, 0)), u8::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&Rect::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_col(&Rect::of(0, 0, 0, u8::MAX)), u8::MAX);
    }

    #[test]
    fn test_delta_max() {
        assert_eq!(delta_max(&Rect::of(0, 5, 10, 10)), 10);
        assert_eq!(delta_max(&Rect::of(5, 0, 9, 9)), 9);
    }

    #[test]
    fn delta_max_0() {
        assert_eq!(delta_max(&Rect::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_max(&Rect::of(1, 1, 1, 1)), 0);
        assert_eq!(delta_max(&Rect::of(5, 10, 5, 10)), 0);
    }

    #[test]
    fn delta_max_1() {
        assert_eq!(delta_max(&Rect::of(0, 0, 1, 1)), 1);
        assert_eq!(delta_max(&Rect::of(5, 5, 6, 6)), 1);
        assert_eq!(delta_max(&Rect::of(0, 0, 0, 1)), 1);
        assert_eq!(delta_max(&Rect::of(5, 9, 5, 10)), 1);
    }

    #[test]
    fn delta_max_bounds() {
        assert_eq!(delta_max(&Rect::of(0, 0, u8::MAX, u8::MAX - 1)), u8::MAX);
        assert_eq!(delta_max(&Rect::of(0, 0, u8::MAX - 1, u8::MAX)), u8::MAX);
    }
}

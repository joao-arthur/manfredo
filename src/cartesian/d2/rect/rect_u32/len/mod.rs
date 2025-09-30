use crate::cartesian::d2::rect::rect_u32::{Rect, delta_x, delta_y};

pub fn len_x(r: &Rect) -> u32 {
    delta_x(r) + 1
}

pub fn len_y(r: &Rect) -> u32 {
    delta_y(r) + 1
}

pub fn max_len(r: &Rect) -> u32 {
    std::cmp::max(len_x(r), len_y(r))
}

#[cfg(test)]
mod tests {
    use super::{len_x, len_y, max_len};
    use crate::cartesian::d2::rect::rect_u32::Rect;

    #[test]
    fn test_len_x() {
        assert_eq!(len_x(&Rect::of(0, 0, 0, 0)), 1);
        assert_eq!(len_x(&Rect::of(0, 0, u32::MAX - 1, 0)), u32::MAX);
    }

    #[test]
    fn test_len_y() {
        assert_eq!(len_y(&Rect::of(0, 0, 0, 0)), 1);
        assert_eq!(len_y(&Rect::of(0, 0, 0, u32::MAX - 1)), u32::MAX);
    }

    #[test]
    fn test_max_len() {
        assert_eq!(max_len(&Rect::of(0, 5, 10, 10)), 11);
        assert_eq!(max_len(&Rect::of(5, 0, 9, 9)), 10);
    }

    #[test]
    fn max_len_1() {
        assert_eq!(max_len(&Rect::of(0, 0, 0, 0)), 1);
        assert_eq!(max_len(&Rect::of(1, 1, 1, 1)), 1);
        assert_eq!(max_len(&Rect::of(5, 10, 5, 10)), 1);
    }

    #[test]
    fn max_len_2() {
        assert_eq!(max_len(&Rect::of(0, 0, 1, 1)), 2);
        assert_eq!(max_len(&Rect::of(5, 5, 6, 6)), 2);
        assert_eq!(max_len(&Rect::of(0, 0, 0, 1)), 2);
        assert_eq!(max_len(&Rect::of(5, 9, 5, 10)), 2);
    }

    #[test]
    fn max_len_bounds() {
        assert_eq!(max_len(&Rect::of(1, 0, u32::MAX - 1, u32::MAX - 1)), u32::MAX);
        assert_eq!(max_len(&Rect::of(0, 1, u32::MAX - 1, u32::MAX - 1)), u32::MAX);
        assert_eq!(max_len(&Rect::of(0, 0, u32::MAX - 2, u32::MAX - 1)), u32::MAX);
        assert_eq!(max_len(&Rect::of(0, 0, u32::MAX - 1, u32::MAX - 2)), u32::MAX);
    }
}

use crate::matrix::d2::{
    point::point_u8,
    rect::rect_u8::{Rect, delta_col, delta_row},
};

pub fn len_row(r: &Rect) -> u8 {
    delta_row(r) + 1
}

pub fn len_col(r: &Rect) -> u8 {
    delta_col(r) + 1
}

pub fn max_len(r: &Rect) -> u8 {
    std::cmp::max(len_row(r), len_col(r))
}

#[cfg(test)]
mod tests {
    use super::{len_col, len_row, max_len};
    use crate::matrix::d2::rect::rect_u8::Rect;

    #[test]
    fn test_len_row() {
        assert_eq!(len_row(&Rect::of(0, 0, 0, 0)), 1);
        assert_eq!(len_row(&Rect::of(0, 0, u8::MAX - 1, 0)), u8::MAX);
    }

    #[test]
    fn test_len_col() {
        assert_eq!(len_col(&Rect::of(0, 0, 0, 0)), 1);
        assert_eq!(len_col(&Rect::of(0, 0, 0, u8::MAX - 1)), u8::MAX);
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
        assert_eq!(max_len(&Rect::of(1, 0, u8::MAX - 1, u8::MAX - 1)), u8::MAX);
        assert_eq!(max_len(&Rect::of(0, 1, u8::MAX - 1, u8::MAX - 1)), u8::MAX);
        assert_eq!(max_len(&Rect::of(0, 0, u8::MAX - 2, u8::MAX - 1)), u8::MAX);
        assert_eq!(max_len(&Rect::of(0, 0, u8::MAX - 1, u8::MAX - 2)), u8::MAX);
    }
}

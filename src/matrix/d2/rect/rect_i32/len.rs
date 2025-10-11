use super::{Rect, delta_col, delta_row};

pub fn len_row(r: &Rect) -> u32 {
    delta_row(r) + 1
}

pub fn len_col(r: &Rect) -> u32 {
    delta_col(r) + 1
}

pub fn len_max(r: &Rect) -> u32 {
    std::cmp::max(len_row(r), len_col(r))
}

#[cfg(test)]
mod tests {
    use super::{len_col, len_max, len_row};
    use crate::matrix::d2::rect::rect_i32::Rect;

    #[test]
    fn test_len_row() {
        assert_eq!(len_row(&Rect::of(0, i32::MIN, 0, i32::MAX)), 1);
        assert_eq!(len_row(&Rect::of(i32::MIN, 0, i32::MAX - 1, 0)), u32::MAX);
    }

    #[test]
    fn test_len_col() {
        assert_eq!(len_col(&Rect::of(i32::MIN, 0, i32::MAX, 0)), 1);
        assert_eq!(len_col(&Rect::of(0, i32::MIN, 0, i32::MAX - 1)), u32::MAX);
    }

    #[test]
    fn test_len_max() {
        assert_eq!(len_max(&Rect::of(0, 5, 10, 10)), 11);
        assert_eq!(len_max(&Rect::of(-10, -10, -5, 0)), 11);
        assert_eq!(len_max(&Rect::of(-5, 0, 5, 5)), 11);
    }

    #[test]
    fn len_max_1() {
        assert_eq!(len_max(&Rect::of(0, 0, 0, 0)), 1);
        assert_eq!(len_max(&Rect::of(1, 1, 1, 1)), 1);
        assert_eq!(len_max(&Rect::of(-1, -1, -1, -1)), 1);
        assert_eq!(len_max(&Rect::of(5, 10, 5, 10)), 1);
    }

    #[test]
    fn len_max_2() {
        assert_eq!(len_max(&Rect::of(0, 0, 1, 1)), 2);
        assert_eq!(len_max(&Rect::of(5, 5, 6, 6)), 2);
        assert_eq!(len_max(&Rect::of(-6, -6, -5, -5)), 2);
        assert_eq!(len_max(&Rect::of(0, 0, 0, 1)), 2);
        assert_eq!(len_max(&Rect::of(5, 9, 5, 10)), 2);
    }

    #[test]
    fn len_max_bounds() {
        assert_eq!(len_max(&Rect::of(i32::MIN + 1, i32::MIN, i32::MAX - 1, i32::MAX - 1)), u32::MAX);
        assert_eq!(len_max(&Rect::of(i32::MIN, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1)), u32::MAX);
        assert_eq!(len_max(&Rect::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 1)), u32::MAX);
        assert_eq!(len_max(&Rect::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 2)), u32::MAX);
    }
}

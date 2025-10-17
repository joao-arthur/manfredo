use super::{Rect, delta_x, delta_y};

pub fn len_x(r: &Rect) -> u8 {
    delta_x(r) + 1
}

pub fn len_y(r: &Rect) -> u8 {
    delta_y(r) + 1
}

pub fn len_max(r: &Rect) -> u8 {
    len_x(r).max(len_y(r))
}

#[cfg(test)]
mod tests {
    use super::{len_max, len_x, len_y};
    use crate::cartesian::d2::rect::rect_i8::Rect;

    #[test]
    fn test_len_x() {
        assert_eq!(len_x(&Rect::of(0, i8::MIN, 0, i8::MAX)), 1);
        assert_eq!(len_x(&Rect::of(i8::MIN, 0, i8::MAX - 1, 0)), u8::MAX);
    }

    #[test]
    fn test_len_y() {
        assert_eq!(len_y(&Rect::of(i8::MIN, 0, i8::MAX, 0)), 1);
        assert_eq!(len_y(&Rect::of(0, i8::MIN, 0, i8::MAX - 1)), u8::MAX);
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
        assert_eq!(len_max(&Rect::of(i8::MIN + 1, i8::MIN, i8::MAX - 1, i8::MAX - 1)), u8::MAX);
        assert_eq!(len_max(&Rect::of(i8::MIN, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1)), u8::MAX);
        assert_eq!(len_max(&Rect::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 1)), u8::MAX);
        assert_eq!(len_max(&Rect::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 2)), u8::MAX);
    }
}

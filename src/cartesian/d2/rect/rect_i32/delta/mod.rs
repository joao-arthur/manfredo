use crate::cartesian::d2::{point::point_i32, rect::rect_i32::Rect};

pub fn delta_x(r: &Rect) -> u32 {
    point_i32::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &Rect) -> u32 {
    point_i32::delta_y(&r.min, &r.max)
}

pub fn delta_max(r: &Rect) -> u32 {
    std::cmp::max(delta_x(r), delta_y(r))
}

#[cfg(test)]
mod tests {
    use super::{delta_max, delta_x, delta_y};
    use crate::cartesian::d2::rect::rect_i32::Rect;

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Rect::of(0, i32::MIN, 0, i32::MAX)), 0);
        assert_eq!(delta_x(&Rect::of(i32::MIN, 0, i32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&Rect::of(i32::MIN, 0, i32::MAX, 0)), 0);
        assert_eq!(delta_y(&Rect::of(0, i32::MIN, 0, i32::MAX)), u32::MAX);
    }

    #[test]
    fn test_delta_max() {
        assert_eq!(delta_max(&Rect::of(0, 5, 10, 10)), 10);
        assert_eq!(delta_max(&Rect::of(-10, -10, -5, 0)), 10);
        assert_eq!(delta_max(&Rect::of(-5, 0, 5, 5)), 10);
    }

    #[test]
    fn delta_max_0() {
        assert_eq!(delta_max(&Rect::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_max(&Rect::of(1, 1, 1, 1)), 0);
        assert_eq!(delta_max(&Rect::of(-1, -1, -1, -1)), 0);
        assert_eq!(delta_max(&Rect::of(5, 10, 5, 10)), 0);
    }

    #[test]
    fn delta_max_1() {
        assert_eq!(delta_max(&Rect::of(0, 0, 1, 1)), 1);
        assert_eq!(delta_max(&Rect::of(5, 5, 6, 6)), 1);
        assert_eq!(delta_max(&Rect::of(-6, -6, -5, -5)), 1);
        assert_eq!(delta_max(&Rect::of(0, 0, 0, 1)), 1);
        assert_eq!(delta_max(&Rect::of(5, 9, 5, 10)), 1);
    }

    #[test]
    fn delta_max_bounds() {
        assert_eq!(delta_max(&Rect::of(i32::MIN + 1, i32::MIN, i32::MAX, i32::MAX)), u32::MAX);
        assert_eq!(delta_max(&Rect::of(i32::MIN, i32::MIN + 1, i32::MAX, i32::MAX)), u32::MAX);
        assert_eq!(delta_max(&Rect::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX)), u32::MAX);
        assert_eq!(delta_max(&Rect::of(i32::MIN, i32::MIN, i32::MAX, i32::MAX - 1)), u32::MAX);
    }
}

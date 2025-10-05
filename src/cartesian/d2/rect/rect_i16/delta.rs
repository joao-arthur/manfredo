use crate::cartesian::d2::{point::point_i16, rect::rect_i16::Rect};

pub fn delta_x(r: &Rect) -> u16 {
    point_i16::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &Rect) -> u16 {
    point_i16::delta_y(&r.min, &r.max)
}

pub fn delta_min(r: &Rect) -> u16 {
    std::cmp::min(delta_x(r), delta_y(r))
}

pub fn delta_max(r: &Rect) -> u16 {
    std::cmp::max(delta_x(r), delta_y(r))
}

#[cfg(test)]
mod tests {
    use super::{delta_max, delta_min, delta_x, delta_y};
    use crate::cartesian::d2::rect::rect_i16::Rect;

    const MIN: i16 = i16::MIN;
    const MAX: i16 = i16::MAX;

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Rect::of(0, MIN, 0, MAX)), 0);
        assert_eq!(delta_x(&Rect::of(MIN, 0, MAX, 0)), u16::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&Rect::of(MIN, 0, MAX, 0)), 0);
        assert_eq!(delta_y(&Rect::of(0, MIN, 0, MAX)), u16::MAX);
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
        assert_eq!(delta_max(&Rect::of(MIN + 1, MIN, MAX, MAX)), u16::MAX);
        assert_eq!(delta_max(&Rect::of(MIN, MIN + 1, MAX, MAX)), u16::MAX);
        assert_eq!(delta_max(&Rect::of(MIN, MIN, MAX - 1, MAX)), u16::MAX);
        assert_eq!(delta_max(&Rect::of(MIN, MIN, MAX, MAX - 1)), u16::MAX);
    }
}

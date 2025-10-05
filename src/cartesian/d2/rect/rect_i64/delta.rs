use crate::cartesian::d2::{point::point_i64, rect::rect_i64::Rect};

pub fn delta_x(r: &Rect) -> u64 {
    point_i64::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &Rect) -> u64 {
    point_i64::delta_y(&r.min, &r.max)
}

pub fn delta_min(r: &Rect) -> u64 {
    std::cmp::min(delta_x(r), delta_y(r))
}

pub fn delta_max(r: &Rect) -> u64 {
    std::cmp::max(delta_x(r), delta_y(r))
}

#[cfg(test)]
mod tests {
    use super::{delta_max, delta_min, delta_x, delta_y};
    use crate::cartesian::d2::rect::rect_i64::Rect;

    const MIN: i64 = i64::MIN;
    const MAX: i64 = i64::MAX;

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Rect::of(0, MIN, 0, MAX)), 0);
        assert_eq!(delta_x(&Rect::of(MIN, 0, MAX, 0)), u64::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&Rect::of(MIN, 0, MAX, 0)), 0);
        assert_eq!(delta_y(&Rect::of(0, MIN, 0, MAX)), u64::MAX);
    }

    #[test]
    fn test_delta_min() {
        assert_eq!(delta_min(&Rect::of(0, -5, 5, 5)), 5);
        assert_eq!(delta_min(&Rect::of(-5, 0, 4, 4)), 4);
    }

    #[test]
    fn delta_min_0() {
        assert_eq!(delta_min(&Rect::of(-1, 0, 0, 0)), 0);
        assert_eq!(delta_min(&Rect::of(0, -1, 0, 0)), 0);
        assert_eq!(delta_min(&Rect::of(0, 0, 1, 0)), 0);
        assert_eq!(delta_min(&Rect::of(0, 0, 0, 1)), 0);
    }

    #[test]
    fn delta_min_1() {
        assert_eq!(delta_min(&Rect::of(4, -5, 5, 5)), 1);
        assert_eq!(delta_min(&Rect::of(-5, 4, 5, 5)), 1);
        assert_eq!(delta_min(&Rect::of(-5, -5, -4, 5)), 1);
        assert_eq!(delta_min(&Rect::of(-5, -5, 5, -4)), 1);
    }

    #[test]
    fn delta_min_bounds() {
        assert_eq!(delta_min(&Rect::of(MIN, MIN, MAX, MAX)), u64::MAX);
    }

    #[test]
    fn test_delta_max() {
        assert_eq!(delta_max(&Rect::of(0, -5, 5, 5)), 10);
        assert_eq!(delta_max(&Rect::of(-5, 0, 4, 4)), 9);
    }

    #[test]
    fn delta_max_0() {
        assert_eq!(delta_max(&Rect::of(1, 1, 1, 1)), 0);
        assert_eq!(delta_max(&Rect::of(-1, -1, -1, -1)), 0);
        assert_eq!(delta_max(&Rect::of(5, 10, 5, 10)), 0);
    }

    #[test]
    fn delta_max_1() {
        assert_eq!(delta_max(&Rect::of(-1, 0, 0, 0)), 1);
        assert_eq!(delta_max(&Rect::of(0, -1, 0, 0)), 1);
        assert_eq!(delta_max(&Rect::of(0, 0, 1, 0)), 1);
        assert_eq!(delta_max(&Rect::of(0, 0, 0, 1)), 1);
    }

    #[test]
    fn delta_max_bounds() {
        assert_eq!(delta_max(&Rect::of(MIN + 1, MIN, MAX, MAX)), u64::MAX);
        assert_eq!(delta_max(&Rect::of(MIN, MIN + 1, MAX, MAX)), u64::MAX);
        assert_eq!(delta_max(&Rect::of(MIN, MIN, MAX - 1, MAX)), u64::MAX);
        assert_eq!(delta_max(&Rect::of(MIN, MIN, MAX, MAX - 1)), u64::MAX);
    }
}

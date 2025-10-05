use crate::cartesian::d2::{point::point_u64, rect::rect_u64::Rect};

pub fn delta_x(r: &Rect) -> u64 {
    point_u64::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &Rect) -> u64 {
    point_u64::delta_y(&r.min, &r.max)
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
    use crate::cartesian::d2::rect::rect_u64::Rect;

    const MAX : u64 = u64::MAX;

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Rect::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_x(&Rect::of(0, 0, MAX, 0)), MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&Rect::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_y(&Rect::of(0, 0, 0, MAX)), MAX);
    }

    #[test]
    fn test_delta_min() {
        assert_eq!(delta_min(&Rect::of(0, 5, 10, 10)), 5);
        assert_eq!(delta_min(&Rect::of(5, 0, 9, 9)), 4);
    }
 
    #[test]
    fn delta_min_0() {
        assert_eq!(delta_min(&Rect::of(4, 5, 5, 5)), 0);
        assert_eq!(delta_min(&Rect::of(5, 4, 5, 5)), 0);
        assert_eq!(delta_min(&Rect::of(5, 5, 6, 5)), 0);
        assert_eq!(delta_min(&Rect::of(5, 5, 5, 6)), 0);
    }

    #[test]
    fn delta_min_1() {
        assert_eq!(delta_min(&Rect::of(0, 5, 6, 6)), 1);
        assert_eq!(delta_min(&Rect::of(5, 0, 6, 6)), 1);
        assert_eq!(delta_min(&Rect::of(5, 5, 10, 6)), 1);
        assert_eq!(delta_min(&Rect::of(5, 5, 6, 10)), 1);
    }

    #[test]
    fn delta_min_bounds() {
        assert_eq!(delta_min(&Rect::of(0, 0, MAX, MAX)), MAX);
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
        assert_eq!(delta_max(&Rect::of(4, 5, 5, 5)), 1);
        assert_eq!(delta_max(&Rect::of(5, 4, 5, 5)), 1);
        assert_eq!(delta_max(&Rect::of(5, 5, 6, 5)), 1);
        assert_eq!(delta_max(&Rect::of(5, 5, 5, 6)), 1);
    }

    #[test]
    fn delta_max_bounds() {
        assert_eq!(delta_max(&Rect::of(1, 0, MAX, MAX)), MAX);
        assert_eq!(delta_max(&Rect::of(0, 1, MAX, MAX)), MAX);
        assert_eq!(delta_max(&Rect::of(0, 0, MAX - 1, MAX)), MAX);
        assert_eq!(delta_max(&Rect::of(0, 0, MAX, MAX - 1)), MAX);
    }
}

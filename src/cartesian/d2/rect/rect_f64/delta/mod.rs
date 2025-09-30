use crate::cartesian::d2::{point::point_f64, rect::rect_f64::Rect};

pub fn delta_x(r: &Rect) -> f64 {
    point_f64::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &Rect) -> f64 {
    point_f64::delta_y(&r.min, &r.max)
}

pub fn max_delta(r: &Rect) -> f64 {
    delta_x(r).max(delta_y(r))
}

#[cfg(test)]
mod tests {
    use super::{delta_x, delta_y, max_delta};
    use crate::cartesian::d2::{
        point::point_f64::{MAX, MIN},
        rect::rect_f64::Rect,
    };

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Rect::of(0.0, 0.0, 0.0, MAX)), 0.0);
        assert_eq!(delta_x(&Rect::of(0.0, -4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0)), 0.0);
        assert_eq!(delta_x(&Rect::of(0.0, 0.0, MAX, 0.0)), MAX);
        assert_eq!(delta_x(&Rect::of(-4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0, 0.0)), MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&Rect::of(0.0, 0.0, MAX, 0.0)), 0.0);
        assert_eq!(delta_y(&Rect::of(-4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0, 0.0)), 0.0);
        assert_eq!(delta_y(&Rect::of(0.0, 0.0, 0.0, MAX)), MAX);
        assert_eq!(delta_y(&Rect::of(0.0, -4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0)), MAX);
    }

    #[test]
    fn test_max_delta() {
        assert_eq!(max_delta(&Rect::of(0.0, 5.0, 10.0, 10.0)), 10.0);
        assert_eq!(max_delta(&Rect::of(-10.0, -10.0, -5.0, 0.0)), 10.0);
        assert_eq!(max_delta(&Rect::of(-5.0, 0.0, 5.0, 5.0)), 10.0);
    }

    #[test]
    fn max_delta_0() {
        assert_eq!(max_delta(&Rect::of(0.0, 0.0, 0.0, 0.0)), 0.0);
        assert_eq!(max_delta(&Rect::of(1.0, 1.0, 1.0, 1.0)), 0.0);
        assert_eq!(max_delta(&Rect::of(-1.0, -1.0, -1.0, -1.0)), 0.0);
        assert_eq!(max_delta(&Rect::of(5.0, 10.0, 5.0, 10.0)), 0.0);
    }

    #[test]
    fn max_delta_1() {
        assert_eq!(max_delta(&Rect::of(0.0, 0.0, 1.0, 1.0)), 1.0);
        assert_eq!(max_delta(&Rect::of(5.0, 5.0, 6.0, 6.0)), 1.0);
        assert_eq!(max_delta(&Rect::of(-6.0, -6.0, -5.0, -5.0)), 1.0);
        assert_eq!(max_delta(&Rect::of(0.0, 0.0, 0.0, 1.0)), 1.0);
        assert_eq!(max_delta(&Rect::of(5.0, 9.0, 5.0, 10.0)), 1.0);
    }

    #[test]
    fn max_delta_bounds() {
        assert_eq!(max_delta(&Rect::of(MIN + 1.0, MIN + 2.0, 0.0, 0.0)), MAX);
        assert_eq!(max_delta(&Rect::of(MIN + 2.0, MIN + 1.0, 0.0, 0.0)), MAX);
        assert_eq!(max_delta(&Rect::of(0.0, 0.0, MAX - 1.0, MAX)), MAX);
        assert_eq!(max_delta(&Rect::of(0.0, 0.0, MAX, MAX - 1.0)), MAX);
    }
}

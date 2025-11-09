use super::{Rect, delta_x, delta_y};

pub fn len_x(r: &Rect) -> f64 {
    delta_x(r) + 1.0
}

pub fn len_y(r: &Rect) -> f64 {
    delta_y(r) + 1.0
}

pub fn len_max(r: &Rect) -> f64 {
    len_x(r).max(len_y(r))
}

#[cfg(test)]
mod tests {
    use super::{len_max, len_x, len_y};
    use crate::cartesian::{
        d1::point::point_f64::{MAX, MIN},
        d2::rect::rect_f64::Rect,
    };

    #[test]
    fn test_len_x() {
        assert_eq!(len_x(&Rect::new((0.0, 0.0), (0.0, MAX))), 1.0);
        assert_eq!(len_x(&Rect::new((0.0, -4_503_599_627_370_496.0), (0.0, 4_503_599_627_370_495.0))), 1.0);
        assert_eq!(len_x(&Rect::new((0.0, 0.0), (MAX - 1.0, 0.0))), MAX);
        assert_eq!(len_x(&Rect::new((-4_503_599_627_370_496.0, 0.0), (4_503_599_627_370_494.0, 0.0))), MAX);
    }

    #[test]
    fn test_len_y() {
        assert_eq!(len_y(&Rect::new((0.0, 0.0), (MAX, 0.0))), 1.0);
        assert_eq!(len_y(&Rect::new((-4_503_599_627_370_496.0, 0.0), (4_503_599_627_370_495.0, 0.0))), 1.0);
        assert_eq!(len_y(&Rect::new((0.0, 0.0), (0.0, MAX - 1.0))), MAX);
        assert_eq!(len_y(&Rect::new((0.0, -4_503_599_627_370_496.0), (0.0, 4_503_599_627_370_494.0))), MAX);
    }

    #[test]
    fn test_len_max() {
        assert_eq!(len_max(&Rect::new((0.0, 5.0), (10.0, 10.0))), 11.0);
        assert_eq!(len_max(&Rect::new((-10.0, -10.0), (-5.0, 0.0))), 11.0);
        assert_eq!(len_max(&Rect::new((-5.0, 0.0), (5.0, 5.0))), 11.0);
    }

    #[test]
    fn len_max_1() {
        assert_eq!(len_max(&Rect::zero()), 1.0);
        assert_eq!(len_max(&Rect::new((1.0, 1.0), (1.0, 1.0))), 1.0);
        assert_eq!(len_max(&Rect::new((-1.0, -1.0), (-1.0, -1.0))), 1.0);
        assert_eq!(len_max(&Rect::new((5.0, 10.0), (5.0, 10.0))), 1.0);
    }

    #[test]
    fn len_max_2() {
        assert_eq!(len_max(&Rect::new((0.0, 0.0), (1.0, 1.0))), 2.0);
        assert_eq!(len_max(&Rect::new((5.0, 5.0), (6.0, 6.0))), 2.0);
        assert_eq!(len_max(&Rect::new((-6.0, -6.0), (-5.0, -5.0))), 2.0);
        assert_eq!(len_max(&Rect::new((0.0, 0.0), (0.0, 1.0))), 2.0);
        assert_eq!(len_max(&Rect::new((5.0, 9.0), (5.0, 10.0))), 2.0);
    }

    #[test]
    fn len_max_bounds() {
        assert_eq!(len_max(&Rect::new((MIN + 2.0, MIN + 3.0), (0.0, 0.0))), MAX);
        assert_eq!(len_max(&Rect::new((MIN + 3.0, MIN + 2.0), (0.0, 0.0))), MAX);
        assert_eq!(len_max(&Rect::new((0.0, 0.0), (MAX - 2.0, MAX - 1.0))), MAX);
        assert_eq!(len_max(&Rect::new((0.0, 0.0), (MAX - 1.0, MAX - 2.0))), MAX);
    }
}

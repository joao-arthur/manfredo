use crate::cartesian::point::point_i16;

#[derive(PartialEq, Debug, Clone)]
pub struct RectI16 {
    pub min: point_i16::PointI16,
    pub max: point_i16::PointI16,
}

impl RectI16 {
    pub fn of(x1: i16, y1: i16, x2: i16, y2: i16) -> Self {
        RectI16 { min: point_i16::PointI16::of(x1, y1), max: point_i16::PointI16::of(x2, y2) }
    }
}

impl std::fmt::Display for RectI16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_x(r: &RectI16) -> u16 {
    point_i16::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectI16) -> u16 {
    point_i16::delta_y(&r.min, &r.max)
}

pub fn inflate(r: &mut RectI16) {
    let is_min_x = r.min.x == i16::MIN;
    let is_max_x = r.max.x == i16::MAX;
    let is_min_y = r.min.y == i16::MIN;
    let is_max_y = r.max.y == i16::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return;
    }
    let min_x_modifier = 1 - i16::from(is_min_x) + i16::from(is_max_x);
    let max_x_modifier = 1 + i16::from(is_min_x) - i16::from(is_max_x);
    let min_y_modifier = 1 - i16::from(is_min_y) + i16::from(is_max_y);
    let max_y_modifier = 1 + i16::from(is_min_y) - i16::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
}

pub fn deflate(r: &mut RectI16) {
    if delta_x(r) < 3 || delta_y(r) < 3 {
        return;
    }
    r.min.x += 1;
    r.min.y += 1;
    r.max.x -= 1;
    r.max.y -= 1;
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_i16::PointI16;

    use super::{RectI16, deflate, delta_x, delta_y, inflate};

    #[test]
    fn rect_i16() {
        assert_eq!(RectI16::of(i16::MIN, -1, 1, i16::MAX), RectI16 { min: PointI16 { x: i16::MIN, y: -1 }, max: PointI16 { x: 1, y: i16::MAX } });
        assert_eq!(RectI16::of(i16::MIN, -0, 0, i16::MAX).to_string(), "((-32768, 0), (0, 32767))");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectI16::of(0, i16::MIN, 0, i16::MAX)), 0);
        assert_eq!(delta_x(&RectI16::of(i16::MIN, 0, i16::MAX, 0)), u16::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectI16::of(i16::MIN, 0, i16::MAX, 0)), 0);
        assert_eq!(delta_y(&RectI16::of(0, i16::MIN, 0, i16::MAX)), u16::MAX);
    }

    #[test]
    fn inflate_min_bounds() {
        let mut r = RectI16::of(-32761, -32766, 4, 13);
        inflate(&mut r);
        assert_eq!(r, RectI16::of(-32762, -32767, 5, 14));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(-32763, i16::MIN, 6, 15));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(-32764, i16::MIN, 7, 17));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(-32765, i16::MIN, 8, 19));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(-32766, i16::MIN, 9, 21));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(-32767, i16::MIN, 10, 23));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, 11, 25));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, 13, 27));
    }

    #[test]
    fn inflate_max_bounds() {
        let mut r = RectI16::of(-100, 30, 32762, 32764);
        inflate(&mut r);
        assert_eq!(r, RectI16::of(-101, 29, 32763, 32765));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(-102, 28, 32764, 32766));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(-103, 27, 32765, i16::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(-104, 25, 32766, i16::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(-105, 23, i16::MAX, i16::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(-107, 21, i16::MAX, i16::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(-109, 19, i16::MAX, i16::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(-111, 17, i16::MAX, i16::MAX));
    }

    #[test]
    fn inflate_almost_min_bounds() {
        let mut r = RectI16::of(-32767, -32767, i16::MAX, i16::MAX);
        inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MAX, i16::MAX));
    }

    #[test]
    fn inflate_almost_max_bounds() {
        let mut r = RectI16::of(i16::MIN, i16::MIN, 32766, 32766);
        inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MAX, i16::MAX));
    }

    #[test]
    fn inflate_max_width() {
        let mut r = RectI16::of(i16::MIN, 10, i16::MAX, 50);
        inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN, 10, i16::MAX, 50));
    }

    #[test]
    fn inflate_max_height() {
        let mut r = RectI16::of(10, i16::MIN, 50, i16::MAX);
        inflate(&mut r);
        assert_eq!(r, RectI16::of(10, i16::MIN, 50, i16::MAX));
    }

    #[test]
    fn deflate_odd_size() {
        let mut r = RectI16::of(-5, -5, 5, 5);
        deflate(&mut r);
        assert_eq!(r, RectI16::of(-4, -4, 4, 4));
        deflate(&mut r);
        assert_eq!(r, RectI16::of(-3, -3, 3, 3));
        deflate(&mut r);
        assert_eq!(r, RectI16::of(-2, -2, 2, 2));
        deflate(&mut r);
        assert_eq!(r, RectI16::of(-1, -1, 1, 1));
        deflate(&mut r);
        assert_eq!(r, RectI16::of(-1, -1, 1, 1));
    }

    #[test]
    fn deflate_even_size() {
        let mut r = RectI16::of(-5, -5, 4, 4);
        deflate(&mut r);
        assert_eq!(r, RectI16::of(-4, -4, 3, 3));
        deflate(&mut r);
        assert_eq!(r, RectI16::of(-3, -3, 2, 2));
        deflate(&mut r);
        assert_eq!(r, RectI16::of(-2, -2, 1, 1));
        deflate(&mut r);
        assert_eq!(r, RectI16::of(-1, -1, 0, 0));
        deflate(&mut r);
        assert_eq!(r, RectI16::of(-1, -1, 0, 0));
    }
}

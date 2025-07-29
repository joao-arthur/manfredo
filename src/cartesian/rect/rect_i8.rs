use crate::cartesian::point::point_i8;

#[derive(PartialEq, Debug, Clone)]
pub struct RectI8 {
    pub min: point_i8::PointI8,
    pub max: point_i8::PointI8,
}

impl RectI8 {
    pub fn of(x1: i8, y1: i8, x2: i8, y2: i8) -> Self {
        RectI8 { min: point_i8::PointI8::of(x1, y1), max: point_i8::PointI8::of(x2, y2) }
    }
}

impl std::fmt::Display for RectI8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_x(r: &RectI8) -> u8 {
    point_i8::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectI8) -> u8 {
    point_i8::delta_y(&r.min, &r.max)
}

pub fn max_delta(r: &RectI8) -> u8 {
    std::cmp::max(delta_x(r), delta_y(r))
}

pub fn len_x(r: &RectI8) -> u8 {
    delta_x(r) + 1
}

pub fn len_y(r: &RectI8) -> u8 {
    delta_y(r) + 1
}

pub fn inflate(r: &mut RectI8) {
    let is_min_x = r.min.x == i8::MIN;
    let is_min_y = r.min.y == i8::MIN;
    let is_max_x = r.max.x == i8::MAX;
    let is_max_y = r.max.y == i8::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return;
    }
    let min_x_modifier = 1 - i8::from(is_min_x) + i8::from(is_max_x);
    let min_y_modifier = 1 - i8::from(is_min_y) + i8::from(is_max_y);
    let max_x_modifier = 1 + i8::from(is_min_x) - i8::from(is_max_x);
    let max_y_modifier = 1 + i8::from(is_min_y) - i8::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
}

pub fn deflate(r: &mut RectI8) {
    if delta_x(r) < 3 || delta_y(r) < 3 {
        return;
    }
    r.min.x += 1;
    r.min.y += 1;
    r.max.x -= 1;
    r.max.y -= 1;
}

pub fn translate(r: &mut RectI8, delta: &point_i8::PointI8) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i16::from(r.min.x) + i16::from(delta.x);
    let temp_min_y = i16::from(r.min.y) + i16::from(delta.y);
    let min_x = temp_min_x.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(dx));
    let min_y = temp_min_y.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(dy));
    r.min.x = min_x as i8;
    r.min.y = min_y as i8;
    r.max.x = (min_x + i16::from(dx)) as i8;
    r.max.y = (min_y + i16::from(dy)) as i8;
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_i8::PointI8;

    use super::{RectI8, deflate, delta_x, delta_y, inflate, len_x, len_y, max_delta, translate};

    #[test]
    fn rect_i8() {
        assert_eq!(RectI8::of(i8::MIN, -1, 1, i8::MAX), RectI8 { min: PointI8 { x: i8::MIN, y: -1 }, max: PointI8 { x: 1, y: i8::MAX } });
        assert_eq!(RectI8::of(i8::MIN, -0, 0, i8::MAX).to_string(), "((-128, 0), (0, 127))");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectI8::of(0, i8::MIN, 0, i8::MAX)), 0);
        assert_eq!(delta_x(&RectI8::of(i8::MIN, 0, i8::MAX, 0)), u8::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectI8::of(i8::MIN, 0, i8::MAX, 0)), 0);
        assert_eq!(delta_y(&RectI8::of(0, i8::MIN, 0, i8::MAX)), u8::MAX);
    }

    #[test]
    fn test_max_delta() {
        assert_eq!(max_delta(&RectI8::of(0, 5, 10, 10)), 10);
        assert_eq!(max_delta(&RectI8::of(-10, -10, -5, 0)), 10);
        assert_eq!(max_delta(&RectI8::of(-5, 0, 5, 5)), 10);
    }

    #[test]
    fn max_delta_0() {
        assert_eq!(max_delta(&RectI8::of(0, 0, 0, 0)), 0);
        assert_eq!(max_delta(&RectI8::of(1, 1, 1, 1)), 0);
        assert_eq!(max_delta(&RectI8::of(-1, -1, -1, -1)), 0);
        assert_eq!(max_delta(&RectI8::of(5, 10, 5, 10)), 0);
    }

    #[test]
    fn max_delta_1() {
        assert_eq!(max_delta(&RectI8::of(0, 0, 1, 1)), 1);
        assert_eq!(max_delta(&RectI8::of(5, 5, 6, 6)), 1);
        assert_eq!(max_delta(&RectI8::of(-6, -6, -5, -5)), 1);
        assert_eq!(max_delta(&RectI8::of(0, 0, 0, 1)), 1);
        assert_eq!(max_delta(&RectI8::of(5, 9, 5, 10)), 1);
    }

    #[test]
    fn max_delta_bounds() {
        assert_eq!(max_delta(&RectI8::of(i8::MIN + 1, i8::MIN, i8::MAX, i8::MAX)), u8::MAX);
        assert_eq!(max_delta(&RectI8::of(i8::MIN, i8::MIN + 1, i8::MAX, i8::MAX)), u8::MAX);
        assert_eq!(max_delta(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX)), u8::MAX);
        assert_eq!(max_delta(&RectI8::of(i8::MIN, i8::MIN, i8::MAX, i8::MAX - 1)), u8::MAX);
    }

    #[test]
    fn test_len_x() {
        assert_eq!(len_x(&RectI8::of(0, i8::MIN, 0, i8::MAX)), 1);
        assert_eq!(len_x(&RectI8::of(i8::MIN, 0, i8::MAX - 1, 0)), u8::MAX);
    }

    #[test]
    fn test_len_y() {
        assert_eq!(len_y(&RectI8::of(i8::MIN, 0, i8::MAX, 0)), 1);
        assert_eq!(len_y(&RectI8::of(0, i8::MIN, 0, i8::MAX - 1)), u8::MAX);
    }

    #[test]
    fn inflate_min_bounds() {
        let mut r = RectI8::of(i8::MIN + 7, i8::MIN + 2, 4, 13);
        inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 6, i8::MIN + 1, 5, 14));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 5, i8::MIN, 6, 15));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 4, i8::MIN, 7, 17));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 3, i8::MIN, 8, 19));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 2, i8::MIN, 9, 21));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 1, i8::MIN, 10, 23));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN, i8::MIN, 11, 25));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN, i8::MIN, 13, 27));
    }

    #[test]
    fn inflate_max_bounds() {
        let mut r = RectI8::of(-100, 30, i8::MAX - 5, i8::MAX - 3);
        inflate(&mut r);
        assert_eq!(r, RectI8::of(-101, 29, i8::MAX - 4, i8::MAX - 2));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(-102, 28, i8::MAX - 3, i8::MAX - 1));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(-103, 27, i8::MAX - 2, i8::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(-104, 25, i8::MAX - 1, i8::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(-105, 23, i8::MAX, i8::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(-107, 21, i8::MAX, i8::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(-109, 19, i8::MAX, i8::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(-111, 17, i8::MAX, i8::MAX));
    }

    #[test]
    fn inflate_almost_min_bounds() {
        let mut r = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX);
        inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN, i8::MIN, i8::MAX, i8::MAX));
    }

    #[test]
    fn inflate_almost_max_bounds() {
        let mut r = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1);
        inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN, i8::MIN, i8::MAX, i8::MAX));
    }

    #[test]
    fn inflate_max_width() {
        let mut r = RectI8::of(i8::MIN, 10, i8::MAX, 50);
        inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN, 10, i8::MAX, 50));
    }

    #[test]
    fn inflate_max_height() {
        let mut r = RectI8::of(10, i8::MIN, 50, i8::MAX);
        inflate(&mut r);
        assert_eq!(r, RectI8::of(10, i8::MIN, 50, i8::MAX));
    }

    #[test]
    fn deflate_odd_size() {
        let mut r = RectI8::of(-5, -5, 5, 5);
        deflate(&mut r);
        assert_eq!(r, RectI8::of(-4, -4, 4, 4));
        deflate(&mut r);
        assert_eq!(r, RectI8::of(-3, -3, 3, 3));
        deflate(&mut r);
        assert_eq!(r, RectI8::of(-2, -2, 2, 2));
        deflate(&mut r);
        assert_eq!(r, RectI8::of(-1, -1, 1, 1));
        deflate(&mut r);
        assert_eq!(r, RectI8::of(-1, -1, 1, 1));
    }

    #[test]
    fn deflate_even_size() {
        let mut r = RectI8::of(-5, -5, 4, 4);
        deflate(&mut r);
        assert_eq!(r, RectI8::of(-4, -4, 3, 3));
        deflate(&mut r);
        assert_eq!(r, RectI8::of(-3, -3, 2, 2));
        deflate(&mut r);
        assert_eq!(r, RectI8::of(-2, -2, 1, 1));
        deflate(&mut r);
        assert_eq!(r, RectI8::of(-1, -1, 0, 0));
        deflate(&mut r);
        assert_eq!(r, RectI8::of(-1, -1, 0, 0));
    }

    #[test]
    fn test_translate() {
        let mut r = RectI8::of(0, 0, 10, 10);
        translate(&mut r, &PointI8::of(10, 10));
        assert_eq!(r, RectI8::of(10, 10, 20, 20));
        translate(&mut r, &PointI8::of(-20, -20));
        assert_eq!(r, RectI8::of(-10, -10, 0, 0));
        translate(&mut r, &PointI8::of(2, 2));
        assert_eq!(r, RectI8::of(-8, -8, 2, 2));
    }

    #[test]
    fn translate_min_bounds() {
        let mut r = RectI8::of(i8::MIN + 5, i8::MIN + 10, 12, 15);
        translate(&mut r, &PointI8::of(-10, -10));
        assert_eq!(r, RectI8::of(i8::MIN, i8::MIN, 7, 5));
    }

    #[test]
    fn translate_max_bounds() {
        let mut r = RectI8::of(40, 35, i8::MAX - 5, i8::MAX - 10);
        translate(&mut r, &PointI8::of(20, 20));
        assert_eq!(r, RectI8::of(45, 45, i8::MAX, i8::MAX));
    }

    #[test]
    fn translate_min_bounds_big_delta() {
        let mut r = RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 10);
        translate(&mut r, &PointI8::of(i8::MIN, i8::MIN));
        assert_eq!(r, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 10));
    }

    #[test]
    fn translate_max_bounds_big_delta() {
        let mut r = RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX, i8::MAX);
        translate(&mut r, &PointI8::of(i8::MAX, i8::MAX));
        assert_eq!(r, RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX, i8::MAX));
    }

    #[test]
    fn translate_min_bounds_big_rect_big_delta() {
        let mut r = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX);
        translate(&mut r, &PointI8::of(i8::MIN, i8::MIN));
        assert_eq!(r, RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1));
    }

    #[test]
    fn translate_max_bounds_big_rect_big_delta() {
        let mut r = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1);
        translate(&mut r, &PointI8::of(i8::MAX, i8::MAX));
        assert_eq!(r, RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX));
    }
}

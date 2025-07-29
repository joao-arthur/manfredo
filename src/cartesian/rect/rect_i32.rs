use crate::cartesian::point::point_i32;

#[derive(PartialEq, Debug, Clone)]
pub struct RectI32 {
    pub min: point_i32::PointI32,
    pub max: point_i32::PointI32,
}

impl RectI32 {
    pub fn of(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        RectI32 { min: point_i32::PointI32::of(x1, y1), max: point_i32::PointI32::of(x2, y2) }
    }
}

impl std::fmt::Display for RectI32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_x(r: &RectI32) -> u32 {
    point_i32::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectI32) -> u32 {
    point_i32::delta_y(&r.min, &r.max)
}

pub fn max_delta(r: &RectI32) -> u32 {
    std::cmp::max(delta_x(r), delta_y(r))
}

pub fn len_x(r: &RectI32) -> u32 {
    delta_x(r) + 1
}

pub fn len_y(r: &RectI32) -> u32 {
    delta_y(r) + 1
}

pub fn max_len(r: &RectI32) -> u32 {
    std::cmp::max(len_x(r), len_y(r))
}

pub fn inflate(r: &mut RectI32) {
    let is_min_x = r.min.x == i32::MIN;
    let is_min_y = r.min.y == i32::MIN;
    let is_max_x = r.max.x == i32::MAX;
    let is_max_y = r.max.y == i32::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return;
    }
    let min_x_modifier = 1 - i32::from(is_min_x) + i32::from(is_max_x);
    let min_y_modifier = 1 - i32::from(is_min_y) + i32::from(is_max_y);
    let max_x_modifier = 1 + i32::from(is_min_x) - i32::from(is_max_x);
    let max_y_modifier = 1 + i32::from(is_min_y) - i32::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
}

pub fn deflate(r: &mut RectI32) {
    if delta_x(r) < 3 || delta_y(r) < 3 {
        return;
    }
    r.min.x += 1;
    r.min.y += 1;
    r.max.x -= 1;
    r.max.y -= 1;
}

pub fn translate(r: &mut RectI32, delta: &point_i32::PointI32) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i64::from(r.min.x) + i64::from(delta.x);
    let temp_min_y = i64::from(r.min.y) + i64::from(delta.y);
    let min_x = temp_min_x.clamp(i64::from(i32::MIN), i64::from(i32::MAX) - i64::from(dx));
    let min_y = temp_min_y.clamp(i64::from(i32::MIN), i64::from(i32::MAX) - i64::from(dy));
    r.min.x = min_x as i32;
    r.min.y = min_y as i32;
    r.max.x = (min_x + i64::from(dx)) as i32;
    r.max.y = (min_y + i64::from(dy)) as i32;
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_i32::PointI32;

    use super::{RectI32, deflate, delta_x, delta_y, inflate, len_x, len_y, max_delta, max_len, translate};

    #[test]
    fn rect_i32() {
        assert_eq!(RectI32::of(i32::MIN, -1, 1, i32::MAX), RectI32 { min: PointI32 { x: i32::MIN, y: -1 }, max: PointI32 { x: 1, y: i32::MAX } });
        assert_eq!(RectI32::of(i32::MIN, -0, 0, i32::MAX).to_string(), "((-2147483648, 0), (0, 2147483647))");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectI32::of(0, i32::MIN, 0, i32::MAX)), 0);
        assert_eq!(delta_x(&RectI32::of(i32::MIN, 0, i32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectI32::of(i32::MIN, 0, i32::MAX, 0)), 0);
        assert_eq!(delta_y(&RectI32::of(0, i32::MIN, 0, i32::MAX)), u32::MAX);
    }

    #[test]
    fn test_max_delta() {
        assert_eq!(max_delta(&RectI32::of(0, 5, 10, 10)), 10);
        assert_eq!(max_delta(&RectI32::of(-10, -10, -5, 0)), 10);
        assert_eq!(max_delta(&RectI32::of(-5, 0, 5, 5)), 10);
    }

    #[test]
    fn max_delta_0() {
        assert_eq!(max_delta(&RectI32::of(0, 0, 0, 0)), 0);
        assert_eq!(max_delta(&RectI32::of(1, 1, 1, 1)), 0);
        assert_eq!(max_delta(&RectI32::of(-1, -1, -1, -1)), 0);
        assert_eq!(max_delta(&RectI32::of(5, 10, 5, 10)), 0);
    }

    #[test]
    fn max_delta_1() {
        assert_eq!(max_delta(&RectI32::of(0, 0, 1, 1)), 1);
        assert_eq!(max_delta(&RectI32::of(5, 5, 6, 6)), 1);
        assert_eq!(max_delta(&RectI32::of(-6, -6, -5, -5)), 1);
        assert_eq!(max_delta(&RectI32::of(0, 0, 0, 1)), 1);
        assert_eq!(max_delta(&RectI32::of(5, 9, 5, 10)), 1);
    }

    #[test]
    fn max_delta_bounds() {
        assert_eq!(max_delta(&RectI32::of(i32::MIN + 1, i32::MIN, i32::MAX, i32::MAX)), u32::MAX);
        assert_eq!(max_delta(&RectI32::of(i32::MIN, i32::MIN + 1, i32::MAX, i32::MAX)), u32::MAX);
        assert_eq!(max_delta(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX)), u32::MAX);
        assert_eq!(max_delta(&RectI32::of(i32::MIN, i32::MIN, i32::MAX, i32::MAX - 1)), u32::MAX);
    }

    #[test]
    fn test_len_x() {
        assert_eq!(len_x(&RectI32::of(0, i32::MIN, 0, i32::MAX)), 1);
        assert_eq!(len_x(&RectI32::of(i32::MIN, 0, i32::MAX - 1, 0)), u32::MAX);
    }

    #[test]
    fn test_len_y() {
        assert_eq!(len_y(&RectI32::of(i32::MIN, 0, i32::MAX, 0)), 1);
        assert_eq!(len_y(&RectI32::of(0, i32::MIN, 0, i32::MAX - 1)), u32::MAX);
    }

    #[test]
    fn test_max_len() {
        assert_eq!(max_len(&RectI32::of(0, 5, 10, 10)), 11);
        assert_eq!(max_len(&RectI32::of(-10, -10, -5, 0)), 11);
        assert_eq!(max_len(&RectI32::of(-5, 0, 5, 5)), 11);
    }

    #[test]
    fn max_len_1() {
        assert_eq!(max_len(&RectI32::of(0, 0, 0, 0)), 1);
        assert_eq!(max_len(&RectI32::of(1, 1, 1, 1)), 1);
        assert_eq!(max_len(&RectI32::of(-1, -1, -1, -1)), 1);
        assert_eq!(max_len(&RectI32::of(5, 10, 5, 10)), 1);
    }

    #[test]
    fn max_len_2() {
        assert_eq!(max_len(&RectI32::of(0, 0, 1, 1)), 2);
        assert_eq!(max_len(&RectI32::of(5, 5, 6, 6)), 2);
        assert_eq!(max_len(&RectI32::of(-6, -6, -5, -5)), 2);
        assert_eq!(max_len(&RectI32::of(0, 0, 0, 1)), 2);
        assert_eq!(max_len(&RectI32::of(5, 9, 5, 10)), 2);
    }

    #[test]
    fn max_len_bounds() {
        assert_eq!(max_len(&RectI32::of(i32::MIN + 1, i32::MIN, i32::MAX - 1, i32::MAX - 1)), u32::MAX);
        assert_eq!(max_len(&RectI32::of(i32::MIN, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1)), u32::MAX);
        assert_eq!(max_len(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 1)), u32::MAX);
        assert_eq!(max_len(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 2)), u32::MAX);
    }

    #[test]
    fn inflate_min_bounds() {
        let mut r = RectI32::of(i32::MIN + 7, i32::MIN + 2, 4, 13);
        inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN + 6, i32::MIN + 1, 5, 14));
        inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN + 5, i32::MIN, 6, 15));
        inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN + 4, i32::MIN, 7, 17));
        inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN + 3, i32::MIN, 8, 19));
        inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN + 2, i32::MIN, 9, 21));
        inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN + 1, i32::MIN, 10, 23));
        inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN, i32::MIN, 11, 25));
        inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN, i32::MIN, 13, 27));
    }

    #[test]
    fn inflate_max_bounds() {
        let mut r = RectI32::of(-100, 30, i32::MAX - 5, i32::MAX - 3);
        inflate(&mut r);
        assert_eq!(r, RectI32::of(-101, 29, i32::MAX - 4, i32::MAX - 2));
        inflate(&mut r);
        assert_eq!(r, RectI32::of(-102, 28, i32::MAX - 3, i32::MAX - 1));
        inflate(&mut r);
        assert_eq!(r, RectI32::of(-103, 27, i32::MAX - 2, i32::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI32::of(-104, 25, i32::MAX - 1, i32::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI32::of(-105, 23, i32::MAX, i32::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI32::of(-107, 21, i32::MAX, i32::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI32::of(-109, 19, i32::MAX, i32::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI32::of(-111, 17, i32::MAX, i32::MAX));
    }

    #[test]
    fn inflate_almost_min_bounds() {
        let mut r = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX);
        inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN, i32::MIN, i32::MAX, i32::MAX));
    }

    #[test]
    fn inflate_almost_max_bounds() {
        let mut r = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1);
        inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN, i32::MIN, i32::MAX, i32::MAX));
    }

    #[test]
    fn inflate_max_width() {
        let mut r = RectI32::of(i32::MIN, 10, i32::MAX, 50);
        inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN, 10, i32::MAX, 50));
    }

    #[test]
    fn inflate_max_height() {
        let mut r = RectI32::of(10, i32::MIN, 50, i32::MAX);
        inflate(&mut r);
        assert_eq!(r, RectI32::of(10, i32::MIN, 50, i32::MAX));
    }

    #[test]
    fn deflate_odd_size() {
        let mut r = RectI32::of(-5, -5, 5, 5);
        deflate(&mut r);
        assert_eq!(r, RectI32::of(-4, -4, 4, 4));
        deflate(&mut r);
        assert_eq!(r, RectI32::of(-3, -3, 3, 3));
        deflate(&mut r);
        assert_eq!(r, RectI32::of(-2, -2, 2, 2));
        deflate(&mut r);
        assert_eq!(r, RectI32::of(-1, -1, 1, 1));
        deflate(&mut r);
        assert_eq!(r, RectI32::of(-1, -1, 1, 1));
    }

    #[test]
    fn deflate_even_size() {
        let mut r = RectI32::of(-5, -5, 4, 4);
        deflate(&mut r);
        assert_eq!(r, RectI32::of(-4, -4, 3, 3));
        deflate(&mut r);
        assert_eq!(r, RectI32::of(-3, -3, 2, 2));
        deflate(&mut r);
        assert_eq!(r, RectI32::of(-2, -2, 1, 1));
        deflate(&mut r);
        assert_eq!(r, RectI32::of(-1, -1, 0, 0));
        deflate(&mut r);
        assert_eq!(r, RectI32::of(-1, -1, 0, 0));
    }

    #[test]
    fn test_translate() {
        let mut r = RectI32::of(0, 0, 10, 10);
        translate(&mut r, &PointI32::of(10, 10));
        assert_eq!(r, RectI32::of(10, 10, 20, 20));
        translate(&mut r, &PointI32::of(-20, -20));
        assert_eq!(r, RectI32::of(-10, -10, 0, 0));
        translate(&mut r, &PointI32::of(2, 2));
        assert_eq!(r, RectI32::of(-8, -8, 2, 2));
    }

    #[test]
    fn translate_min_bounds() {
        let mut r = RectI32::of(i32::MIN + 5, i32::MIN + 10, 12, 15);
        translate(&mut r, &PointI32::of(-10, -10));
        assert_eq!(r, RectI32::of(i32::MIN, i32::MIN, 7, 5));
    }

    #[test]
    fn translate_max_bounds() {
        let mut r = RectI32::of(40, 35, i32::MAX - 5, i32::MAX - 10);
        translate(&mut r, &PointI32::of(20, 20));
        assert_eq!(r, RectI32::of(45, 45, i32::MAX, i32::MAX));
    }

    #[test]
    fn translate_min_bounds_big_delta() {
        let mut r = RectI32::of(i32::MIN, i32::MIN, i32::MIN + 10, i32::MIN + 10);
        translate(&mut r, &PointI32::of(i32::MIN, i32::MIN));
        assert_eq!(r, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 10, i32::MIN + 10));
    }

    #[test]
    fn translate_max_bounds_big_delta() {
        let mut r = RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX, i32::MAX);
        translate(&mut r, &PointI32::of(i32::MAX, i32::MAX));
        assert_eq!(r, RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX, i32::MAX));
    }

    #[test]
    fn translate_min_bounds_big_rect_big_delta() {
        let mut r = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX);
        translate(&mut r, &PointI32::of(i32::MIN, i32::MIN));
        assert_eq!(r, RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1));
    }

    #[test]
    fn translate_max_bounds_big_rect_big_delta() {
        let mut r = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1);
        translate(&mut r, &PointI32::of(i32::MAX, i32::MAX));
        assert_eq!(r, RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX));
    }
}

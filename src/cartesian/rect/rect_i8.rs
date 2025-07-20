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
        write!(f, "({}, {})", self.min.to_string(), self.max.to_string())
    }
}

pub fn delta_x(r: &RectI8) -> u8 {
    point_i8::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectI8) -> u8 {
    point_i8::delta_y(&r.min, &r.max)
}

pub fn inflate(r: &mut RectI8) {
    let is_min_x = r.min.x == i8::MIN;
    let is_max_x = r.max.x == i8::MAX;
    let is_min_y = r.min.y == i8::MIN;
    let is_max_y = r.max.y == i8::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return;
    }
    let min_x_modifier = 1 - i8::from(is_min_x) + i8::from(is_max_x);
    let max_x_modifier = 1 + i8::from(is_min_x) - i8::from(is_max_x);
    let min_y_modifier = 1 - i8::from(is_min_y) + i8::from(is_max_y);
    let max_y_modifier = 1 + i8::from(is_min_y) - i8::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
}

pub fn deflate(r: &mut RectI8) {
    if delta_x(&r) < 3 || delta_y(&r) < 3 {
        return;
    }
    r.min.x += 1;
    r.min.y += 1;
    r.max.x -= 1;
    r.max.y -= 1;
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_i8::PointI8;

    use super::{RectI8, deflate, delta_x, delta_y, inflate};

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
    fn inflate_min_bounds() {
        let mut r = RectI8::of(-121, -126, 4, 13);
        inflate(&mut r);
        assert_eq!(r, RectI8::of(-122, -127, 5, 14));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(-123, i8::MIN, 6, 15));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(-124, i8::MIN, 7, 17));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(-125, i8::MIN, 8, 19));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(-126, i8::MIN, 9, 21));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(-127, i8::MIN, 10, 23));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN, i8::MIN, 11, 25));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN, i8::MIN, 13, 27));
    }

    #[test]
    fn inflate_max_bounds() {
        let mut r = RectI8::of(-100, 30, 122, 124);
        inflate(&mut r);
        assert_eq!(r, RectI8::of(-101, 29, 123, 125));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(-102, 28, 124, 126));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(-103, 27, 125, i8::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI8::of(-104, 25, 126, i8::MAX));
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
        let mut r = RectI8::of(-127, -127, i8::MAX, i8::MAX);
        inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN, i8::MIN, i8::MAX, i8::MAX));
    }

    #[test]
    fn inflate_almost_max_bounds() {
        let mut r = RectI8::of(i8::MIN, i8::MIN, 126, 126);
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
}

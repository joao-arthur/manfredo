use crate::cartesian::point::point_i64;

#[derive(PartialEq, Debug, Clone)]
pub struct RectI64 {
    pub min: point_i64::PointI64,
    pub max: point_i64::PointI64,
}

impl RectI64 {
    pub fn of(x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
        RectI64 { min: point_i64::PointI64::of(x1, y1), max: point_i64::PointI64::of(x2, y2) }
    }
}

impl std::fmt::Display for RectI64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min.to_string(), self.max.to_string())
    }
}

pub fn delta_x(r: &RectI64) -> u64 {
    point_i64::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectI64) -> u64 {
    point_i64::delta_y(&r.min, &r.max)
}

pub fn inflate(r: &mut RectI64) {
    let is_min_x = r.min.x == i64::MIN;
    let is_max_x = r.max.x == i64::MAX;
    let is_min_y = r.min.y == i64::MIN;
    let is_max_y = r.max.y == i64::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return;
    }
    let min_x_modifier = 1 - i64::from(is_min_x) + i64::from(is_max_x);
    let max_x_modifier = 1 + i64::from(is_min_x) - i64::from(is_max_x);
    let min_y_modifier = 1 - i64::from(is_min_y) + i64::from(is_max_y);
    let max_y_modifier = 1 + i64::from(is_min_y) - i64::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_i64::PointI64;

    use super::{RectI64, delta_x, delta_y, inflate};

    #[test]
    fn rect_i64() {
        assert_eq!(RectI64::of(i64::MIN, -1, 1, i64::MAX), RectI64 { min: PointI64 { x: i64::MIN, y: -1 }, max: PointI64 { x: 1, y: i64::MAX } });
        assert_eq!(RectI64::of(i64::MIN, -0, 0, i64::MAX).to_string(), "((-9223372036854775808, 0), (0, 9223372036854775807))");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectI64::of(0, i64::MIN, 0, i64::MAX)), 0);
        assert_eq!(delta_x(&RectI64::of(i64::MIN, 0, i64::MAX, 0)), u64::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectI64::of(i64::MIN, 0, i64::MAX, 0)), 0);
        assert_eq!(delta_y(&RectI64::of(0, i64::MIN, 0, i64::MAX)), u64::MAX);
    }

    #[test]
    fn inflate_min_bounds() {
        let mut r = RectI64::of(-9223372036854775801, -9223372036854775806, 4, 13);
        inflate(&mut r);
        assert_eq!(r, RectI64::of(-9223372036854775802, -9223372036854775807, 5, 14));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(-9223372036854775803, i64::MIN, 6, 15));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(-9223372036854775804, i64::MIN, 7, 17));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(-9223372036854775805, i64::MIN, 8, 19));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(-9223372036854775806, i64::MIN, 9, 21));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(-9223372036854775807, i64::MIN, 10, 23));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, 11, 25));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, 13, 27));
    }

    #[test]
    fn inflate_max_bounds() {
        let mut r = RectI64::of(-100, 30, 9223372036854775802, 9223372036854775804);
        inflate(&mut r);
        assert_eq!(r, RectI64::of(-101, 29, 9223372036854775803, 9223372036854775805));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(-102, 28, 9223372036854775804, 9223372036854775806));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(-103, 27, 9223372036854775805, i64::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(-104, 25, 9223372036854775806, i64::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(-105, 23, i64::MAX, i64::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(-107, 21, i64::MAX, i64::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(-109, 19, i64::MAX, i64::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(-111, 17, i64::MAX, i64::MAX));
    }

    #[test]
    fn inflate_almost_min_bounds() {
        let mut r = RectI64::of(-9223372036854775807, -9223372036854775807, i64::MAX, i64::MAX);
        inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MAX, i64::MAX));
    }

    #[test]
    fn inflate_almost_max_bounds() {
        let mut r = RectI64::of(i64::MIN, i64::MIN, 9223372036854775806, 9223372036854775806);
        inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MAX, i64::MAX));
    }

    #[test]
    fn inflate_max_width() {
        let mut r = RectI64::of(i64::MIN, 10, i64::MAX, 50);
        inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN, 10, i64::MAX, 50));
    }

    #[test]
    fn inflate_max_height() {
        let mut r = RectI64::of(10, i64::MIN, 50, i64::MAX);
        inflate(&mut r);
        assert_eq!(r, RectI64::of(10, i64::MIN, 50, i64::MAX));
    }
}

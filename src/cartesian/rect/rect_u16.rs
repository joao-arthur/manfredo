use crate::cartesian::point::point_u16;

#[derive(PartialEq, Debug, Clone)]
pub struct RectU16 {
    pub min: point_u16::PointU16,
    pub max: point_u16::PointU16,
}

impl RectU16 {
    pub fn of(x1: u16, y1: u16, x2: u16, y2: u16) -> Self {
        RectU16 { min: point_u16::PointU16::of(x1, y1), max: point_u16::PointU16::of(x2, y2) }
    }
}

impl std::fmt::Display for RectU16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_x(r: &RectU16) -> u16 {
    point_u16::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectU16) -> u16 {
    point_u16::delta_y(&r.min, &r.max)
}

pub fn inflate(r: &mut RectU16) {
    let is_min_x = r.min.x == 0;
    let is_max_x = r.max.x == u16::MAX;
    let is_min_y = r.min.y == 0;
    let is_max_y = r.max.y == u16::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return;
    }
    let min_x_modifier = 1 - u16::from(is_min_x) + u16::from(is_max_x);
    let max_x_modifier = 1 + u16::from(is_min_x) - u16::from(is_max_x);
    let min_y_modifier = 1 - u16::from(is_min_y) + u16::from(is_max_y);
    let max_y_modifier = 1 + u16::from(is_min_y) - u16::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
}

pub fn deflate(r: &mut RectU16) {
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
    use crate::cartesian::point::point_u16::PointU16;

    use super::{RectU16, deflate, delta_x, delta_y, inflate};

    #[test]
    fn rect_u16() {
        assert_eq!(RectU16::of(16, 32, 64, 128), RectU16 { min: PointU16 { x: 16, y: 32 }, max: PointU16 { x: 64, y: 128 } });
        assert_eq!(RectU16::of(u16::MAX, 0, 0, u16::MAX).to_string(), "((65535, 0), (0, 65535))");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectU16::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_x(&RectU16::of(0, 0, u16::MAX, 0)), u16::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectU16::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_y(&RectU16::of(0, 0, 0, u16::MAX)), u16::MAX);
    }

    #[test]
    fn inflate_min_bounds() {
        let mut r = RectU16::of(7, 2, 4, 13);
        inflate(&mut r);
        assert_eq!(r, RectU16::of(6, 1, 5, 14));
        inflate(&mut r);
        assert_eq!(r, RectU16::of(5, 0, 6, 15));
        inflate(&mut r);
        assert_eq!(r, RectU16::of(4, 0, 7, 17));
        inflate(&mut r);
        assert_eq!(r, RectU16::of(3, 0, 8, 19));
        inflate(&mut r);
        assert_eq!(r, RectU16::of(2, 0, 9, 21));
        inflate(&mut r);
        assert_eq!(r, RectU16::of(1, 0, 10, 23));
        inflate(&mut r);
        assert_eq!(r, RectU16::of(0, 0, 11, 25));
        inflate(&mut r);
        assert_eq!(r, RectU16::of(0, 0, 13, 27));
    }

    #[test]
    fn inflate_max_bounds() {
        let mut r = RectU16::of(200, 230, 65530, 65532);
        inflate(&mut r);
        assert_eq!(r, RectU16::of(199, 229, 65531, 65533));
        inflate(&mut r);
        assert_eq!(r, RectU16::of(198, 228, 65532, 65534));
        inflate(&mut r);
        assert_eq!(r, RectU16::of(197, 227, 65533, u16::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU16::of(196, 225, 65534, u16::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU16::of(195, 223, u16::MAX, u16::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU16::of(193, 221, u16::MAX, u16::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU16::of(191, 219, u16::MAX, u16::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU16::of(189, 217, u16::MAX, u16::MAX));
    }

    #[test]
    fn inflate_almost_min_bounds() {
        let mut r = RectU16::of(1, 1, u16::MAX, u16::MAX);
        inflate(&mut r);
        assert_eq!(r, RectU16::of(0, 0, u16::MAX, u16::MAX));
    }

    #[test]
    fn inflate_almost_max_bounds() {
        let mut r = RectU16::of(0, 0, 65534, 65534);
        inflate(&mut r);
        assert_eq!(r, RectU16::of(0, 0, u16::MAX, u16::MAX));
    }

    #[test]
    fn inflate_max_width() {
        let mut r = RectU16::of(0, 10, u16::MAX, 250);
        inflate(&mut r);
        assert_eq!(r, RectU16::of(0, 10, u16::MAX, 250));
    }

    #[test]
    fn inflate_max_height() {
        let mut r = RectU16::of(10, 0, 250, u16::MAX);
        inflate(&mut r);
        assert_eq!(r, RectU16::of(10, 0, 250, u16::MAX));
    }

    #[test]
    fn deflate_odd() {
        let mut r = RectU16::of(0, 0, 9, 9);
        deflate(&mut r);
        assert_eq!(r, RectU16::of(1, 1, 8, 8));
        deflate(&mut r);
        assert_eq!(r, RectU16::of(2, 2, 7, 7));
        deflate(&mut r);
        assert_eq!(r, RectU16::of(3, 3, 6, 6));
        deflate(&mut r);
        assert_eq!(r, RectU16::of(4, 4, 5, 5));
        deflate(&mut r);
        assert_eq!(r, RectU16::of(4, 4, 5, 5));
    }

    #[test]
    fn deflate_even() {
        let mut r = RectU16::of(0, 0, 10, 10);
        deflate(&mut r);
        assert_eq!(r, RectU16::of(1, 1, 9, 9));
        deflate(&mut r);
        assert_eq!(r, RectU16::of(2, 2, 8, 8));
        deflate(&mut r);
        assert_eq!(r, RectU16::of(3, 3, 7, 7));
        deflate(&mut r);
        assert_eq!(r, RectU16::of(4, 4, 6, 6));
        deflate(&mut r);
        assert_eq!(r, RectU16::of(4, 4, 6, 6));
    }
}

use crate::cartesian::point::point_u8;

#[derive(PartialEq, Debug, Clone)]
pub struct RectU8 {
    pub min: point_u8::PointU8,
    pub max: point_u8::PointU8,
}

impl RectU8 {
    pub fn of(x1: u8, y1: u8, x2: u8, y2: u8) -> Self {
        RectU8 { min: point_u8::PointU8::of(x1, y1), max: point_u8::PointU8::of(x2, y2) }
    }
}

impl std::fmt::Display for RectU8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min.to_string(), self.max.to_string())
    }
}

pub fn delta_x(r: &RectU8) -> u8 {
    point_u8::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectU8) -> u8 {
    point_u8::delta_y(&r.min, &r.max)
}

pub fn inflate(r: &mut RectU8) {
    let is_min_x = r.min.x == 0;
    let is_max_x = r.max.x == u8::MAX;
    let is_min_y = r.min.y == 0;
    let is_max_y = r.max.y == u8::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return;
    }
    let min_x_modifier = 1 - u8::from(is_min_x) + u8::from(is_max_x);
    let max_x_modifier = 1 + u8::from(is_min_x) - u8::from(is_max_x);
    let min_y_modifier = 1 - u8::from(is_min_y) + u8::from(is_max_y);
    let max_y_modifier = 1 + u8::from(is_min_y) - u8::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
}

pub fn deflate(r: &mut RectU8) {
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
    use crate::cartesian::point::point_u8::PointU8;

    use super::{RectU8, deflate, delta_x, delta_y, inflate};

    #[test]
    fn rect_u8() {
        assert_eq!(
            RectU8::of(0, 2, 4, 8),
            RectU8 { min: PointU8 { x: 0, y: 2 }, max: PointU8 { x: 4, y: 8 } }
        );
        assert_eq!(RectU8::of(u8::MAX, 0, 0, u8::MAX).to_string(), "((255, 0), (0, 255))");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectU8::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_x(&RectU8::of(0, 0, u8::MAX, 0)), u8::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectU8::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_y(&RectU8::of(0, 0, 0, u8::MAX)), u8::MAX);
    }

    #[test]
    fn inflate_min_bounds() {
        let mut r = RectU8::of(7, 2, 4, 13);
        inflate(&mut r);
        assert_eq!(r, RectU8::of(6, 1, 5, 14));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(5, 0, 6, 15));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(4, 0, 7, 17));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(3, 0, 8, 19));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(2, 0, 9, 21));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(1, 0, 10, 23));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(0, 0, 11, 25));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(0, 0, 13, 27));
    }

    #[test]
    fn inflate_max_bounds() {
        let mut r = RectU8::of(200, 230, 250, 252);
        inflate(&mut r);
        assert_eq!(r, RectU8::of(199, 229, 251, 253));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(198, 228, 252, 254));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(197, 227, 253, u8::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(196, 225, 254, u8::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(195, 223, u8::MAX, u8::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(193, 221, u8::MAX, u8::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(191, 219, u8::MAX, u8::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(189, 217, u8::MAX, u8::MAX));
    }

    #[test]
    fn inflate_almost_min_bounds() {
        let mut r = RectU8::of(1, 1, u8::MAX, u8::MAX);
        inflate(&mut r);
        assert_eq!(r, RectU8::of(0, 0, u8::MAX, u8::MAX));
    }

    #[test]
    fn inflate_almost_max_bounds() {
        let mut r = RectU8::of(0, 0, 254, 254);
        inflate(&mut r);
        assert_eq!(r, RectU8::of(0, 0, u8::MAX, u8::MAX));
    }

    #[test]
    fn inflate_max_width() {
        let mut r = RectU8::of(0, 10, u8::MAX, 250);
        inflate(&mut r);
        assert_eq!(r, RectU8::of(0, 10, u8::MAX, 250));
    }

    #[test]
    fn inflate_max_height() {
        let mut r = RectU8::of(10, 0, 250, u8::MAX);
        inflate(&mut r);
        assert_eq!(r, RectU8::of(10, 0, 250, u8::MAX));
    }

    #[test]
    fn deflate_odd() {
        let mut r = RectU8::of(0, 0, 9, 9);
        deflate(&mut r);
        assert_eq!(r, RectU8::of(1, 1, 8, 8));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(2, 2, 7, 7));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(3, 3, 6, 6));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(4, 4, 5, 5));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(4, 4, 5, 5));
    }

    #[test]
    fn deflate_even() {
        let mut r = RectU8::of(0, 0, 10, 10);
        deflate(&mut r);
        assert_eq!(r, RectU8::of(1, 1, 9, 9));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(2, 2, 8, 8));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(3, 3, 7, 7));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(4, 4, 6, 6));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(4, 4, 6, 6));
    }
}

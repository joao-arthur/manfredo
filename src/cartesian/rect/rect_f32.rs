use crate::cartesian::point::point_f32;

#[derive(PartialEq, Debug, Clone)]
pub struct RectF32 {
    pub min: point_f32::PointF32,
    pub max: point_f32::PointF32,
}

impl RectF32 {
    pub fn of(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        RectF32 { min: point_f32::PointF32 { x: x1, y: y1 }, max: point_f32::PointF32 { x: x2, y: y2 } }
    }
}

impl std::fmt::Display for RectF32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

const ART_MIN: f32 = -16_777_216.0;
const ART_MAX: f32 = 16_777_215.0;

pub fn delta_x(r: &RectF32) -> f32 {
    point_f32::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectF32) -> f32 {
    point_f32::delta_y(&r.min, &r.max)
}

pub fn inflate(r: &mut RectF32) {
    let is_min_x = r.min.x == ART_MIN;
    let is_max_x = r.max.x == ART_MAX;
    let is_min_y = r.min.y == ART_MIN;
    let is_max_y = r.max.y == ART_MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return;
    }
    let min_x_modifier = 1.0 - f32::from(is_min_x) + f32::from(is_max_x);
    let max_x_modifier = 1.0 + f32::from(is_min_x) - f32::from(is_max_x);
    let min_y_modifier = 1.0 - f32::from(is_min_y) + f32::from(is_max_y);
    let max_y_modifier = 1.0 + f32::from(is_min_y) - f32::from(is_max_y);
    r.min.x = (r.min.x - min_x_modifier).max(ART_MIN);
    r.max.x = (r.max.x + max_x_modifier).min(ART_MAX);
    r.min.y = (r.min.y - min_y_modifier).max(ART_MIN);
    r.max.y = (r.max.y + max_y_modifier).min(ART_MAX);
}

pub fn deflate(r: &mut RectF32) {
    if delta_x(r) < 3.0 || delta_y(r) < 3.0 {
        return;
    }
    r.min.x += 1.0;
    r.min.y += 1.0;
    r.max.x -= 1.0;
    r.max.y -= 1.0;
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_f32::PointF32;

    use super::{ART_MAX, ART_MIN, RectF32, deflate, delta_x, delta_y, inflate};

    #[test]
    fn rect_f32() {
        assert_eq!(RectF32::of(ART_MIN, -0.0, 0.0, ART_MAX), RectF32 { min: PointF32 { x: ART_MIN, y: -0.0 }, max: PointF32 { x: 0.0, y: ART_MAX } });
        assert_eq!(RectF32::of(ART_MIN, -0.0, 0.0, ART_MAX).to_string(), "((-16777216, -0), (0, 16777215))");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectF32::of(0.0, 0.0, 0.0, ART_MAX)), 0.0);
        assert_eq!(delta_x(&RectF32::of(0.0, -8_388_608.0, 0.0, 8_388_607.0)), 0.0);
        assert_eq!(delta_x(&RectF32::of(0.0, 0.0, ART_MAX, 0.0)), ART_MAX);
        assert_eq!(delta_x(&RectF32::of(-8_388_608.0, 0.0, 8_388_607.0, 0.0)), ART_MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectF32::of(0.0, 0.0, ART_MAX, 0.0)), 0.0);
        assert_eq!(delta_y(&RectF32::of(-8_388_608.0, 0.0, 8_388_607.0, 0.0)), 0.0);
        assert_eq!(delta_y(&RectF32::of(0.0, 0.0, 0.0, ART_MAX)), ART_MAX);
        assert_eq!(delta_y(&RectF32::of(0.0, -8_388_608.0, 0.0, 8_388_607.0)), ART_MAX);
    }

    #[test]
    fn inflate_min_bounds() {
        let mut r = RectF32::of(ART_MIN + 7.0, ART_MIN + 2.0, 4.0, 13.0);
        inflate(&mut r);
        assert_eq!(r, RectF32::of(ART_MIN + 6.0, ART_MIN + 1.0, 5.0, 14.0));
        inflate(&mut r);
        assert_eq!(r, RectF32::of(ART_MIN + 5.0, ART_MIN, 6.0, 15.0));
        inflate(&mut r);
        assert_eq!(r, RectF32::of(ART_MIN + 4.0, ART_MIN, 7.0, 17.0));
        inflate(&mut r);
        assert_eq!(r, RectF32::of(ART_MIN + 3.0, ART_MIN, 8.0, 19.0));
        inflate(&mut r);
        assert_eq!(r, RectF32::of(ART_MIN + 2.0, ART_MIN, 9.0, 21.0));
        inflate(&mut r);
        assert_eq!(r, RectF32::of(ART_MIN + 1.0, ART_MIN, 10.0, 23.0));
        inflate(&mut r);
        assert_eq!(r, RectF32::of(ART_MIN, ART_MIN, 11.0, 25.0));
        inflate(&mut r);
        assert_eq!(r, RectF32::of(ART_MIN, ART_MIN, 13.0, 27.0));
    }

    #[test]
    fn inflate_max_bounds() {
        let mut r = RectF32::of(-100.0, 30.0, ART_MAX - 5.0, ART_MAX - 3.0);
        inflate(&mut r);
        assert_eq!(r, RectF32::of(-101.0, 29.0, ART_MAX - 4.0, ART_MAX - 2.0));
        inflate(&mut r);
        assert_eq!(r, RectF32::of(-102.0, 28.0, ART_MAX - 3.0, ART_MAX - 1.0));
        inflate(&mut r);
        assert_eq!(r, RectF32::of(-103.0, 27.0, ART_MAX - 2.0, ART_MAX));
        inflate(&mut r);
        assert_eq!(r, RectF32::of(-104.0, 25.0, ART_MAX - 1.0, ART_MAX));
        inflate(&mut r);
        assert_eq!(r, RectF32::of(-105.0, 23.0, ART_MAX, ART_MAX));
        inflate(&mut r);
        assert_eq!(r, RectF32::of(-107.0, 21.0, ART_MAX, ART_MAX));
        inflate(&mut r);
        assert_eq!(r, RectF32::of(-109.0, 19.0, ART_MAX, ART_MAX));
        inflate(&mut r);
        assert_eq!(r, RectF32::of(-111.0, 17.0, ART_MAX, ART_MAX));
    }

    #[test]
    fn inflate_almost_min_bounds() {
        let mut r = RectF32::of(ART_MIN + 1.0, ART_MIN + 1.0, ART_MAX, ART_MAX);
        inflate(&mut r);
        assert_eq!(r, RectF32::of(ART_MIN, ART_MIN, ART_MAX, ART_MAX));
    }

    #[test]
    fn inflate_almost_max_bounds() {
        let mut r = RectF32::of(ART_MIN, ART_MIN, ART_MAX - 1.0, ART_MAX - 1.0);
        inflate(&mut r);
        assert_eq!(r, RectF32::of(ART_MIN, ART_MIN, ART_MAX, ART_MAX));
    }

    #[test]
    fn inflate_max_width() {
        let mut r = RectF32::of(ART_MIN, 10.0, ART_MAX, 50.0);
        inflate(&mut r);
        assert_eq!(r, RectF32::of(ART_MIN, 10.0, ART_MAX, 50.0));
    }

    #[test]
    fn inflate_max_height() {
        let mut r = RectF32::of(10.0, ART_MIN, 50.0, ART_MAX);
        inflate(&mut r);
        assert_eq!(r, RectF32::of(10.0, ART_MIN, 50.0, ART_MAX));
    }

    #[test]
    fn deflate_odd_size() {
        let mut r = RectF32::of(-5.0, -5.0, 5.0, 5.0);
        deflate(&mut r);
        assert_eq!(r, RectF32::of(-4.0, -4.0, 4.0, 4.0));
        deflate(&mut r);
        assert_eq!(r, RectF32::of(-3.0, -3.0, 3.0, 3.0));
        deflate(&mut r);
        assert_eq!(r, RectF32::of(-2.0, -2.0, 2.0, 2.0));
        deflate(&mut r);
        assert_eq!(r, RectF32::of(-1.0, -1.0, 1.0, 1.0));
        deflate(&mut r);
        assert_eq!(r, RectF32::of(-1.0, -1.0, 1.0, 1.0));
    }

    #[test]
    fn deflate_even_size() {
        let mut r = RectF32::of(-5.0, -5.0, 4.0, 4.0);
        deflate(&mut r);
        assert_eq!(r, RectF32::of(-4.0, -4.0, 3.0, 3.0));
        deflate(&mut r);
        assert_eq!(r, RectF32::of(-3.0, -3.0, 2.0, 2.0));
        deflate(&mut r);
        assert_eq!(r, RectF32::of(-2.0, -2.0, 1.0, 1.0));
        deflate(&mut r);
        assert_eq!(r, RectF32::of(-1.0, -1.0, 0.0, 0.0));
        deflate(&mut r);
        assert_eq!(r, RectF32::of(-1.0, -1.0, 0.0, 0.0));
    }
}

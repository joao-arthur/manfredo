use crate::cartesian::point::point_f64;

#[derive(PartialEq, Debug, Clone)]
pub struct RectF64 {
    pub min: point_f64::PointF64,
    pub max: point_f64::PointF64,
}

pub struct RectF64Iterator {
    begin: f64,
    end: f64,
}

impl Iterator for RectF64Iterator {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.begin > self.end {
            return None;
        }
        let result = self.begin;
        self.begin += 1.0;
        Some(result)
    }
}

impl DoubleEndedIterator for RectF64Iterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.begin > self.end {
            return None;
        }
        let result = self.end;
        self.end -= 1.0;
        Some(result)
    }
}

impl RectF64 {
    pub fn of(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        RectF64 { min: point_f64::PointF64 { x: x1, y: y1 }, max: point_f64::PointF64 { x: x2, y: y2 } }
    }

    pub fn iter_x(&self) -> RectF64Iterator {
        RectF64Iterator { begin: self.min.x, end: self.max.x }
    }

    pub fn iter_y(&self) -> RectF64Iterator {
        RectF64Iterator { begin: self.min.y, end: self.max.y }
    }
}

impl std::fmt::Display for RectF64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_x(r: &RectF64) -> f64 {
    point_f64::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectF64) -> f64 {
    point_f64::delta_y(&r.min, &r.max)
}

pub fn max_delta(r: &RectF64) -> f64 {
    delta_x(r).max(delta_y(r))
}

pub fn len_x(r: &RectF64) -> f64 {
    delta_x(r) + 1.0
}

pub fn len_y(r: &RectF64) -> f64 {
    delta_y(r) + 1.0
}

pub fn max_len(r: &RectF64) -> f64 {
    len_x(r).max(len_y(r))
}

pub fn inflate(r: &mut RectF64) {
    let is_min_x = r.min.x == point_f64::MIN;
    let is_min_y = r.min.y == point_f64::MIN;
    let is_max_x = r.max.x == point_f64::MAX;
    let is_max_y = r.max.y == point_f64::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return;
    }
    let min_x_modifier = 1.0 - f64::from(is_min_x) + f64::from(is_max_x);
    let min_y_modifier = 1.0 - f64::from(is_min_y) + f64::from(is_max_y);
    let max_x_modifier = 1.0 + f64::from(is_min_x) - f64::from(is_max_x);
    let max_y_modifier = 1.0 + f64::from(is_min_y) - f64::from(is_max_y);
    r.min.x = (r.min.x - min_x_modifier).max(point_f64::MIN);
    r.min.y = (r.min.y - min_y_modifier).max(point_f64::MIN);
    r.max.x = (r.max.x + max_x_modifier).min(point_f64::MAX);
    r.max.y = (r.max.y + max_y_modifier).min(point_f64::MAX);
}

pub fn deflate(r: &mut RectF64) {
    if delta_x(r) < 3.0 || delta_y(r) < 3.0 {
        return;
    }
    r.min.x += 1.0;
    r.min.y += 1.0;
    r.max.x -= 1.0;
    r.max.y -= 1.0;
}

pub fn resize(r: &mut RectF64, size: f64) {
    if size < 3.0 {
        return;
    }
    let diff_x = delta_x(r) + 1.0 - size;
    let diff_y = delta_y(r) + 1.0 - size;
    let temp_min_x = r.min.x + diff_x / 2.0;
    let temp_min_y = r.min.y + diff_y / 2.0;
    let min_x = temp_min_x.clamp(point_f64::MIN, point_f64::MAX - size + 1.0);
    let min_y = temp_min_y.clamp(point_f64::MIN, point_f64::MAX - size + 1.0);
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = min_x + size - 1.0;
    r.max.y = min_y + size - 1.0;
}

pub fn saturating_translate(r: &mut RectF64, delta: &point_f64::PointF64) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = r.min.x + delta.x;
    let temp_min_y = r.min.y + delta.y;
    let min_x = temp_min_x.clamp(point_f64::MIN, point_f64::MAX - dx);
    let min_y = temp_min_y.clamp(point_f64::MIN, point_f64::MAX - dy);
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = min_x + dx;
    r.max.y = min_y + dy;
}

pub fn contains(r: &RectF64, p: &point_f64::PointF64) -> bool {
    p.x >= r.min.x && p.x <= r.max.x && p.y >= r.min.y && p.y <= r.max.y
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_f64::{MAX, MIN, PointF64};

    use super::{RectF64, contains, deflate, delta_x, delta_y, inflate, len_x, len_y, max_delta, max_len, resize, saturating_translate};

    #[test]
    fn rect_f64() {
        assert_eq!(RectF64::of(MIN, -0.0, 0.0, MAX), RectF64 { min: PointF64 { x: MIN, y: -0.0 }, max: PointF64 { x: 0.0, y: MAX } });
        assert_eq!(RectF64::of(MIN, -0.0, 0.0, MAX).to_string(), "((-9007199254740992, -0), (0, 9007199254740991))");
    }

    #[test]
    fn iter_x() {
        assert_eq!(RectF64::of(-6.0, -8.0, -7.0, -6.0).iter_x().collect::<Vec<f64>>(), []);
        assert_eq!(RectF64::of(-6.0, -8.0, -6.0, -6.0).iter_x().collect::<Vec<f64>>(), [-6.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -5.0, -6.0).iter_x().collect::<Vec<f64>>(), [-6.0, -5.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -6.0).iter_x().collect::<Vec<f64>>(), [-6.0, -5.0, -4.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -3.0, -6.0).iter_x().collect::<Vec<f64>>(), [-6.0, -5.0, -4.0, -3.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -3.0, -6.0).iter_x().rev().collect::<Vec<f64>>(), [-3.0, -4.0, -5.0, -6.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -6.0).iter_x().rev().collect::<Vec<f64>>(), [-4.0, -5.0, -6.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -5.0, -6.0).iter_x().rev().collect::<Vec<f64>>(), [-5.0, -6.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -6.0, -6.0).iter_x().rev().collect::<Vec<f64>>(), [-6.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -7.0, -6.0).iter_x().rev().collect::<Vec<f64>>(), []);
    }

    #[test]
    fn iter_y() {
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -9.0).iter_y().collect::<Vec<f64>>(), []);
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -8.0).iter_y().collect::<Vec<f64>>(), [-8.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -7.0).iter_y().collect::<Vec<f64>>(), [-8.0, -7.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -6.0).iter_y().collect::<Vec<f64>>(), [-8.0, -7.0, -6.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -5.0).iter_y().collect::<Vec<f64>>(), [-8.0, -7.0, -6.0, -5.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -5.0).iter_y().rev().collect::<Vec<f64>>(), [-5.0, -6.0, -7.0, -8.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -6.0).iter_y().rev().collect::<Vec<f64>>(), [-6.0, -7.0, -8.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -7.0).iter_y().rev().collect::<Vec<f64>>(), [-7.0, -8.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -8.0).iter_y().rev().collect::<Vec<f64>>(), [-8.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -9.0).iter_y().rev().collect::<Vec<f64>>(), []);
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectF64::of(0.0, 0.0, 0.0, MAX)), 0.0);
        assert_eq!(delta_x(&RectF64::of(0.0, -4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0)), 0.0);
        assert_eq!(delta_x(&RectF64::of(0.0, 0.0, MAX, 0.0)), MAX);
        assert_eq!(delta_x(&RectF64::of(-4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0, 0.0)), MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectF64::of(0.0, 0.0, MAX, 0.0)), 0.0);
        assert_eq!(delta_y(&RectF64::of(-4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0, 0.0)), 0.0);
        assert_eq!(delta_y(&RectF64::of(0.0, 0.0, 0.0, MAX)), MAX);
        assert_eq!(delta_y(&RectF64::of(0.0, -4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0)), MAX);
    }

    #[test]
    fn test_max_delta() {
        assert_eq!(max_delta(&RectF64::of(0.0, 5.0, 10.0, 10.0)), 10.0);
        assert_eq!(max_delta(&RectF64::of(-10.0, -10.0, -5.0, 0.0)), 10.0);
        assert_eq!(max_delta(&RectF64::of(-5.0, 0.0, 5.0, 5.0)), 10.0);
    }

    #[test]
    fn max_delta_0() {
        assert_eq!(max_delta(&RectF64::of(0.0, 0.0, 0.0, 0.0)), 0.0);
        assert_eq!(max_delta(&RectF64::of(1.0, 1.0, 1.0, 1.0)), 0.0);
        assert_eq!(max_delta(&RectF64::of(-1.0, -1.0, -1.0, -1.0)), 0.0);
        assert_eq!(max_delta(&RectF64::of(5.0, 10.0, 5.0, 10.0)), 0.0);
    }

    #[test]
    fn max_delta_1() {
        assert_eq!(max_delta(&RectF64::of(0.0, 0.0, 1.0, 1.0)), 1.0);
        assert_eq!(max_delta(&RectF64::of(5.0, 5.0, 6.0, 6.0)), 1.0);
        assert_eq!(max_delta(&RectF64::of(-6.0, -6.0, -5.0, -5.0)), 1.0);
        assert_eq!(max_delta(&RectF64::of(0.0, 0.0, 0.0, 1.0)), 1.0);
        assert_eq!(max_delta(&RectF64::of(5.0, 9.0, 5.0, 10.0)), 1.0);
    }

    #[test]
    fn max_delta_bounds() {
        assert_eq!(max_delta(&RectF64::of(MIN + 1.0, MIN + 2.0, 0.0, 0.0)), MAX);
        assert_eq!(max_delta(&RectF64::of(MIN + 2.0, MIN + 1.0, 0.0, 0.0)), MAX);
        assert_eq!(max_delta(&RectF64::of(0.0, 0.0, MAX - 1.0, MAX)), MAX);
        assert_eq!(max_delta(&RectF64::of(0.0, 0.0, MAX, MAX - 1.0)), MAX);
    }

    #[test]
    fn test_len_x() {
        assert_eq!(len_x(&RectF64::of(0.0, 0.0, 0.0, MAX)), 1.0);
        assert_eq!(len_x(&RectF64::of(0.0, -4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0)), 1.0);
        assert_eq!(len_x(&RectF64::of(0.0, 0.0, MAX - 1.0, 0.0)), MAX);
        assert_eq!(len_x(&RectF64::of(-4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_494.0, 0.0)), MAX);
    }

    #[test]
    fn test_len_y() {
        assert_eq!(len_y(&RectF64::of(0.0, 0.0, MAX, 0.0)), 1.0);
        assert_eq!(len_y(&RectF64::of(-4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0, 0.0)), 1.0);
        assert_eq!(len_y(&RectF64::of(0.0, 0.0, 0.0, MAX - 1.0)), MAX);
        assert_eq!(len_y(&RectF64::of(0.0, -4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_494.0)), MAX);
    }

    #[test]
    fn test_max_len() {
        assert_eq!(max_len(&RectF64::of(0.0, 5.0, 10.0, 10.0)), 11.0);
        assert_eq!(max_len(&RectF64::of(-10.0, -10.0, -5.0, 0.0)), 11.0);
        assert_eq!(max_len(&RectF64::of(-5.0, 0.0, 5.0, 5.0)), 11.0);
    }

    #[test]
    fn max_len_1() {
        assert_eq!(max_len(&RectF64::of(0.0, 0.0, 0.0, 0.0)), 1.0);
        assert_eq!(max_len(&RectF64::of(1.0, 1.0, 1.0, 1.0)), 1.0);
        assert_eq!(max_len(&RectF64::of(-1.0, -1.0, -1.0, -1.0)), 1.0);
        assert_eq!(max_len(&RectF64::of(5.0, 10.0, 5.0, 10.0)), 1.0);
    }

    #[test]
    fn max_len_2() {
        assert_eq!(max_len(&RectF64::of(0.0, 0.0, 1.0, 1.0)), 2.0);
        assert_eq!(max_len(&RectF64::of(5.0, 5.0, 6.0, 6.0)), 2.0);
        assert_eq!(max_len(&RectF64::of(-6.0, -6.0, -5.0, -5.0)), 2.0);
        assert_eq!(max_len(&RectF64::of(0.0, 0.0, 0.0, 1.0)), 2.0);
        assert_eq!(max_len(&RectF64::of(5.0, 9.0, 5.0, 10.0)), 2.0);
    }

    #[test]
    fn max_len_bounds() {
        assert_eq!(max_len(&RectF64::of(MIN + 2.0, MIN + 3.0, 0.0, 0.0)), MAX);
        assert_eq!(max_len(&RectF64::of(MIN + 3.0, MIN + 2.0, 0.0, 0.0)), MAX);
        assert_eq!(max_len(&RectF64::of(0.0, 0.0, MAX - 2.0, MAX - 1.0)), MAX);
        assert_eq!(max_len(&RectF64::of(0.0, 0.0, MAX - 1.0, MAX - 2.0)), MAX);
    }

    #[test]
    fn inflate_min_bounds() {
        let mut r = RectF64::of(MIN + 7.0, MIN + 2.0, 4.0, 13.0);
        inflate(&mut r);
        assert_eq!(r, RectF64::of(MIN + 6.0, MIN + 1.0, 5.0, 14.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(MIN + 5.0, MIN, 6.0, 15.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(MIN + 4.0, MIN, 7.0, 17.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(MIN + 3.0, MIN, 8.0, 19.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(MIN + 2.0, MIN, 9.0, 21.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(MIN + 1.0, MIN, 10.0, 23.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(MIN, MIN, 11.0, 25.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(MIN, MIN, 13.0, 27.0));
    }

    #[test]
    fn inflate_max_bounds() {
        let mut r = RectF64::of(-100.0, 30.0, MAX - 5.0, MAX - 3.0);
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-101.0, 29.0, MAX - 4.0, MAX - 2.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-102.0, 28.0, MAX - 3.0, MAX - 1.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-103.0, 27.0, MAX - 2.0, MAX));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-104.0, 25.0, MAX - 1.0, MAX));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-105.0, 23.0, MAX, MAX));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-107.0, 21.0, MAX, MAX));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-109.0, 19.0, MAX, MAX));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-111.0, 17.0, MAX, MAX));
    }

    #[test]
    fn inflate_almost_min_bounds() {
        let mut r = RectF64::of(MIN + 1.0, MIN + 1.0, MAX, MAX);
        inflate(&mut r);
        assert_eq!(r, RectF64::of(MIN, MIN, MAX, MAX));
    }

    #[test]
    fn inflate_almost_max_bounds() {
        let mut r = RectF64::of(MIN, MIN, MAX - 1.0, MAX - 1.0);
        inflate(&mut r);
        assert_eq!(r, RectF64::of(MIN, MIN, MAX, MAX));
    }

    #[test]
    fn inflate_max_width() {
        let mut r = RectF64::of(MIN, 10.0, MAX, 50.0);
        inflate(&mut r);
        assert_eq!(r, RectF64::of(MIN, 10.0, MAX, 50.0));
    }

    #[test]
    fn inflate_max_height() {
        let mut r = RectF64::of(10.0, MIN, 50.0, MAX);
        inflate(&mut r);
        assert_eq!(r, RectF64::of(10.0, MIN, 50.0, MAX));
    }

    #[test]
    fn deflate_odd() {
        let mut r = RectF64::of(-5.0, -5.0, 5.0, 5.0);
        deflate(&mut r);
        assert_eq!(r, RectF64::of(-4.0, -4.0, 4.0, 4.0));
        deflate(&mut r);
        assert_eq!(r, RectF64::of(-3.0, -3.0, 3.0, 3.0));
        deflate(&mut r);
        assert_eq!(r, RectF64::of(-2.0, -2.0, 2.0, 2.0));
        deflate(&mut r);
        assert_eq!(r, RectF64::of(-1.0, -1.0, 1.0, 1.0));
        deflate(&mut r);
        assert_eq!(r, RectF64::of(-1.0, -1.0, 1.0, 1.0));
    }

    #[test]
    fn deflate_even() {
        let mut r = RectF64::of(-5.0, -5.0, 4.0, 4.0);
        deflate(&mut r);
        assert_eq!(r, RectF64::of(-4.0, -4.0, 3.0, 3.0));
        deflate(&mut r);
        assert_eq!(r, RectF64::of(-3.0, -3.0, 2.0, 2.0));
        deflate(&mut r);
        assert_eq!(r, RectF64::of(-2.0, -2.0, 1.0, 1.0));
        deflate(&mut r);
        assert_eq!(r, RectF64::of(-1.0, -1.0, 0.0, 0.0));
        deflate(&mut r);
        assert_eq!(r, RectF64::of(-1.0, -1.0, 0.0, 0.0));
    }

    #[test]
    fn resize_odd() {
        let mut r = RectF64::of(-5.0, -5.0, 5.0, 5.0);
        resize(&mut r, 11.0);
        assert_eq!(r, RectF64::of(-5.0, -5.0, 5.0, 5.0));
        resize(&mut r, 9.0);
        assert_eq!(r, RectF64::of(-4.0, -4.0, 4.0, 4.0));
        resize(&mut r, 7.0);
        assert_eq!(r, RectF64::of(-3.0, -3.0, 3.0, 3.0));
        resize(&mut r, 5.0);
        assert_eq!(r, RectF64::of(-2.0, -2.0, 2.0, 2.0));
        resize(&mut r, 3.0);
        assert_eq!(r, RectF64::of(-1.0, -1.0, 1.0, 1.0));
        resize(&mut r, 1.0);
        assert_eq!(r, RectF64::of(-1.0, -1.0, 1.0, 1.0));
        resize(&mut r, 1.0);
        assert_eq!(r, RectF64::of(-1.0, -1.0, 1.0, 1.0));
        resize(&mut r, 3.0);
        assert_eq!(r, RectF64::of(-1.0, -1.0, 1.0, 1.0));
        resize(&mut r, 5.0);
        assert_eq!(r, RectF64::of(-2.0, -2.0, 2.0, 2.0));
        resize(&mut r, 7.0);
        assert_eq!(r, RectF64::of(-3.0, -3.0, 3.0, 3.0));
    }

    #[test]
    fn resize_even() {
        let mut r = RectF64::of(-5.0, -5.0, 4.0, 4.0);
        resize(&mut r, 10.0);
        assert_eq!(r, RectF64::of(-5.0, -5.0, 4.0, 4.0));
        resize(&mut r, 8.0);
        assert_eq!(r, RectF64::of(-4.0, -4.0, 3.0, 3.0));
        resize(&mut r, 6.0);
        assert_eq!(r, RectF64::of(-3.0, -3.0, 2.0, 2.0));
        resize(&mut r, 4.0);
        assert_eq!(r, RectF64::of(-2.0, -2.0, 1.0, 1.0));
        resize(&mut r, 2.0);
        assert_eq!(r, RectF64::of(-2.0, -2.0, 1.0, 1.0));
        resize(&mut r, 2.0);
        assert_eq!(r, RectF64::of(-2.0, -2.0, 1.0, 1.0));
        resize(&mut r, 4.0);
        assert_eq!(r, RectF64::of(-2.0, -2.0, 1.0, 1.0));
        resize(&mut r, 6.0);
        assert_eq!(r, RectF64::of(-3.0, -3.0, 2.0, 2.0));
    }

    #[test]
    fn resize_even_2nd_scenario() {
        let mut r = RectF64::of(-4.0, -4.0, 5.0, 5.0);
        resize(&mut r, 10.0);
        assert_eq!(r, RectF64::of(-4.0, -4.0, 5.0, 5.0));
        resize(&mut r, 8.0);
        assert_eq!(r, RectF64::of(-3.0, -3.0, 4.0, 4.0));
        resize(&mut r, 6.0);
        assert_eq!(r, RectF64::of(-2.0, -2.0, 3.0, 3.0));
        resize(&mut r, 4.0);
        assert_eq!(r, RectF64::of(-1.0, -1.0, 2.0, 2.0));
        resize(&mut r, 2.0);
        assert_eq!(r, RectF64::of(-1.0, -1.0, 2.0, 2.0));
        resize(&mut r, 2.0);
        assert_eq!(r, RectF64::of(-1.0, -1.0, 2.0, 2.0));
        resize(&mut r, 4.0);
        assert_eq!(r, RectF64::of(-1.0, -1.0, 2.0, 2.0));
        resize(&mut r, 6.0);
        assert_eq!(r, RectF64::of(-2.0, -2.0, 3.0, 3.0));
    }

    #[test]
    fn resize_odd_min_bounds_big_delta() {
        let mut r = RectF64::of(MIN, MIN, MIN + 2.0, MIN + 2.0);
        resize(&mut r, MAX);
        assert_eq!(r, RectF64::of(MIN, MIN, -2.0, -2.0));
    }

    #[test]
    fn resize_even_min_bounds_big_delta() {
        let mut r = RectF64::of(MIN, MIN, MIN + 3.0, MIN + 3.0);
        resize(&mut r, MAX - 1.0);
        assert_eq!(r, RectF64::of(MIN, MIN, -3.0, -3.0));
    }

    #[test]
    fn resize_odd_max_bounds_big_delta() {
        let mut r = RectF64::of(MAX - 2.0, MAX - 2.0, MAX, MAX);
        resize(&mut r, MAX);
        assert_eq!(r, RectF64::of(1.0, 1.0, MAX, MAX));
    }

    #[test]
    fn resize_even_max_bounds_big_delta() {
        let mut r = RectF64::of(MAX - 3.0, MAX - 3.0, MAX, MAX);
        resize(&mut r, MAX - 1.0);
        assert_eq!(r, RectF64::of(2.0, 2.0, MAX, MAX));
    }

    #[test]
    fn test_saturating_translate() {
        let mut r = RectF64::of(0.0, 0.0, 10.0, 10.0);
        saturating_translate(&mut r, &PointF64::of(10.0, 10.0));
        assert_eq!(r, RectF64::of(10.0, 10.0, 20.0, 20.0));
        saturating_translate(&mut r, &PointF64::of(-20.0, -20.0));
        assert_eq!(r, RectF64::of(-10.0, -10.0, 0.0, 0.0));
        saturating_translate(&mut r, &PointF64::of(2.0, 2.0));
        assert_eq!(r, RectF64::of(-8.0, -8.0, 2.0, 2.0));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut r = RectF64::of(MIN + 5.0, MIN + 10.0, -100.0, -100.0);
        saturating_translate(&mut r, &PointF64::of(-10.0, -10.0));
        assert_eq!(r, RectF64::of(MIN, MIN, -105.0, -110.0));
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut r = RectF64::of(100.0, 100.0, MAX - 5.0, MAX - 10.0);
        saturating_translate(&mut r, &PointF64::of(20.0, 20.0));
        assert_eq!(r, RectF64::of(105.0, 110.0, MAX, MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_big_delta() {
        let mut r = RectF64::of(MIN, MIN, MIN + 10.0, MIN + 10.0);
        saturating_translate(&mut r, &PointF64::min());
        assert_eq!(r, RectF64::of(MIN, MIN, MIN + 10.0, MIN + 10.0));
    }

    #[test]
    fn saturating_translate_max_bounds_big_delta() {
        let mut r = RectF64::of(MAX - 10.0, MAX - 10.0, MAX, MAX);
        saturating_translate(&mut r, &PointF64::max());
        assert_eq!(r, RectF64::of(MAX - 10.0, MAX - 10.0, MAX, MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_big_rect_big_delta() {
        let mut r = RectF64::of(MIN + 1.0, MIN + 1.0, MAX, MAX);
        saturating_translate(&mut r, &PointF64::min());
        assert_eq!(r, RectF64::of(MIN, MIN, MAX - 1.0, MAX - 1.0));
    }

    #[test]
    fn saturating_translate_max_bounds_big_rect_big_delta() {
        let mut r = RectF64::of(MIN, MIN, MAX - 1.0, MAX - 1.0);
        saturating_translate(&mut r, &PointF64::max());
        assert_eq!(r, RectF64::of(MIN + 1.0, MIN + 1.0, MAX, MAX));
    }

    #[test]
    fn contains_inside_borders() {
        assert!(contains(&RectF64::of(MIN + 1.0, MIN + 1.0, -1.0, -1.0), &PointF64::of(MIN + 1.0, MIN + 1.0)));
        assert!(contains(&RectF64::of(MIN + 1.0, MIN + 1.0, -1.0, -1.0), &PointF64::of(MIN + 1.0, -1.0)));
        assert!(contains(&RectF64::of(MIN + 1.0, MIN + 1.0, -1.0, -1.0), &PointF64::of(-1.0, MIN + 1.0)));
        assert!(contains(&RectF64::of(MIN + 1.0, MIN + 1.0, -1.0, -1.0), &PointF64::of(-1.0, -1.0)));

        assert!(contains(&RectF64::of(1.0, 1.0, MAX - 1.0, MAX - 1.0), &PointF64::of(1.0, 1.0)));
        assert!(contains(&RectF64::of(1.0, 1.0, MAX - 1.0, MAX - 1.0), &PointF64::of(1.0, MAX - 1.0)));
        assert!(contains(&RectF64::of(1.0, 1.0, MAX - 1.0, MAX - 1.0), &PointF64::of(MAX - 1.0, 1.0)));
        assert!(contains(&RectF64::of(1.0, 1.0, MAX - 1.0, MAX - 1.0), &PointF64::of(MAX - 1.0, MAX - 1.0)));
    }

    #[test]
    fn contains_outside_borders() {
        let r_negative = RectF64::of(MIN + 1.0, MIN + 1.0, -1.0, -1.0);
        assert!(!contains(&r_negative, &PointF64::of(MIN, MIN)));
        assert!(!contains(&r_negative, &PointF64::of(MIN, 0.0)));
        assert!(!contains(&r_negative, &PointF64::of(0.0, MIN)));
        assert!(!contains(&r_negative, &PointF64::of(0.0, 0.0)));
        assert!(!contains(&r_negative, &PointF64::of(MIN + 1.0, MIN)));
        assert!(!contains(&r_negative, &PointF64::of(MIN + 1.0, 0.0)));
        assert!(!contains(&r_negative, &PointF64::of(-1.0, MIN)));
        assert!(!contains(&r_negative, &PointF64::of(-1.0, 0.0)));
        assert!(!contains(&r_negative, &PointF64::of(MIN, MIN + 1.0)));
        assert!(!contains(&r_negative, &PointF64::of(MIN, -1.0)));
        assert!(!contains(&r_negative, &PointF64::of(0.0, MIN + 1.0)));
        assert!(!contains(&r_negative, &PointF64::of(0.0, -1.0)));
        let r_positive = RectF64::of(1.0, 1.0, MAX - 1.0, MAX - 1.0);
        assert!(!contains(&r_positive, &PointF64::of(0.0, 0.0)));
        assert!(!contains(&r_positive, &PointF64::of(0.0, MAX)));
        assert!(!contains(&r_positive, &PointF64::of(MAX, 0.0)));
        assert!(!contains(&r_positive, &PointF64::of(MAX, MAX)));
        assert!(!contains(&r_positive, &PointF64::of(1.0, 0.0)));
        assert!(!contains(&r_positive, &PointF64::of(1.0, MAX)));
        assert!(!contains(&r_positive, &PointF64::of(MAX - 1.0, 0.0)));
        assert!(!contains(&r_positive, &PointF64::of(MAX - 1.0, MAX)));
        assert!(!contains(&r_positive, &PointF64::of(0.0, 1.0)));
        assert!(!contains(&r_positive, &PointF64::of(0.0, MAX - 1.0)));
        assert!(!contains(&r_positive, &PointF64::of(MAX, 1.0)));
        assert!(!contains(&r_positive, &PointF64::of(MAX, MAX - 1.0)));
    }

    #[test]
    fn contains_inside() {
        assert!(contains(&RectF64::of(MIN + 1.0, MIN + 1.0, -1.0, -1.0), &PointF64::of(MIN + 10.0, MIN + 10.0)));
        assert!(contains(&RectF64::of(MIN + 1.0, MIN + 1.0, -1.0, -1.0), &PointF64::of(-10.0, -10.0)));

        assert!(contains(&RectF64::of(1.0, 1.0, MAX - 1.0, MAX - 1.0), &PointF64::of(10.0, 10.0)));
        assert!(contains(&RectF64::of(1.0, 1.0, MAX - 1.0, MAX - 1.0), &PointF64::of(MAX - 10.0, MAX - 10.0)));
    }
}

use crate::cartesian::point::point_f32;

#[derive(PartialEq, Debug, Clone)]
pub struct RectF32 {
    pub min: point_f32::PointF32,
    pub max: point_f32::PointF32,
}

impl RectF32 {
    pub fn of(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        RectF32 {
            min: point_f32::PointF32 { x: x1, y: y1 },
            max: point_f32::PointF32 { x: x2, y: y2 },
        }
    }
}

impl std::fmt::Display for RectF32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min.to_string(), self.max.to_string())
    }
}

pub fn delta_x(r: &RectF32) -> f32 {
    point_f32::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectF32) -> f32 {
    point_f32::delta_y(&r.min, &r.max)
}

//pub fn inflate(r: &mut RectF32) {
//    let is_min_x = r.min.x == f32::MIN;
//    let is_max_x = r.max.x == f32::MAX;
//    let is_min_y = r.min.y == f32::MIN;
//    let is_max_y = r.max.y == f32::MAX;
//    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
//        return;
//    }
//    let min_x_modifier = 1.0 - f32::from(is_min_x) + f32::from(is_max_x);
//    let max_x_modifier = 1.0 + f32::from(is_min_x) - f32::from(is_max_x);
//    let min_y_modifier = 1.0 - f32::from(is_min_y) + f32::from(is_max_y);
//    let max_y_modifier = 1.0 + f32::from(is_min_y) - f32::from(is_max_y);
//    r.min.x = min_x_modifier;
//    r.max.x = max_x_modifier;
//    r.min.y = min_y_modifier;
//    r.max.y = max_y_modifier;
//}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_f32::PointF32;

    use super::{RectF32, delta_x, delta_y};

    #[test]
    fn rect_f32() {
        assert_eq!(
            RectF32::of(-16_777_216.0, -0.0, 0.0, 16_777_215.0),
            RectF32 {
                min: PointF32 { x: -16_777_216.0, y: -0.0 },
                max: PointF32 { x: 0.0, y: 16_777_215.0 }
            }
        );
        assert_eq!(
            RectF32::of(-16_777_216.0, -0.0, 0.0, 16_777_215.0).to_string(),
            "((-16777216, -0), (0, 16777215))"
        );
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectF32::of(0.0, 0.0, 0.0, 16_777_215.0)), 0.0);
        assert_eq!(delta_x(&RectF32::of(0.0, -8_388_608.0, 0.0, 8_388_607.0)), 0.0);
        assert_eq!(delta_x(&RectF32::of(0.0, 0.0, 16_777_215.0, 0.0)), 16_777_215.0);
        assert_eq!(delta_x(&RectF32::of(-8_388_608.0, 0.0, 8_388_607.0, 0.0)), 16_777_215.0);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectF32::of(0.0, 0.0, 16_777_215.0, 0.0)), 0.0);
        assert_eq!(delta_y(&RectF32::of(-8_388_608.0, 0.0, 8_388_607.0, 0.0)), 0.0);
        assert_eq!(delta_y(&RectF32::of(0.0, 0.0, 0.0, 16_777_215.0)), 16_777_215.0);
        assert_eq!(delta_y(&RectF32::of(0.0, -8_388_608.0, 0.0, 8_388_607.0)), 16_777_215.0);
    }
}

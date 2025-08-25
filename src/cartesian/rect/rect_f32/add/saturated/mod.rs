use crate::cartesian::{
    point::point_f32::{MAX, MIN, PointF32},
    rect::rect_f32::RectF32,
};

pub fn saturating_add_assign(r: &mut RectF32, delta: &RectF32) {
    r.min.x = (r.min.x + delta.min.x).clamp(MIN, MAX);
    r.min.y = (r.min.y + delta.min.y).clamp(MIN, MAX);
    r.max.x = (r.max.x + delta.max.x).clamp(MIN, MAX);
    r.max.y = (r.max.y + delta.max.y).clamp(MIN, MAX);
}

pub fn saturating_add(r: &RectF32, delta: &RectF32) -> RectF32 {
    let min_x = (r.min.x + delta.min.x).clamp(MIN, MAX);
    let min_y = (r.min.y + delta.min.y).clamp(MIN, MAX);
    let max_x = (r.max.x + delta.max.x).clamp(MIN, MAX);
    let max_y = (r.max.y + delta.max.y).clamp(MIN, MAX);
    RectF32 { min: PointF32 { x: min_x, y: min_y }, max: PointF32 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod test_saturating_add_assign;

#[cfg(test)]
mod test_saturating_add;

use crate::cartesian::{point::point_i32::PointI32, rect::rect_i32::RectI32};

pub fn saturating_add_assign(r: &mut RectI32, delta: &RectI32) {
    r.min.x = r.min.x.saturating_add(delta.min.x);
    r.min.y = r.min.y.saturating_add(delta.min.y);
    r.max.x = r.max.x.saturating_add(delta.max.x);
    r.max.y = r.max.y.saturating_add(delta.max.y);
}

pub fn saturating_add(r: &RectI32, delta: &RectI32) -> RectI32 {
    let min_x = r.min.x.saturating_add(delta.min.x);
    let min_y = r.min.y.saturating_add(delta.min.y);
    let max_x = r.max.x.saturating_add(delta.max.x);
    let max_y = r.max.y.saturating_add(delta.max.y);
    RectI32 { min: PointI32 { x: min_x, y: min_y }, max: PointI32 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod test_saturating_add_assign;

#[cfg(test)]
mod test_saturating_add;

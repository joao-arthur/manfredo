use crate::cartesian::{point::point_i16::PointI16, rect::rect_i16::RectI16};

pub fn wrapping_add_assign(r: &mut RectI16, delta: &RectI16) {
    r.min.x = r.min.x.wrapping_add(delta.min.x);
    r.min.y = r.min.y.wrapping_add(delta.min.y);
    r.max.x = r.max.x.wrapping_add(delta.max.x);
    r.max.y = r.max.y.wrapping_add(delta.max.y);
}

pub fn wrapping_add(r: &RectI16, delta: &RectI16) -> RectI16 {
    let min_x = r.min.x.wrapping_add(delta.min.x);
    let min_y = r.min.y.wrapping_add(delta.min.y);
    let max_x = r.max.x.wrapping_add(delta.max.x);
    let max_y = r.max.y.wrapping_add(delta.max.y);
    RectI16 { min: PointI16 { x: min_x, y: min_y }, max: PointI16 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod test_wrapping_add_assign;

#[cfg(test)]
mod test_wrapping_add;

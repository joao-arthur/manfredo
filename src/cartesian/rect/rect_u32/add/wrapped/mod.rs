use crate::cartesian::{
    point::point_u32::PointU32,
    rect::{rect_i32::RectI32, rect_u32::RectU32},
};

pub fn wrapping_add_assign(r: &mut RectU32, delta: &RectI32) {
    r.min.x = r.min.x.wrapping_add_signed(delta.min.x);
    r.min.y = r.min.y.wrapping_add_signed(delta.min.y);
    r.max.x = r.max.x.wrapping_add_signed(delta.max.x);
    r.max.y = r.max.y.wrapping_add_signed(delta.max.y);
}

pub fn wrapping_add(r: &RectU32, delta: &RectI32) -> RectU32 {
    let min_x = r.min.x.wrapping_add_signed(delta.min.x);
    let min_y = r.min.y.wrapping_add_signed(delta.min.y);
    let max_x = r.max.x.wrapping_add_signed(delta.max.x);
    let max_y = r.max.y.wrapping_add_signed(delta.max.y);
    RectU32 { min: PointU32 { x: min_x, y: min_y }, max: PointU32 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod test_wrapping_add_assign;

#[cfg(test)]
mod test_wrapping_add;

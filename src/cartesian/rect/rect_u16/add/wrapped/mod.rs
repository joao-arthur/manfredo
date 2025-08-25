use crate::cartesian::{
    point::point_u16::PointU16,
    rect::{rect_i16::RectI16, rect_u16::RectU16},
};

pub fn wrapping_add_assign(r: &mut RectU16, delta: &RectI16) {
    r.min.x = r.min.x.wrapping_add_signed(delta.min.x);
    r.min.y = r.min.y.wrapping_add_signed(delta.min.y);
    r.max.x = r.max.x.wrapping_add_signed(delta.max.x);
    r.max.y = r.max.y.wrapping_add_signed(delta.max.y);
}

pub fn wrapping_add(r: &RectU16, delta: &RectI16) -> RectU16 {
    let min_x = r.min.x.wrapping_add_signed(delta.min.x);
    let min_y = r.min.y.wrapping_add_signed(delta.min.y);
    let max_x = r.max.x.wrapping_add_signed(delta.max.x);
    let max_y = r.max.y.wrapping_add_signed(delta.max.y);
    RectU16 { min: PointU16 { x: min_x, y: min_y }, max: PointU16 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod test_wrapping_add_assign;

#[cfg(test)]
mod test_wrapping_add;

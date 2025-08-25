use crate::cartesian::{
    point::point_u8::PointU8,
    rect::{rect_i8::RectI8, rect_u8::RectU8},
};

pub fn saturating_add_assign(r: &mut RectU8, delta: &RectI8) {
    r.min.x = r.min.x.saturating_add_signed(delta.min.x);
    r.min.y = r.min.y.saturating_add_signed(delta.min.y);
    r.max.x = r.max.x.saturating_add_signed(delta.max.x);
    r.max.y = r.max.y.saturating_add_signed(delta.max.y);
}

pub fn saturating_add(r: &RectU8, delta: &RectI8) -> RectU8 {
    let min_x = r.min.x.saturating_add_signed(delta.min.x);
    let min_y = r.min.y.saturating_add_signed(delta.min.y);
    let max_x = r.max.x.saturating_add_signed(delta.max.x);
    let max_y = r.max.y.saturating_add_signed(delta.max.y);
    RectU8 { min: PointU8 { x: min_x, y: min_y }, max: PointU8 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod test_saturating_add_assign;

#[cfg(test)]
mod test_saturating_add;

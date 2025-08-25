use crate::matrix::{
    point::point_u32::PointU32,
    rect::{rect_i32::RectI32, rect_u32::RectU32},
};

pub fn wrapping_add_assign(r: &mut RectU32, delta: &RectI32) {
    r.min.row = r.min.row.wrapping_add_signed(delta.min.row);
    r.min.col = r.min.col.wrapping_add_signed(delta.min.col);
    r.max.row = r.max.row.wrapping_add_signed(delta.max.row);
    r.max.col = r.max.col.wrapping_add_signed(delta.max.col);
}

pub fn wrapping_add(r: &RectU32, delta: &RectI32) -> RectU32 {
    let min_row = r.min.row.wrapping_add_signed(delta.min.row);
    let min_col = r.min.col.wrapping_add_signed(delta.min.col);
    let max_row = r.max.row.wrapping_add_signed(delta.max.row);
    let max_col = r.max.col.wrapping_add_signed(delta.max.col);
    RectU32 { min: PointU32 { row: min_row, col: min_col }, max: PointU32 { row: max_row, col: max_col } }
}

#[cfg(test)]
mod test_wrapping_add_assign;

#[cfg(test)]
mod test_wrapping_add;

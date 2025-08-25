use crate::matrix::{
    point::point_u8::PointU8,
    rect::{rect_i8::RectI8, rect_u8::RectU8},
};

pub fn saturating_add_assign(r: &mut RectU8, delta: &RectI8) {
    r.min.row = r.min.row.saturating_add_signed(delta.min.row);
    r.min.col = r.min.col.saturating_add_signed(delta.min.col);
    r.max.row = r.max.row.saturating_add_signed(delta.max.row);
    r.max.col = r.max.col.saturating_add_signed(delta.max.col);
}

pub fn saturating_add(r: &RectU8, delta: &RectI8) -> RectU8 {
    let min_row = r.min.row.saturating_add_signed(delta.min.row);
    let min_col = r.min.col.saturating_add_signed(delta.min.col);
    let max_row = r.max.row.saturating_add_signed(delta.max.row);
    let max_col = r.max.col.saturating_add_signed(delta.max.col);
    RectU8 { min: PointU8 { row: min_row, col: min_col }, max: PointU8 { row: max_row, col: max_col } }
}

#[cfg(test)]
mod test_saturating_add_assign;

#[cfg(test)]
mod test_saturating_add;

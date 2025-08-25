use crate::matrix::{
    point::point_u64::PointU64,
    rect::{rect_i64::RectI64, rect_u64::RectU64},
};

pub fn saturating_add_assign(r: &mut RectU64, delta: &RectI64) {
    r.min.row = r.min.row.saturating_add_signed(delta.min.row);
    r.min.col = r.min.col.saturating_add_signed(delta.min.col);
    r.max.row = r.max.row.saturating_add_signed(delta.max.row);
    r.max.col = r.max.col.saturating_add_signed(delta.max.col);
}

pub fn saturating_add(r: &RectU64, delta: &RectI64) -> RectU64 {
    let min_row = r.min.row.saturating_add_signed(delta.min.row);
    let min_col = r.min.col.saturating_add_signed(delta.min.col);
    let max_row = r.max.row.saturating_add_signed(delta.max.row);
    let max_col = r.max.col.saturating_add_signed(delta.max.col);
    RectU64 { min: PointU64 { row: min_row, col: min_col }, max: PointU64 { row: max_row, col: max_col } }
}

#[cfg(test)]
mod test_saturating_add_assign;

#[cfg(test)]
mod test_saturating_add;

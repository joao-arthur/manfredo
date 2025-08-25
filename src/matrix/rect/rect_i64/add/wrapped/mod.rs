use crate::matrix::{point::point_i64::PointI64, rect::rect_i64::RectI64};

pub fn wrapping_add_assign(r: &mut RectI64, delta: &RectI64) {
    r.min.row = r.min.row.wrapping_add(delta.min.row);
    r.min.col = r.min.col.wrapping_add(delta.min.col);
    r.max.row = r.max.row.wrapping_add(delta.max.row);
    r.max.col = r.max.col.wrapping_add(delta.max.col);
}

pub fn wrapping_add(r: &RectI64, delta: &RectI64) -> RectI64 {
    let min_row = r.min.row.wrapping_add(delta.min.row);
    let min_col = r.min.col.wrapping_add(delta.min.col);
    let max_row = r.max.row.wrapping_add(delta.max.row);
    let max_col = r.max.col.wrapping_add(delta.max.col);
    RectI64 { min: PointI64 { row: min_row, col: min_col }, max: PointI64 { row: max_row, col: max_col } }
}

#[cfg(test)]
mod test_wrapping_add_assign;

#[cfg(test)]
mod test_wrapping_add;

use crate::matrix::{point::point_i16::PointI16, rect::rect_i16::RectI16};

pub fn saturating_add_assign(r: &mut RectI16, delta: &RectI16) {
    r.min.row = r.min.row.saturating_add(delta.min.row);
    r.min.col = r.min.col.saturating_add(delta.min.col);
    r.max.row = r.max.row.saturating_add(delta.max.row);
    r.max.col = r.max.col.saturating_add(delta.max.col);
}

pub fn saturating_add(r: &RectI16, delta: &RectI16) -> RectI16 {
    let min_row = r.min.row.saturating_add(delta.min.row);
    let min_col = r.min.col.saturating_add(delta.min.col);
    let max_row = r.max.row.saturating_add(delta.max.row);
    let max_col = r.max.col.saturating_add(delta.max.col);
    RectI16 { min: PointI16 { row: min_row, col: min_col }, max: PointI16 { row: max_row, col: max_col } }
}

#[cfg(test)]
mod test_saturating_add_assign;

#[cfg(test)]
mod test_saturating_add;

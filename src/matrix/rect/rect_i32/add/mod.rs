use crate::matrix::{point::point_i32::PointI32, rect::rect_i32::RectI32};

pub fn try_checked_add_assign(r: &mut RectI32, delta: &RectI32) -> Option<()> {
    let min_row = r.min.row.checked_add(delta.min.row)?;
    let min_col = r.min.col.checked_add(delta.min.col)?;
    let max_row = r.max.row.checked_add(delta.max.row)?;
    let max_col = r.max.col.checked_add(delta.max.col)?;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
    Some(())
}

pub fn try_checked_add(r: &RectI32, delta: &RectI32) -> Option<RectI32> {
    let min_row = r.min.row.checked_add(delta.min.row)?;
    let min_col = r.min.col.checked_add(delta.min.col)?;
    let max_row = r.max.row.checked_add(delta.max.row)?;
    let max_col = r.max.col.checked_add(delta.max.col)?;
    Some(RectI32 { min: PointI32 { row: min_row, col: min_col }, max: PointI32 { row: max_row, col: max_col } })
}

pub fn checked_add_assign(r: &mut RectI32, delta: &RectI32) {
    try_checked_add_assign(r, delta).unwrap()
}

pub fn checked_add(r: &RectI32, delta: &RectI32) -> RectI32 {
    try_checked_add(r, delta).unwrap()
}

pub fn saturating_add_assign(r: &mut RectI32, delta: &RectI32) {
    r.min.row = r.min.row.saturating_add(delta.min.row);
    r.min.col = r.min.col.saturating_add(delta.min.col);
    r.max.row = r.max.row.saturating_add(delta.max.row);
    r.max.col = r.max.col.saturating_add(delta.max.col);
}

pub fn saturating_add(r: &RectI32, delta: &RectI32) -> RectI32 {
    let min_row = r.min.row.saturating_add(delta.min.row);
    let min_col = r.min.col.saturating_add(delta.min.col);
    let max_row = r.max.row.saturating_add(delta.max.row);
    let max_col = r.max.col.saturating_add(delta.max.col);
    RectI32 { min: PointI32 { row: min_row, col: min_col }, max: PointI32 { row: max_row, col: max_col } }
}

pub fn wrapping_add_assign(r: &mut RectI32, delta: &RectI32) {
    r.min.row = r.min.row.wrapping_add(delta.min.row);
    r.min.col = r.min.col.wrapping_add(delta.min.col);
    r.max.row = r.max.row.wrapping_add(delta.max.row);
    r.max.col = r.max.col.wrapping_add(delta.max.col);
}

pub fn wrapping_add(r: &RectI32, delta: &RectI32) -> RectI32 {
    let min_row = r.min.row.wrapping_add(delta.min.row);
    let min_col = r.min.col.wrapping_add(delta.min.col);
    let max_row = r.max.row.wrapping_add(delta.max.row);
    let max_col = r.max.col.wrapping_add(delta.max.col);
    RectI32 { min: PointI32 { row: min_row, col: min_col }, max: PointI32 { row: max_row, col: max_col } }
}

#[cfg(test)]
mod test_checked_add_assign;

#[cfg(test)]
mod test_checked_add;

#[cfg(test)]
mod test_try_checked_add_assign;

#[cfg(test)]
mod test_try_checked_add;

#[cfg(test)]
mod test_saturating_add_assign;

#[cfg(test)]
mod test_saturating_add;

#[cfg(test)]
mod test_wrapping_add_assign;

#[cfg(test)]
mod test_wrapping_add;

use crate::matrix::{
    point::point_u64::PointU64,
    rect::{rect_i64::RectI64, rect_u64::RectU64},
};

pub fn try_checked_add_assign(r: &mut RectU64, delta: &RectI64) -> Option<()> {
    let min_row = r.min.row.checked_add_signed(delta.min.row)?;
    let min_col = r.min.col.checked_add_signed(delta.min.col)?;
    let max_row = r.max.row.checked_add_signed(delta.max.row)?;
    let max_col = r.max.col.checked_add_signed(delta.max.col)?;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
    Some(())
}

pub fn try_checked_add(r: &RectU64, delta: &RectI64) -> Option<RectU64> {
    let min_row = r.min.row.checked_add_signed(delta.min.row)?;
    let min_col = r.min.col.checked_add_signed(delta.min.col)?;
    let max_row = r.max.row.checked_add_signed(delta.max.row)?;
    let max_col = r.max.col.checked_add_signed(delta.max.col)?;
    Some(RectU64 { min: PointU64 { row: min_row, col: min_col }, max: PointU64 { row: max_row, col: max_col } })
}

pub fn checked_add_assign(r: &mut RectU64, delta: &RectI64) {
    try_checked_add_assign(r, delta).unwrap()
}

pub fn checked_add(r: &RectU64, delta: &RectI64) -> RectU64 {
    try_checked_add(r, delta).unwrap()
}

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

pub fn wrapping_add_assign(r: &mut RectU64, delta: &RectI64) {
    r.min.row = r.min.row.wrapping_add_signed(delta.min.row);
    r.min.col = r.min.col.wrapping_add_signed(delta.min.col);
    r.max.row = r.max.row.wrapping_add_signed(delta.max.row);
    r.max.col = r.max.col.wrapping_add_signed(delta.max.col);
}

pub fn wrapping_add(r: &RectU64, delta: &RectI64) -> RectU64 {
    let min_row = r.min.row.wrapping_add_signed(delta.min.row);
    let min_col = r.min.col.wrapping_add_signed(delta.min.col);
    let max_row = r.max.row.wrapping_add_signed(delta.max.row);
    let max_col = r.max.col.wrapping_add_signed(delta.max.col);
    RectU64 { min: PointU64 { row: min_row, col: min_col }, max: PointU64 { row: max_row, col: max_col } }
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

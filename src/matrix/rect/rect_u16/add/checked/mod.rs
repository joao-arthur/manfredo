use crate::matrix::{
    point::point_u16::PointU16,
    rect::{rect_i16::RectI16, rect_u16::RectU16},
};

pub fn try_checked_add_assign(r: &mut RectU16, delta: &RectI16) -> Option<()> {
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

pub fn try_checked_add(r: &RectU16, delta: &RectI16) -> Option<RectU16> {
    let min_row = r.min.row.checked_add_signed(delta.min.row)?;
    let min_col = r.min.col.checked_add_signed(delta.min.col)?;
    let max_row = r.max.row.checked_add_signed(delta.max.row)?;
    let max_col = r.max.col.checked_add_signed(delta.max.col)?;
    Some(RectU16 { min: PointU16 { row: min_row, col: min_col }, max: PointU16 { row: max_row, col: max_col } })
}

pub fn checked_add_assign(r: &mut RectU16, delta: &RectI16) {
    try_checked_add_assign(r, delta).unwrap()
}

pub fn checked_add(r: &RectU16, delta: &RectI16) -> RectU16 {
    try_checked_add(r, delta).unwrap()
}

#[cfg(test)]
mod test_checked_add_assign;

#[cfg(test)]
mod test_checked_add;

#[cfg(test)]
mod test_try_checked_add_assign;

#[cfg(test)]
mod test_try_checked_add;

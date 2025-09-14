use crate::matrix::d2::{
    point::point_i32::Point,
    rect::rect_u32::{Rect, delta_col, delta_row},
};

pub fn try_checked_translate_assign(r: &mut Rect, delta: &Point) -> Option<()> {
    let min_row = r.min.row.checked_add_signed(delta.row)?;
    let min_col = r.min.col.checked_add_signed(delta.col)?;
    let max_row = r.max.row.checked_add_signed(delta.row)?;
    let max_col = r.max.col.checked_add_signed(delta.col)?;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
    Some(())
}

pub fn try_checked_translate(r: &Rect, delta: &Point) -> Option<Rect> {
    let min_row = r.min.row.checked_add_signed(delta.row)?;
    let min_col = r.min.col.checked_add_signed(delta.col)?;
    let max_row = r.max.row.checked_add_signed(delta.row)?;
    let max_col = r.max.col.checked_add_signed(delta.col)?;
    Some(Rect::of(min_row, min_col, max_row, max_col))
}

pub fn checked_translate_assign(r: &mut Rect, delta: &Point) {
    try_checked_translate_assign(r, delta).unwrap()
}

pub fn checked_translate(r: &Rect, delta: &Point) -> Rect {
    try_checked_translate(r, delta).unwrap()
}

pub fn saturating_translate_assign(r: &mut Rect, delta: &Point) {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let temp_min_row = i64::from(r.min.row) + i64::from(delta.row);
    let temp_min_col = i64::from(r.min.col) + i64::from(delta.col);
    let clamped_row = temp_min_row.clamp(0, i64::from(u32::MAX) - i64::from(d_row));
    let clamped_col = temp_min_col.clamp(0, i64::from(u32::MAX) - i64::from(d_col));
    let min_row = clamped_row as u32;
    let min_col = clamped_col as u32;
    let max_row = min_row + d_row;
    let max_col = min_col + d_col;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
}

pub fn saturating_translate(r: &Rect, delta: &Point) -> Rect {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let temp_min_row = i64::from(r.min.row) + i64::from(delta.row);
    let temp_min_col = i64::from(r.min.col) + i64::from(delta.col);
    let clamped_row = temp_min_row.clamp(0, i64::from(u32::MAX) - i64::from(d_row));
    let clamped_col = temp_min_col.clamp(0, i64::from(u32::MAX) - i64::from(d_col));
    let min_row = clamped_row as u32;
    let min_col = clamped_col as u32;
    let max_row = min_row + d_row;
    let max_col = min_col + d_col;
    Rect::of(min_row, min_col, max_row, max_col)
}

pub fn wrapping_translate_assign(r: &mut Rect, delta: &Point) {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let min_row = r.min.row.wrapping_add_signed(delta.row);
    let min_col = r.min.col.wrapping_add_signed(delta.col);
    let max_row = min_row.wrapping_add(d_row);
    let max_col = min_col.wrapping_add(d_col);
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
}

pub fn wrapping_translate(r: &Rect, delta: &Point) -> Rect {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let min_row = r.min.row.wrapping_add_signed(delta.row);
    let min_col = r.min.col.wrapping_add_signed(delta.col);
    let max_row = min_row.wrapping_add(d_row);
    let max_col = min_col.wrapping_add(d_col);
    Rect::of(min_row, min_col, max_row, max_col)
}

#[cfg(test)]
mod test_try_checked_translate_assign;

#[cfg(test)]
mod test_try_checked_translate;

#[cfg(test)]
mod test_checked_translate_assign;

#[cfg(test)]
mod test_checked_translate;

#[cfg(test)]
mod test_saturating_translate_assign;

#[cfg(test)]
mod test_saturating_translate;

#[cfg(test)]
mod test_wrapping_translate_assign;

#[cfg(test)]
mod test_wrapping_translate;

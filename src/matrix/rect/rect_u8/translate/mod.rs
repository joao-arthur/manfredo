use crate::matrix::{
    point::{point_i8::PointI8, point_u8::PointU8},
    rect::rect_u8::{RectU8, delta_col, delta_row},
};

pub fn try_checked_translate_assign(r: &mut RectU8, delta: &PointI8) -> Option<()> {
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

pub fn try_checked_translate(r: &RectU8, delta: &PointI8) -> Option<RectU8> {
    let min_row = r.min.row.checked_add_signed(delta.row)?;
    let min_col = r.min.col.checked_add_signed(delta.col)?;
    let max_row = r.max.row.checked_add_signed(delta.row)?;
    let max_col = r.max.col.checked_add_signed(delta.col)?;
    Some(RectU8 { min: PointU8 { row: min_row, col: min_col }, max: PointU8 { row: max_row, col: max_col } })
}

pub fn checked_translate_assign(r: &mut RectU8, delta: &PointI8) {
    try_checked_translate_assign(r, delta).unwrap()
}

pub fn checked_translate(r: &RectU8, delta: &PointI8) -> RectU8 {
    try_checked_translate(r, delta).unwrap()
}

pub fn saturating_translate_assign(r: &mut RectU8, delta: &PointI8) {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let temp_min_row = i16::from(r.min.row) + i16::from(delta.row);
    let temp_min_col = i16::from(r.min.col) + i16::from(delta.col);
    let clamped_row = temp_min_row.clamp(0, i16::from(u8::MAX) - i16::from(d_row));
    let clamped_col = temp_min_col.clamp(0, i16::from(u8::MAX) - i16::from(d_col));
    let min_row = clamped_row as u8;
    let min_col = clamped_col as u8;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = min_row + d_row;
    r.max.col = min_col + d_col;
}

pub fn saturating_translate(r: &RectU8, delta: &PointI8) -> RectU8 {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let temp_min_row = i16::from(r.min.row) + i16::from(delta.row);
    let temp_min_col = i16::from(r.min.col) + i16::from(delta.col);
    let clamped_row = temp_min_row.clamp(0, i16::from(u8::MAX) - i16::from(d_row));
    let clamped_col = temp_min_col.clamp(0, i16::from(u8::MAX) - i16::from(d_col));
    let min_row = clamped_row as u8;
    let min_col = clamped_col as u8;
    let max_row = min_row + d_row;
    let max_col = min_col + d_col;
    RectU8 { min: PointU8 { row: min_row, col: min_col }, max: PointU8 { row: max_row, col: max_col } }
}

pub fn wrapping_translate_assign(r: &mut RectU8, delta: &PointI8) {
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

pub fn wrapping_translate(r: &RectU8, delta: &PointI8) -> RectU8 {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let min_row = r.min.row.wrapping_add_signed(delta.row);
    let min_col = r.min.col.wrapping_add_signed(delta.col);
    let max_row = min_row.wrapping_add(d_row);
    let max_col = min_col.wrapping_add(d_col);
    RectU8 { min: PointU8 { row: min_row, col: min_col }, max: PointU8 { row: max_row, col: max_col } }
}

pub mod checked;
pub mod saturated;
pub mod wrapped;

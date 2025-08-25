use crate::matrix::{
    point::point_i16::PointI16,
    rect::rect_i16::{RectI16, delta_col, delta_row},
};

pub fn try_checked_resize_assign(r: &mut RectI16, size: u16) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_row = i32::from(delta_row(r)) + 1 - i32::from(size);
    let diff_col = i32::from(delta_col(r)) + 1 - i32::from(size);
    let temp_min_row = i32::from(r.min.row) + diff_row / 2;
    let temp_min_col = i32::from(r.min.col) + diff_col / 2;
    let min_row = i16::try_from(temp_min_row).ok()?;
    let min_col = i16::try_from(temp_min_col).ok()?;
    let max_row = min_row.checked_add_unsigned(size - 1)?;
    let max_col = min_col.checked_add_unsigned(size - 1)?;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
    Some(())
}

pub fn try_checked_resize(r: &RectI16, size: u16) -> Option<RectI16> {
    if size < 3 {
        return None;
    }
    let diff_row = i32::from(delta_row(r)) + 1 - i32::from(size);
    let diff_col = i32::from(delta_col(r)) + 1 - i32::from(size);
    let temp_min_row = i32::from(r.min.row) + diff_row / 2;
    let temp_min_col = i32::from(r.min.col) + diff_col / 2;
    let min_row = i16::try_from(temp_min_row).ok()?;
    let min_col = i16::try_from(temp_min_col).ok()?;
    let max_row = min_row.checked_add_unsigned(size - 1)?;
    let max_col = min_col.checked_add_unsigned(size - 1)?;
    Some(RectI16 { min: PointI16 { row: min_row, col: min_col }, max: PointI16 { row: max_row, col: max_col } })
}

pub fn checked_resize_assign(r: &mut RectI16, size: u16) {
    try_checked_resize_assign(r, size).unwrap()
}

pub fn checked_resize(r: &RectI16, size: u16) -> RectI16 {
    try_checked_resize(r, size).unwrap()
}

pub fn try_saturating_resize_assign(r: &mut RectI16, size: u16) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_row = i32::from(delta_row(r)) + 1 - i32::from(size);
    let diff_col = i32::from(delta_col(r)) + 1 - i32::from(size);
    let temp_min_row = i32::from(r.min.row) + diff_row / 2;
    let temp_min_col = i32::from(r.min.col) + diff_col / 2;
    let min_row = temp_min_row.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(size) + 1);
    let min_col = temp_min_col.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(size) + 1);
    r.min.row = min_row as i16;
    r.min.col = min_col as i16;
    r.max.row = (min_row + i32::from(size) - 1) as i16;
    r.max.col = (min_col + i32::from(size) - 1) as i16;
    Some(())
}

pub fn try_saturating_resize(r: &RectI16, size: u16) -> Option<RectI16> {
    if size < 3 {
        return None;
    }
    let diff_row = i32::from(delta_row(r)) + 1 - i32::from(size);
    let diff_col = i32::from(delta_col(r)) + 1 - i32::from(size);
    let temp_min_row = i32::from(r.min.row) + diff_row / 2;
    let temp_min_col = i32::from(r.min.col) + diff_col / 2;
    let clamped_min_row = temp_min_row.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(size) + 1);
    let clamped_min_col = temp_min_col.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(size) + 1);
    let min_row = clamped_min_row as i16;
    let min_col = clamped_min_col as i16;
    let max_row = (clamped_min_row + i32::from(size) - 1) as i16;
    let max_col = (clamped_min_col + i32::from(size) - 1) as i16;
    Some(RectI16 { min: PointI16 { row: min_row, col: min_col }, max: PointI16 { row: max_row, col: max_col } })
}

pub fn saturating_resize_assign(r: &mut RectI16, size: u16) {
    try_saturating_resize_assign(r, size).unwrap()
}

pub fn saturating_resize(r: &RectI16, size: u16) -> RectI16 {
    try_saturating_resize(r, size).unwrap()
}

pub mod checked;
pub mod saturated;

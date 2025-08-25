use crate::matrix::{
    point::point_u64::PointU64,
    rect::rect_u64::{RectU64, delta_col, delta_row},
};

pub fn try_checked_resize_assign(r: &mut RectU64, size: u64) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_row = i128::from(delta_row(r)) + 1 - i128::from(size);
    let diff_col = i128::from(delta_col(r)) + 1 - i128::from(size);
    let temp_min_row = i128::from(r.min.row) + diff_row / 2;
    let temp_min_col = i128::from(r.min.col) + diff_col / 2;
    let min_row = u64::try_from(temp_min_row).ok()?;
    let min_col = u64::try_from(temp_min_col).ok()?;
    let max_row = min_row.checked_add(size - 1)?;
    let max_col = min_col.checked_add(size - 1)?;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
    Some(())
}

pub fn try_checked_resize(r: &RectU64, size: u64) -> Option<RectU64> {
    if size < 3 {
        return None;
    }
    let diff_row = i128::from(delta_row(r)) + 1 - i128::from(size);
    let diff_col = i128::from(delta_col(r)) + 1 - i128::from(size);
    let temp_min_row = i128::from(r.min.row) + diff_row / 2;
    let temp_min_col = i128::from(r.min.col) + diff_col / 2;
    let min_row = u64::try_from(temp_min_row).ok()?;
    let min_col = u64::try_from(temp_min_col).ok()?;
    let max_row = min_row.checked_add(size - 1)?;
    let max_col = min_col.checked_add(size - 1)?;
    Some(RectU64 { min: PointU64 { row: min_row, col: min_col }, max: PointU64 { row: max_row, col: max_col } })
}

pub fn checked_resize_assign(r: &mut RectU64, size: u64) {
    try_checked_resize_assign(r, size).unwrap()
}

pub fn checked_resize(r: &RectU64, size: u64) -> RectU64 {
    try_checked_resize(r, size).unwrap()
}

pub fn try_saturating_resize_assign(r: &mut RectU64, size: u64) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_row = i128::from(delta_row(r)) + 1 - i128::from(size);
    let diff_col = i128::from(delta_col(r)) + 1 - i128::from(size);
    let temp_min_row = i128::from(r.min.row) + diff_row / 2;
    let temp_min_col = i128::from(r.min.col) + diff_col / 2;
    let min_row = temp_min_row.clamp(0, i128::from(u64::MAX) - i128::from(size) + 1);
    let min_col = temp_min_col.clamp(0, i128::from(u64::MAX) - i128::from(size) + 1);
    r.min.row = min_row as u64;
    r.min.col = min_col as u64;
    r.max.row = (min_row + i128::from(size) - 1) as u64;
    r.max.col = (min_col + i128::from(size) - 1) as u64;
    Some(())
}

pub fn try_saturating_resize(r: &RectU64, size: u64) -> Option<RectU64> {
    if size < 3 {
        return None;
    }
    let diff_row = i128::from(delta_row(r)) + 1 - i128::from(size);
    let diff_col = i128::from(delta_col(r)) + 1 - i128::from(size);
    let temp_min_row = i128::from(r.min.row) + diff_row / 2;
    let temp_min_col = i128::from(r.min.col) + diff_col / 2;
    let clamped_min_row = temp_min_row.clamp(0, i128::from(u64::MAX) - i128::from(size) + 1);
    let clamped_min_col = temp_min_col.clamp(0, i128::from(u64::MAX) - i128::from(size) + 1);
    let min_row = clamped_min_row as u64;
    let min_col = clamped_min_col as u64;
    let max_row = (clamped_min_row + i128::from(size) - 1) as u64;
    let max_col = (clamped_min_col + i128::from(size) - 1) as u64;
    Some(RectU64 { min: PointU64 { row: min_row, col: min_col }, max: PointU64 { row: max_row, col: max_col } })
}

pub fn saturating_resize_assign(r: &mut RectU64, size: u64) {
    try_saturating_resize_assign(r, size).unwrap()
}

pub fn saturating_resize(r: &RectU64, size: u64) -> RectU64 {
    try_saturating_resize(r, size).unwrap()
}

pub mod checked;
pub mod saturated;

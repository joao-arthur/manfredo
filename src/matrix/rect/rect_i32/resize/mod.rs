use crate::matrix::{
    point::point_i32::PointI32,
    rect::rect_i32::{RectI32, delta_col, delta_row},
};

pub fn try_checked_resize_assign(r: &mut RectI32, size: u32) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_row = i64::from(delta_row(r)) + 1 - i64::from(size);
    let diff_col = i64::from(delta_col(r)) + 1 - i64::from(size);
    let temp_min_row = i64::from(r.min.row) + diff_row / 2;
    let temp_min_col = i64::from(r.min.col) + diff_col / 2;
    let min_row = i32::try_from(temp_min_row).ok()?;
    let min_col = i32::try_from(temp_min_col).ok()?;
    let max_row = min_row.checked_add_unsigned(size - 1)?;
    let max_col = min_col.checked_add_unsigned(size - 1)?;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
    Some(())
}

pub fn try_checked_resize(r: &RectI32, size: u32) -> Option<RectI32> {
    if size < 3 {
        return None;
    }
    let diff_row = i64::from(delta_row(r)) + 1 - i64::from(size);
    let diff_col = i64::from(delta_col(r)) + 1 - i64::from(size);
    let temp_min_row = i64::from(r.min.row) + diff_row / 2;
    let temp_min_col = i64::from(r.min.col) + diff_col / 2;
    let min_row = i32::try_from(temp_min_row).ok()?;
    let min_col = i32::try_from(temp_min_col).ok()?;
    let max_row = min_row.checked_add_unsigned(size - 1)?;
    let max_col = min_col.checked_add_unsigned(size - 1)?;
    Some(RectI32 { min: PointI32 { row: min_row, col: min_col }, max: PointI32 { row: max_row, col: max_col } })
}

pub fn checked_resize_assign(r: &mut RectI32, size: u32) {
    try_checked_resize_assign(r, size).unwrap()
}

pub fn checked_resize(r: &RectI32, size: u32) -> RectI32 {
    try_checked_resize(r, size).unwrap()
}

pub fn try_saturating_resize_assign(r: &mut RectI32, size: u32) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_row = i64::from(delta_row(r)) + 1 - i64::from(size);
    let diff_col = i64::from(delta_col(r)) + 1 - i64::from(size);
    let temp_min_row = i64::from(r.min.row) + diff_row / 2;
    let temp_min_col = i64::from(r.min.col) + diff_col / 2;
    let min_row = temp_min_row.clamp(i64::from(i32::MIN), i64::from(i32::MAX) - i64::from(size) + 1);
    let min_col = temp_min_col.clamp(i64::from(i32::MIN), i64::from(i32::MAX) - i64::from(size) + 1);
    r.min.row = min_row as i32;
    r.min.col = min_col as i32;
    r.max.row = (min_row + i64::from(size) - 1) as i32;
    r.max.col = (min_col + i64::from(size) - 1) as i32;
    Some(())
}

pub fn try_saturating_resize(r: &RectI32, size: u32) -> Option<RectI32> {
    if size < 3 {
        return None;
    }
    let diff_row = i64::from(delta_row(r)) + 1 - i64::from(size);
    let diff_col = i64::from(delta_col(r)) + 1 - i64::from(size);
    let temp_min_row = i64::from(r.min.row) + diff_row / 2;
    let temp_min_col = i64::from(r.min.col) + diff_col / 2;
    let clamped_min_row = temp_min_row.clamp(i64::from(i32::MIN), i64::from(i32::MAX) - i64::from(size) + 1);
    let clamped_min_col = temp_min_col.clamp(i64::from(i32::MIN), i64::from(i32::MAX) - i64::from(size) + 1);
    let min_row = clamped_min_row as i32;
    let min_col = clamped_min_col as i32;
    let max_row = (clamped_min_row + i64::from(size) - 1) as i32;
    let max_col = (clamped_min_col + i64::from(size) - 1) as i32;
    Some(RectI32 { min: PointI32 { row: min_row, col: min_col }, max: PointI32 { row: max_row, col: max_col } })
}

pub fn saturating_resize_assign(r: &mut RectI32, size: u32) {
    try_saturating_resize_assign(r, size).unwrap()
}

pub fn saturating_resize(r: &RectI32, size: u32) -> RectI32 {
    try_saturating_resize(r, size).unwrap()
}

pub fn try_wrapping_resize_assign(r: &mut RectI32, size: u32) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_row = i64::from(delta_row(r)) + 1 - i64::from(size);
    let diff_col = i64::from(delta_col(r)) + 1 - i64::from(size);
    let temp_min_row = i64::from(r.min.row) + diff_row / 2;
    let temp_min_col = i64::from(r.min.col) + diff_col / 2;
    let min_row = temp_min_row as i32;
    let min_col = temp_min_col as i32;
    let max_row = min_row.wrapping_add_unsigned(size - 1);
    let max_col = min_col.wrapping_add_unsigned(size - 1);
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
    Some(())
}

pub fn try_wrapping_resize(r: &RectI32, size: u32) -> Option<RectI32> {
    if size < 3 {
        return None;
    }
    let diff_row = i64::from(delta_row(r)) + 1 - i64::from(size);
    let diff_col = i64::from(delta_col(r)) + 1 - i64::from(size);
    let temp_min_row = i64::from(r.min.row) + diff_row / 2;
    let temp_min_col = i64::from(r.min.col) + diff_col / 2;
    let min_row = temp_min_row as i32;
    let min_col = temp_min_col as i32;
    let max_row = min_row.wrapping_add_unsigned(size - 1);
    let max_col = min_col.wrapping_add_unsigned(size - 1);
    Some(RectI32 { min: PointI32 { row: min_row, col: min_col }, max: PointI32 { row: max_row, col: max_col } })
}

pub fn wrapping_resize_assign(r: &mut RectI32, size: u32) {
    try_wrapping_resize_assign(r, size).unwrap()
}

pub fn wrapping_resize(r: &RectI32, size: u32) -> RectI32 {
    try_wrapping_resize(r, size).unwrap()
}

#[cfg(test)]
mod test_try_checked_resize_assign;

#[cfg(test)]
mod test_try_checked_resize;

#[cfg(test)]
mod test_checked_resize_assign;

#[cfg(test)]
mod test_checked_resize;

#[cfg(test)]
mod test_try_saturating_resize_assign;

#[cfg(test)]
mod test_try_saturating_resize;

#[cfg(test)]
mod test_saturating_resize_assign;

#[cfg(test)]
mod test_saturating_resize;

#[cfg(test)]
mod test_try_wrapping_resize_assign;

#[cfg(test)]
mod test_try_wrapping_resize;

#[cfg(test)]
mod test_wrapping_resize_assign;

#[cfg(test)]
mod test_wrapping_resize;

use super::{Rect, delta_col, delta_row};
use crate::matrix::d2::point::point_i8::Point;

pub fn try_checked_resize_assign(r: &mut Rect, size: u8) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_row = i16::from(delta_row(r)) + 1 - i16::from(size);
    let diff_col = i16::from(delta_col(r)) + 1 - i16::from(size);
    let temp_min_row = i16::from(r.min.row) + diff_row / 2;
    let temp_min_col = i16::from(r.min.col) + diff_col / 2;
    let min_row = i8::try_from(temp_min_row).ok()?;
    let min_col = i8::try_from(temp_min_col).ok()?;
    let max_row = min_row.checked_add_unsigned(size - 1)?;
    let max_col = min_col.checked_add_unsigned(size - 1)?;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
    Some(())
}

pub fn try_checked_resize(r: &Rect, size: u8) -> Option<Rect> {
    if size < 3 {
        return None;
    }
    let diff_row = i16::from(delta_row(r)) + 1 - i16::from(size);
    let diff_col = i16::from(delta_col(r)) + 1 - i16::from(size);
    let temp_min_row = i16::from(r.min.row) + diff_row / 2;
    let temp_min_col = i16::from(r.min.col) + diff_col / 2;
    let min_row = i8::try_from(temp_min_row).ok()?;
    let min_col = i8::try_from(temp_min_col).ok()?;
    let max_row = min_row.checked_add_unsigned(size - 1)?;
    let max_col = min_col.checked_add_unsigned(size - 1)?;
    Some(Rect { min: Point { row: min_row, col: min_col }, max: Point { row: max_row, col: max_col } })
}

pub fn checked_resize_assign(r: &mut Rect, size: u8) {
    try_checked_resize_assign(r, size).unwrap()
}

pub fn checked_resize(r: &Rect, size: u8) -> Rect {
    try_checked_resize(r, size).unwrap()
}

pub fn try_saturating_resize_assign(r: &mut Rect, size: u8) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_row = i16::from(delta_row(r)) + 1 - i16::from(size);
    let diff_col = i16::from(delta_col(r)) + 1 - i16::from(size);
    let temp_min_row = i16::from(r.min.row) + diff_row / 2;
    let temp_min_col = i16::from(r.min.col) + diff_col / 2;
    let min_row = temp_min_row.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(size) + 1);
    let min_col = temp_min_col.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(size) + 1);
    r.min.row = min_row as i8;
    r.min.col = min_col as i8;
    r.max.row = (min_row + i16::from(size) - 1) as i8;
    r.max.col = (min_col + i16::from(size) - 1) as i8;
    Some(())
}

pub fn try_saturating_resize(r: &Rect, size: u8) -> Option<Rect> {
    if size < 3 {
        return None;
    }
    let diff_row = i16::from(delta_row(r)) + 1 - i16::from(size);
    let diff_col = i16::from(delta_col(r)) + 1 - i16::from(size);
    let temp_min_row = i16::from(r.min.row) + diff_row / 2;
    let temp_min_col = i16::from(r.min.col) + diff_col / 2;
    let clamped_min_row = temp_min_row.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(size) + 1);
    let clamped_min_col = temp_min_col.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(size) + 1);
    let min_row = clamped_min_row as i8;
    let min_col = clamped_min_col as i8;
    let max_row = (clamped_min_row + i16::from(size) - 1) as i8;
    let max_col = (clamped_min_col + i16::from(size) - 1) as i8;
    Some(Rect { min: Point { row: min_row, col: min_col }, max: Point { row: max_row, col: max_col } })
}

pub fn saturating_resize_assign(r: &mut Rect, size: u8) {
    try_saturating_resize_assign(r, size).unwrap()
}

pub fn saturating_resize(r: &Rect, size: u8) -> Rect {
    try_saturating_resize(r, size).unwrap()
}

pub fn try_wrapping_resize_assign(r: &mut Rect, size: u8) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_row = i16::from(delta_row(r)) + 1 - i16::from(size);
    let diff_col = i16::from(delta_col(r)) + 1 - i16::from(size);
    let temp_min_row = i16::from(r.min.row) + diff_row / 2;
    let temp_min_col = i16::from(r.min.col) + diff_col / 2;
    let min_row = temp_min_row as i8;
    let min_col = temp_min_col as i8;
    let max_row = min_row.wrapping_add_unsigned(size - 1);
    let max_col = min_col.wrapping_add_unsigned(size - 1);
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
    Some(())
}

pub fn try_wrapping_resize(r: &Rect, size: u8) -> Option<Rect> {
    if size < 3 {
        return None;
    }
    let diff_row = i16::from(delta_row(r)) + 1 - i16::from(size);
    let diff_col = i16::from(delta_col(r)) + 1 - i16::from(size);
    let temp_min_row = i16::from(r.min.row) + diff_row / 2;
    let temp_min_col = i16::from(r.min.col) + diff_col / 2;
    let min_row = temp_min_row as i8;
    let min_col = temp_min_col as i8;
    let max_row = min_row.wrapping_add_unsigned(size - 1);
    let max_col = min_col.wrapping_add_unsigned(size - 1);
    Some(Rect { min: Point { row: min_row, col: min_col }, max: Point { row: max_row, col: max_col } })
}

pub fn wrapping_resize_assign(r: &mut Rect, size: u8) {
    try_wrapping_resize_assign(r, size).unwrap()
}

pub fn wrapping_resize(r: &Rect, size: u8) -> Rect {
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

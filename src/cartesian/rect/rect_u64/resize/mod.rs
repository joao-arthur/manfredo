use crate::cartesian::{
    point::point_u64::PointU64,
    rect::rect_u64::{RectU64, delta_x, delta_y},
};

pub fn try_checked_resize_assign(r: &mut RectU64, size: u64) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_x = i128::from(delta_x(r)) + 1 - i128::from(size);
    let diff_y = i128::from(delta_y(r)) + 1 - i128::from(size);
    let temp_min_x = i128::from(r.min.x) + diff_x / 2;
    let temp_min_y = i128::from(r.min.y) + diff_y / 2;
    let min_x = u64::try_from(temp_min_x).ok()?;
    let min_y = u64::try_from(temp_min_y).ok()?;
    let max_x = min_x.checked_add(size - 1)?;
    let max_y = min_y.checked_add(size - 1)?;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

pub fn try_checked_resize(r: &RectU64, size: u64) -> Option<RectU64> {
    if size < 3 {
        return None;
    }
    let diff_x = i128::from(delta_x(r)) + 1 - i128::from(size);
    let diff_y = i128::from(delta_y(r)) + 1 - i128::from(size);
    let temp_min_x = i128::from(r.min.x) + diff_x / 2;
    let temp_min_y = i128::from(r.min.y) + diff_y / 2;
    let min_x = u64::try_from(temp_min_x).ok()?;
    let min_y = u64::try_from(temp_min_y).ok()?;
    let max_x = min_x.checked_add(size - 1)?;
    let max_y = min_y.checked_add(size - 1)?;
    Some(RectU64 { min: PointU64 { x: min_x, y: min_y }, max: PointU64 { x: max_x, y: max_y } })
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
    let diff_x = i128::from(delta_x(r)) + 1 - i128::from(size);
    let diff_y = i128::from(delta_y(r)) + 1 - i128::from(size);
    let temp_min_x = i128::from(r.min.x) + diff_x / 2;
    let temp_min_y = i128::from(r.min.y) + diff_y / 2;
    let min_x = temp_min_x.clamp(0, i128::from(u64::MAX) - i128::from(size) + 1);
    let min_y = temp_min_y.clamp(0, i128::from(u64::MAX) - i128::from(size) + 1);
    r.min.x = min_x as u64;
    r.min.y = min_y as u64;
    r.max.x = (min_x + i128::from(size) - 1) as u64;
    r.max.y = (min_y + i128::from(size) - 1) as u64;
    Some(())
}

pub fn try_saturating_resize(r: &RectU64, size: u64) -> Option<RectU64> {
    if size < 3 {
        return None;
    }
    let diff_x = i128::from(delta_x(r)) + 1 - i128::from(size);
    let diff_y = i128::from(delta_y(r)) + 1 - i128::from(size);
    let temp_min_x = i128::from(r.min.x) + diff_x / 2;
    let temp_min_y = i128::from(r.min.y) + diff_y / 2;
    let clamped_min_x = temp_min_x.clamp(0, i128::from(u64::MAX) - i128::from(size) + 1);
    let clamped_min_y = temp_min_y.clamp(0, i128::from(u64::MAX) - i128::from(size) + 1);
    let min_x = clamped_min_x as u64;
    let min_y = clamped_min_y as u64;
    let max_x = (clamped_min_x + i128::from(size) - 1) as u64;
    let max_y = (clamped_min_y + i128::from(size) - 1) as u64;
    Some(RectU64 { min: PointU64 { x: min_x, y: min_y }, max: PointU64 { x: max_x, y: max_y } })
}

pub fn saturating_resize_assign(r: &mut RectU64, size: u64) {
    try_saturating_resize_assign(r, size).unwrap()
}

pub fn saturating_resize(r: &RectU64, size: u64) -> RectU64 {
    try_saturating_resize(r, size).unwrap()
}

pub mod checked;
pub mod saturated;

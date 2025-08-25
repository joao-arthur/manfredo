use crate::cartesian::{
    point::point_i32::PointI32,
    rect::rect_i32::{RectI32, delta_x, delta_y},
};

pub fn try_checked_resize_assign(r: &mut RectI32, size: u32) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_x = i64::from(delta_x(r)) + 1 - i64::from(size);
    let diff_y = i64::from(delta_y(r)) + 1 - i64::from(size);
    let temp_min_x = i64::from(r.min.x) + diff_x / 2;
    let temp_min_y = i64::from(r.min.y) + diff_y / 2;
    let min_x = i32::try_from(temp_min_x).ok()?;
    let min_y = i32::try_from(temp_min_y).ok()?;
    let max_x = min_x.checked_add_unsigned(size - 1)?;
    let max_y = min_y.checked_add_unsigned(size - 1)?;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

pub fn try_checked_resize(r: &RectI32, size: u32) -> Option<RectI32> {
    if size < 3 {
        return None;
    }
    let diff_x = i64::from(delta_x(r)) + 1 - i64::from(size);
    let diff_y = i64::from(delta_y(r)) + 1 - i64::from(size);
    let temp_min_x = i64::from(r.min.x) + diff_x / 2;
    let temp_min_y = i64::from(r.min.y) + diff_y / 2;
    let min_x = i32::try_from(temp_min_x).ok()?;
    let min_y = i32::try_from(temp_min_y).ok()?;
    let max_x = min_x.checked_add_unsigned(size - 1)?;
    let max_y = min_y.checked_add_unsigned(size - 1)?;
    Some(RectI32 { min: PointI32 { x: min_x, y: min_y }, max: PointI32 { x: max_x, y: max_y } })
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
    let diff_x = i64::from(delta_x(r)) + 1 - i64::from(size);
    let diff_y = i64::from(delta_y(r)) + 1 - i64::from(size);
    let temp_min_x = i64::from(r.min.x) + diff_x / 2;
    let temp_min_y = i64::from(r.min.y) + diff_y / 2;
    let min_x = temp_min_x.clamp(i64::from(i32::MIN), i64::from(i32::MAX) - i64::from(size) + 1);
    let min_y = temp_min_y.clamp(i64::from(i32::MIN), i64::from(i32::MAX) - i64::from(size) + 1);
    r.min.x = min_x as i32;
    r.min.y = min_y as i32;
    r.max.x = (min_x + i64::from(size) - 1) as i32;
    r.max.y = (min_y + i64::from(size) - 1) as i32;
    Some(())
}

pub fn try_saturating_resize(r: &RectI32, size: u32) -> Option<RectI32> {
    if size < 3 {
        return None;
    }
    let diff_x = i64::from(delta_x(r)) + 1 - i64::from(size);
    let diff_y = i64::from(delta_y(r)) + 1 - i64::from(size);
    let temp_min_x = i64::from(r.min.x) + diff_x / 2;
    let temp_min_y = i64::from(r.min.y) + diff_y / 2;
    let clamped_min_x = temp_min_x.clamp(i64::from(i32::MIN), i64::from(i32::MAX) - i64::from(size) + 1);
    let clamped_min_y = temp_min_y.clamp(i64::from(i32::MIN), i64::from(i32::MAX) - i64::from(size) + 1);
    let min_x = clamped_min_x as i32;
    let min_y = clamped_min_y as i32;
    let max_x = (clamped_min_x + i64::from(size) - 1) as i32;
    let max_y = (clamped_min_y + i64::from(size) - 1) as i32;
    Some(RectI32 { min: PointI32 { x: min_x, y: min_y }, max: PointI32 { x: max_x, y: max_y } })
}

pub fn saturating_resize_assign(r: &mut RectI32, size: u32) {
    try_saturating_resize_assign(r, size).unwrap()
}

pub fn saturating_resize(r: &RectI32, size: u32) -> RectI32 {
    try_saturating_resize(r, size).unwrap()
}

pub mod checked;
pub mod saturated;

use crate::cartesian::{
    point::point_i8::PointI8,
    rect::rect_i8::{RectI8, delta_x, delta_y},
};

pub fn try_checked_resize_assign(r: &mut RectI8, size: u8) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_x = i16::from(delta_x(r)) + 1 - i16::from(size);
    let diff_y = i16::from(delta_y(r)) + 1 - i16::from(size);
    let temp_min_x = i16::from(r.min.x) + diff_x / 2;
    let temp_min_y = i16::from(r.min.y) + diff_y / 2;
    let min_x = i8::try_from(temp_min_x).ok()?;
    let min_y = i8::try_from(temp_min_y).ok()?;
    let max_x = min_x.checked_add_unsigned(size - 1)?;
    let max_y = min_y.checked_add_unsigned(size - 1)?;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

pub fn try_checked_resize(r: &RectI8, size: u8) -> Option<RectI8> {
    if size < 3 {
        return None;
    }
    let diff_x = i16::from(delta_x(r)) + 1 - i16::from(size);
    let diff_y = i16::from(delta_y(r)) + 1 - i16::from(size);
    let temp_min_x = i16::from(r.min.x) + diff_x / 2;
    let temp_min_y = i16::from(r.min.y) + diff_y / 2;
    let min_x = i8::try_from(temp_min_x).ok()?;
    let min_y = i8::try_from(temp_min_y).ok()?;
    let max_x = min_x.checked_add_unsigned(size - 1)?;
    let max_y = min_y.checked_add_unsigned(size - 1)?;
    Some(RectI8 { min: PointI8 { x: min_x, y: min_y }, max: PointI8 { x: max_x, y: max_y } })
}

pub fn checked_resize_assign(r: &mut RectI8, size: u8) {
    try_checked_resize_assign(r, size).unwrap()
}

pub fn checked_resize(r: &RectI8, size: u8) -> RectI8 {
    try_checked_resize(r, size).unwrap()
}

pub fn try_saturating_resize_assign(r: &mut RectI8, size: u8) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_x = i16::from(delta_x(r)) + 1 - i16::from(size);
    let diff_y = i16::from(delta_y(r)) + 1 - i16::from(size);
    let temp_min_x = i16::from(r.min.x) + diff_x / 2;
    let temp_min_y = i16::from(r.min.y) + diff_y / 2;
    let min_x = temp_min_x.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(size) + 1);
    let min_y = temp_min_y.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(size) + 1);
    r.min.x = min_x as i8;
    r.min.y = min_y as i8;
    r.max.x = (min_x + i16::from(size) - 1) as i8;
    r.max.y = (min_y + i16::from(size) - 1) as i8;
    Some(())
}

pub fn try_saturating_resize(r: &RectI8, size: u8) -> Option<RectI8> {
    if size < 3 {
        return None;
    }
    let diff_x = i16::from(delta_x(r)) + 1 - i16::from(size);
    let diff_y = i16::from(delta_y(r)) + 1 - i16::from(size);
    let temp_min_x = i16::from(r.min.x) + diff_x / 2;
    let temp_min_y = i16::from(r.min.y) + diff_y / 2;
    let clamped_min_x = temp_min_x.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(size) + 1);
    let clamped_min_y = temp_min_y.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(size) + 1);
    let min_x = clamped_min_x as i8;
    let min_y = clamped_min_y as i8;
    let max_x = (clamped_min_x + i16::from(size) - 1) as i8;
    let max_y = (clamped_min_y + i16::from(size) - 1) as i8;
    Some(RectI8 { min: PointI8 { x: min_x, y: min_y }, max: PointI8 { x: max_x, y: max_y } })
}

pub fn saturating_resize_assign(r: &mut RectI8, size: u8) {
    try_saturating_resize_assign(r, size).unwrap()
}

pub fn saturating_resize(r: &RectI8, size: u8) -> RectI8 {
    try_saturating_resize(r, size).unwrap()
}

pub mod checked;
pub mod saturated;

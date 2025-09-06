use crate::cartesian::{
    point::point_i16::PointI16,
    rect::rect_i16::{RectI16, delta_x, delta_y},
};

pub fn try_checked_resize_assign(r: &mut RectI16, size: u16) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_x = i32::from(delta_x(r)) + 1 - i32::from(size);
    let diff_y = i32::from(delta_y(r)) + 1 - i32::from(size);
    let temp_min_x = i32::from(r.min.x) + diff_x / 2;
    let temp_min_y = i32::from(r.min.y) + diff_y / 2;
    let min_x = i16::try_from(temp_min_x).ok()?;
    let min_y = i16::try_from(temp_min_y).ok()?;
    let max_x = min_x.checked_add_unsigned(size - 1)?;
    let max_y = min_y.checked_add_unsigned(size - 1)?;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

pub fn try_checked_resize(r: &RectI16, size: u16) -> Option<RectI16> {
    if size < 3 {
        return None;
    }
    let diff_x = i32::from(delta_x(r)) + 1 - i32::from(size);
    let diff_y = i32::from(delta_y(r)) + 1 - i32::from(size);
    let temp_min_x = i32::from(r.min.x) + diff_x / 2;
    let temp_min_y = i32::from(r.min.y) + diff_y / 2;
    let min_x = i16::try_from(temp_min_x).ok()?;
    let min_y = i16::try_from(temp_min_y).ok()?;
    let max_x = min_x.checked_add_unsigned(size - 1)?;
    let max_y = min_y.checked_add_unsigned(size - 1)?;
    Some(RectI16 { min: PointI16 { x: min_x, y: min_y }, max: PointI16 { x: max_x, y: max_y } })
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
    let diff_x = i32::from(delta_x(r)) + 1 - i32::from(size);
    let diff_y = i32::from(delta_y(r)) + 1 - i32::from(size);
    let temp_min_x = i32::from(r.min.x) + diff_x / 2;
    let temp_min_y = i32::from(r.min.y) + diff_y / 2;
    let min_x = temp_min_x.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(size) + 1);
    let min_y = temp_min_y.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(size) + 1);
    r.min.x = min_x as i16;
    r.min.y = min_y as i16;
    r.max.x = (min_x + i32::from(size) - 1) as i16;
    r.max.y = (min_y + i32::from(size) - 1) as i16;
    Some(())
}

pub fn try_saturating_resize(r: &RectI16, size: u16) -> Option<RectI16> {
    if size < 3 {
        return None;
    }
    let diff_x = i32::from(delta_x(r)) + 1 - i32::from(size);
    let diff_y = i32::from(delta_y(r)) + 1 - i32::from(size);
    let temp_min_x = i32::from(r.min.x) + diff_x / 2;
    let temp_min_y = i32::from(r.min.y) + diff_y / 2;
    let clamped_min_x = temp_min_x.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(size) + 1);
    let clamped_min_y = temp_min_y.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(size) + 1);
    let min_x = clamped_min_x as i16;
    let min_y = clamped_min_y as i16;
    let max_x = (clamped_min_x + i32::from(size) - 1) as i16;
    let max_y = (clamped_min_y + i32::from(size) - 1) as i16;
    Some(RectI16 { min: PointI16 { x: min_x, y: min_y }, max: PointI16 { x: max_x, y: max_y } })
}

pub fn saturating_resize_assign(r: &mut RectI16, size: u16) {
    try_saturating_resize_assign(r, size).unwrap()
}

pub fn saturating_resize(r: &RectI16, size: u16) -> RectI16 {
    try_saturating_resize(r, size).unwrap()
}

pub fn try_wrapping_resize_assign(r: &mut RectI16, size: u16) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_x = i32::from(delta_x(r)) + 1 - i32::from(size);
    let diff_y = i32::from(delta_y(r)) + 1 - i32::from(size);
    let temp_min_x = i32::from(r.min.x) + diff_x / 2;
    let temp_min_y = i32::from(r.min.y) + diff_y / 2;
    let min_x = temp_min_x as i16;
    let min_y = temp_min_y as i16;
    let max_x = min_x.wrapping_add_unsigned(size - 1);
    let max_y = min_y.wrapping_add_unsigned(size - 1);
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

pub fn try_wrapping_resize(r: &RectI16, size: u16) -> Option<RectI16> {
    if size < 3 {
        return None;
    }
    let diff_x = i32::from(delta_x(r)) + 1 - i32::from(size);
    let diff_y = i32::from(delta_y(r)) + 1 - i32::from(size);
    let temp_min_x = i32::from(r.min.x) + diff_x / 2;
    let temp_min_y = i32::from(r.min.y) + diff_y / 2;
    let min_x = temp_min_x as i16;
    let min_y = temp_min_y as i16;
    let max_x = min_x.wrapping_add_unsigned(size - 1);
    let max_y = min_y.wrapping_add_unsigned(size - 1);
    Some(RectI16 { min: PointI16 { x: min_x, y: min_y }, max: PointI16 { x: max_x, y: max_y } })
}

pub fn wrapping_resize_assign(r: &mut RectI16, size: u16) {
    try_wrapping_resize_assign(r, size).unwrap()
}

pub fn wrapping_resize(r: &RectI16, size: u16) -> RectI16 {
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

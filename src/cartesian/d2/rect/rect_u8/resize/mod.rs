use crate::cartesian::d2::rect::rect_u8::{Rect, delta_x, delta_y};

pub fn try_checked_resize_assign(r: &mut Rect, size: u8) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_x = i16::from(delta_x(r)) + 1 - i16::from(size);
    let diff_y = i16::from(delta_y(r)) + 1 - i16::from(size);
    let temp_min_x = i16::from(r.min.x) + diff_x / 2;
    let temp_min_y = i16::from(r.min.y) + diff_y / 2;
    let min_x = u8::try_from(temp_min_x).ok()?;
    let min_y = u8::try_from(temp_min_y).ok()?;
    let max_x = min_x.checked_add(size - 1)?;
    let max_y = min_y.checked_add(size - 1)?;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

pub fn try_checked_resize(r: &Rect, size: u8) -> Option<Rect> {
    if size < 3 {
        return None;
    }
    let diff_x = i16::from(delta_x(r)) + 1 - i16::from(size);
    let diff_y = i16::from(delta_y(r)) + 1 - i16::from(size);
    let temp_min_x = i16::from(r.min.x) + diff_x / 2;
    let temp_min_y = i16::from(r.min.y) + diff_y / 2;
    let min_x = u8::try_from(temp_min_x).ok()?;
    let min_y = u8::try_from(temp_min_y).ok()?;
    let max_x = min_x.checked_add(size - 1)?;
    let max_y = min_y.checked_add(size - 1)?;
    Some(Rect::of(min_x, min_y, max_x, max_y))
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
    let diff_x = i16::from(delta_x(r)) + 1 - i16::from(size);
    let diff_y = i16::from(delta_y(r)) + 1 - i16::from(size);
    let temp_min_x = i16::from(r.min.x) + diff_x / 2;
    let temp_min_y = i16::from(r.min.y) + diff_y / 2;
    let min_x = temp_min_x.clamp(0, i16::from(u8::MAX) - i16::from(size) + 1);
    let min_y = temp_min_y.clamp(0, i16::from(u8::MAX) - i16::from(size) + 1);
    r.min.x = min_x as u8;
    r.min.y = min_y as u8;
    r.max.x = (min_x + i16::from(size) - 1) as u8;
    r.max.y = (min_y + i16::from(size) - 1) as u8;
    Some(())
}

pub fn try_saturating_resize(r: &Rect, size: u8) -> Option<Rect> {
    if size < 3 {
        return None;
    }
    let diff_x = i16::from(delta_x(r)) + 1 - i16::from(size);
    let diff_y = i16::from(delta_y(r)) + 1 - i16::from(size);
    let temp_min_x = i16::from(r.min.x) + diff_x / 2;
    let temp_min_y = i16::from(r.min.y) + diff_y / 2;
    let clamped_min_x = temp_min_x.clamp(0, i16::from(u8::MAX) - i16::from(size) + 1);
    let clamped_min_y = temp_min_y.clamp(0, i16::from(u8::MAX) - i16::from(size) + 1);
    let min_x = clamped_min_x as u8;
    let min_y = clamped_min_y as u8;
    let max_x = (clamped_min_x + i16::from(size) - 1) as u8;
    let max_y = (clamped_min_y + i16::from(size) - 1) as u8;
    Some(Rect::of(min_x, min_y, max_x, max_y))
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
    let diff_x = i16::from(delta_x(r)) + 1 - i16::from(size);
    let diff_y = i16::from(delta_y(r)) + 1 - i16::from(size);
    let temp_min_x = i16::from(r.min.x) + diff_x / 2;
    let temp_min_y = i16::from(r.min.y) + diff_y / 2;
    let min_x = temp_min_x as u8;
    let min_y = temp_min_y as u8;
    let max_x = min_x.wrapping_add(size - 1);
    let max_y = min_y.wrapping_add(size - 1);
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

pub fn try_wrapping_resize(r: &Rect, size: u8) -> Option<Rect> {
    if size < 3 {
        return None;
    }
    let diff_x = i16::from(delta_x(r)) + 1 - i16::from(size);
    let diff_y = i16::from(delta_y(r)) + 1 - i16::from(size);
    let temp_min_x = i16::from(r.min.x) + diff_x / 2;
    let temp_min_y = i16::from(r.min.y) + diff_y / 2;
    let min_x = temp_min_x as u8;
    let min_y = temp_min_y as u8;
    let max_x = min_x.wrapping_add(size - 1);
    let max_y = min_y.wrapping_add(size - 1);
    Some(Rect::of(min_x, min_y, max_x, max_y))
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

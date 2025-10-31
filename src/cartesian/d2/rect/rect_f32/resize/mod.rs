use super::{Rect, delta_x, delta_y};
use crate::cartesian::d1::point::point_f32::{MAX, MIN};

pub fn try_checked_resize_assign(r: &mut Rect, size: f32) -> Option<()> {
    if !(3.0..=MAX).contains(&size) {
        return None;
    }
    let diff_x = ((delta_x(r) + 1.0 - size) / 2.0).floor();
    let diff_y = ((delta_y(r) + 1.0 - size) / 2.0).floor();
    if diff_x < MIN - r.min.x || diff_y < MIN - r.min.y {
        return None;
    }
    let min_x = r.min.x + diff_x;
    let min_y = r.min.y + diff_y;
    let max_x = min_x + size - 1.0;
    let max_y = min_y + size - 1.0;
    if (size - 1.0) > MAX - min_x || (size - 1.0) > MAX - min_y {
        return None;
    }
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

pub fn try_checked_resize(r: &Rect, size: f32) -> Option<Rect> {
    if !(3.0..=MAX).contains(&size) {
        return None;
    }
    let diff_x = ((delta_x(r) + 1.0 - size) / 2.0).floor();
    let diff_y = ((delta_y(r) + 1.0 - size) / 2.0).floor();
    if diff_x < MIN - r.min.x || diff_y < MIN - r.min.y {
        return None;
    }
    let min_x = r.min.x + diff_x;
    let min_y = r.min.y + diff_y;
    let max_x = min_x + size - 1.0;
    let max_y = min_y + size - 1.0;
    if (size - 1.0) > MAX - min_x || (size - 1.0) > MAX - min_y {
        return None;
    }
    Some(Rect::of((min_x, min_y), (max_x, max_y)))
}

pub fn checked_resize_assign(r: &mut Rect, size: f32) {
    try_checked_resize_assign(r, size).unwrap()
}

pub fn checked_resize(r: &Rect, size: f32) -> Rect {
    try_checked_resize(r, size).unwrap()
}

pub fn try_saturating_resize_assign(r: &mut Rect, size: f32) -> Option<()> {
    if !(3.0..=MAX).contains(&size) {
        return None;
    }
    let diff_x = delta_x(r) + 1.0 - size;
    let diff_y = delta_y(r) + 1.0 - size;
    let temp_min_x = r.min.x + diff_x / 2.0;
    let temp_min_y = r.min.y + diff_y / 2.0;
    let min_x = temp_min_x.clamp(MIN, MAX - size + 1.0);
    let min_y = temp_min_y.clamp(MIN, MAX - size + 1.0);
    let max_x = min_x + size - 1.0;
    let max_y = min_y + size - 1.0;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

pub fn try_saturating_resize(r: &Rect, size: f32) -> Option<Rect> {
    if !(3.0..=MAX).contains(&size) {
        return None;
    }
    let diff_x = delta_x(r) + 1.0 - size;
    let diff_y = delta_y(r) + 1.0 - size;
    let temp_min_x = r.min.x + diff_x / 2.0;
    let temp_min_y = r.min.y + diff_y / 2.0;
    let min_x = temp_min_x.clamp(MIN, MAX - size + 1.0);
    let min_y = temp_min_y.clamp(MIN, MAX - size + 1.0);
    let max_x = min_x + size - 1.0;
    let max_y = min_y + size - 1.0;
    Some(Rect::of((min_x, min_y), (max_x, max_y)))
}

pub fn saturating_resize_assign(r: &mut Rect, size: f32) {
    try_saturating_resize_assign(r, size).unwrap()
}

pub fn saturating_resize(r: &Rect, size: f32) -> Rect {
    try_saturating_resize(r, size).unwrap()
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

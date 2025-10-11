use super::Rect;
use crate::cartesian::d2::point::point_i32::Point;

pub fn try_checked_inflate_assign(r: &mut Rect) -> Option<()> {
    let min_x = r.min.x.checked_sub(1)?;
    let min_y = r.min.y.checked_sub(1)?;
    let max_x = r.max.x.checked_add(1)?;
    let max_y = r.max.y.checked_add(1)?;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

pub fn try_checked_inflate(r: &Rect) -> Option<Rect> {
    let min_x = r.min.x.checked_sub(1)?;
    let min_y = r.min.y.checked_sub(1)?;
    let max_x = r.max.x.checked_add(1)?;
    let max_y = r.max.y.checked_add(1)?;
    Some(Rect { min: Point { x: min_x, y: min_y }, max: Point { x: max_x, y: max_y } })
}

pub fn checked_inflate_assign(r: &mut Rect) {
    try_checked_inflate_assign(r).unwrap()
}

pub fn checked_inflate(r: &Rect) -> Rect {
    try_checked_inflate(r).unwrap()
}

pub fn try_saturating_inflate_assign(r: &mut Rect) -> Option<()> {
    let is_min_x = r.min.x == i32::MIN;
    let is_min_y = r.min.y == i32::MIN;
    let is_max_x = r.max.x == i32::MAX;
    let is_max_y = r.max.y == i32::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - i32::from(is_min_x) + i32::from(is_max_x);
    let min_y_modifier = 1 - i32::from(is_min_y) + i32::from(is_max_y);
    let max_x_modifier = 1 + i32::from(is_min_x) - i32::from(is_max_x);
    let max_y_modifier = 1 + i32::from(is_min_y) - i32::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
    Some(())
}

pub fn try_saturating_inflate(r: &Rect) -> Option<Rect> {
    let is_min_x = r.min.x == i32::MIN;
    let is_min_y = r.min.y == i32::MIN;
    let is_max_x = r.max.x == i32::MAX;
    let is_max_y = r.max.y == i32::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - i32::from(is_min_x) + i32::from(is_max_x);
    let min_y_modifier = 1 - i32::from(is_min_y) + i32::from(is_max_y);
    let max_x_modifier = 1 + i32::from(is_min_x) - i32::from(is_max_x);
    let max_y_modifier = 1 + i32::from(is_min_y) - i32::from(is_max_y);
    let min_x = r.min.x.saturating_sub(min_x_modifier);
    let min_y = r.min.y.saturating_sub(min_y_modifier);
    let max_x = r.max.x.saturating_add(max_x_modifier);
    let max_y = r.max.y.saturating_add(max_y_modifier);
    Some(Rect { min: Point { x: min_x, y: min_y }, max: Point { x: max_x, y: max_y } })
}

pub fn saturating_inflate_assign(r: &mut Rect) {
    try_saturating_inflate_assign(r).unwrap()
}

pub fn saturating_inflate(r: &Rect) -> Rect {
    try_saturating_inflate(r).unwrap()
}

pub fn wrapping_inflate_assign(r: &mut Rect) {
    let min_x = r.min.x.wrapping_sub(1);
    let min_y = r.min.y.wrapping_sub(1);
    let max_x = r.max.x.wrapping_add(1);
    let max_y = r.max.y.wrapping_add(1);
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
}

pub fn wrapping_inflate(r: &Rect) -> Rect {
    let min_x = r.min.x.wrapping_sub(1);
    let min_y = r.min.y.wrapping_sub(1);
    let max_x = r.max.x.wrapping_add(1);
    let max_y = r.max.y.wrapping_add(1);
    Rect { min: Point { x: min_x, y: min_y }, max: Point { x: max_x, y: max_y } }
}

#[cfg(test)]
mod test_try_checked_inflate_assign;

#[cfg(test)]
mod test_try_checked_inflate;

#[cfg(test)]
mod test_checked_inflate_assign;

#[cfg(test)]
mod test_checked_inflate;

#[cfg(test)]
mod test_try_saturating_inflate_assign;

#[cfg(test)]
mod test_try_saturating_inflate;

#[cfg(test)]
mod test_saturating_inflate_assign;

#[cfg(test)]
mod test_saturating_inflate;

#[cfg(test)]
mod test_wrapping_inflate_assign;

#[cfg(test)]
mod test_wrapping_inflate;

use crate::cartesian::{
    point::{point_i32::PointI32, point_u32::Point},
    rect::rect_u32::{Rect, delta_x, delta_y},
};

pub fn try_checked_translate_assign(r: &mut Rect, delta: &PointI32) -> Option<()> {
    let min_x = r.min.x.checked_add_signed(delta.x)?;
    let min_y = r.min.y.checked_add_signed(delta.y)?;
    let max_x = r.max.x.checked_add_signed(delta.x)?;
    let max_y = r.max.y.checked_add_signed(delta.y)?;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

pub fn try_checked_translate(r: &Rect, delta: &PointI32) -> Option<Rect> {
    let min_x = r.min.x.checked_add_signed(delta.x)?;
    let min_y = r.min.y.checked_add_signed(delta.y)?;
    let max_x = r.max.x.checked_add_signed(delta.x)?;
    let max_y = r.max.y.checked_add_signed(delta.y)?;
    Some(Rect { min: Point { x: min_x, y: min_y }, max: Point { x: max_x, y: max_y } })
}

pub fn checked_translate_assign(r: &mut Rect, delta: &PointI32) {
    try_checked_translate_assign(r, delta).unwrap()
}

pub fn checked_translate(r: &Rect, delta: &PointI32) -> Rect {
    try_checked_translate(r, delta).unwrap()
}

pub fn saturating_translate_assign(r: &mut Rect, delta: &PointI32) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i64::from(r.min.x) + i64::from(delta.x);
    let temp_min_y = i64::from(r.min.y) + i64::from(delta.y);
    let clamped_x = temp_min_x.clamp(0, i64::from(u32::MAX) - i64::from(dx));
    let clamped_y = temp_min_y.clamp(0, i64::from(u32::MAX) - i64::from(dy));
    let min_x = clamped_x as u32;
    let min_y = clamped_y as u32;
    let max_x = min_x + dx;
    let max_y = min_y + dy;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
}

pub fn saturating_translate(r: &Rect, delta: &PointI32) -> Rect {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i64::from(r.min.x) + i64::from(delta.x);
    let temp_min_y = i64::from(r.min.y) + i64::from(delta.y);
    let clamped_x = temp_min_x.clamp(0, i64::from(u32::MAX) - i64::from(dx));
    let clamped_y = temp_min_y.clamp(0, i64::from(u32::MAX) - i64::from(dy));
    let min_x = clamped_x as u32;
    let min_y = clamped_y as u32;
    let max_x = min_x + dx;
    let max_y = min_y + dy;
    Rect { min: Point { x: min_x, y: min_y }, max: Point { x: max_x, y: max_y } }
}

pub fn wrapping_translate_assign(r: &mut Rect, delta: &PointI32) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let min_x = r.min.x.wrapping_add_signed(delta.x);
    let min_y = r.min.y.wrapping_add_signed(delta.y);
    let max_x = min_x.wrapping_add(dx);
    let max_y = min_y.wrapping_add(dy);
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
}

pub fn wrapping_translate(r: &Rect, delta: &PointI32) -> Rect {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let min_x = r.min.x.wrapping_add_signed(delta.x);
    let min_y = r.min.y.wrapping_add_signed(delta.y);
    let max_x = min_x.wrapping_add(dx);
    let max_y = min_y.wrapping_add(dy);
    Rect { min: Point { x: min_x, y: min_y }, max: Point { x: max_x, y: max_y } }
}

#[cfg(test)]
mod test_try_checked_translate_assign;

#[cfg(test)]
mod test_try_checked_translate;

#[cfg(test)]
mod test_checked_translate_assign;

#[cfg(test)]
mod test_checked_translate;

#[cfg(test)]
mod test_saturating_translate_assign;

#[cfg(test)]
mod test_saturating_translate;

#[cfg(test)]
mod test_wrapping_translate_assign;

#[cfg(test)]
mod test_wrapping_translate;

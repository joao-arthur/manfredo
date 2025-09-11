use crate::cartesian::{
    point::point_i16::PointI16,
    rect::rect_i16::{Rect, delta_x, delta_y},
};

pub fn try_checked_translate_assign(r: &mut Rect, delta: &PointI16) -> Option<()> {
    let min_x = r.min.x.checked_add(delta.x)?;
    let min_y = r.min.y.checked_add(delta.y)?;
    let max_x = r.max.x.checked_add(delta.x)?;
    let max_y = r.max.y.checked_add(delta.y)?;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

pub fn try_checked_translate(r: &Rect, delta: &PointI16) -> Option<Rect> {
    let min_x = r.min.x.checked_add(delta.x)?;
    let min_y = r.min.y.checked_add(delta.y)?;
    let max_x = r.max.x.checked_add(delta.x)?;
    let max_y = r.max.y.checked_add(delta.y)?;
    Some(Rect { min: PointI16 { x: min_x, y: min_y }, max: PointI16 { x: max_x, y: max_y } })
}

pub fn checked_translate_assign(r: &mut Rect, delta: &PointI16) {
    try_checked_translate_assign(r, delta).unwrap()
}

pub fn checked_translate(r: &Rect, delta: &PointI16) -> Rect {
    try_checked_translate(r, delta).unwrap()
}

pub fn saturating_translate_assign(r: &mut Rect, delta: &PointI16) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i32::from(r.min.x) + i32::from(delta.x);
    let temp_min_y = i32::from(r.min.y) + i32::from(delta.y);
    let min_x = temp_min_x.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(dx));
    let min_y = temp_min_y.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(dy));
    r.min.x = min_x as i16;
    r.min.y = min_y as i16;
    r.max.x = (min_x + i32::from(dx)) as i16;
    r.max.y = (min_y + i32::from(dy)) as i16;
}

pub fn saturating_translate(r: &Rect, delta: &PointI16) -> Rect {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i32::from(r.min.x) + i32::from(delta.x);
    let temp_min_y = i32::from(r.min.y) + i32::from(delta.y);
    let min_x = temp_min_x.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(dx));
    let min_y = temp_min_y.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(dy));
    let max_x = min_x + i32::from(dx);
    let max_y = min_y + i32::from(dy);
    Rect { min: PointI16 { x: min_x as i16, y: min_y as i16 }, max: PointI16 { x: max_x as i16, y: max_y as i16 } }
}

pub fn wrapping_translate_assign(r: &mut Rect, delta: &PointI16) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let min_x = r.min.x.wrapping_add(delta.x);
    let min_y = r.min.y.wrapping_add(delta.y);
    let max_x = min_x.wrapping_add_unsigned(dx);
    let max_y = min_y.wrapping_add_unsigned(dy);
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
}

pub fn wrapping_translate(r: &Rect, delta: &PointI16) -> Rect {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let min_x = r.min.x.wrapping_add(delta.x);
    let min_y = r.min.y.wrapping_add(delta.y);
    let max_x = min_x.wrapping_add_unsigned(dx);
    let max_y = min_y.wrapping_add_unsigned(dy);
    Rect { min: PointI16 { x: min_x, y: min_y }, max: PointI16 { x: max_x, y: max_y } }
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

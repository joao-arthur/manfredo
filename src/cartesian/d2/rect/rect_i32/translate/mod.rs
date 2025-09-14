use crate::cartesian::d2::{
    point::point_i32::Point,
    rect::rect_i32::{Rect, delta_x, delta_y},
};

pub fn try_checked_translate_assign(r: &mut Rect, delta: &Point) -> Option<()> {
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

pub fn try_checked_translate(r: &Rect, delta: &Point) -> Option<Rect> {
    let min_x = r.min.x.checked_add(delta.x)?;
    let min_y = r.min.y.checked_add(delta.y)?;
    let max_x = r.max.x.checked_add(delta.x)?;
    let max_y = r.max.y.checked_add(delta.y)?;
    Some(Rect { min: Point { x: min_x, y: min_y }, max: Point { x: max_x, y: max_y } })
}

pub fn checked_translate_assign(r: &mut Rect, delta: &Point) {
    try_checked_translate_assign(r, delta).unwrap()
}

pub fn checked_translate(r: &Rect, delta: &Point) -> Rect {
    try_checked_translate(r, delta).unwrap()
}

pub fn saturating_translate_assign(r: &mut Rect, delta: &Point) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i64::from(r.min.x) + i64::from(delta.x);
    let temp_min_y = i64::from(r.min.y) + i64::from(delta.y);
    let min_x = temp_min_x.clamp(i64::from(i32::MIN), i64::from(i32::MAX) - i64::from(dx));
    let min_y = temp_min_y.clamp(i64::from(i32::MIN), i64::from(i32::MAX) - i64::from(dy));
    r.min.x = min_x as i32;
    r.min.y = min_y as i32;
    r.max.x = (min_x + i64::from(dx)) as i32;
    r.max.y = (min_y + i64::from(dy)) as i32;
}

pub fn saturating_translate(r: &Rect, delta: &Point) -> Rect {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i64::from(r.min.x) + i64::from(delta.x);
    let temp_min_y = i64::from(r.min.y) + i64::from(delta.y);
    let min_x = temp_min_x.clamp(i64::from(i32::MIN), i64::from(i32::MAX) - i64::from(dx));
    let min_y = temp_min_y.clamp(i64::from(i32::MIN), i64::from(i32::MAX) - i64::from(dy));
    let max_x = min_x + i64::from(dx);
    let max_y = min_y + i64::from(dy);
    Rect { min: Point { x: min_x as i32, y: min_y as i32 }, max: Point { x: max_x as i32, y: max_y as i32 } }
}

pub fn wrapping_translate_assign(r: &mut Rect, delta: &Point) {
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

pub fn wrapping_translate(r: &Rect, delta: &Point) -> Rect {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let min_x = r.min.x.wrapping_add(delta.x);
    let min_y = r.min.y.wrapping_add(delta.y);
    let max_x = min_x.wrapping_add_unsigned(dx);
    let max_y = min_y.wrapping_add_unsigned(dy);
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

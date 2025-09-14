use crate::cartesian::{
    point::point_i8::Point,
    rect::rect_i8::{Rect, delta_x, delta_y},
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
    let temp_min_x = i16::from(r.min.x) + i16::from(delta.x);
    let temp_min_y = i16::from(r.min.y) + i16::from(delta.y);
    let min_x = temp_min_x.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(dx));
    let min_y = temp_min_y.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(dy));
    r.min.x = min_x as i8;
    r.min.y = min_y as i8;
    r.max.x = (min_x + i16::from(dx)) as i8;
    r.max.y = (min_y + i16::from(dy)) as i8;
}

pub fn saturating_translate(r: &Rect, delta: &Point) -> Rect {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i16::from(r.min.x) + i16::from(delta.x);
    let temp_min_y = i16::from(r.min.y) + i16::from(delta.y);
    let min_x = temp_min_x.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(dx));
    let min_y = temp_min_y.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(dy));
    let max_x = min_x + i16::from(dx);
    let max_y = min_y + i16::from(dy);
    Rect { min: Point { x: min_x as i8, y: min_y as i8 }, max: Point { x: max_x as i8, y: max_y as i8 } }
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

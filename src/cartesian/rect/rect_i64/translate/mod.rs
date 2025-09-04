use crate::cartesian::{
    point::point_i64::PointI64,
    rect::rect_i64::{RectI64, delta_x, delta_y},
};

pub fn try_checked_translate_assign(r: &mut RectI64, delta: &PointI64) -> Option<()> {
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

pub fn try_checked_translate(r: &RectI64, delta: &PointI64) -> Option<RectI64> {
    let min_x = r.min.x.checked_add(delta.x)?;
    let min_y = r.min.y.checked_add(delta.y)?;
    let max_x = r.max.x.checked_add(delta.x)?;
    let max_y = r.max.y.checked_add(delta.y)?;
    Some(RectI64 { min: PointI64 { x: min_x, y: min_y }, max: PointI64 { x: max_x, y: max_y } })
}

pub fn checked_translate_assign(r: &mut RectI64, delta: &PointI64) {
    try_checked_translate_assign(r, delta).unwrap()
}

pub fn checked_translate(r: &RectI64, delta: &PointI64) -> RectI64 {
    try_checked_translate(r, delta).unwrap()
}

pub fn saturating_translate_assign(r: &mut RectI64, delta: &PointI64) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i128::from(r.min.x) + i128::from(delta.x);
    let temp_min_y = i128::from(r.min.y) + i128::from(delta.y);
    let min_x = temp_min_x.clamp(i128::from(i64::MIN), i128::from(i64::MAX) - i128::from(dx));
    let min_y = temp_min_y.clamp(i128::from(i64::MIN), i128::from(i64::MAX) - i128::from(dy));
    r.min.x = min_x as i64;
    r.min.y = min_y as i64;
    r.max.x = (min_x + i128::from(dx)) as i64;
    r.max.y = (min_y + i128::from(dy)) as i64;
}

pub fn saturating_translate(r: &RectI64, delta: &PointI64) -> RectI64 {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i128::from(r.min.x) + i128::from(delta.x);
    let temp_min_y = i128::from(r.min.y) + i128::from(delta.y);
    let min_x = temp_min_x.clamp(i128::from(i64::MIN), i128::from(i64::MAX) - i128::from(dx));
    let min_y = temp_min_y.clamp(i128::from(i64::MIN), i128::from(i64::MAX) - i128::from(dy));
    let max_x = min_x + i128::from(dx);
    let max_y = min_y + i128::from(dy);
    RectI64 { min: PointI64 { x: min_x as i64, y: min_y as i64 }, max: PointI64 { x: max_x as i64, y: max_y as i64 } }
}

pub fn wrapping_translate_assign(r: &mut RectI64, delta: &PointI64) {
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

pub fn wrapping_translate(r: &RectI64, delta: &PointI64) -> RectI64 {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let min_x = r.min.x.wrapping_add(delta.x);
    let min_y = r.min.y.wrapping_add(delta.y);
    let max_x = min_x.wrapping_add_unsigned(dx);
    let max_y = min_y.wrapping_add_unsigned(dy);
    RectI64 { min: PointI64 { x: min_x, y: min_y }, max: PointI64 { x: max_x, y: max_y } }
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

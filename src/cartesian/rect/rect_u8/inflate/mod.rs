use crate::cartesian::{point::point_u8::PointU8, rect::rect_u8::RectU8};

pub fn try_checked_inflate_assign(r: &mut RectU8) -> Option<()> {
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

pub fn try_checked_inflate(r: &RectU8) -> Option<RectU8> {
    let min_x = r.min.x.checked_sub(1)?;
    let min_y = r.min.y.checked_sub(1)?;
    let max_x = r.max.x.checked_add(1)?;
    let max_y = r.max.y.checked_add(1)?;
    Some(RectU8 { min: PointU8 { x: min_x, y: min_y }, max: PointU8 { x: max_x, y: max_y } })
}

pub fn checked_inflate_assign(r: &mut RectU8) {
    try_checked_inflate_assign(r).unwrap()
}

pub fn checked_inflate(r: &RectU8) -> RectU8 {
    try_checked_inflate(r).unwrap()
}

pub fn try_saturating_inflate_assign(r: &mut RectU8) -> Option<()> {
    let is_min_x = r.min.x == 0;
    let is_min_y = r.min.y == 0;
    let is_max_x = r.max.x == u8::MAX;
    let is_max_y = r.max.y == u8::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - u8::from(is_min_x) + u8::from(is_max_x);
    let min_y_modifier = 1 - u8::from(is_min_y) + u8::from(is_max_y);
    let max_x_modifier = 1 + u8::from(is_min_x) - u8::from(is_max_x);
    let max_y_modifier = 1 + u8::from(is_min_y) - u8::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
    Some(())
}

pub fn try_saturating_inflate(r: &RectU8) -> Option<RectU8> {
    let is_min_x = r.min.x == 0;
    let is_min_y = r.min.y == 0;
    let is_max_x = r.max.x == u8::MAX;
    let is_max_y = r.max.y == u8::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - u8::from(is_min_x) + u8::from(is_max_x);
    let min_y_modifier = 1 - u8::from(is_min_y) + u8::from(is_max_y);
    let max_x_modifier = 1 + u8::from(is_min_x) - u8::from(is_max_x);
    let max_y_modifier = 1 + u8::from(is_min_y) - u8::from(is_max_y);
    let min_x = r.min.x.saturating_sub(min_x_modifier);
    let min_y = r.min.y.saturating_sub(min_y_modifier);
    let max_x = r.max.x.saturating_add(max_x_modifier);
    let max_y = r.max.y.saturating_add(max_y_modifier);
    Some(RectU8 { min: PointU8 { x: min_x, y: min_y }, max: PointU8 { x: max_x, y: max_y } })
}

pub fn saturating_inflate_assign(r: &mut RectU8) {
    try_saturating_inflate_assign(r).unwrap()
}

pub fn saturating_inflate(r: &RectU8) -> RectU8 {
    try_saturating_inflate(r).unwrap()
}

pub mod checked;
pub mod saturated;

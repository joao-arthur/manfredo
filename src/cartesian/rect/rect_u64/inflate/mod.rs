use crate::cartesian::{point::point_u64::PointU64, rect::rect_u64::RectU64};

pub fn try_checked_inflate_assign(r: &mut RectU64) -> Option<()> {
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

pub fn try_checked_inflate(r: &RectU64) -> Option<RectU64> {
    let min_x = r.min.x.checked_sub(1)?;
    let min_y = r.min.y.checked_sub(1)?;
    let max_x = r.max.x.checked_add(1)?;
    let max_y = r.max.y.checked_add(1)?;
    Some(RectU64 { min: PointU64 { x: min_x, y: min_y }, max: PointU64 { x: max_x, y: max_y } })
}

pub fn checked_inflate_assign(r: &mut RectU64) {
    try_checked_inflate_assign(r).unwrap()
}

pub fn checked_inflate(r: &RectU64) -> RectU64 {
    try_checked_inflate(r).unwrap()
}

pub fn try_saturating_inflate_assign(r: &mut RectU64) -> Option<()> {
    let is_min_x = r.min.x == 0;
    let is_min_y = r.min.y == 0;
    let is_max_x = r.max.x == u64::MAX;
    let is_max_y = r.max.y == u64::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - u64::from(is_min_x) + u64::from(is_max_x);
    let min_y_modifier = 1 - u64::from(is_min_y) + u64::from(is_max_y);
    let max_x_modifier = 1 + u64::from(is_min_x) - u64::from(is_max_x);
    let max_y_modifier = 1 + u64::from(is_min_y) - u64::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
    Some(())
}

pub fn try_saturating_inflate(r: &RectU64) -> Option<RectU64> {
    let is_min_x = r.min.x == 0;
    let is_min_y = r.min.y == 0;
    let is_max_x = r.max.x == u64::MAX;
    let is_max_y = r.max.y == u64::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - u64::from(is_min_x) + u64::from(is_max_x);
    let min_y_modifier = 1 - u64::from(is_min_y) + u64::from(is_max_y);
    let max_x_modifier = 1 + u64::from(is_min_x) - u64::from(is_max_x);
    let max_y_modifier = 1 + u64::from(is_min_y) - u64::from(is_max_y);
    let min_x = r.min.x.saturating_sub(min_x_modifier);
    let min_y = r.min.y.saturating_sub(min_y_modifier);
    let max_x = r.max.x.saturating_add(max_x_modifier);
    let max_y = r.max.y.saturating_add(max_y_modifier);
    Some(RectU64 { min: PointU64 { x: min_x, y: min_y }, max: PointU64 { x: max_x, y: max_y } })
}

pub fn saturating_inflate_assign(r: &mut RectU64) {
    try_saturating_inflate_assign(r).unwrap()
}

pub fn saturating_inflate(r: &RectU64) -> RectU64 {
    try_saturating_inflate(r).unwrap()
}

pub mod checked;
pub mod saturated;

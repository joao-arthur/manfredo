use crate::cartesian::{
    point::point_f32::{MAX, MIN, PointF32},
    rect::rect_f32::RectF32,
};

pub fn try_checked_add_assign(r: &mut RectF32, delta: &RectF32) -> Option<()> {
    if delta.min.x < MIN - r.min.x || delta.min.y < MIN - r.min.y || delta.max.x > MAX - r.max.x || delta.max.y > MAX - r.max.y {
        return None;
    }
    r.min.x += delta.min.x;
    r.min.y += delta.min.y;
    r.max.x += delta.max.x;
    r.max.y += delta.max.y;
    Some(())
}

pub fn try_checked_add(r: &RectF32, delta: &RectF32) -> Option<RectF32> {
    if delta.min.x < MIN - r.min.x || delta.min.y < MIN - r.min.y || delta.max.x > MAX - r.max.x || delta.max.y > MAX - r.max.y {
        return None;
    }
    let min_x = r.min.x + delta.min.x;
    let min_y = r.min.y + delta.min.y;
    let max_x = r.max.x + delta.max.x;
    let max_y = r.max.y + delta.max.y;
    Some(RectF32 { min: PointF32 { x: min_x, y: min_y }, max: PointF32 { x: max_x, y: max_y } })
}

pub fn checked_add_assign(r: &mut RectF32, delta: &RectF32) {
    try_checked_add_assign(r, delta).unwrap()
}

pub fn checked_add(r: &RectF32, delta: &RectF32) -> RectF32 {
    try_checked_add(r, delta).unwrap()
}

#[cfg(test)]
mod test_checked_add_assign;

#[cfg(test)]
mod test_checked_add;

#[cfg(test)]
mod test_try_checked_add_assign;

#[cfg(test)]
mod test_try_checked_add;

use crate::cartesian::{point::point_i32::PointI32, rect::rect_i32::RectI32};

pub fn try_checked_add_assign(r: &mut RectI32, delta: &RectI32) -> Option<()> {
    let min_x = r.min.x.checked_add(delta.min.x)?;
    let min_y = r.min.y.checked_add(delta.min.y)?;
    let max_x = r.max.x.checked_add(delta.max.x)?;
    let max_y = r.max.y.checked_add(delta.max.y)?;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

pub fn try_checked_add(r: &RectI32, delta: &RectI32) -> Option<RectI32> {
    let min_x = r.min.x.checked_add(delta.min.x)?;
    let min_y = r.min.y.checked_add(delta.min.y)?;
    let max_x = r.max.x.checked_add(delta.max.x)?;
    let max_y = r.max.y.checked_add(delta.max.y)?;
    Some(RectI32 { min: PointI32 { x: min_x, y: min_y }, max: PointI32 { x: max_x, y: max_y } })
}

pub fn checked_add_assign(r: &mut RectI32, delta: &RectI32) {
    try_checked_add_assign(r, delta).unwrap()
}

pub fn checked_add(r: &RectI32, delta: &RectI32) -> RectI32 {
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

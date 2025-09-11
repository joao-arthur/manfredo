use crate::cartesian::{
    point::point_u32::Point,
    rect::{rect_i32::Rect as RectI, rect_u32::Rect},
};

pub fn try_checked_add_assign(r: &mut Rect, delta: &RectI) -> Option<()> {
    let min_x = r.min.x.checked_add_signed(delta.min.x)?;
    let min_y = r.min.y.checked_add_signed(delta.min.y)?;
    let max_x = r.max.x.checked_add_signed(delta.max.x)?;
    let max_y = r.max.y.checked_add_signed(delta.max.y)?;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

pub fn try_checked_add(r: &Rect, delta: &RectI) -> Option<Rect> {
    let min_x = r.min.x.checked_add_signed(delta.min.x)?;
    let min_y = r.min.y.checked_add_signed(delta.min.y)?;
    let max_x = r.max.x.checked_add_signed(delta.max.x)?;
    let max_y = r.max.y.checked_add_signed(delta.max.y)?;
    Some(Rect { min: Point { x: min_x, y: min_y }, max: Point { x: max_x, y: max_y } })
}

pub fn checked_add_assign(r: &mut Rect, delta: &RectI) {
    try_checked_add_assign(r, delta).unwrap()
}

pub fn checked_add(r: &Rect, delta: &RectI) -> Rect {
    try_checked_add(r, delta).unwrap()
}

pub fn saturating_add_assign(r: &mut Rect, delta: &RectI) {
    r.min.x = r.min.x.saturating_add_signed(delta.min.x);
    r.min.y = r.min.y.saturating_add_signed(delta.min.y);
    r.max.x = r.max.x.saturating_add_signed(delta.max.x);
    r.max.y = r.max.y.saturating_add_signed(delta.max.y);
}

pub fn saturating_add(r: &Rect, delta: &RectI) -> Rect {
    let min_x = r.min.x.saturating_add_signed(delta.min.x);
    let min_y = r.min.y.saturating_add_signed(delta.min.y);
    let max_x = r.max.x.saturating_add_signed(delta.max.x);
    let max_y = r.max.y.saturating_add_signed(delta.max.y);
    Rect { min: Point { x: min_x, y: min_y }, max: Point { x: max_x, y: max_y } }
}

pub fn wrapping_add_assign(r: &mut Rect, delta: &RectI) {
    r.min.x = r.min.x.wrapping_add_signed(delta.min.x);
    r.min.y = r.min.y.wrapping_add_signed(delta.min.y);
    r.max.x = r.max.x.wrapping_add_signed(delta.max.x);
    r.max.y = r.max.y.wrapping_add_signed(delta.max.y);
}

pub fn wrapping_add(r: &Rect, delta: &RectI) -> Rect {
    let min_x = r.min.x.wrapping_add_signed(delta.min.x);
    let min_y = r.min.y.wrapping_add_signed(delta.min.y);
    let max_x = r.max.x.wrapping_add_signed(delta.max.x);
    let max_y = r.max.y.wrapping_add_signed(delta.max.y);
    Rect { min: Point { x: min_x, y: min_y }, max: Point { x: max_x, y: max_y } }
}

#[cfg(test)]
mod test_checked_add_assign;

#[cfg(test)]
mod test_checked_add;

#[cfg(test)]
mod test_try_checked_add_assign;

#[cfg(test)]
mod test_try_checked_add;

#[cfg(test)]
mod test_saturating_add_assign;

#[cfg(test)]
mod test_saturating_add;

#[cfg(test)]
mod test_wrapping_add_assign;

#[cfg(test)]
mod test_wrapping_add;

use crate::cartesian::rect::rect_i64::Rect;

pub fn try_checked_add_assign(r: &mut Rect, delta: &Rect) -> Option<()> {
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

pub fn try_checked_add(r: &Rect, delta: &Rect) -> Option<Rect> {
    let min_x = r.min.x.checked_add(delta.min.x)?;
    let min_y = r.min.y.checked_add(delta.min.y)?;
    let max_x = r.max.x.checked_add(delta.max.x)?;
    let max_y = r.max.y.checked_add(delta.max.y)?;
    Some(Rect::of(min_x, min_y, max_x, max_y))
}

pub fn checked_add_assign(r: &mut Rect, delta: &Rect) {
    try_checked_add_assign(r, delta).unwrap()
}

pub fn checked_add(r: &Rect, delta: &Rect) -> Rect {
    try_checked_add(r, delta).unwrap()
}

pub fn saturating_add_assign(r: &mut Rect, delta: &Rect) {
    r.min.x = r.min.x.saturating_add(delta.min.x);
    r.min.y = r.min.y.saturating_add(delta.min.y);
    r.max.x = r.max.x.saturating_add(delta.max.x);
    r.max.y = r.max.y.saturating_add(delta.max.y);
}

pub fn saturating_add(r: &Rect, delta: &Rect) -> Rect {
    let min_x = r.min.x.saturating_add(delta.min.x);
    let min_y = r.min.y.saturating_add(delta.min.y);
    let max_x = r.max.x.saturating_add(delta.max.x);
    let max_y = r.max.y.saturating_add(delta.max.y);
    Rect::of(min_x, min_y, max_x, max_y)
}

pub fn wrapping_add_assign(r: &mut Rect, delta: &Rect) {
    r.min.x = r.min.x.wrapping_add(delta.min.x);
    r.min.y = r.min.y.wrapping_add(delta.min.y);
    r.max.x = r.max.x.wrapping_add(delta.max.x);
    r.max.y = r.max.y.wrapping_add(delta.max.y);
}

pub fn wrapping_add(r: &Rect, delta: &Rect) -> Rect {
    let min_x = r.min.x.wrapping_add(delta.min.x);
    let min_y = r.min.y.wrapping_add(delta.min.y);
    let max_x = r.max.x.wrapping_add(delta.max.x);
    let max_y = r.max.y.wrapping_add(delta.max.y);
    Rect::of(min_x, min_y, max_x, max_y)
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

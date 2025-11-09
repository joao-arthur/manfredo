use super::Rect;
use crate::cartesian::d2::rect::rect_i16;

pub fn try_checked_add_assign(r: &mut Rect, delta: &rect_i16::Rect) -> Option<()> {
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

pub fn try_checked_add(r: &Rect, delta: &rect_i16::Rect) -> Option<Rect> {
    let min_x = r.min.x.checked_add_signed(delta.min.x)?;
    let min_y = r.min.y.checked_add_signed(delta.min.y)?;
    let max_x = r.max.x.checked_add_signed(delta.max.x)?;
    let max_y = r.max.y.checked_add_signed(delta.max.y)?;
    Some(Rect::new((min_x, min_y), (max_x, max_y)))
}

pub fn checked_add_assign(r: &mut Rect, delta: &rect_i16::Rect) {
    try_checked_add_assign(r, delta).unwrap()
}

pub fn checked_add(r: &Rect, delta: &rect_i16::Rect) -> Rect {
    try_checked_add(r, delta).unwrap()
}

pub fn saturating_add_assign(r: &mut Rect, delta: &rect_i16::Rect) {
    r.min.x = r.min.x.saturating_add_signed(delta.min.x);
    r.min.y = r.min.y.saturating_add_signed(delta.min.y);
    r.max.x = r.max.x.saturating_add_signed(delta.max.x);
    r.max.y = r.max.y.saturating_add_signed(delta.max.y);
}

pub fn saturating_add(r: &Rect, delta: &rect_i16::Rect) -> Rect {
    let min_x = r.min.x.saturating_add_signed(delta.min.x);
    let min_y = r.min.y.saturating_add_signed(delta.min.y);
    let max_x = r.max.x.saturating_add_signed(delta.max.x);
    let max_y = r.max.y.saturating_add_signed(delta.max.y);
    Rect::new((min_x, min_y), (max_x, max_y))
}

pub fn wrapping_add_assign(r: &mut Rect, delta: &rect_i16::Rect) {
    r.min.x = r.min.x.wrapping_add_signed(delta.min.x);
    r.min.y = r.min.y.wrapping_add_signed(delta.min.y);
    r.max.x = r.max.x.wrapping_add_signed(delta.max.x);
    r.max.y = r.max.y.wrapping_add_signed(delta.max.y);
}

pub fn wrapping_add(r: &Rect, delta: &rect_i16::Rect) -> Rect {
    let min_x = r.min.x.wrapping_add_signed(delta.min.x);
    let min_y = r.min.y.wrapping_add_signed(delta.min.y);
    let max_x = r.max.x.wrapping_add_signed(delta.max.x);
    let max_y = r.max.y.wrapping_add_signed(delta.max.y);
    Rect::new((min_x, min_y), (max_x, max_y))
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

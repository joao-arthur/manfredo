use crate::cartesian::{
    point::point_f32::{MAX, MIN},
    rect::rect_f32::Rect,
};

pub fn try_checked_inflate_assign(r: &mut Rect) -> Option<()> {
    if r.min.x == MIN || r.min.y == MIN || r.max.x == MAX || r.max.y == MAX {
        return None;
    }
    r.min.x -= 1.0;
    r.min.y -= 1.0;
    r.max.x += 1.0;
    r.max.y += 1.0;
    Some(())
}

pub fn try_checked_inflate(r: &Rect) -> Option<Rect> {
    if r.min.x == MIN || r.min.y == MIN || r.max.x == MAX || r.max.y == MAX {
        return None;
    }
    let min_x = r.min.x - 1.0;
    let min_y = r.min.y - 1.0;
    let max_x = r.max.x + 1.0;
    let max_y = r.max.y + 1.0;
    Some(Rect::of(min_x, min_y, max_x, max_y))
}

pub fn checked_inflate_assign(r: &mut Rect) {
    try_checked_inflate_assign(r).unwrap()
}

pub fn checked_inflate(r: &Rect) -> Rect {
    try_checked_inflate(r).unwrap()
}

pub fn try_saturating_inflate_assign(r: &mut Rect) -> Option<()> {
    let is_min_x = r.min.x == MIN;
    let is_min_y = r.min.y == MIN;
    let is_max_x = r.max.x == MAX;
    let is_max_y = r.max.y == MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1.0 - f32::from(is_min_x) + f32::from(is_max_x);
    let min_y_modifier = 1.0 - f32::from(is_min_y) + f32::from(is_max_y);
    let max_x_modifier = 1.0 + f32::from(is_min_x) - f32::from(is_max_x);
    let max_y_modifier = 1.0 + f32::from(is_min_y) - f32::from(is_max_y);
    r.min.x = (r.min.x - min_x_modifier).max(MIN);
    r.min.y = (r.min.y - min_y_modifier).max(MIN);
    r.max.x = (r.max.x + max_x_modifier).min(MAX);
    r.max.y = (r.max.y + max_y_modifier).min(MAX);
    Some(())
}

pub fn try_saturating_inflate(r: &Rect) -> Option<Rect> {
    let is_min_x = r.min.x == MIN;
    let is_min_y = r.min.y == MIN;
    let is_max_x = r.max.x == MAX;
    let is_max_y = r.max.y == MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1.0 - f32::from(is_min_x) + f32::from(is_max_x);
    let min_y_modifier = 1.0 - f32::from(is_min_y) + f32::from(is_max_y);
    let max_x_modifier = 1.0 + f32::from(is_min_x) - f32::from(is_max_x);
    let max_y_modifier = 1.0 + f32::from(is_min_y) - f32::from(is_max_y);
    let min_x = (r.min.x - min_x_modifier).max(MIN);
    let min_y = (r.min.y - min_y_modifier).max(MIN);
    let max_x = (r.max.x + max_x_modifier).min(MAX);
    let max_y = (r.max.y + max_y_modifier).min(MAX);
    Some(Rect::of(min_x, min_y, max_x, max_y))
}

pub fn saturating_inflate_assign(r: &mut Rect) {
    try_saturating_inflate_assign(r).unwrap()
}

pub fn saturating_inflate(r: &Rect) -> Rect {
    try_saturating_inflate(r).unwrap()
}

#[cfg(test)]
mod test_try_checked_inflate_assign;

#[cfg(test)]
mod test_try_checked_inflate;

#[cfg(test)]
mod test_checked_inflate_assign;

#[cfg(test)]
mod test_checked_inflate;

#[cfg(test)]
mod test_try_saturating_inflate_assign;

#[cfg(test)]
mod test_try_saturating_inflate;

#[cfg(test)]
mod test_saturating_inflate_assign;

#[cfg(test)]
mod test_saturating_inflate;

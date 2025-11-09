use super::Rect;
use crate::cartesian::d1::point::point_f64::{MAX, MIN};

pub fn try_checked_add_assign(r: &mut Rect, delta: &Rect) -> Option<()> {
    if delta.min.x < MIN - r.min.x || delta.min.y < MIN - r.min.y || delta.max.x > MAX - r.max.x || delta.max.y > MAX - r.max.y {
        return None;
    }
    r.min.x += delta.min.x;
    r.min.y += delta.min.y;
    r.max.x += delta.max.x;
    r.max.y += delta.max.y;
    Some(())
}

pub fn try_checked_add(r: &Rect, delta: &Rect) -> Option<Rect> {
    if delta.min.x < MIN - r.min.x || delta.min.y < MIN - r.min.y || delta.max.x > MAX - r.max.x || delta.max.y > MAX - r.max.y {
        return None;
    }
    let min_x = r.min.x + delta.min.x;
    let min_y = r.min.y + delta.min.y;
    let max_x = r.max.x + delta.max.x;
    let max_y = r.max.y + delta.max.y;
    Some(Rect::new((min_x, min_y), (max_x, max_y)))
}

pub fn checked_add_assign(r: &mut Rect, delta: &Rect) {
    try_checked_add_assign(r, delta).unwrap()
}

pub fn checked_add(r: &Rect, delta: &Rect) -> Rect {
    try_checked_add(r, delta).unwrap()
}

pub fn saturating_add_assign(r: &mut Rect, delta: &Rect) {
    r.min.x = (r.min.x + delta.min.x).clamp(MIN, MAX);
    r.min.y = (r.min.y + delta.min.y).clamp(MIN, MAX);
    r.max.x = (r.max.x + delta.max.x).clamp(MIN, MAX);
    r.max.y = (r.max.y + delta.max.y).clamp(MIN, MAX);
}

pub fn saturating_add(r: &Rect, delta: &Rect) -> Rect {
    let min_x = (r.min.x + delta.min.x).clamp(MIN, MAX);
    let min_y = (r.min.y + delta.min.y).clamp(MIN, MAX);
    let max_x = (r.max.x + delta.max.x).clamp(MIN, MAX);
    let max_y = (r.max.y + delta.max.y).clamp(MIN, MAX);
    Rect::new((min_x, min_y), (max_x, max_y))
}

pub fn wrapping_add_assign(r: &mut Rect, delta: &Rect) {
    if delta.min.x > 0.0 && MAX - r.min.x < delta.min.x {
        let diff = MAX - r.min.x;
        let delta_adjusted = delta.min.x - diff - 1.0;
        r.min.x = MIN + delta_adjusted;
    } else if delta.min.x < 0.0 && -(r.min.x - MIN) > delta.min.x {
        let diff = MIN - r.min.x;
        let delta_adjusted = delta.min.x - diff + 1.0;
        r.min.x = MAX + delta_adjusted;
    } else {
        r.min.x += delta.min.x;
    }
    if delta.min.y > 0.0 && MAX - r.min.y < delta.min.y {
        let diff = MAX - r.min.y;
        let delta_adjusted = delta.min.y - diff - 1.0;
        r.min.y = MIN + delta_adjusted;
    } else if delta.min.y < 0.0 && -(r.min.y - MIN) > delta.min.y {
        let diff = MIN - r.min.y;
        let delta_adjusted = delta.min.y - diff + 1.0;
        r.min.y = MAX + delta_adjusted;
    } else {
        r.min.y += delta.min.y;
    }
    if delta.max.x > 0.0 && MAX - r.max.x < delta.max.x {
        let diff = MAX - r.max.x;
        let delta_adjusted = delta.max.x - diff - 1.0;
        r.max.x = MIN + delta_adjusted;
    } else if delta.max.x < 0.0 && -(r.max.x - MIN) > delta.max.x {
        let diff = MIN - r.max.x;
        let delta_adjusted = delta.max.x - diff + 1.0;
        r.max.x = MAX + delta_adjusted;
    } else {
        r.max.x += delta.max.x;
    }
    if delta.max.y > 0.0 && MAX - r.max.y < delta.max.y {
        let diff = MAX - r.max.y;
        let delta_adjusted = delta.max.y - diff - 1.0;
        r.max.y = MIN + delta_adjusted;
    } else if delta.max.y < 0.0 && -(r.max.y - MIN) > delta.max.y {
        let diff = MIN - r.max.y;
        let delta_adjusted = delta.max.y - diff + 1.0;
        r.max.y = MAX + delta_adjusted;
    } else {
        r.max.y += delta.max.y;
    }
}

pub fn wrapping_add(r: &Rect, delta: &Rect) -> Rect {
    let mut min_x = r.min.x;
    let mut min_y = r.min.y;
    let mut max_x = r.max.x;
    let mut max_y = r.max.y;
    if delta.min.x > 0.0 && MAX - r.min.x < delta.min.x {
        let diff = MAX - min_x;
        let delta_adjusted = delta.min.x - diff - 1.0;
        min_x = MIN + delta_adjusted;
    } else if delta.min.x < 0.0 && -(r.min.x - MIN) > delta.min.x {
        let diff = MIN - min_x;
        let delta_adjusted = delta.min.x - diff + 1.0;
        min_x = MAX + delta_adjusted;
    } else {
        min_x += delta.min.x;
    }
    if delta.min.y > 0.0 && MAX - r.min.y < delta.min.y {
        let diff = MAX - min_y;
        let delta_adjusted = delta.min.y - diff - 1.0;
        min_y = MIN + delta_adjusted;
    } else if delta.min.y < 0.0 && -(r.min.y - MIN) > delta.min.y {
        let diff = MIN - min_y;
        let delta_adjusted = delta.min.y - diff + 1.0;
        min_y = MAX + delta_adjusted;
    } else {
        min_y += delta.min.y;
    }
    if delta.max.x > 0.0 && MAX - r.max.x < delta.max.x {
        let diff = MAX - max_x;
        let delta_adjusted = delta.max.x - diff - 1.0;
        max_x = MIN + delta_adjusted;
    } else if delta.max.x < 0.0 && -(r.max.x - MIN) > delta.max.x {
        let diff = MIN - max_x;
        let delta_adjusted = delta.max.x - diff + 1.0;
        max_x = MAX + delta_adjusted;
    } else {
        max_x += delta.max.x;
    }
    if delta.max.y > 0.0 && MAX - r.max.y < delta.max.y {
        let diff = MAX - max_y;
        let delta_adjusted = delta.max.y - diff - 1.0;
        max_y = MIN + delta_adjusted;
    } else if delta.max.y < 0.0 && -(r.max.y - MIN) > delta.max.y {
        let diff = MIN - max_y;
        let delta_adjusted = delta.max.y - diff + 1.0;
        max_y = MAX + delta_adjusted;
    } else {
        max_y += delta.max.y;
    }
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

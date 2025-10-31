use super::{Rect, delta_x, delta_y};
use crate::cartesian::{
    d1::point::point_f64::{MAX, MIN},
    d2::point::point_f64::Point,
};

pub fn try_checked_translate_assign(r: &mut Rect, delta: &Point) -> Option<()> {
    if delta.x < MIN - r.min.x || delta.y < MIN - r.min.y || delta.x > MAX - r.max.x || delta.y > MAX - r.max.y {
        return None;
    }
    r.min.x += delta.x;
    r.min.y += delta.y;
    r.max.x += delta.x;
    r.max.y += delta.y;
    Some(())
}

pub fn try_checked_translate(r: &Rect, delta: &Point) -> Option<Rect> {
    if delta.x < MIN - r.min.x || delta.y < MIN - r.min.y || delta.x > MAX - r.max.x || delta.y > MAX - r.max.y {
        return None;
    }
    let min_x = r.min.x + delta.x;
    let min_y = r.min.y + delta.y;
    let max_x = r.max.x + delta.x;
    let max_y = r.max.y + delta.y;
    Some(Rect::of((min_x, min_y), (max_x, max_y)))
}

pub fn checked_translate_assign(r: &mut Rect, delta: &Point) {
    try_checked_translate_assign(r, delta).unwrap()
}

pub fn checked_translate(r: &Rect, delta: &Point) -> Rect {
    try_checked_translate(r, delta).unwrap()
}

pub fn saturating_translate_assign(r: &mut Rect, delta: &Point) {
    let dx = delta_x(r).clamp(MIN, MAX);
    let dy = delta_y(r).clamp(MIN, MAX);
    let temp_min_x = r.min.x + delta.x;
    let temp_min_y = r.min.y + delta.y;
    let min_x = temp_min_x.clamp(MIN, MAX - dx);
    let min_y = temp_min_y.clamp(MIN, MAX - dy);
    let max_x = min_x + dx;
    let max_y = min_y + dy;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
}

pub fn saturating_translate(r: &Rect, delta: &Point) -> Rect {
    let dx = delta_x(r).clamp(MIN, MAX);
    let dy = delta_y(r).clamp(MIN, MAX);
    let temp_min_x = r.min.x + delta.x;
    let temp_min_y = r.min.y + delta.y;
    let min_x = temp_min_x.clamp(MIN, MAX - dx);
    let min_y = temp_min_y.clamp(MIN, MAX - dy);
    let max_x = min_x + dx;
    let max_y = min_y + dy;
    Rect::of((min_x, min_y), (max_x, max_y))
}

pub fn wrapping_translate_assign(r: &mut Rect, delta: &Point) {
    if delta.x > 0.0 && MAX - r.min.x < delta.x {
        let diff = MAX - r.min.x;
        let delta_adjusted = delta.x - diff - 1.0;
        r.min.x = MIN + delta_adjusted;
    } else if delta.x < 0.0 && -(r.min.x - MIN) > delta.x {
        let diff = MIN - r.min.x;
        let delta_adjusted = delta.x - diff + 1.0;
        r.min.x = MAX + delta_adjusted;
    } else {
        r.min.x += delta.x;
    }
    if delta.y > 0.0 && MAX - r.min.y < delta.y {
        let diff = MAX - r.min.y;
        let delta_adjusted = delta.y - diff - 1.0;
        r.min.y = MIN + delta_adjusted;
    } else if delta.y < 0.0 && -(r.min.y - MIN) > delta.y {
        let diff = MIN - r.min.y;
        let delta_adjusted = delta.y - diff + 1.0;
        r.min.y = MAX + delta_adjusted;
    } else {
        r.min.y += delta.y;
    }
    if delta.x > 0.0 && MAX - r.max.x < delta.x {
        let diff = MAX - r.max.x;
        let delta_adjusted = delta.x - diff - 1.0;
        r.max.x = MIN + delta_adjusted;
    } else if delta.x < 0.0 && -(r.max.x - MIN) > delta.x {
        let diff = MIN - r.max.x;
        let delta_adjusted = delta.x - diff + 1.0;
        r.max.x = MAX + delta_adjusted;
    } else {
        r.max.x += delta.x;
    }
    if delta.y > 0.0 && MAX - r.max.y < delta.y {
        let diff = MAX - r.max.y;
        let delta_adjusted = delta.y - diff - 1.0;
        r.max.y = MIN + delta_adjusted;
    } else if delta.y < 0.0 && -(r.max.y - MIN) > delta.y {
        let diff = MIN - r.max.y;
        let delta_adjusted = delta.y - diff + 1.0;
        r.max.y = MAX + delta_adjusted;
    } else {
        r.max.y += delta.y;
    }
}

pub fn wrapping_translate(r: &Rect, delta: &Point) -> Rect {
    let mut min_x = r.min.x;
    let mut min_y = r.min.y;
    let mut max_x = r.max.x;
    let mut max_y = r.max.y;
    if delta.x > 0.0 && MAX - r.min.x < delta.x {
        let diff = MAX - min_x;
        let delta_adjusted = delta.x - diff - 1.0;
        min_x = MIN + delta_adjusted;
    } else if delta.x < 0.0 && -(r.min.x - MIN) > delta.x {
        let diff = MIN - min_x;
        let delta_adjusted = delta.x - diff + 1.0;
        min_x = MAX + delta_adjusted;
    } else {
        min_x += delta.x;
    }
    if delta.y > 0.0 && MAX - r.min.y < delta.y {
        let diff = MAX - min_y;
        let delta_adjusted = delta.y - diff - 1.0;
        min_y = MIN + delta_adjusted;
    } else if delta.y < 0.0 && -(r.min.y - MIN) > delta.y {
        let diff = MIN - min_y;
        let delta_adjusted = delta.y - diff + 1.0;
        min_y = MAX + delta_adjusted;
    } else {
        min_y += delta.y;
    }
    if delta.x > 0.0 && MAX - r.max.x < delta.x {
        let diff = MAX - max_x;
        let delta_adjusted = delta.x - diff - 1.0;
        max_x = MIN + delta_adjusted;
    } else if delta.x < 0.0 && -(r.max.x - MIN) > delta.x {
        let diff = MIN - max_x;
        let delta_adjusted = delta.x - diff + 1.0;
        max_x = MAX + delta_adjusted;
    } else {
        max_x += delta.x;
    }
    if delta.y > 0.0 && MAX - r.max.y < delta.y {
        let diff = MAX - max_y;
        let delta_adjusted = delta.y - diff - 1.0;
        max_y = MIN + delta_adjusted;
    } else if delta.y < 0.0 && -(r.max.y - MIN) > delta.y {
        let diff = MIN - max_y;
        let delta_adjusted = delta.y - diff + 1.0;
        max_y = MAX + delta_adjusted;
    } else {
        max_y += delta.y;
    }
    Rect::of((min_x, min_y), (max_x, max_y))
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

use crate::cartesian::{
    point::point_f32::{MAX, MIN, PointF32},
    rect::rect_f32::{RectF32, delta_x, delta_y},
};

pub fn try_checked_translate_assign(r: &mut RectF32, delta: &PointF32) -> Option<()> {
    if delta.x < MIN - r.min.x || delta.y < MIN - r.min.y || delta.x > MAX - r.max.x || delta.y > MAX - r.max.y {
        return None;
    }
    r.min.x += delta.x;
    r.min.y += delta.y;
    r.max.x += delta.x;
    r.max.y += delta.y;
    Some(())
}

pub fn try_checked_translate(r: &RectF32, delta: &PointF32) -> Option<RectF32> {
    if delta.x < MIN - r.min.x || delta.y < MIN - r.min.y || delta.x > MAX - r.max.x || delta.y > MAX - r.max.y {
        return None;
    }
    let min_x = r.min.x + delta.x;
    let min_y = r.min.y + delta.y;
    let max_x = r.max.x + delta.x;
    let max_y = r.max.y + delta.y;
    Some(RectF32 { min: PointF32 { x: min_x, y: min_y }, max: PointF32 { x: max_x, y: max_y } })
}

pub fn checked_translate_assign(r: &mut RectF32, delta: &PointF32) {
    try_checked_translate_assign(r, delta).unwrap()
}

pub fn checked_translate(r: &RectF32, delta: &PointF32) -> RectF32 {
    try_checked_translate(r, delta).unwrap()
}

pub fn saturating_translate_assign(r: &mut RectF32, delta: &PointF32) {
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

pub fn saturating_translate(r: &RectF32, delta: &PointF32) -> RectF32 {
    let dx = delta_x(r).clamp(MIN, MAX);
    let dy = delta_y(r).clamp(MIN, MAX);
    let temp_min_x = r.min.x + delta.x;
    let temp_min_y = r.min.y + delta.y;
    let min_x = temp_min_x.clamp(MIN, MAX - dx);
    let min_y = temp_min_y.clamp(MIN, MAX - dy);
    let max_x = min_x + dx;
    let max_y = min_y + dy;
    RectF32 { min: PointF32 { x: min_x, y: min_y }, max: PointF32 { x: max_x, y: max_y } }
}

pub fn wrapping_translate_assign(r: &mut RectF32, delta: &PointF32) {
    if r.min.x + delta.x > MAX {
        let diff_min_x = MAX - r.min.x;
        let delta_x_adjusted = delta.x - diff_min_x - 1.0;
        r.min.x = MIN + delta_x_adjusted;
    } else if r.min.x + delta.x < MIN {
        let diff_min_x = MIN - r.min.x;
        let delta_x_adjusted = delta.x - diff_min_x + 1.0;
        r.min.x = MAX + delta_x_adjusted;
    } else {
        r.min.x += delta.x;
    }
    if r.min.y + delta.y > MAX {
        let diff_min_y = MAX - r.min.y;
        let delta_y_adjusted = delta.y - diff_min_y - 1.0;
        r.min.y = MIN + delta_y_adjusted;
    } else if r.min.y + delta.y < MIN {
        let diff_min_y = MIN - r.min.y;
        let delta_y_adjusted = delta.y - diff_min_y + 1.0;
        r.min.y = MAX + delta_y_adjusted;
    } else {
        r.min.y += delta.y;
    }
    if r.max.x + delta.x > MAX {
        let diff_min_x = MAX - r.max.x;
        let delta_x_adjusted = delta.x - diff_min_x - 1.0;
        r.max.x = MIN + delta_x_adjusted;
    } else if r.max.x + delta.x < MIN {
        let diff_min_x = MIN - r.max.x;
        let delta_x_adjusted = delta.x - diff_min_x + 1.0;
        r.max.x = MAX + delta_x_adjusted;
    } else {
        r.max.x += delta.x;
    }
    if r.max.y + delta.y > MAX {
        let diff_min_y = MAX - r.max.y;
        let delta_y_adjusted = delta.y - diff_min_y - 1.0;
        r.max.y = MIN + delta_y_adjusted;
    } else if r.max.y + delta.y < MIN {
        let diff_min_y = MIN - r.max.y;
        let delta_y_adjusted = delta.y - diff_min_y + 1.0;
        r.max.y = MAX + delta_y_adjusted;
    } else {
        r.max.y += delta.y;
    }
}

pub fn wrapping_translate(r: &RectF32, delta: &PointF32) -> RectF32 {
    let mut min_x = r.min.x;
    let mut min_y = r.min.y;
    let mut max_x = r.max.x;
    let mut max_y = r.max.y;

    if min_x + delta.x > MAX {
        let diff_min_x = MAX - min_x;
        let delta_x_adjusted = delta.x - diff_min_x - 1.0;
        min_x = MIN + delta_x_adjusted;
    } else if min_x + delta.x < MIN {
        let diff_min_x = MIN - min_x;
        let delta_x_adjusted = delta.x - diff_min_x + 1.0;
        min_x = MAX + delta_x_adjusted;
    } else {
        min_x += delta.x;
    }
    if min_y + delta.y > MAX {
        let diff_min_y = MAX - min_y;
        let delta_y_adjusted = delta.y - diff_min_y - 1.0;
        min_y = MIN + delta_y_adjusted;
    } else if min_y + delta.y < MIN {
        let diff_min_y = MIN - min_y;
        let delta_y_adjusted = delta.y - diff_min_y + 1.0;
        min_y = MAX + delta_y_adjusted;
    } else {
        min_y += delta.y;
    }
    if max_x + delta.x > MAX {
        let diff_min_x = MAX - max_x;
        let delta_x_adjusted = delta.x - diff_min_x - 1.0;
        max_x = MIN + delta_x_adjusted;
    } else if max_x + delta.x < MIN {
        let diff_min_x = MIN - max_x;
        let delta_x_adjusted = delta.x - diff_min_x + 1.0;
        max_x = MAX + delta_x_adjusted;
    } else {
        max_x += delta.x;
    }
    if max_y + delta.y > MAX {
        let diff_min_y = MAX - max_y;
        let delta_y_adjusted = delta.y - diff_min_y - 1.0;
        max_y = MIN + delta_y_adjusted;
    } else if max_y + delta.y < MIN {
        let diff_min_y = MIN - max_y;
        let delta_y_adjusted = delta.y - diff_min_y + 1.0;
        max_y = MAX + delta_y_adjusted;
    } else {
        max_y += delta.y;
    }

    RectF32 { min: PointF32 { x: min_x, y: min_y }, max: PointF32 { x: max_x, y: max_y } }
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

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

pub fn saturating_add_assign(r: &mut RectF32, delta: &RectF32) {
    r.min.x = (r.min.x + delta.min.x).clamp(MIN, MAX);
    r.min.y = (r.min.y + delta.min.y).clamp(MIN, MAX);
    r.max.x = (r.max.x + delta.max.x).clamp(MIN, MAX);
    r.max.y = (r.max.y + delta.max.y).clamp(MIN, MAX);
}

pub fn saturating_add(r: &RectF32, delta: &RectF32) -> RectF32 {
    let min_x = (r.min.x + delta.min.x).clamp(MIN, MAX);
    let min_y = (r.min.y + delta.min.y).clamp(MIN, MAX);
    let max_x = (r.max.x + delta.max.x).clamp(MIN, MAX);
    let max_y = (r.max.y + delta.max.y).clamp(MIN, MAX);
    RectF32 { min: PointF32 { x: min_x, y: min_y }, max: PointF32 { x: max_x, y: max_y } }
}

pub fn wrapping_add_assign(r: &mut RectF32, delta: &RectF32) {
    if r.min.x + delta.min.x > MAX {
        let diff_min_x = MAX - r.min.x;
        let delta_x_adjusted = delta.min.x - diff_min_x - 1.0;
        r.min.x = MIN + delta_x_adjusted;
    } else if r.min.x + delta.min.x < MIN {
        let diff_min_x = MIN - r.min.x;
        let delta_x_adjusted = delta.min.x - diff_min_x + 1.0;
        r.min.x = MAX + delta_x_adjusted;
    } else {
        r.min.x += delta.min.x;
    }
    if r.min.y + delta.min.y > MAX {
        let diff_min_y = MAX - r.min.y;
        let delta_y_adjusted = delta.min.y - diff_min_y - 1.0;
        r.min.y = MIN + delta_y_adjusted;
    } else if r.min.y + delta.min.y < MIN {
        let diff_min_y = MIN - r.min.y;
        let delta_y_adjusted = delta.min.y - diff_min_y + 1.0;
        r.min.y = MAX + delta_y_adjusted;
    } else {
        r.min.y += delta.min.y;
    }
    if r.max.x + delta.max.x > MAX {
        let diff_min_x = MAX - r.max.x;
        let delta_x_adjusted = delta.max.x - diff_min_x - 1.0;
        r.max.x = MIN + delta_x_adjusted;
    } else if r.max.x + delta.max.x < MIN {
        let diff_min_x = MIN - r.max.x;
        let delta_x_adjusted = delta.max.x - diff_min_x + 1.0;
        r.max.x = MAX + delta_x_adjusted;
    } else {
        r.max.x += delta.max.x;
    }
    if r.max.y + delta.max.y > MAX {
        let diff_min_y = MAX - r.max.y;
        let delta_y_adjusted = delta.max.y - diff_min_y - 1.0;
        r.max.y = MIN + delta_y_adjusted;
    } else if r.max.y + delta.max.y < MIN {
        let diff_min_y = MIN - r.max.y;
        let delta_y_adjusted = delta.max.y - diff_min_y + 1.0;
        r.max.y = MAX + delta_y_adjusted;
    } else {
        r.max.y += delta.max.y;
    }
}

pub fn wrapping_add(r: &RectF32, delta: &RectF32) -> RectF32 {
    let mut min_x = r.min.x;
    let mut min_y = r.min.y;
    let mut max_x = r.max.x;
    let mut max_y = r.max.y;
    if min_x + delta.min.x > MAX {
        let diff_min_x = MAX - min_x;
        let delta_x_adjusted = delta.min.x - diff_min_x - 1.0;
        min_x = MIN + delta_x_adjusted;
    } else if min_x + delta.min.x < MIN {
        let diff_min_x = MIN - min_x;
        let delta_x_adjusted = delta.min.x - diff_min_x + 1.0;
        min_x = MAX + delta_x_adjusted;
    } else {
        min_x += delta.min.x;
    }
    if min_y + delta.min.y > MAX {
        let diff_min_y = MAX - min_y;
        let delta_y_adjusted = delta.min.y - diff_min_y - 1.0;
        min_y = MIN + delta_y_adjusted;
    } else if min_y + delta.min.y < MIN {
        let diff_min_y = MIN - min_y;
        let delta_y_adjusted = delta.min.y - diff_min_y + 1.0;
        min_y = MAX + delta_y_adjusted;
    } else {
        min_y += delta.min.y;
    }
    if max_x + delta.max.x > MAX {
        let diff_min_x = MAX - max_x;
        let delta_x_adjusted = delta.max.x - diff_min_x - 1.0;
        max_x = MIN + delta_x_adjusted;
    } else if max_x + delta.max.x < MIN {
        let diff_min_x = MIN - max_x;
        let delta_x_adjusted = delta.max.x - diff_min_x + 1.0;
        max_x = MAX + delta_x_adjusted;
    } else {
        max_x += delta.max.x;
    }
    if max_y + delta.max.y > MAX {
        let diff_min_y = MAX - max_y;
        let delta_y_adjusted = delta.max.y - diff_min_y - 1.0;
        max_y = MIN + delta_y_adjusted;
    } else if max_y + delta.max.y < MIN {
        let diff_min_y = MIN - max_y;
        let delta_y_adjusted = delta.max.y - diff_min_y + 1.0;
        max_y = MAX + delta_y_adjusted;
    } else {
        max_y += delta.max.y;
    }
    RectF32 { min: PointF32 { x: min_x, y: min_y }, max: PointF32 { x: max_x, y: max_y } }
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

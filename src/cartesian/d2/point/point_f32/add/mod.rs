use super::Point;
use crate::cartesian::d1::point::point_f32::{MAX, MIN};

pub fn try_checked_add_assign(p: &mut Point, delta: &Point) -> Option<()> {
    let x = p.x + delta.x;
    let y = p.y + delta.y;
    if !(MIN..=MAX).contains(&x) || !(MIN..=MAX).contains(&y) {
        return None;
    }
    p.x = x;
    p.y = y;
    Some(())
}

pub fn try_checked_add(p: &Point, delta: &Point) -> Option<Point> {
    let x = p.x + delta.x;
    let y = p.y + delta.y;
    if !(MIN..=MAX).contains(&x) || !(MIN..=MAX).contains(&y) {
        return None;
    }
    Some(Point { x, y })
}

pub fn checked_add_assign(p: &mut Point, delta: &Point) {
    try_checked_add_assign(p, delta).unwrap()
}

pub fn checked_add(p: &Point, delta: &Point) -> Point {
    try_checked_add(p, delta).unwrap()
}

pub fn saturating_add_assign(p: &mut Point, delta: &Point) {
    let temp_x = p.x + delta.x;
    let temp_y = p.y + delta.y;
    p.x = temp_x.clamp(MIN, MAX);
    p.y = temp_y.clamp(MIN, MAX);
}

pub fn saturating_add(p: &Point, delta: &Point) -> Point {
    let temp_x = p.x + delta.x;
    let temp_y = p.y + delta.y;
    Point::of(temp_x.clamp(MIN, MAX), temp_y.clamp(MIN, MAX))
}

pub fn wrapping_add_assign(p: &mut Point, delta: &Point) {
    if p.x + delta.x > MAX {
        let diff_min_x = MAX - p.x;
        let delta_x_adjusted = delta.x - diff_min_x - 1.0;
        p.x = MIN + delta_x_adjusted;
    } else if p.x + delta.x < MIN {
        let diff_min_x = MIN - p.x;
        let delta_x_adjusted = delta.x - diff_min_x + 1.0;
        p.x = MAX + delta_x_adjusted;
    } else {
        p.x += delta.x;
    }
    if p.y + delta.y > MAX {
        let diff_min_y = MAX - p.y;
        let delta_y_adjusted = delta.y - diff_min_y - 1.0;
        p.y = MIN + delta_y_adjusted;
    } else if p.y + delta.y < MIN {
        let diff_min_y = MIN - p.y;
        let delta_y_adjusted = delta.y - diff_min_y + 1.0;
        p.y = MAX + delta_y_adjusted;
    } else {
        p.y += delta.y;
    }
}

pub fn wrapping_add(p: &Point, delta: &Point) -> Point {
    let mut x = p.x;
    let mut y = p.y;
    if x + delta.x > MAX {
        let diff_min_x = MAX - x;
        let delta_x_adjusted = delta.x - diff_min_x - 1.0;
        x = MIN + delta_x_adjusted;
    } else if x + delta.x < MIN {
        let diff_min_x = MIN - x;
        let delta_x_adjusted = delta.x - diff_min_x + 1.0;
        x = MAX + delta_x_adjusted;
    } else {
        x += delta.x;
    }
    if y + delta.y > MAX {
        let diff_min_y = MAX - y;
        let delta_y_adjusted = delta.y - diff_min_y - 1.0;
        y = MIN + delta_y_adjusted;
    } else if y + delta.y < MIN {
        let diff_min_y = MIN - y;
        let delta_y_adjusted = delta.y - diff_min_y + 1.0;
        y = MAX + delta_y_adjusted;
    } else {
        y += delta.y;
    }
    Point { x, y }
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

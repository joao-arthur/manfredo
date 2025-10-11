use super::{MAX, MIN, Point};

pub fn try_checked_add_assign(p: &mut Point, delta: &Point) -> Option<()> {
    let x = p.x + delta.x;
    if !(MIN..=MAX).contains(&x) {
        return None;
    }
    p.x = x;
    Some(())
}

pub fn try_checked_add(p: &Point, delta: &Point) -> Option<Point> {
    let x = p.x + delta.x;
    if !(MIN..=MAX).contains(&x) {
        return None;
    }
    Some(Point { x })
}

pub fn checked_add_assign(p: &mut Point, delta: &Point) {
    try_checked_add_assign(p, delta).unwrap()
}

pub fn checked_add(p: &Point, delta: &Point) -> Point {
    try_checked_add(p, delta).unwrap()
}

pub fn saturating_add_assign(p: &mut Point, delta: &Point) {
    let temp_x = p.x + delta.x;
    p.x = temp_x.clamp(MIN, MAX);
}

pub fn saturating_add(p: &Point, delta: &Point) -> Point {
    let temp_x = p.x + delta.x;
    Point::of(temp_x.clamp(MIN, MAX))
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
}

pub fn wrapping_add(p: &Point, delta: &Point) -> Point {
    let mut x = p.x;
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
    Point { x }
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

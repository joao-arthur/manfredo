use super::{MAX, MIN, Point};

pub fn try_checked_add_assign(p: &mut Point, delta: &Point) -> Option<()> {
    let i = p.i + delta.i;
    if !(MIN..=MAX).contains(&i) {
        return None;
    }
    p.i = i;
    Some(())
}

pub fn try_checked_add(p: &Point, delta: &Point) -> Option<Point> {
    let i = p.i + delta.i;
    if !(MIN..=MAX).contains(&i) {
        return None;
    }
    Some(Point { i })
}

pub fn checked_add_assign(p: &mut Point, delta: &Point) {
    try_checked_add_assign(p, delta).unwrap()
}

pub fn checked_add(p: &Point, delta: &Point) -> Point {
    try_checked_add(p, delta).unwrap()
}

pub fn saturating_add_assign(p: &mut Point, delta: &Point) {
    let temp_i = p.i + delta.i;
    p.i = temp_i.clamp(MIN, MAX);
}

pub fn saturating_add(p: &Point, delta: &Point) -> Point {
    let temp_i = p.i + delta.i;
    Point::of(temp_i.clamp(MIN, MAX))
}

pub fn wrapping_add_assign(p: &mut Point, delta: &Point) {
    if p.i + delta.i > MAX {
        let diff_min_i = MAX - p.i;
        let delta_i_adjusted = delta.i - diff_min_i - 1.0;
        p.i = MIN + delta_i_adjusted;
    } else if p.i + delta.i < MIN {
        let diff_min_i = MIN - p.i;
        let delta_i_adjusted = delta.i - diff_min_i + 1.0;
        p.i = MAX + delta_i_adjusted;
    } else {
        p.i += delta.i;
    }
}

pub fn wrapping_add(p: &Point, delta: &Point) -> Point {
    let mut i = p.i;
    if i + delta.i > MAX {
        let diff_min_i = MAX - i;
        let delta_i_adjusted = delta.i - diff_min_i - 1.0;
        i = MIN + delta_i_adjusted;
    } else if i + delta.i < MIN {
        let diff_min_i = MIN - i;
        let delta_i_adjusted = delta.i - diff_min_i + 1.0;
        i = MAX + delta_i_adjusted;
    } else {
        i += delta.i;
    }
    Point { i }
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

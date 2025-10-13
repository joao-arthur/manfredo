use super::Point;

pub fn try_checked_add_assign(p: &mut Point, delta: &Point) -> Option<()> {
    let i = p.i.checked_add(delta.i)?;
    p.i = i;
    Some(())
}

pub fn try_checked_add(p: &Point, delta: &Point) -> Option<Point> {
    let i = p.i.checked_add(delta.i)?;
    Some(Point { i })
}

pub fn checked_add_assign(p: &mut Point, delta: &Point) {
    try_checked_add_assign(p, delta).unwrap()
}

pub fn checked_add(p: &Point, delta: &Point) -> Point {
    try_checked_add(p, delta).unwrap()
}

pub fn saturating_add_assign(p: &mut Point, delta: &Point) {
    p.i = p.i.saturating_add(delta.i);
}

pub fn saturating_add(p: &Point, delta: &Point) -> Point {
    let i = p.i.saturating_add(delta.i);
    Point { i }
}

pub fn wrapping_add_assign(p: &mut Point, delta: &Point) {
    p.i = p.i.wrapping_add(delta.i);
}

pub fn wrapping_add(p: &Point, delta: &Point) -> Point {
    let i = p.i.wrapping_add(delta.i);
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

use crate::cartesian::d2::point::{point_i8, point_u8::Point};

pub fn try_checked_add_assign(p: &mut Point, delta: &point_i8::Point) -> Option<()> {
    let x = p.x.checked_add_signed(delta.x)?;
    let y = p.y.checked_add_signed(delta.y)?;
    p.x = x;
    p.y = y;
    Some(())
}

pub fn try_checked_add(p: &Point, delta: &point_i8::Point) -> Option<Point> {
    let x = p.x.checked_add_signed(delta.x)?;
    let y = p.y.checked_add_signed(delta.y)?;
    Some(Point { x, y })
}

pub fn checked_add_assign(p: &mut Point, delta: &point_i8::Point) {
    try_checked_add_assign(p, delta).unwrap()
}

pub fn checked_add(p: &Point, delta: &point_i8::Point) -> Point {
    try_checked_add(p, delta).unwrap()
}

pub fn saturating_add_assign(p: &mut Point, delta: &point_i8::Point) {
    p.x = p.x.saturating_add_signed(delta.x);
    p.y = p.y.saturating_add_signed(delta.y);
}

pub fn saturating_add(p: &Point, delta: &point_i8::Point) -> Point {
    let x = p.x.saturating_add_signed(delta.x);
    let y = p.y.saturating_add_signed(delta.y);
    Point { x, y }
}

pub fn wrapping_add_assign(p: &mut Point, delta: &point_i8::Point) {
    p.x = p.x.wrapping_add_signed(delta.x);
    p.y = p.y.wrapping_add_signed(delta.y);
}

pub fn wrapping_add(p: &Point, delta: &point_i8::Point) -> Point {
    let x = p.x.wrapping_add_signed(delta.x);
    let y = p.y.wrapping_add_signed(delta.y);
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

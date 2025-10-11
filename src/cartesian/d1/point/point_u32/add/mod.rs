use super::Point;
use crate::cartesian::d1::point::point_i32;

pub fn try_checked_add_assign(p: &mut Point, delta: &point_i32::Point) -> Option<()> {
    let x = p.x.checked_add_signed(delta.x)?;
    p.x = x;
    Some(())
}

pub fn try_checked_add(p: &Point, delta: &point_i32::Point) -> Option<Point> {
    let x = p.x.checked_add_signed(delta.x)?;
    Some(Point { x })
}

pub fn checked_add_assign(p: &mut Point, delta: &point_i32::Point) {
    try_checked_add_assign(p, delta).unwrap()
}

pub fn checked_add(p: &Point, delta: &point_i32::Point) -> Point {
    try_checked_add(p, delta).unwrap()
}

pub fn saturating_add_assign(p: &mut Point, delta: &point_i32::Point) {
    p.x = p.x.saturating_add_signed(delta.x);
}

pub fn saturating_add(p: &Point, delta: &point_i32::Point) -> Point {
    let x = p.x.saturating_add_signed(delta.x);
    Point { x }
}

pub fn wrapping_add_assign(p: &mut Point, delta: &point_i32::Point) {
    p.x = p.x.wrapping_add_signed(delta.x);
}

pub fn wrapping_add(p: &Point, delta: &point_i32::Point) -> Point {
    let x = p.x.wrapping_add_signed(delta.x);
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

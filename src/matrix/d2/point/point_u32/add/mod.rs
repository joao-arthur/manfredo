use crate::matrix::point::{point_i32, point_u32::Point};

pub fn try_checked_add_assign(p: &mut Point, delta: &point_i32::Point) -> Option<()> {
    let row = p.row.checked_add_signed(delta.row)?;
    let col = p.col.checked_add_signed(delta.col)?;
    p.row = row;
    p.col = col;
    Some(())
}

pub fn try_checked_add(p: &Point, delta: &point_i32::Point) -> Option<Point> {
    let row = p.row.checked_add_signed(delta.row)?;
    let col = p.col.checked_add_signed(delta.col)?;
    Some(Point { row, col })
}

pub fn checked_add_assign(p: &mut Point, delta: &point_i32::Point) {
    try_checked_add_assign(p, delta).unwrap()
}

pub fn checked_add(p: &Point, delta: &point_i32::Point) -> Point {
    try_checked_add(p, delta).unwrap()
}

pub fn saturating_add_assign(p: &mut Point, delta: &point_i32::Point) {
    p.row = p.row.saturating_add_signed(delta.row);
    p.col = p.col.saturating_add_signed(delta.col);
}

pub fn saturating_add(p: &Point, delta: &point_i32::Point) -> Point {
    let row = p.row.saturating_add_signed(delta.row);
    let col = p.col.saturating_add_signed(delta.col);
    Point { row, col }
}

pub fn wrapping_add_assign(p: &mut Point, delta: &point_i32::Point) {
    p.row = p.row.wrapping_add_signed(delta.row);
    p.col = p.col.wrapping_add_signed(delta.col);
}

pub fn wrapping_add(p: &Point, delta: &point_i32::Point) -> Point {
    let row = p.row.wrapping_add_signed(delta.row);
    let col = p.col.wrapping_add_signed(delta.col);
    Point { row, col }
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

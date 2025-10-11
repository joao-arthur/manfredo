use super::Point;
use crate::cartesian::d2::point::point_u32;

pub fn delta_x(p1: &Point, p2: &Point) -> u32 {
    (i64::from(p2.x) - i64::from(p1.x)).unsigned_abs() as u32
}

pub fn delta_y(p1: &Point, p2: &Point) -> u32 {
    (i64::from(p2.y) - i64::from(p1.y)).unsigned_abs() as u32
}

pub fn delta_min(p1: &Point, p2: &Point) -> u32 {
    std::cmp::min(delta_x(p1, p2), delta_y(p1, p2))
}

pub fn delta_max(p1: &Point, p2: &Point) -> u32 {
    std::cmp::max(delta_x(p1, p2), delta_y(p1, p2))
}

pub fn delta(p1: &Point, p2: &Point) -> point_u32::Point {
    point_u32::Point { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

#[cfg(test)]
mod test_delta_max;

#[cfg(test)]
mod test_delta_min;

#[cfg(test)]
mod test_delta_x;

#[cfg(test)]
mod test_delta_y;

#[cfg(test)]
mod test_delta;

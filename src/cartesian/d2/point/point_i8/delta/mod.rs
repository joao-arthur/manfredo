use crate::cartesian::d2::point::{point_i8::Point, point_u8};

pub fn delta_x(p1: &Point, p2: &Point) -> u8 {
    (i16::from(p2.x) - i16::from(p1.x)).unsigned_abs() as u8
}

pub fn delta_y(p1: &Point, p2: &Point) -> u8 {
    (i16::from(p2.y) - i16::from(p1.y)).unsigned_abs() as u8
}

pub fn delta_min(p1: &Point, p2: &Point) -> u8 {
    std::cmp::min(delta_x(p1, p2), delta_y(p1, p2))
}

pub fn delta_max(p1: &Point, p2: &Point) -> u8 {
    std::cmp::max(delta_x(p1, p2), delta_y(p1, p2))
}

pub fn delta(p1: &Point, p2: &Point) -> point_u8::Point {
    point_u8::Point { x: delta_x(p1, p2), y: delta_y(p1, p2) }
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

use crate::matrix::d2::point::{point_i32::Point, point_u32};

pub fn delta_row(p1: &Point, p2: &Point) -> u32 {
    (i64::from(p2.row) - i64::from(p1.row)).unsigned_abs() as u32
}

pub fn delta_col(p1: &Point, p2: &Point) -> u32 {
    (i64::from(p2.col) - i64::from(p1.col)).unsigned_abs() as u32
}

pub fn delta_min(p1: &Point, p2: &Point) -> u32 {
    std::cmp::min(delta_row(p1, p2), delta_col(p1, p2))
}

pub fn delta_max(p1: &Point, p2: &Point) -> u32 {
    std::cmp::max(delta_row(p1, p2), delta_col(p1, p2))
}

pub fn delta(p1: &Point, p2: &Point) -> point_u32::Point {
    point_u32::Point { row: delta_row(p1, p2), col: delta_col(p1, p2) }
}

#[cfg(test)]
mod test_delta_max;

#[cfg(test)]
mod test_delta_min;

#[cfg(test)]
mod test_delta_row;

#[cfg(test)]
mod test_delta_col;

#[cfg(test)]
mod test_delta;

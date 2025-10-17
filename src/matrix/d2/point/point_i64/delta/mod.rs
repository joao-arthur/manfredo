use super::Point;
use crate::matrix::d2::point::point_u64;

pub fn delta_row(p1: &Point, p2: &Point) -> u64 {
    (i128::from(p2.row) - i128::from(p1.row)).unsigned_abs() as u64
}

pub fn delta_col(p1: &Point, p2: &Point) -> u64 {
    (i128::from(p2.col) - i128::from(p1.col)).unsigned_abs() as u64
}

pub fn delta_min(p1: &Point, p2: &Point) -> u64 {
    delta_row(p1, p2).min(delta_col(p1, p2))
}

pub fn delta_max(p1: &Point, p2: &Point) -> u64 {
    delta_row(p1, p2).max(delta_col(p1, p2))
}

pub fn delta(p1: &Point, p2: &Point) -> point_u64::Point {
    point_u64::Point { row: delta_row(p1, p2), col: delta_col(p1, p2) }
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

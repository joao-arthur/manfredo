use super::Point;

pub fn delta_row(p1: &Point, p2: &Point) -> u64 {
    p2.row - p1.row
}

pub fn delta_col(p1: &Point, p2: &Point) -> u64 {
    p2.col - p1.col
}

pub fn delta_min(p1: &Point, p2: &Point) -> u64 {
    std::cmp::min(delta_row(p1, p2), delta_col(p1, p2))
}

pub fn delta_max(p1: &Point, p2: &Point) -> u64 {
    std::cmp::max(delta_row(p1, p2), delta_col(p1, p2))
}

pub fn delta(p1: &Point, p2: &Point) -> Point {
    Point { row: delta_row(p1, p2), col: delta_col(p1, p2) }
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

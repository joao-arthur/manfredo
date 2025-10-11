use super::Point;

pub fn delta_x(p1: &Point, p2: &Point) -> u8 {
    p2.x - p1.x
}

pub fn delta_y(p1: &Point, p2: &Point) -> u8 {
    p2.y - p1.y
}

pub fn delta_min(p1: &Point, p2: &Point) -> u8 {
    std::cmp::min(delta_x(p1, p2), delta_y(p1, p2))
}

pub fn delta_max(p1: &Point, p2: &Point) -> u8 {
    std::cmp::max(delta_x(p1, p2), delta_y(p1, p2))
}

pub fn delta(p1: &Point, p2: &Point) -> Point {
    Point { x: delta_x(p1, p2), y: delta_y(p1, p2) }
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

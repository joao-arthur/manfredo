use super::Point;

pub fn delta_x(p1: &Point, p2: &Point) -> f64 {
    p2.x - p1.x
}

pub fn delta_y(p1: &Point, p2: &Point) -> f64 {
    p2.y - p1.y
}

pub fn delta_min(p1: &Point, p2: &Point) -> f64 {
    delta_x(p1, p2).min(delta_y(p1, p2))
}

pub fn delta_max(p1: &Point, p2: &Point) -> f64 {
    delta_x(p1, p2).max(delta_y(p1, p2))
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

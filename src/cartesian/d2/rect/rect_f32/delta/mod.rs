use crate::cartesian::d2::{point::point_f32, rect::rect_f32::Rect};

pub fn delta_x(r: &Rect) -> f32 {
    point_f32::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &Rect) -> f32 {
    point_f32::delta_y(&r.min, &r.max)
}

pub fn delta_min(r: &Rect) -> f32 {
    delta_x(r).min(delta_y(r))
}

pub fn delta_max(r: &Rect) -> f32 {
    delta_x(r).max(delta_y(r))
}

#[cfg(test)]
mod test_delta_max;

#[cfg(test)]
mod test_delta_min;

#[cfg(test)]
mod test_delta_x;

#[cfg(test)]
mod test_delta_y;

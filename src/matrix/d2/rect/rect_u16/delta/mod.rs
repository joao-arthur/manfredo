use crate::matrix::d2::{point::point_u16, rect::rect_u16::Rect};

pub fn delta_row(r: &Rect) -> u16 {
    point_u16::delta_row(&r.min, &r.max)
}

pub fn delta_col(r: &Rect) -> u16 {
    point_u16::delta_col(&r.min, &r.max)
}

pub fn delta_min(r: &Rect) -> u16 {
    std::cmp::min(delta_row(r), delta_col(r))
}

pub fn delta_max(r: &Rect) -> u16 {
    std::cmp::max(delta_row(r), delta_col(r))
}

#[cfg(test)]
mod test_delta_max;

#[cfg(test)]
mod test_delta_min;

#[cfg(test)]
mod test_delta_row;

#[cfg(test)]
mod test_delta_col;

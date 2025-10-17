use super::Rect;
use crate::matrix::d2::point::point_i16;

pub fn delta_row(r: &Rect) -> u16 {
    point_i16::delta_row(&r.min, &r.max)
}

pub fn delta_col(r: &Rect) -> u16 {
    point_i16::delta_col(&r.min, &r.max)
}

pub fn delta_min(r: &Rect) -> u16 {
    delta_row(r).min(delta_col(r))
}

pub fn delta_max(r: &Rect) -> u16 {
    delta_row(r).max(delta_col(r))
}

#[cfg(test)]
mod test_delta_max;

#[cfg(test)]
mod test_delta_min;

#[cfg(test)]
mod test_delta_row;

#[cfg(test)]
mod test_delta_col;

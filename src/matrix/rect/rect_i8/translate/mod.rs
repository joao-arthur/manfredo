use crate::matrix::{
    point::point_i8::PointI8,
    rect::rect_i8::{RectI8, delta_col, delta_row},
};

pub fn try_checked_translate_assign(r: &mut RectI8, delta: &PointI8) -> Option<()> {
    let min_row = r.min.row.checked_add(delta.row)?;
    let min_col = r.min.col.checked_add(delta.col)?;
    let max_row = r.max.row.checked_add(delta.row)?;
    let max_col = r.max.col.checked_add(delta.col)?;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
    Some(())
}

pub fn try_checked_translate(r: &RectI8, delta: &PointI8) -> Option<RectI8> {
    let min_row = r.min.row.checked_add(delta.row)?;
    let min_col = r.min.col.checked_add(delta.col)?;
    let max_row = r.max.row.checked_add(delta.row)?;
    let max_col = r.max.col.checked_add(delta.col)?;
    Some(RectI8 { min: PointI8 { row: min_row, col: min_col }, max: PointI8 { row: max_row, col: max_col } })
}

pub fn checked_translate_assign(r: &mut RectI8, delta: &PointI8) {
    try_checked_translate_assign(r, delta).unwrap()
}

pub fn checked_translate(r: &RectI8, delta: &PointI8) -> RectI8 {
    try_checked_translate(r, delta).unwrap()
}

pub fn saturating_translate_assign(r: &mut RectI8, delta: &PointI8) {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let temp_min_row = i16::from(r.min.row) + i16::from(delta.row);
    let temp_min_col = i16::from(r.min.col) + i16::from(delta.col);
    let min_row = temp_min_row.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(d_row));
    let min_col = temp_min_col.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(d_col));
    r.min.row = min_row as i8;
    r.min.col = min_col as i8;
    r.max.row = (min_row + i16::from(d_row)) as i8;
    r.max.col = (min_col + i16::from(d_col)) as i8;
}

pub fn saturating_translate(r: &RectI8, delta: &PointI8) -> RectI8 {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let temp_min_row = i16::from(r.min.row) + i16::from(delta.row);
    let temp_min_col = i16::from(r.min.col) + i16::from(delta.col);
    let min_row = temp_min_row.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(d_row));
    let min_col = temp_min_col.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(d_col));
    let max_row = min_row + i16::from(d_row);
    let max_col = min_col + i16::from(d_col);
    RectI8 { min: PointI8 { row: min_row as i8, col: min_col as i8 }, max: PointI8 { row: max_row as i8, col: max_col as i8 } }
}

pub fn wrapping_translate_assign(r: &mut RectI8, delta: &PointI8) {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let min_row = r.min.row.wrapping_add(delta.row);
    let min_col = r.min.col.wrapping_add(delta.col);
    let max_row = min_row.wrapping_add_unsigned(d_row);
    let max_col = min_col.wrapping_add_unsigned(d_col);
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
}

pub fn wrapping_translate(r: &RectI8, delta: &PointI8) -> RectI8 {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let min_row = r.min.row.wrapping_add(delta.row);
    let min_col = r.min.col.wrapping_add(delta.col);
    let max_row = min_row.wrapping_add_unsigned(d_row);
    let max_col = min_col.wrapping_add_unsigned(d_col);
    RectI8 { min: PointI8 { row: min_row, col: min_col }, max: PointI8 { row: max_row, col: max_col } }
}

#[cfg(test)]
mod test_try_checked_translate_assign;

#[cfg(test)]
mod test_try_checked_translate;

#[cfg(test)]
mod test_checked_translate_assign;

#[cfg(test)]
mod test_checked_translate;

#[cfg(test)]
mod test_saturating_translate_assign;

#[cfg(test)]
mod test_saturating_translate;

#[cfg(test)]
mod test_wrapping_translate_assign;

#[cfg(test)]
mod test_wrapping_translate;

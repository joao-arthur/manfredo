use crate::matrix::{
    point::point_i16::Point,
    rect::rect_i16::{Rect, delta_col, delta_row},
};

pub fn try_checked_translate_assign(r: &mut Rect, delta: &Point) -> Option<()> {
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

pub fn try_checked_translate(r: &Rect, delta: &Point) -> Option<Rect> {
    let min_row = r.min.row.checked_add(delta.row)?;
    let min_col = r.min.col.checked_add(delta.col)?;
    let max_row = r.max.row.checked_add(delta.row)?;
    let max_col = r.max.col.checked_add(delta.col)?;
    Some(Rect { min: Point { row: min_row, col: min_col }, max: Point { row: max_row, col: max_col } })
}

pub fn checked_translate_assign(r: &mut Rect, delta: &Point) {
    try_checked_translate_assign(r, delta).unwrap()
}

pub fn checked_translate(r: &Rect, delta: &Point) -> Rect {
    try_checked_translate(r, delta).unwrap()
}

pub fn saturating_translate_assign(r: &mut Rect, delta: &Point) {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let temp_min_row = i32::from(r.min.row) + i32::from(delta.row);
    let temp_min_col = i32::from(r.min.col) + i32::from(delta.col);
    let min_row = temp_min_row.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(d_row));
    let min_col = temp_min_col.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(d_col));
    r.min.row = min_row as i16;
    r.min.col = min_col as i16;
    r.max.row = (min_row + i32::from(d_row)) as i16;
    r.max.col = (min_col + i32::from(d_col)) as i16;
}

pub fn saturating_translate(r: &Rect, delta: &Point) -> Rect {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let temp_min_row = i32::from(r.min.row) + i32::from(delta.row);
    let temp_min_col = i32::from(r.min.col) + i32::from(delta.col);
    let min_row = temp_min_row.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(d_row));
    let min_col = temp_min_col.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(d_col));
    let max_row = min_row + i32::from(d_row);
    let max_col = min_col + i32::from(d_col);
    Rect { min: Point { row: min_row as i16, col: min_col as i16 }, max: Point { row: max_row as i16, col: max_col as i16 } }
}

pub fn wrapping_translate_assign(r: &mut Rect, delta: &Point) {
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

pub fn wrapping_translate(r: &Rect, delta: &Point) -> Rect {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let min_row = r.min.row.wrapping_add(delta.row);
    let min_col = r.min.col.wrapping_add(delta.col);
    let max_row = min_row.wrapping_add_unsigned(d_row);
    let max_col = min_col.wrapping_add_unsigned(d_col);
    Rect { min: Point { row: min_row, col: min_col }, max: Point { row: max_row, col: max_col } }
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

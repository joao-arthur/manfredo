use super::Rect;

pub fn try_checked_inflate_assign(r: &mut Rect) -> Option<()> {
    let min_row = r.min.row.checked_sub(1)?;
    let min_col = r.min.col.checked_sub(1)?;
    let max_row = r.max.row.checked_add(1)?;
    let max_col = r.max.col.checked_add(1)?;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
    Some(())
}

pub fn try_checked_inflate(r: &Rect) -> Option<Rect> {
    let min_row = r.min.row.checked_sub(1)?;
    let min_col = r.min.col.checked_sub(1)?;
    let max_row = r.max.row.checked_add(1)?;
    let max_col = r.max.col.checked_add(1)?;
    Some(Rect::new((min_row, min_col), (max_row, max_col)))
}

pub fn checked_inflate_assign(r: &mut Rect) {
    try_checked_inflate_assign(r).unwrap()
}

pub fn checked_inflate(r: &Rect) -> Rect {
    try_checked_inflate(r).unwrap()
}

pub fn try_saturating_inflate_assign(r: &mut Rect) -> Option<()> {
    let is_min_row = r.min.row == 0;
    let is_min_col = r.min.col == 0;
    let is_max_row = r.max.row == u64::MAX;
    let is_max_col = r.max.col == u64::MAX;
    if (is_min_row && is_max_row) || (is_min_col && is_max_col) {
        return None;
    }
    let min_row_modifier = 1 - u64::from(is_min_row) + u64::from(is_max_row);
    let min_col_modifier = 1 - u64::from(is_min_col) + u64::from(is_max_col);
    let max_row_modifier = 1 + u64::from(is_min_row) - u64::from(is_max_row);
    let max_col_modifier = 1 + u64::from(is_min_col) - u64::from(is_max_col);
    r.min.row = r.min.row.saturating_sub(min_row_modifier);
    r.min.col = r.min.col.saturating_sub(min_col_modifier);
    r.max.row = r.max.row.saturating_add(max_row_modifier);
    r.max.col = r.max.col.saturating_add(max_col_modifier);
    Some(())
}

pub fn try_saturating_inflate(r: &Rect) -> Option<Rect> {
    let is_min_row = r.min.row == 0;
    let is_min_col = r.min.col == 0;
    let is_max_row = r.max.row == u64::MAX;
    let is_max_col = r.max.col == u64::MAX;
    if (is_min_row && is_max_row) || (is_min_col && is_max_col) {
        return None;
    }
    let min_row_modifier = 1 - u64::from(is_min_row) + u64::from(is_max_row);
    let min_col_modifier = 1 - u64::from(is_min_col) + u64::from(is_max_col);
    let max_row_modifier = 1 + u64::from(is_min_row) - u64::from(is_max_row);
    let max_col_modifier = 1 + u64::from(is_min_col) - u64::from(is_max_col);
    let min_row = r.min.row.saturating_sub(min_row_modifier);
    let min_col = r.min.col.saturating_sub(min_col_modifier);
    let max_row = r.max.row.saturating_add(max_row_modifier);
    let max_col = r.max.col.saturating_add(max_col_modifier);
    Some(Rect::new((min_row, min_col), (max_row, max_col)))
}

pub fn saturating_inflate_assign(r: &mut Rect) {
    try_saturating_inflate_assign(r).unwrap()
}

pub fn saturating_inflate(r: &Rect) -> Rect {
    try_saturating_inflate(r).unwrap()
}

pub fn wrapping_inflate_assign(r: &mut Rect) {
    let min_row = r.min.row.wrapping_sub(1);
    let min_col = r.min.col.wrapping_sub(1);
    let max_row = r.max.row.wrapping_add(1);
    let max_col = r.max.col.wrapping_add(1);
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
}

pub fn wrapping_inflate(r: &Rect) -> Rect {
    let min_row = r.min.row.wrapping_sub(1);
    let min_col = r.min.col.wrapping_sub(1);
    let max_row = r.max.row.wrapping_add(1);
    let max_col = r.max.col.wrapping_add(1);
    Rect::new((min_row, min_col), (max_row, max_col))
}

#[cfg(test)]
mod test_try_checked_inflate_assign;

#[cfg(test)]
mod test_try_checked_inflate;

#[cfg(test)]
mod test_checked_inflate_assign;

#[cfg(test)]
mod test_checked_inflate;

#[cfg(test)]
mod test_try_saturating_inflate_assign;

#[cfg(test)]
mod test_try_saturating_inflate;

#[cfg(test)]
mod test_saturating_inflate_assign;

#[cfg(test)]
mod test_saturating_inflate;

#[cfg(test)]
mod test_wrapping_inflate_assign;

#[cfg(test)]
mod test_wrapping_inflate;

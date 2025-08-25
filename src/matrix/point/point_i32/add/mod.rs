use crate::matrix::point::point_i32::PointI32;

pub fn try_checked_add_assign(p: &mut PointI32, delta: &PointI32) -> Option<()> {
    let row = p.row.checked_add(delta.row)?;
    let col = p.col.checked_add(delta.col)?;
    p.row = row;
    p.col = col;
    Some(())
}

pub fn try_checked_add(p: &PointI32, delta: &PointI32) -> Option<PointI32> {
    let row = p.row.checked_add(delta.row)?;
    let col = p.col.checked_add(delta.col)?;
    Some(PointI32 { row, col })
}

pub fn checked_add_assign(p: &mut PointI32, delta: &PointI32) {
    try_checked_add_assign(p, delta).unwrap()
}

pub fn checked_add(p: &PointI32, delta: &PointI32) -> PointI32 {
    try_checked_add(p, delta).unwrap()
}

pub fn saturating_add_assign(p: &mut PointI32, delta: &PointI32) {
    p.row = p.row.saturating_add(delta.row);
    p.col = p.col.saturating_add(delta.col);
}

pub fn saturating_add(p: &PointI32, delta: &PointI32) -> PointI32 {
    let row = p.row.saturating_add(delta.row);
    let col = p.col.saturating_add(delta.col);
    PointI32 { row, col }
}

pub fn wrapping_add_assign(p: &mut PointI32, delta: &PointI32) {
    p.row = p.row.wrapping_add(delta.row);
    p.col = p.col.wrapping_add(delta.col);
}

pub fn wrapping_add(p: &PointI32, delta: &PointI32) -> PointI32 {
    let row = p.row.wrapping_add(delta.row);
    let col = p.col.wrapping_add(delta.col);
    PointI32 { row, col }
}

pub mod checked;
pub mod saturated;
pub mod wrapped;

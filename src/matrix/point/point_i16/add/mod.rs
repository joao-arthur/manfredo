use crate::matrix::point::point_i16::PointI16;

pub fn try_checked_add_assign(p: &mut PointI16, delta: &PointI16) -> Option<()> {
    let row = p.row.checked_add(delta.row)?;
    let col = p.col.checked_add(delta.col)?;
    p.row = row;
    p.col = col;
    Some(())
}

pub fn try_checked_add(p: &PointI16, delta: &PointI16) -> Option<PointI16> {
    let row = p.row.checked_add(delta.row)?;
    let col = p.col.checked_add(delta.col)?;
    Some(PointI16 { row, col })
}

pub fn checked_add_assign(p: &mut PointI16, delta: &PointI16) {
    try_checked_add_assign(p, delta).unwrap()
}

pub fn checked_add(p: &PointI16, delta: &PointI16) -> PointI16 {
    try_checked_add(p, delta).unwrap()
}

pub fn saturating_add_assign(p: &mut PointI16, delta: &PointI16) {
    p.row = p.row.saturating_add(delta.row);
    p.col = p.col.saturating_add(delta.col);
}

pub fn saturating_add(p: &PointI16, delta: &PointI16) -> PointI16 {
    let row = p.row.saturating_add(delta.row);
    let col = p.col.saturating_add(delta.col);
    PointI16 { row, col }
}

pub fn wrapping_add_assign(p: &mut PointI16, delta: &PointI16) {
    p.row = p.row.wrapping_add(delta.row);
    p.col = p.col.wrapping_add(delta.col);
}

pub fn wrapping_add(p: &PointI16, delta: &PointI16) -> PointI16 {
    let row = p.row.wrapping_add(delta.row);
    let col = p.col.wrapping_add(delta.col);
    PointI16 { row, col }
}

pub mod checked;
pub mod saturated;
pub mod wrapped;

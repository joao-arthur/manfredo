use crate::matrix::point::{point_i32::PointI32, point_u32::PointU32};

pub fn try_checked_add_assign(p: &mut PointU32, delta: &PointI32) -> Option<()> {
    let row = p.row.checked_add_signed(delta.row)?;
    let col = p.col.checked_add_signed(delta.col)?;
    p.row = row;
    p.col = col;
    Some(())
}

pub fn try_checked_add(p: &PointU32, delta: &PointI32) -> Option<PointU32> {
    let row = p.row.checked_add_signed(delta.row)?;
    let col = p.col.checked_add_signed(delta.col)?;
    Some(PointU32 { row, col })
}

pub fn checked_add_assign(p: &mut PointU32, delta: &PointI32) {
    try_checked_add_assign(p, delta).unwrap()
}

pub fn checked_add(p: &PointU32, delta: &PointI32) -> PointU32 {
    try_checked_add(p, delta).unwrap()
}

pub fn saturating_add_assign(p: &mut PointU32, delta: &PointI32) {
    p.row = p.row.saturating_add_signed(delta.row);
    p.col = p.col.saturating_add_signed(delta.col);
}

pub fn saturating_add(p: &PointU32, delta: &PointI32) -> PointU32 {
    let row = p.row.saturating_add_signed(delta.row);
    let col = p.col.saturating_add_signed(delta.col);
    PointU32 { row, col }
}

pub fn wrapping_add_assign(p: &mut PointU32, delta: &PointI32) {
    p.row = p.row.wrapping_add_signed(delta.row);
    p.col = p.col.wrapping_add_signed(delta.col);
}

pub fn wrapping_add(p: &PointU32, delta: &PointI32) -> PointU32 {
    let row = p.row.wrapping_add_signed(delta.row);
    let col = p.col.wrapping_add_signed(delta.col);
    PointU32 { row, col }
}

pub mod checked;
pub mod saturated;
pub mod wrapped;

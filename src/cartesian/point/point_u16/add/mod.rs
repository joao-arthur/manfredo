use crate::cartesian::point::{point_i16::PointI16, point_u16::PointU16};

pub fn try_checked_add_assign(p: &mut PointU16, delta: &PointI16) -> Option<()> {
    let x = p.x.checked_add_signed(delta.x)?;
    let y = p.y.checked_add_signed(delta.y)?;
    p.x = x;
    p.y = y;
    Some(())
}

pub fn try_checked_add(p: &PointU16, delta: &PointI16) -> Option<PointU16> {
    let x = p.x.checked_add_signed(delta.x)?;
    let y = p.y.checked_add_signed(delta.y)?;
    Some(PointU16 { x, y })
}

pub fn checked_add_assign(p: &mut PointU16, delta: &PointI16) {
    try_checked_add_assign(p, delta).unwrap()
}

pub fn checked_add(p: &PointU16, delta: &PointI16) -> PointU16 {
    try_checked_add(p, delta).unwrap()
}

pub fn saturating_add_assign(p: &mut PointU16, delta: &PointI16) {
    p.x = p.x.saturating_add_signed(delta.x);
    p.y = p.y.saturating_add_signed(delta.y);
}

pub fn saturating_add(p: &PointU16, delta: &PointI16) -> PointU16 {
    let x = p.x.saturating_add_signed(delta.x);
    let y = p.y.saturating_add_signed(delta.y);
    PointU16 { x, y }
}

pub fn wrapping_add_assign(p: &mut PointU16, delta: &PointI16) {
    p.x = p.x.wrapping_add_signed(delta.x);
    p.y = p.y.wrapping_add_signed(delta.y);
}

pub fn wrapping_add(p: &PointU16, delta: &PointI16) -> PointU16 {
    let x = p.x.wrapping_add_signed(delta.x);
    let y = p.y.wrapping_add_signed(delta.y);
    PointU16 { x, y }
}

pub mod checked;
pub mod saturated;
pub mod wrapped;

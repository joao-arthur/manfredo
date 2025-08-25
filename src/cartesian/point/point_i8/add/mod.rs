use crate::cartesian::point::point_i8::PointI8;

pub fn try_checked_add_assign(p: &mut PointI8, delta: &PointI8) -> Option<()> {
    let x = p.x.checked_add(delta.x)?;
    let y = p.y.checked_add(delta.y)?;
    p.x = x;
    p.y = y;
    Some(())
}

pub fn try_checked_add(p: &PointI8, delta: &PointI8) -> Option<PointI8> {
    let x = p.x.checked_add(delta.x)?;
    let y = p.y.checked_add(delta.y)?;
    Some(PointI8 { x, y })
}

pub fn checked_add_assign(p: &mut PointI8, delta: &PointI8) {
    try_checked_add_assign(p, delta).unwrap()
}

pub fn checked_add(p: &PointI8, delta: &PointI8) -> PointI8 {
    try_checked_add(p, delta).unwrap()
}

pub fn saturating_add_assign(p: &mut PointI8, delta: &PointI8) {
    p.x = p.x.saturating_add(delta.x);
    p.y = p.y.saturating_add(delta.y);
}

pub fn saturating_add(p: &PointI8, delta: &PointI8) -> PointI8 {
    let x = p.x.saturating_add(delta.x);
    let y = p.y.saturating_add(delta.y);
    PointI8 { x, y }
}

pub fn wrapping_add_assign(p: &mut PointI8, delta: &PointI8) {
    p.x = p.x.wrapping_add(delta.x);
    p.y = p.y.wrapping_add(delta.y);
}

pub fn wrapping_add(p: &PointI8, delta: &PointI8) -> PointI8 {
    let x = p.x.wrapping_add(delta.x);
    let y = p.y.wrapping_add(delta.y);
    PointI8 { x, y }
}

pub mod checked;
pub mod saturated;
pub mod wrapped;

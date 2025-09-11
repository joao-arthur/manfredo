use crate::cartesian::point::{point_i32::PointI32 as PointI, point_u32::PointU32};

pub fn try_checked_add_assign(p: &mut PointU32, delta: &PointI) -> Option<()> {
    let x = p.x.checked_add_signed(delta.x)?;
    let y = p.y.checked_add_signed(delta.y)?;
    p.x = x;
    p.y = y;
    Some(())
}

pub fn try_checked_add(p: &PointU32, delta: &PointI) -> Option<PointU32> {
    let x = p.x.checked_add_signed(delta.x)?;
    let y = p.y.checked_add_signed(delta.y)?;
    Some(PointU32 { x, y })
}

pub fn checked_add_assign(p: &mut PointU32, delta: &PointI) {
    try_checked_add_assign(p, delta).unwrap()
}

pub fn checked_add(p: &PointU32, delta: &PointI) -> PointU32 {
    try_checked_add(p, delta).unwrap()
}

pub fn saturating_add_assign(p: &mut PointU32, delta: &PointI) {
    p.x = p.x.saturating_add_signed(delta.x);
    p.y = p.y.saturating_add_signed(delta.y);
}

pub fn saturating_add(p: &PointU32, delta: &PointI) -> PointU32 {
    let x = p.x.saturating_add_signed(delta.x);
    let y = p.y.saturating_add_signed(delta.y);
    PointU32 { x, y }
}

pub fn wrapping_add_assign(p: &mut PointU32, delta: &PointI) {
    p.x = p.x.wrapping_add_signed(delta.x);
    p.y = p.y.wrapping_add_signed(delta.y);
}

pub fn wrapping_add(p: &PointU32, delta: &PointI) -> PointU32 {
    let x = p.x.wrapping_add_signed(delta.x);
    let y = p.y.wrapping_add_signed(delta.y);
    PointU32 { x, y }
}

#[cfg(test)]
mod test_checked_add_assign;

#[cfg(test)]
mod test_checked_add;

#[cfg(test)]
mod test_try_checked_add_assign;

#[cfg(test)]
mod test_try_checked_add;

#[cfg(test)]
mod test_saturating_add_assign;

#[cfg(test)]
mod test_saturating_add;

#[cfg(test)]
mod test_wrapping_add_assign;

#[cfg(test)]
mod test_wrapping_add;

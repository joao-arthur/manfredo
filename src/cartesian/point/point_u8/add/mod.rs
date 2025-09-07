use crate::cartesian::point::{point_i8::PointI8, point_u8::PointU8};

pub fn try_checked_add_assign(p: &mut PointU8, delta: &PointI8) -> Option<()> {
    let x = p.x.checked_add_signed(delta.x)?;
    let y = p.y.checked_add_signed(delta.y)?;
    p.x = x;
    p.y = y;
    Some(())
}

pub fn try_checked_add(p: &PointU8, delta: &PointI8) -> Option<PointU8> {
    let x = p.x.checked_add_signed(delta.x)?;
    let y = p.y.checked_add_signed(delta.y)?;
    Some(PointU8 { x, y })
}

pub fn checked_add_assign(p: &mut PointU8, delta: &PointI8) {
    try_checked_add_assign(p, delta).unwrap()
}

pub fn checked_add(p: &PointU8, delta: &PointI8) -> PointU8 {
    try_checked_add(p, delta).unwrap()
}

pub fn saturating_add_assign(p: &mut PointU8, delta: &PointI8) {
    p.x = p.x.saturating_add_signed(delta.x);
    p.y = p.y.saturating_add_signed(delta.y);
}

pub fn saturating_add(p: &PointU8, delta: &PointI8) -> PointU8 {
    let x = p.x.saturating_add_signed(delta.x);
    let y = p.y.saturating_add_signed(delta.y);
    PointU8 { x, y }
}

pub fn wrapping_add_assign(p: &mut PointU8, delta: &PointI8) {
    p.x = p.x.wrapping_add_signed(delta.x);
    p.y = p.y.wrapping_add_signed(delta.y);
}

pub fn wrapping_add(p: &PointU8, delta: &PointI8) -> PointU8 {
    let x = p.x.wrapping_add_signed(delta.x);
    let y = p.y.wrapping_add_signed(delta.y);
    PointU8 { x, y }
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

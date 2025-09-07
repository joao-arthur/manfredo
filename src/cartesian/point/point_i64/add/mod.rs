use crate::cartesian::point::point_i64::PointI64;

pub fn try_checked_add_assign(p: &mut PointI64, delta: &PointI64) -> Option<()> {
    let x = p.x.checked_add(delta.x)?;
    let y = p.y.checked_add(delta.y)?;
    p.x = x;
    p.y = y;
    Some(())
}

pub fn try_checked_add(p: &PointI64, delta: &PointI64) -> Option<PointI64> {
    let x = p.x.checked_add(delta.x)?;
    let y = p.y.checked_add(delta.y)?;
    Some(PointI64 { x, y })
}

pub fn checked_add_assign(p: &mut PointI64, delta: &PointI64) {
    try_checked_add_assign(p, delta).unwrap()
}

pub fn checked_add(p: &PointI64, delta: &PointI64) -> PointI64 {
    try_checked_add(p, delta).unwrap()
}

pub fn saturating_add_assign(p: &mut PointI64, delta: &PointI64) {
    p.x = p.x.saturating_add(delta.x);
    p.y = p.y.saturating_add(delta.y);
}

pub fn saturating_add(p: &PointI64, delta: &PointI64) -> PointI64 {
    let x = p.x.saturating_add(delta.x);
    let y = p.y.saturating_add(delta.y);
    PointI64 { x, y }
}

pub fn wrapping_add_assign(p: &mut PointI64, delta: &PointI64) {
    p.x = p.x.wrapping_add(delta.x);
    p.y = p.y.wrapping_add(delta.y);
}

pub fn wrapping_add(p: &PointI64, delta: &PointI64) -> PointI64 {
    let x = p.x.wrapping_add(delta.x);
    let y = p.y.wrapping_add(delta.y);
    PointI64 { x, y }
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

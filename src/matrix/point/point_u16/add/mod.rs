use crate::matrix::point::{point_i16::PointI16, point_u16::PointU16};

pub fn try_checked_add_assign(p: &mut PointU16, delta: &PointI16) -> Option<()> {
    let row = p.row.checked_add_signed(delta.row)?;
    let col = p.col.checked_add_signed(delta.col)?;
    p.row = row;
    p.col = col;
    Some(())
}

pub fn try_checked_add(p: &PointU16, delta: &PointI16) -> Option<PointU16> {
    let row = p.row.checked_add_signed(delta.row)?;
    let col = p.col.checked_add_signed(delta.col)?;
    Some(PointU16 { row, col })
}

pub fn checked_add_assign(p: &mut PointU16, delta: &PointI16) {
    try_checked_add_assign(p, delta).unwrap()
}

pub fn checked_add(p: &PointU16, delta: &PointI16) -> PointU16 {
    try_checked_add(p, delta).unwrap()
}

pub fn saturating_add_assign(p: &mut PointU16, delta: &PointI16) {
    p.row = p.row.saturating_add_signed(delta.row);
    p.col = p.col.saturating_add_signed(delta.col);
}

pub fn saturating_add(p: &PointU16, delta: &PointI16) -> PointU16 {
    let row = p.row.saturating_add_signed(delta.row);
    let col = p.col.saturating_add_signed(delta.col);
    PointU16 { row, col }
}

pub fn wrapping_add_assign(p: &mut PointU16, delta: &PointI16) {
    p.row = p.row.wrapping_add_signed(delta.row);
    p.col = p.col.wrapping_add_signed(delta.col);
}

pub fn wrapping_add(p: &PointU16, delta: &PointI16) -> PointU16 {
    let row = p.row.wrapping_add_signed(delta.row);
    let col = p.col.wrapping_add_signed(delta.col);
    PointU16 { row, col }
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

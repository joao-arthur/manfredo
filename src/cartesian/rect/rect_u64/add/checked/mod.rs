use crate::cartesian::{
    point::point_u64::PointU64,
    rect::{rect_i64::RectI64, rect_u64::RectU64},
};

pub fn try_checked_add_assign(r: &mut RectU64, delta: &RectI64) -> Option<()> {
    let min_x = r.min.x.checked_add_signed(delta.min.x)?;
    let min_y = r.min.y.checked_add_signed(delta.min.y)?;
    let max_x = r.max.x.checked_add_signed(delta.max.x)?;
    let max_y = r.max.y.checked_add_signed(delta.max.y)?;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

pub fn try_checked_add(r: &RectU64, delta: &RectI64) -> Option<RectU64> {
    let min_x = r.min.x.checked_add_signed(delta.min.x)?;
    let min_y = r.min.y.checked_add_signed(delta.min.y)?;
    let max_x = r.max.x.checked_add_signed(delta.max.x)?;
    let max_y = r.max.y.checked_add_signed(delta.max.y)?;
    Some(RectU64 { min: PointU64 { x: min_x, y: min_y }, max: PointU64 { x: max_x, y: max_y } })
}

pub fn checked_add_assign(r: &mut RectU64, delta: &RectI64) {
    try_checked_add_assign(r, delta).unwrap()
}

pub fn checked_add(r: &RectU64, delta: &RectI64) -> RectU64 {
    try_checked_add(r, delta).unwrap()
}

#[cfg(test)]
mod test_checked_add_assign;

#[cfg(test)]
mod test_checked_add;

#[cfg(test)]
mod test_try_checked_add_assign;

#[cfg(test)]
mod test_try_checked_add;

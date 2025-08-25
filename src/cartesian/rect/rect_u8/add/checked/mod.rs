use crate::cartesian::{
    point::point_u8::PointU8,
    rect::{rect_i8::RectI8, rect_u8::RectU8},
};

pub fn try_checked_add_assign(r: &mut RectU8, delta: &RectI8) -> Option<()> {
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

pub fn try_checked_add(r: &RectU8, delta: &RectI8) -> Option<RectU8> {
    let min_x = r.min.x.checked_add_signed(delta.min.x)?;
    let min_y = r.min.y.checked_add_signed(delta.min.y)?;
    let max_x = r.max.x.checked_add_signed(delta.max.x)?;
    let max_y = r.max.y.checked_add_signed(delta.max.y)?;
    Some(RectU8 { min: PointU8 { x: min_x, y: min_y }, max: PointU8 { x: max_x, y: max_y } })
}

pub fn checked_add_assign(r: &mut RectU8, delta: &RectI8) {
    try_checked_add_assign(r, delta).unwrap()
}

pub fn checked_add(r: &RectU8, delta: &RectI8) -> RectU8 {
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

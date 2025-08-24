use super::try_assign_add::try_assign_add;
use crate::cartesian::rect::{rect_i32::RectI32, rect_u32::RectU32};

pub fn assign_add(r: &mut RectU32, delta: &RectI32) {
    try_assign_add(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::assign_add;
    use crate::cartesian::rect::{rect_i32::RectI32, rect_u32::RectU32};

    #[test]
    fn test_assign_add() {
        let mut r = RectU32::of(0, 0, 12, 15);
        assign_add(&mut r, &RectI32::of(5, 4, 3, 2));
        assert_eq!(r, RectU32::of(5, 4, 15, 17));
        assign_add(&mut r, &RectI32::of(-4, -3, -2, -1));
        assert_eq!(r, RectU32::of(1, 1, 13, 16));
    }
}

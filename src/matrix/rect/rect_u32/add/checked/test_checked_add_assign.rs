use super::try_assign_add::try_checked_add_assign;
use crate::matrix::rect::{rect_i32::RectI32, rect_u32::RectU32};

pub fn checked_add_assign(r: &mut RectU32, delta: &RectI32) {
    try_checked_add_assign(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::checked_add_assign;
    use crate::matrix::rect::{rect_i32::RectI32, rect_u32::RectU32};

    #[test]
    fn test_checked_add_assign() {
        let mut r = RectU32::of(0, 0, 12, 10);
        checked_add_assign(&mut r, &RectI32::of(5, 4, 3, 2));
        assert_eq!(r, RectU32::of(5, 4, 15, 12));
        checked_add_assign(&mut r, &RectI32::of(-4, -3, -2, -1));
        assert_eq!(r, RectU32::of(1, 1, 13, 11));
    }
}

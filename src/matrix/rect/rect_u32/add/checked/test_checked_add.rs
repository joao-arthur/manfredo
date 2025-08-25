use super::try_add::try_checked_add;
use crate::matrix::rect::{rect_i32::RectI32, rect_u32::RectU32};

pub fn checked_add(r: &RectU32, delta: &RectI32) -> RectU32 {
    try_checked_add(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::checked_add;
    use crate::matrix::rect::{rect_i32::RectI32, rect_u32::RectU32};

    #[test]
    fn test_checked_add() {
        assert_eq!(checked_add(&RectU32::of(0, 0, 12, 15), &RectI32::of(5, 4, 3, 2)), RectU32::of(5, 4, 15, 17));
        assert_eq!(checked_add(&RectU32::of(5, 4, 15, 20), &RectI32::of(-4, -3, -2, -1)), RectU32::of(1, 1, 13, 19));
    }
}

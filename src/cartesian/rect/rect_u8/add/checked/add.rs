use super::try_add::try_checked_add;
use crate::cartesian::rect::{rect_i8::RectI8, rect_u8::RectU8};

pub fn checked_add(r: &RectU8, delta: &RectI8) -> RectU8 {
    try_checked_add(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::checked_add;
    use crate::cartesian::rect::{rect_i8::RectI8, rect_u8::RectU8};

    #[test]
    fn test_checked_add() {
        assert_eq!(checked_add(&RectU8::of(0, 0, 12, 15), &RectI8::of(5, 4, 3, 2)), RectU8::of(5, 4, 15, 17));
        assert_eq!(checked_add(&RectU8::of(5, 4, 15, 20), &RectI8::of(-4, -3, -2, -1)), RectU8::of(1, 1, 13, 19));
    }
}

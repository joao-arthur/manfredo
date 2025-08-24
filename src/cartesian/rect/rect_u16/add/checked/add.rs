use super::try_add::try_checked_add;
use crate::cartesian::rect::{rect_i16::RectI16, rect_u16::RectU16};

pub fn checked_add(r: &RectU16, delta: &RectI16) -> RectU16 {
    try_checked_add(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::checked_add;
    use crate::cartesian::rect::{rect_i16::RectI16, rect_u16::RectU16};

    #[test]
    fn test_checked_add() {
        assert_eq!(checked_add(&RectU16::of(0, 0, 12, 15), &RectI16::of(5, 4, 3, 2)), RectU16::of(5, 4, 15, 17));
        assert_eq!(checked_add(&RectU16::of(5, 4, 15, 20), &RectI16::of(-4, -3, -2, -1)), RectU16::of(1, 1, 13, 19));
    }
}

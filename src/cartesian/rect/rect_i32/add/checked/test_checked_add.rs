use super::try_add::try_checked_add;
use crate::cartesian::rect::rect_i32::RectI32;

pub fn checked_add(r: &RectI32, delta: &RectI32) -> RectI32 {
    try_checked_add(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::checked_add;
    use crate::cartesian::rect::rect_i32::RectI32;

    #[test]
    fn test_checked_add() {
        assert_eq!(checked_add(&RectI32::of(-7, 9, -12, 15), &RectI32::of(5, 4, 3, 2)), RectI32::of(-2, 13, -9, 17));
        assert_eq!(checked_add(&RectI32::of(5, 4, 15, 17), &RectI32::of(-14, -13, -12, -11)), RectI32::of(-9, -9, 3, 6));
    }
}

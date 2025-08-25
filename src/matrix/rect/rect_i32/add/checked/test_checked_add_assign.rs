use super::try_assign_add::try_checked_add_assign;
use crate::matrix::rect::rect_i32::RectI32;

pub fn checked_add_assign(r: &mut RectI32, delta: &RectI32) {
    try_checked_add_assign(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::checked_add_assign;
    use crate::matrix::rect::rect_i32::RectI32;

    #[test]
    fn test_checked_add_assign() {
        let mut r = RectI32::of(0, 0, 12, 15);
        checked_add_assign(&mut r, &RectI32::of(5, 4, 3, 2));
        assert_eq!(r, RectI32::of(5, 4, 15, 17));
        checked_add_assign(&mut r, &RectI32::of(-14, -13, -12, -11));
        assert_eq!(r, RectI32::of(-9, -9, 3, 6));
    }
}

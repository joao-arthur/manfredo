use super::try_assign_add::try_checked_add_assign;
use crate::matrix::rect::rect_i16::RectI16;

pub fn checked_add_assign(r: &mut RectI16, delta: &RectI16) {
    try_checked_add_assign(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::checked_add_assign;
    use crate::matrix::rect::rect_i16::RectI16;

    #[test]
    fn test_checked_add_assign() {
        let mut r = RectI16::of(0, 0, 12, 15);
        checked_add_assign(&mut r, &RectI16::of(5, 4, 3, 2));
        assert_eq!(r, RectI16::of(5, 4, 15, 17));
        checked_add_assign(&mut r, &RectI16::of(-14, -13, -12, -11));
        assert_eq!(r, RectI16::of(-9, -9, 3, 6));
    }
}

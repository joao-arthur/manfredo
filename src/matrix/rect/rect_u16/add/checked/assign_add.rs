use super::try_assign_add::try_assign_add;
use crate::matrix::rect::{rect_i16::RectI16, rect_u16::RectU16};

pub fn assign_add(r: &mut RectU16, delta: &RectI16) {
    try_assign_add(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::assign_add;
    use crate::matrix::rect::{rect_i16::RectI16, rect_u16::RectU16};

    #[test]
    fn test_assign_add() {
        let mut r = RectU16::of(0, 0, 12, 10);
        assign_add(&mut r, &RectI16::of(5, 4, 3, 2));
        assert_eq!(r, RectU16::of(5, 4, 15, 12));
        assign_add(&mut r, &RectI16::of(-4, -3, -2, -1));
        assert_eq!(r, RectU16::of(1, 1, 13, 11));
    }
}

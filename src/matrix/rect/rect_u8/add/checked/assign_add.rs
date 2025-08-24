use super::try_assign_add::try_assign_add;
use crate::matrix::rect::{rect_i8::RectI8, rect_u8::RectU8};

pub fn assign_add(r: &mut RectU8, delta: &RectI8) {
    try_assign_add(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::assign_add;
    use crate::matrix::rect::{rect_i8::RectI8, rect_u8::RectU8};

    #[test]
    fn test_assign_add() {
        let mut r = RectU8::of(0, 0, 12, 15);
        assign_add(&mut r, &RectI8::of(5, 4, 3, 2));
        assert_eq!(r, RectU8::of(5, 4, 15, 17));
        assign_add(&mut r, &RectI8::of(-4, -3, -2, -1));
        assert_eq!(r, RectU8::of(1, 1, 13, 16));
    }
}

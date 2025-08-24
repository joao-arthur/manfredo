use super::try_assign_add::try_assign_add;
use crate::matrix::rect::rect_i8::RectI8;

pub fn assign_add(r: &mut RectI8, delta: &RectI8) {
    try_assign_add(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::assign_add;
    use crate::matrix::rect::rect_i8::RectI8;

    #[test]
    fn test_assign_add() {
        let mut r = RectI8::of(0, 0, 12, 15);
        assign_add(&mut r, &RectI8::of(5, 4, 3, 2));
        assert_eq!(r, RectI8::of(5, 4, 15, 17));
        assign_add(&mut r, &RectI8::of(-14, -13, -12, -11));
        assert_eq!(r, RectI8::of(-9, -9, 3, 6));
    }
}

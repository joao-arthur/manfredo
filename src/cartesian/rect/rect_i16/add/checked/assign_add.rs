use super::try_assign_add::try_assign_add;
use crate::cartesian::rect::rect_i16::RectI16;

pub fn assign_add(r: &mut RectI16, delta: &RectI16) {
    try_assign_add(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::assign_add;
    use crate::cartesian::rect::rect_i16::RectI16;

    #[test]
    fn test_assign_add() {
        let mut r = RectI16::of(0, 0, 12, 15);
        assign_add(&mut r, &RectI16::of(5, 4, 3, 2));
        assert_eq!(r, RectI16::of(5, 4, 15, 17));
        assign_add(&mut r, &RectI16::of(-14, -13, -12, -11));
        assert_eq!(r, RectI16::of(-9, -9, 3, 6));
    }
}

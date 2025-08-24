use super::try_assign_add::try_assign_add;
use crate::cartesian::rect::rect_f32::RectF32;

pub fn assign_add(r: &mut RectF32, delta: &RectF32) {
    try_assign_add(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::assign_add;
    use crate::cartesian::rect::rect_f32::RectF32;

    #[test]
    fn test_assign_add() {
        let mut r = RectF32::of(0.0, 0.0, 12.0, 15.0);
        assign_add(&mut r, &RectF32::of(5.0, 4.0, 3.0, 2.0));
        assert_eq!(r, RectF32::of(5.0, 4.0, 15.0, 17.0));
        assign_add(&mut r, &RectF32::of(-14.0, -13.0, -12.0, -11.0));
        assert_eq!(r, RectF32::of(-9.0, -9.0, 3.0, 6.0));
    }
}

use super::try_assign_add::try_assign_add;
use crate::cartesian::rect::{rect_i64::RectI64, rect_u64::RectU64};

pub fn assign_add(r: &mut RectU64, delta: &RectI64) {
    try_assign_add(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::assign_add;
    use crate::cartesian::rect::{rect_i64::RectI64, rect_u64::RectU64};

    #[test]
    fn test_assign_add() {
        let mut r = RectU64::of(0, 0, 12, 10);
        assign_add(&mut r, &RectI64::of(5, 4, 3, 2));
        assert_eq!(r, RectU64::of(5, 4, 15, 12));
        assign_add(&mut r, &RectI64::of(-4, -3, -2, -1));
        assert_eq!(r, RectU64::of(1, 1, 13, 11));
    }
}

use super::try_assign_add::try_checked_add_assign;
use crate::cartesian::rect::{rect_i64::RectI64, rect_u64::RectU64};

pub fn checked_add_assign(r: &mut RectU64, delta: &RectI64) {
    try_checked_add_assign(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::checked_add_assign;
    use crate::cartesian::rect::{rect_i64::RectI64, rect_u64::RectU64};

    #[test]
    fn test_checked_add_assign() {
        let mut r = RectU64::of(0, 0, 12, 10);
        checked_add_assign(&mut r, &RectI64::of(5, 4, 3, 2));
        assert_eq!(r, RectU64::of(5, 4, 15, 12));
        checked_add_assign(&mut r, &RectI64::of(-4, -3, -2, -1));
        assert_eq!(r, RectU64::of(1, 1, 13, 11));
    }
}

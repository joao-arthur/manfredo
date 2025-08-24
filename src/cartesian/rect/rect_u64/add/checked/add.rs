use super::try_add::try_add;
use crate::cartesian::rect::{rect_i64::RectI64, rect_u64::RectU64};

pub fn add(r: &RectU64, delta: &RectI64) -> RectU64 {
    try_add(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::add;
    use crate::cartesian::rect::{rect_i64::RectI64, rect_u64::RectU64};

    #[test]
    fn test_add() {
        assert_eq!(add(&RectU64::of(0, 0, 12, 15), &RectI64::of(5, 4, 3, 2)), RectU64::of(5, 4, 15, 17));
        assert_eq!(add(&RectU64::of(5, 4, 15, 20), &RectI64::of(-4, -3, -2, -1)), RectU64::of(1, 1, 13, 19));
    }
}

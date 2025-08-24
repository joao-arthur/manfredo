use super::try_add::try_add;
use crate::matrix::rect::rect_i64::RectI64;

pub fn add(r: &RectI64, delta: &RectI64) -> RectI64 {
    try_add(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::add;
    use crate::matrix::rect::rect_i64::RectI64;

    #[test]
    fn test_add() {
        assert_eq!(add(&RectI64::of(-7, 9, -12, 15), &RectI64::of(5, 4, 3, 2)), RectI64::of(-2, 13, -9, 17));
        assert_eq!(add(&RectI64::of(5, 4, 15, 17), &RectI64::of(-14, -13, -12, -11)), RectI64::of(-9, -9, 3, 6));
    }
}

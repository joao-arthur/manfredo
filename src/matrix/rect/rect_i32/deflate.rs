use crate::matrix::{
    point::point_i32::PointI32,
    rect::rect_i32::{RectI32, delta_row, delta_col},
};

pub fn try_assign_deflate(r: &mut RectI32) -> Option<()> {
    if delta_row(r) < 3 || delta_col(r) < 3 {
        return None;
    }
    r.min.row += 1;
    r.min.col += 1;
    r.max.row -= 1;
    r.max.col -= 1;
    Some(())
}

pub fn try_deflate(r: &RectI32) -> Option<RectI32> {
    if delta_row(r) < 3 || delta_col(r) < 3 {
        return None;
    }
    let min_row = r.min.row + 1;
    let min_col = r.min.col + 1;
    let max_row = r.max.row - 1;
    let max_col = r.max.col - 1;
    Some(RectI32 { min: PointI32 { row: min_row, col: min_col }, max: PointI32 { row: max_row, col: max_col } })
}

pub fn assign_deflate(r: &mut RectI32) {
    try_assign_deflate(r).unwrap()
}

pub fn deflate(r: &RectI32) -> RectI32 {
    try_deflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::matrix::rect::rect_i32::RectI32;

    use super::{assign_deflate, deflate, try_assign_deflate, try_deflate};

    #[test]
    fn try_assign_deflate_odd() {
        let mut r = RectI32::of(-5, -5, 5, 5);
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(-4, -4, 4, 4));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(-3, -3, 3, 3));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(-2, -2, 2, 2));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(-1, -1, 1, 1));
    }

    #[test]
    fn try_assign_deflate_even() {
        let mut r = RectI32::of(-5, -5, 6, 6);
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(-4, -4, 5, 5));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(-3, -3, 4, 4));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(-2, -2, 3, 3));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(-1, -1, 2, 2));
    }

    #[test]
    fn try_assign_deflate_small_size() {
        let mut r_0 = RectI32::of(10, 10, 10, 10);
        assert_eq!(try_assign_deflate(&mut r_0), None);
        assert_eq!(r_0, RectI32::of(10, 10, 10, 10));

        let mut r_1 = RectI32::of(10, 10, 11, 11);
        assert_eq!(try_assign_deflate(&mut r_1), None);
        assert_eq!(r_1, RectI32::of(10, 10, 11, 11));

        let mut r_2 = RectI32::of(10, 10, 12, 12);
        assert_eq!(try_assign_deflate(&mut r_2), None);
        assert_eq!(r_2, RectI32::of(10, 10, 12, 12));
    }

    #[test]
    fn try_deflate_odd() {
        assert_eq!(try_deflate(&RectI32::of(-5, -5, 5, 5)), Some(RectI32::of(-4, -4, 4, 4)));
        assert_eq!(try_deflate(&RectI32::of(-4, -4, 4, 4)), Some(RectI32::of(-3, -3, 3, 3)));
        assert_eq!(try_deflate(&RectI32::of(-3, -3, 3, 3)), Some(RectI32::of(-2, -2, 2, 2)));
        assert_eq!(try_deflate(&RectI32::of(-2, -2, 2, 2)), Some(RectI32::of(-1, -1, 1, 1)));
    }

    #[test]
    fn try_deflate_even() {
        assert_eq!(try_deflate(&RectI32::of(-5, -5, 6, 6)), Some(RectI32::of(-4, -4, 5, 5)));
        assert_eq!(try_deflate(&RectI32::of(-4, -4, 5, 5)), Some(RectI32::of(-3, -3, 4, 4)));
        assert_eq!(try_deflate(&RectI32::of(-3, -3, 4, 4)), Some(RectI32::of(-2, -2, 3, 3)));
        assert_eq!(try_deflate(&RectI32::of(-2, -2, 3, 3)), Some(RectI32::of(-1, -1, 2, 2)));
    }

    #[test]
    fn try_deflate_small_size() {
        assert_eq!(try_deflate(&RectI32::of(10, 10, 10, 10)), None);
        assert_eq!(try_deflate(&RectI32::of(10, 10, 11, 11)), None);
        assert_eq!(try_deflate(&RectI32::of(10, 10, 12, 12)), None);
    }

    #[test]
    fn assign_deflate_odd() {
        let mut r = RectI32::of(-5, -5, 5, 5);
        assign_deflate(&mut r);
        assert_eq!(r, RectI32::of(-4, -4, 4, 4));
        assign_deflate(&mut r);
        assert_eq!(r, RectI32::of(-3, -3, 3, 3));
        assign_deflate(&mut r);
        assert_eq!(r, RectI32::of(-2, -2, 2, 2));
        assign_deflate(&mut r);
        assert_eq!(r, RectI32::of(-1, -1, 1, 1));
    }

    #[test]
    fn assign_deflate_even() {
        let mut r = RectI32::of(-5, -5, 6, 6);
        assign_deflate(&mut r);
        assert_eq!(r, RectI32::of(-4, -4, 5, 5));
        assign_deflate(&mut r);
        assert_eq!(r, RectI32::of(-3, -3, 4, 4));
        assign_deflate(&mut r);
        assert_eq!(r, RectI32::of(-2, -2, 3, 3));
        assign_deflate(&mut r);
        assert_eq!(r, RectI32::of(-1, -1, 2, 2));
    }

    #[test]
    fn deflate_odd() {
        assert_eq!(deflate(&RectI32::of(-5, -5, 5, 5)), RectI32::of(-4, -4, 4, 4));
        assert_eq!(deflate(&RectI32::of(-4, -4, 4, 4)), RectI32::of(-3, -3, 3, 3));
        assert_eq!(deflate(&RectI32::of(-3, -3, 3, 3)), RectI32::of(-2, -2, 2, 2));
        assert_eq!(deflate(&RectI32::of(-2, -2, 2, 2)), RectI32::of(-1, -1, 1, 1));
    }

    #[test]
    fn deflate_even() {
        assert_eq!(deflate(&RectI32::of(-5, -5, 6, 6)), RectI32::of(-4, -4, 5, 5));
        assert_eq!(deflate(&RectI32::of(-4, -4, 5, 5)), RectI32::of(-3, -3, 4, 4));
        assert_eq!(deflate(&RectI32::of(-3, -3, 4, 4)), RectI32::of(-2, -2, 3, 3));
        assert_eq!(deflate(&RectI32::of(-2, -2, 3, 3)), RectI32::of(-1, -1, 2, 2));
    }
}

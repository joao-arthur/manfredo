use crate::matrix::{
    point::point_u8::PointU8,
    rect::rect_u8::{RectU8, delta_col, delta_row},
};

pub fn try_assign_deflate(r: &mut RectU8) -> Option<()> {
    if delta_row(r) < 3 || delta_col(r) < 3 {
        return None;
    }
    r.min.row += 1;
    r.min.col += 1;
    r.max.row -= 1;
    r.max.col -= 1;
    Some(())
}

pub fn try_deflate(r: &RectU8) -> Option<RectU8> {
    if delta_row(r) < 3 || delta_col(r) < 3 {
        return None;
    }
    let min_row = r.min.row + 1;
    let min_col = r.min.col + 1;
    let max_row = r.max.row - 1;
    let max_col = r.max.col - 1;
    Some(RectU8 { min: PointU8 { row: min_row, col: min_col }, max: PointU8 { row: max_row, col: max_col } })
}

pub fn assign_deflate(r: &mut RectU8) {
    try_assign_deflate(r).unwrap()
}

pub fn deflate(r: &RectU8) -> RectU8 {
    try_deflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{assign_deflate, deflate, try_assign_deflate, try_deflate};
    use crate::matrix::rect::rect_u8::RectU8;

    #[test]
    fn try_assign_deflate_odd() {
        let mut r = RectU8::of(0, 0, 9, 9);
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(1, 1, 8, 8));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(2, 2, 7, 7));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(3, 3, 6, 6));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(4, 4, 5, 5));
    }

    #[test]
    fn try_assign_deflate_even() {
        let mut r = RectU8::of(0, 0, 10, 10);
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(1, 1, 9, 9));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(2, 2, 8, 8));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(3, 3, 7, 7));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(4, 4, 6, 6));
    }

    #[test]
    fn try_assign_deflate_small_size() {
        let mut r_0 = RectU8::of(10, 10, 10, 10);
        assert_eq!(try_assign_deflate(&mut r_0), None);
        assert_eq!(r_0, RectU8::of(10, 10, 10, 10));

        let mut r_1 = RectU8::of(10, 10, 11, 11);
        assert_eq!(try_assign_deflate(&mut r_1), None);
        assert_eq!(r_1, RectU8::of(10, 10, 11, 11));

        let mut r_2 = RectU8::of(10, 10, 12, 12);
        assert_eq!(try_assign_deflate(&mut r_2), None);
        assert_eq!(r_2, RectU8::of(10, 10, 12, 12));
    }

    #[test]
    fn try_deflate_odd() {
        assert_eq!(try_deflate(&RectU8::of(0, 0, 9, 9)), Some(RectU8::of(1, 1, 8, 8)));
        assert_eq!(try_deflate(&RectU8::of(1, 1, 8, 8)), Some(RectU8::of(2, 2, 7, 7)));
        assert_eq!(try_deflate(&RectU8::of(2, 2, 7, 7)), Some(RectU8::of(3, 3, 6, 6)));
        assert_eq!(try_deflate(&RectU8::of(3, 3, 6, 6)), Some(RectU8::of(4, 4, 5, 5)));
    }

    #[test]
    fn try_deflate_even() {
        assert_eq!(try_deflate(&RectU8::of(0, 0, 10, 10)), Some(RectU8::of(1, 1, 9, 9)));
        assert_eq!(try_deflate(&RectU8::of(1, 1, 9, 9)), Some(RectU8::of(2, 2, 8, 8)));
        assert_eq!(try_deflate(&RectU8::of(2, 2, 8, 8)), Some(RectU8::of(3, 3, 7, 7)));
        assert_eq!(try_deflate(&RectU8::of(3, 3, 7, 7)), Some(RectU8::of(4, 4, 6, 6)));
    }

    #[test]
    fn try_deflate_small_size() {
        assert_eq!(try_deflate(&RectU8::of(10, 10, 10, 10)), None);
        assert_eq!(try_deflate(&RectU8::of(10, 10, 11, 11)), None);
        assert_eq!(try_deflate(&RectU8::of(10, 10, 12, 12)), None);
    }

    #[test]
    fn test_assign_deflate() {
        let mut r_odd = RectU8::of(0, 0, 9, 9);
        assign_deflate(&mut r_odd);
        assert_eq!(r_odd, RectU8::of(1, 1, 8, 8));

        let mut r_even = RectU8::of(0, 0, 10, 10);
        assign_deflate(&mut r_even);
        assert_eq!(r_even, RectU8::of(1, 1, 9, 9));
    }

    #[test]
    fn test_deflate() {
        assert_eq!(deflate(&RectU8::of(0, 0, 9, 9)), RectU8::of(1, 1, 8, 8));
        assert_eq!(deflate(&RectU8::of(0, 0, 10, 10)), RectU8::of(1, 1, 9, 9));
    }
}

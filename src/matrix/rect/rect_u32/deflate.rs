use crate::matrix::rect::rect_u32::{Rect, delta_col, delta_row};

pub fn try_deflate_assign(r: &mut Rect) -> Option<()> {
    if delta_row(r) < 3 || delta_col(r) < 3 {
        return None;
    }
    r.min.row += 1;
    r.min.col += 1;
    r.max.row -= 1;
    r.max.col -= 1;
    Some(())
}

pub fn try_deflate(r: &Rect) -> Option<Rect> {
    if delta_row(r) < 3 || delta_col(r) < 3 {
        return None;
    }
    let min_row = r.min.row + 1;
    let min_col = r.min.col + 1;
    let max_row = r.max.row - 1;
    let max_col = r.max.col - 1;
    Some(Rect::of(min_row, min_col, max_row, max_col))
}

pub fn deflate_assign(r: &mut Rect) {
    try_deflate_assign(r).unwrap()
}

pub fn deflate(r: &Rect) -> Rect {
    try_deflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{deflate_assign, deflate, try_deflate_assign, try_deflate};
    use crate::matrix::rect::rect_u32::Rect;

    #[test]
    fn try_deflate_assign_odd() {
        let mut r = Rect::of(0, 0, 9, 9);
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(1, 1, 8, 8));
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(2, 2, 7, 7));
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(3, 3, 6, 6));
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(4, 4, 5, 5));
    }

    #[test]
    fn try_deflate_assign_even() {
        let mut r = Rect::of(0, 0, 10, 10);
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(1, 1, 9, 9));
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(2, 2, 8, 8));
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(3, 3, 7, 7));
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(4, 4, 6, 6));
    }

    #[test]
    fn try_deflate_assign_small_size() {
        let mut r_0 = Rect::of(10, 10, 10, 10);
        assert_eq!(try_deflate_assign(&mut r_0), None);
        assert_eq!(r_0, Rect::of(10, 10, 10, 10));

        let mut r_1 = Rect::of(10, 10, 11, 11);
        assert_eq!(try_deflate_assign(&mut r_1), None);
        assert_eq!(r_1, Rect::of(10, 10, 11, 11));

        let mut r_2 = Rect::of(10, 10, 12, 12);
        assert_eq!(try_deflate_assign(&mut r_2), None);
        assert_eq!(r_2, Rect::of(10, 10, 12, 12));
    }

    #[test]
    fn try_deflate_odd() {
        assert_eq!(try_deflate(&Rect::of(0, 0, 9, 9)), Some(Rect::of(1, 1, 8, 8)));
        assert_eq!(try_deflate(&Rect::of(1, 1, 8, 8)), Some(Rect::of(2, 2, 7, 7)));
        assert_eq!(try_deflate(&Rect::of(2, 2, 7, 7)), Some(Rect::of(3, 3, 6, 6)));
        assert_eq!(try_deflate(&Rect::of(3, 3, 6, 6)), Some(Rect::of(4, 4, 5, 5)));
    }

    #[test]
    fn try_deflate_even() {
        assert_eq!(try_deflate(&Rect::of(0, 0, 10, 10)), Some(Rect::of(1, 1, 9, 9)));
        assert_eq!(try_deflate(&Rect::of(1, 1, 9, 9)), Some(Rect::of(2, 2, 8, 8)));
        assert_eq!(try_deflate(&Rect::of(2, 2, 8, 8)), Some(Rect::of(3, 3, 7, 7)));
        assert_eq!(try_deflate(&Rect::of(3, 3, 7, 7)), Some(Rect::of(4, 4, 6, 6)));
    }

    #[test]
    fn try_deflate_small_size() {
        assert_eq!(try_deflate(&Rect::of(10, 10, 10, 10)), None);
        assert_eq!(try_deflate(&Rect::of(10, 10, 11, 11)), None);
        assert_eq!(try_deflate(&Rect::of(10, 10, 12, 12)), None);
    }

    #[test]
    fn test_deflate_assign() {
        let mut r_odd = Rect::of(0, 0, 9, 9);
        deflate_assign(&mut r_odd);
        assert_eq!(r_odd, Rect::of(1, 1, 8, 8));

        let mut r_even = Rect::of(0, 0, 10, 10);
        deflate_assign(&mut r_even);
        assert_eq!(r_even, Rect::of(1, 1, 9, 9));
    }

    #[test]
    fn test_deflate() {
        assert_eq!(deflate(&Rect::of(0, 0, 9, 9)), Rect::of(1, 1, 8, 8));
        assert_eq!(deflate(&Rect::of(0, 0, 10, 10)), Rect::of(1, 1, 9, 9));
    }
}

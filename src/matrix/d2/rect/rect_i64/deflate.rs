use crate::matrix::d2::rect::rect_i64::{Rect, delta_col, delta_row};

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
    use super::{deflate, deflate_assign, try_deflate, try_deflate_assign};
    use crate::matrix::d2::rect::rect_i64::Rect;

    #[test]
    fn try_deflate_assign_odd() {
        let mut r = Rect::of(-5, -5, 5, 5);
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(-4, -4, 4, 4));
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(-3, -3, 3, 3));
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(-2, -2, 2, 2));
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(-1, -1, 1, 1));
    }

    #[test]
    fn try_deflate_assign_even() {
        let mut r = Rect::of(-5, -5, 6, 6);
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(-4, -4, 5, 5));
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(-3, -3, 4, 4));
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(-2, -2, 3, 3));
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(-1, -1, 2, 2));
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
        assert_eq!(try_deflate(&Rect::of(-5, -5, 5, 5)), Some(Rect::of(-4, -4, 4, 4)));
        assert_eq!(try_deflate(&Rect::of(-4, -4, 4, 4)), Some(Rect::of(-3, -3, 3, 3)));
        assert_eq!(try_deflate(&Rect::of(-3, -3, 3, 3)), Some(Rect::of(-2, -2, 2, 2)));
        assert_eq!(try_deflate(&Rect::of(-2, -2, 2, 2)), Some(Rect::of(-1, -1, 1, 1)));
    }

    #[test]
    fn try_deflate_even() {
        assert_eq!(try_deflate(&Rect::of(-5, -5, 6, 6)), Some(Rect::of(-4, -4, 5, 5)));
        assert_eq!(try_deflate(&Rect::of(-4, -4, 5, 5)), Some(Rect::of(-3, -3, 4, 4)));
        assert_eq!(try_deflate(&Rect::of(-3, -3, 4, 4)), Some(Rect::of(-2, -2, 3, 3)));
        assert_eq!(try_deflate(&Rect::of(-2, -2, 3, 3)), Some(Rect::of(-1, -1, 2, 2)));
    }

    #[test]
    fn try_deflate_small_size() {
        assert_eq!(try_deflate(&Rect::of(10, 10, 10, 10)), None);
        assert_eq!(try_deflate(&Rect::of(10, 10, 11, 11)), None);
        assert_eq!(try_deflate(&Rect::of(10, 10, 12, 12)), None);
    }

    #[test]
    fn deflate_assign_odd() {
        let mut r = Rect::of(-5, -5, 5, 5);
        deflate_assign(&mut r);
        assert_eq!(r, Rect::of(-4, -4, 4, 4));
        deflate_assign(&mut r);
        assert_eq!(r, Rect::of(-3, -3, 3, 3));
        deflate_assign(&mut r);
        assert_eq!(r, Rect::of(-2, -2, 2, 2));
        deflate_assign(&mut r);
        assert_eq!(r, Rect::of(-1, -1, 1, 1));
    }

    #[test]
    fn deflate_assign_even() {
        let mut r = Rect::of(-5, -5, 6, 6);
        deflate_assign(&mut r);
        assert_eq!(r, Rect::of(-4, -4, 5, 5));
        deflate_assign(&mut r);
        assert_eq!(r, Rect::of(-3, -3, 4, 4));
        deflate_assign(&mut r);
        assert_eq!(r, Rect::of(-2, -2, 3, 3));
        deflate_assign(&mut r);
        assert_eq!(r, Rect::of(-1, -1, 2, 2));
    }

    #[test]
    fn deflate_odd() {
        assert_eq!(deflate(&Rect::of(-5, -5, 5, 5)), Rect::of(-4, -4, 4, 4));
        assert_eq!(deflate(&Rect::of(-4, -4, 4, 4)), Rect::of(-3, -3, 3, 3));
        assert_eq!(deflate(&Rect::of(-3, -3, 3, 3)), Rect::of(-2, -2, 2, 2));
        assert_eq!(deflate(&Rect::of(-2, -2, 2, 2)), Rect::of(-1, -1, 1, 1));
    }

    #[test]
    fn deflate_even() {
        assert_eq!(deflate(&Rect::of(-5, -5, 6, 6)), Rect::of(-4, -4, 5, 5));
        assert_eq!(deflate(&Rect::of(-4, -4, 5, 5)), Rect::of(-3, -3, 4, 4));
        assert_eq!(deflate(&Rect::of(-3, -3, 4, 4)), Rect::of(-2, -2, 3, 3));
        assert_eq!(deflate(&Rect::of(-2, -2, 3, 3)), Rect::of(-1, -1, 2, 2));
    }
}

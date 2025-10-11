use super::{Rect, delta_x, delta_y};

pub fn try_deflate_assign(r: &mut Rect) -> Option<()> {
    if delta_x(r) < 3.0 || delta_y(r) < 3.0 {
        return None;
    }
    r.min.x += 1.0;
    r.min.y += 1.0;
    r.max.x -= 1.0;
    r.max.y -= 1.0;
    Some(())
}

pub fn try_deflate(r: &Rect) -> Option<Rect> {
    if delta_x(r) < 3.0 || delta_y(r) < 3.0 {
        return None;
    }
    let min_x = r.min.x + 1.0;
    let min_y = r.min.y + 1.0;
    let max_x = r.max.x - 1.0;
    let max_y = r.max.y - 1.0;
    Some(Rect::of(min_x, min_y, max_x, max_y))
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
    use crate::cartesian::d2::rect::rect_f64::Rect;

    #[test]
    fn try_deflate_assign_odd() {
        let mut r = Rect::of(-5.0, -5.0, 5.0, 5.0);
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(-4.0, -4.0, 4.0, 4.0));
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(-3.0, -3.0, 3.0, 3.0));
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(-2.0, -2.0, 2.0, 2.0));
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(-1.0, -1.0, 1.0, 1.0));
    }

    #[test]
    fn try_deflate_assign_even() {
        let mut r = Rect::of(-5.0, -5.0, 6.0, 6.0);
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(-4.0, -4.0, 5.0, 5.0));
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(-3.0, -3.0, 4.0, 4.0));
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(-2.0, -2.0, 3.0, 3.0));
        assert_eq!(try_deflate_assign(&mut r), Some(()));
        assert_eq!(r, Rect::of(-1.0, -1.0, 2.0, 2.0));
    }

    #[test]
    fn try_deflate_assign_small_size() {
        let mut r_0 = Rect::of(10.0, 10.0, 10.0, 10.0);
        assert_eq!(try_deflate_assign(&mut r_0), None);
        assert_eq!(r_0, Rect::of(10.0, 10.0, 10.0, 10.0));

        let mut r_1 = Rect::of(10.0, 10.0, 11.0, 11.0);
        assert_eq!(try_deflate_assign(&mut r_1), None);
        assert_eq!(r_1, Rect::of(10.0, 10.0, 11.0, 11.0));

        let mut r_2 = Rect::of(10.0, 10.0, 12.0, 12.0);
        assert_eq!(try_deflate_assign(&mut r_2), None);
        assert_eq!(r_2, Rect::of(10.0, 10.0, 12.0, 12.0));
    }

    #[test]
    fn try_deflate_odd() {
        assert_eq!(try_deflate(&Rect::of(-5.0, -5.0, 5.0, 5.0)), Some(Rect::of(-4.0, -4.0, 4.0, 4.0)));
        assert_eq!(try_deflate(&Rect::of(-4.0, -4.0, 4.0, 4.0)), Some(Rect::of(-3.0, -3.0, 3.0, 3.0)));
        assert_eq!(try_deflate(&Rect::of(-3.0, -3.0, 3.0, 3.0)), Some(Rect::of(-2.0, -2.0, 2.0, 2.0)));
        assert_eq!(try_deflate(&Rect::of(-2.0, -2.0, 2.0, 2.0)), Some(Rect::of(-1.0, -1.0, 1.0, 1.0)));
    }

    #[test]
    fn try_deflate_even() {
        assert_eq!(try_deflate(&Rect::of(-5.0, -5.0, 6.0, 6.0)), Some(Rect::of(-4.0, -4.0, 5.0, 5.0)));
        assert_eq!(try_deflate(&Rect::of(-4.0, -4.0, 5.0, 5.0)), Some(Rect::of(-3.0, -3.0, 4.0, 4.0)));
        assert_eq!(try_deflate(&Rect::of(-3.0, -3.0, 4.0, 4.0)), Some(Rect::of(-2.0, -2.0, 3.0, 3.0)));
        assert_eq!(try_deflate(&Rect::of(-2.0, -2.0, 3.0, 3.0)), Some(Rect::of(-1.0, -1.0, 2.0, 2.0)));
    }

    #[test]
    fn try_deflate_small_size() {
        assert_eq!(try_deflate(&Rect::of(10.0, 10.0, 10.0, 10.0)), None);
        assert_eq!(try_deflate(&Rect::of(10.0, 10.0, 11.0, 11.0)), None);
        assert_eq!(try_deflate(&Rect::of(10.0, 10.0, 12.0, 12.0)), None);
    }

    #[test]
    fn deflate_assign_odd() {
        let mut r = Rect::of(-5.0, -5.0, 5.0, 5.0);
        deflate_assign(&mut r);
        assert_eq!(r, Rect::of(-4.0, -4.0, 4.0, 4.0));
        deflate_assign(&mut r);
        assert_eq!(r, Rect::of(-3.0, -3.0, 3.0, 3.0));
        deflate_assign(&mut r);
        assert_eq!(r, Rect::of(-2.0, -2.0, 2.0, 2.0));
        deflate_assign(&mut r);
        assert_eq!(r, Rect::of(-1.0, -1.0, 1.0, 1.0));
    }

    #[test]
    fn deflate_assign_even() {
        let mut r = Rect::of(-5.0, -5.0, 6.0, 6.0);
        deflate_assign(&mut r);
        assert_eq!(r, Rect::of(-4.0, -4.0, 5.0, 5.0));
        deflate_assign(&mut r);
        assert_eq!(r, Rect::of(-3.0, -3.0, 4.0, 4.0));
        deflate_assign(&mut r);
        assert_eq!(r, Rect::of(-2.0, -2.0, 3.0, 3.0));
        deflate_assign(&mut r);
        assert_eq!(r, Rect::of(-1.0, -1.0, 2.0, 2.0));
    }

    #[test]
    fn deflate_odd() {
        assert_eq!(deflate(&Rect::of(-5.0, -5.0, 5.0, 5.0)), Rect::of(-4.0, -4.0, 4.0, 4.0));
        assert_eq!(deflate(&Rect::of(-4.0, -4.0, 4.0, 4.0)), Rect::of(-3.0, -3.0, 3.0, 3.0));
        assert_eq!(deflate(&Rect::of(-3.0, -3.0, 3.0, 3.0)), Rect::of(-2.0, -2.0, 2.0, 2.0));
        assert_eq!(deflate(&Rect::of(-2.0, -2.0, 2.0, 2.0)), Rect::of(-1.0, -1.0, 1.0, 1.0));
    }

    #[test]
    fn deflate_even() {
        assert_eq!(deflate(&Rect::of(-5.0, -5.0, 6.0, 6.0)), Rect::of(-4.0, -4.0, 5.0, 5.0));
        assert_eq!(deflate(&Rect::of(-4.0, -4.0, 5.0, 5.0)), Rect::of(-3.0, -3.0, 4.0, 4.0));
        assert_eq!(deflate(&Rect::of(-3.0, -3.0, 4.0, 4.0)), Rect::of(-2.0, -2.0, 3.0, 3.0));
        assert_eq!(deflate(&Rect::of(-2.0, -2.0, 3.0, 3.0)), Rect::of(-1.0, -1.0, 2.0, 2.0));
    }
}

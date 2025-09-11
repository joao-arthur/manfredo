use crate::cartesian::{
    point::point_f64::PointF64,
    rect::rect_f64::{Rect, delta_x, delta_y},
};

pub fn try_assign_deflate(r: &mut Rect) -> Option<()> {
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
    Some(Rect { min: PointF64 { x: min_x, y: min_y }, max: PointF64 { x: max_x, y: max_y } })
}

pub fn assign_deflate(r: &mut Rect) {
    try_assign_deflate(r).unwrap()
}

pub fn deflate(r: &Rect) -> Rect {
    try_deflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{assign_deflate, deflate, try_assign_deflate, try_deflate};
    use crate::cartesian::rect::rect_f64::Rect;

    #[test]
    fn try_assign_deflate_odd() {
        let mut r = Rect::of(-5.0, -5.0, 5.0, 5.0);
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, Rect::of(-4.0, -4.0, 4.0, 4.0));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, Rect::of(-3.0, -3.0, 3.0, 3.0));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, Rect::of(-2.0, -2.0, 2.0, 2.0));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, Rect::of(-1.0, -1.0, 1.0, 1.0));
    }

    #[test]
    fn try_assign_deflate_even() {
        let mut r = Rect::of(-5.0, -5.0, 6.0, 6.0);
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, Rect::of(-4.0, -4.0, 5.0, 5.0));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, Rect::of(-3.0, -3.0, 4.0, 4.0));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, Rect::of(-2.0, -2.0, 3.0, 3.0));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, Rect::of(-1.0, -1.0, 2.0, 2.0));
    }

    #[test]
    fn try_assign_deflate_small_size() {
        let mut r_0 = Rect::of(10.0, 10.0, 10.0, 10.0);
        assert_eq!(try_assign_deflate(&mut r_0), None);
        assert_eq!(r_0, Rect::of(10.0, 10.0, 10.0, 10.0));

        let mut r_1 = Rect::of(10.0, 10.0, 11.0, 11.0);
        assert_eq!(try_assign_deflate(&mut r_1), None);
        assert_eq!(r_1, Rect::of(10.0, 10.0, 11.0, 11.0));

        let mut r_2 = Rect::of(10.0, 10.0, 12.0, 12.0);
        assert_eq!(try_assign_deflate(&mut r_2), None);
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
    fn assign_deflate_odd() {
        let mut r = Rect::of(-5.0, -5.0, 5.0, 5.0);
        assign_deflate(&mut r);
        assert_eq!(r, Rect::of(-4.0, -4.0, 4.0, 4.0));
        assign_deflate(&mut r);
        assert_eq!(r, Rect::of(-3.0, -3.0, 3.0, 3.0));
        assign_deflate(&mut r);
        assert_eq!(r, Rect::of(-2.0, -2.0, 2.0, 2.0));
        assign_deflate(&mut r);
        assert_eq!(r, Rect::of(-1.0, -1.0, 1.0, 1.0));
    }

    #[test]
    fn assign_deflate_even() {
        let mut r = Rect::of(-5.0, -5.0, 6.0, 6.0);
        assign_deflate(&mut r);
        assert_eq!(r, Rect::of(-4.0, -4.0, 5.0, 5.0));
        assign_deflate(&mut r);
        assert_eq!(r, Rect::of(-3.0, -3.0, 4.0, 4.0));
        assign_deflate(&mut r);
        assert_eq!(r, Rect::of(-2.0, -2.0, 3.0, 3.0));
        assign_deflate(&mut r);
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

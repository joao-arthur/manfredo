use crate::cartesian::{
    point::point_u32::Point,
    rect::rect_u32::{Rect, delta_x, delta_y},
};

pub fn try_assign_deflate(r: &mut Rect) -> Option<()> {
    if delta_x(r) < 3 || delta_y(r) < 3 {
        return None;
    }
    r.min.x += 1;
    r.min.y += 1;
    r.max.x -= 1;
    r.max.y -= 1;
    Some(())
}

pub fn try_deflate(r: &Rect) -> Option<Rect> {
    if delta_x(r) < 3 || delta_y(r) < 3 {
        return None;
    }
    let min_x = r.min.x + 1;
    let min_y = r.min.y + 1;
    let max_x = r.max.x - 1;
    let max_y = r.max.y - 1;
    Some(Rect { min: Point { x: min_x, y: min_y }, max: Point { x: max_x, y: max_y } })
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
    use crate::cartesian::rect::rect_u32::Rect;

    #[test]
    fn try_assign_deflate_odd() {
        let mut r = Rect::of(0, 0, 9, 9);
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, Rect::of(1, 1, 8, 8));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, Rect::of(2, 2, 7, 7));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, Rect::of(3, 3, 6, 6));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, Rect::of(4, 4, 5, 5));
    }

    #[test]
    fn try_assign_deflate_even() {
        let mut r = Rect::of(0, 0, 10, 10);
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, Rect::of(1, 1, 9, 9));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, Rect::of(2, 2, 8, 8));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, Rect::of(3, 3, 7, 7));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, Rect::of(4, 4, 6, 6));
    }

    #[test]
    fn try_assign_deflate_small_size() {
        let mut r_0 = Rect::of(10, 10, 10, 10);
        assert_eq!(try_assign_deflate(&mut r_0), None);
        assert_eq!(r_0, Rect::of(10, 10, 10, 10));

        let mut r_1 = Rect::of(10, 10, 11, 11);
        assert_eq!(try_assign_deflate(&mut r_1), None);
        assert_eq!(r_1, Rect::of(10, 10, 11, 11));

        let mut r_2 = Rect::of(10, 10, 12, 12);
        assert_eq!(try_assign_deflate(&mut r_2), None);
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
    fn test_assign_deflate() {
        let mut r_odd = Rect::of(0, 0, 9, 9);
        assign_deflate(&mut r_odd);
        assert_eq!(r_odd, Rect::of(1, 1, 8, 8));

        let mut r_even = Rect::of(0, 0, 10, 10);
        assign_deflate(&mut r_even);
        assert_eq!(r_even, Rect::of(1, 1, 9, 9));
    }

    #[test]
    fn test_deflate() {
        assert_eq!(deflate(&Rect::of(0, 0, 9, 9)), Rect::of(1, 1, 8, 8));
        assert_eq!(deflate(&Rect::of(0, 0, 10, 10)), Rect::of(1, 1, 9, 9));
    }
}

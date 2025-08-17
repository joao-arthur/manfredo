use crate::cartesian::{
    point::point_u32::PointU32,
    rect::rect_u32::{RectU32, delta_x, delta_y},
};

pub fn try_assign_deflate(r: &mut RectU32) -> Option<()> {
    if delta_x(r) < 3 || delta_y(r) < 3 {
        return None;
    }
    r.min.x += 1;
    r.min.y += 1;
    r.max.x -= 1;
    r.max.y -= 1;
    Some(())
}

pub fn try_deflate(r: &RectU32) -> Option<RectU32> {
    if delta_x(r) < 3 || delta_y(r) < 3 {
        return None;
    }
    let min_x = r.min.x + 1;
    let min_y = r.min.y + 1;
    let max_x = r.max.x - 1;
    let max_y = r.max.y - 1;
    Some(RectU32 { min: PointU32 { x: min_x, y: min_y }, max: PointU32 { x: max_x, y: max_y } })
}

pub fn assign_deflate(r: &mut RectU32) {
    try_assign_deflate(r).unwrap()
}

pub fn deflate(r: &RectU32) -> RectU32 {
    try_deflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_u32::RectU32;

    use super::{assign_deflate, deflate, try_assign_deflate, try_deflate};

    #[test]
    fn try_assign_deflate_odd() {
        let mut r = RectU32::of(0, 0, 9, 9);
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(1, 1, 8, 8));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(2, 2, 7, 7));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(3, 3, 6, 6));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(4, 4, 5, 5));
    }

    #[test]
    fn try_assign_deflate_even() {
        let mut r = RectU32::of(0, 0, 10, 10);
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(1, 1, 9, 9));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(2, 2, 8, 8));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(3, 3, 7, 7));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(4, 4, 6, 6));
    }

    #[test]
    fn try_assign_deflate_small_size() {
        let mut r_0 = RectU32::of(10, 10, 10, 10);
        assert_eq!(try_assign_deflate(&mut r_0), None);
        assert_eq!(r_0, RectU32::of(10, 10, 10, 10));

        let mut r_1 = RectU32::of(10, 10, 11, 11);
        assert_eq!(try_assign_deflate(&mut r_1), None);
        assert_eq!(r_1, RectU32::of(10, 10, 11, 11));

        let mut r_2 = RectU32::of(10, 10, 12, 12);
        assert_eq!(try_assign_deflate(&mut r_2), None);
        assert_eq!(r_2, RectU32::of(10, 10, 12, 12));
    }

    #[test]
    fn try_deflate_odd() {
        assert_eq!(try_deflate(&RectU32::of(0, 0, 9, 9)), Some(RectU32::of(1, 1, 8, 8)));
        assert_eq!(try_deflate(&RectU32::of(1, 1, 8, 8)), Some(RectU32::of(2, 2, 7, 7)));
        assert_eq!(try_deflate(&RectU32::of(2, 2, 7, 7)), Some(RectU32::of(3, 3, 6, 6)));
        assert_eq!(try_deflate(&RectU32::of(3, 3, 6, 6)), Some(RectU32::of(4, 4, 5, 5)));
    }

    #[test]
    fn try_deflate_even() {
        assert_eq!(try_deflate(&RectU32::of(0, 0, 10, 10)), Some(RectU32::of(1, 1, 9, 9)));
        assert_eq!(try_deflate(&RectU32::of(1, 1, 9, 9)), Some(RectU32::of(2, 2, 8, 8)));
        assert_eq!(try_deflate(&RectU32::of(2, 2, 8, 8)), Some(RectU32::of(3, 3, 7, 7)));
        assert_eq!(try_deflate(&RectU32::of(3, 3, 7, 7)), Some(RectU32::of(4, 4, 6, 6)));
    }

    #[test]
    fn try_deflate_small_size() {
        assert_eq!(try_deflate(&RectU32::of(10, 10, 10, 10)), None);
        assert_eq!(try_deflate(&RectU32::of(10, 10, 11, 11)), None);
        assert_eq!(try_deflate(&RectU32::of(10, 10, 12, 12)), None);
    }

    #[test]
    fn test_assign_deflate() {
        let mut r_odd = RectU32::of(0, 0, 9, 9);
        assign_deflate(&mut r_odd);
        assert_eq!(r_odd, RectU32::of(1, 1, 8, 8));

        let mut r_even = RectU32::of(0, 0, 10, 10);
        assign_deflate(&mut r_even);
        assert_eq!(r_even, RectU32::of(1, 1, 9, 9));
    }

    #[test]
    fn test_deflate() {
        assert_eq!(deflate(&RectU32::of(0, 0, 9, 9)), RectU32::of(1, 1, 8, 8));
        assert_eq!(deflate(&RectU32::of(0, 0, 10, 10)), RectU32::of(1, 1, 9, 9));
    }
}

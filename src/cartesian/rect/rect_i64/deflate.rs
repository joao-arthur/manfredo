use crate::cartesian::{
    point::point_i64::PointI64,
    rect::rect_i64::{RectI64, delta_x, delta_y},
};

pub fn try_assign_deflate(r: &mut RectI64) -> Option<()> {
    if delta_x(r) < 3 || delta_y(r) < 3 {
        return None;
    }
    r.min.x += 1;
    r.min.y += 1;
    r.max.x -= 1;
    r.max.y -= 1;
    Some(())
}

pub fn try_deflate(r: &RectI64) -> Option<RectI64> {
    if delta_x(r) < 3 || delta_y(r) < 3 {
        return None;
    }
    let min_x = r.min.x + 1;
    let min_y = r.min.y + 1;
    let max_x = r.max.x - 1;
    let max_y = r.max.y - 1;
    Some(RectI64 { min: PointI64 { x: min_x, y: min_y }, max: PointI64 { x: max_x, y: max_y } })
}

pub fn assign_deflate(r: &mut RectI64) {
    try_assign_deflate(r).unwrap()
}

pub fn deflate(r: &RectI64) -> RectI64 {
    try_deflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_i64::RectI64;

    use super::{assign_deflate, deflate, try_assign_deflate, try_deflate};

    #[test]
    fn try_assign_deflate_odd() {
        let mut r = RectI64::of(-5, -5, 5, 5);
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(-4, -4, 4, 4));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(-3, -3, 3, 3));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(-2, -2, 2, 2));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(-1, -1, 1, 1));
    }

    #[test]
    fn try_assign_deflate_even() {
        let mut r = RectI64::of(-5, -5, 6, 6);
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(-4, -4, 5, 5));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(-3, -3, 4, 4));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(-2, -2, 3, 3));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(-1, -1, 2, 2));
    }

    #[test]
    fn try_assign_deflate_small_size() {
        let mut r_0 = RectI64::of(10, 10, 10, 10);
        assert_eq!(try_assign_deflate(&mut r_0), None);
        assert_eq!(r_0, RectI64::of(10, 10, 10, 10));

        let mut r_1 = RectI64::of(10, 10, 11, 11);
        assert_eq!(try_assign_deflate(&mut r_1), None);
        assert_eq!(r_1, RectI64::of(10, 10, 11, 11));

        let mut r_2 = RectI64::of(10, 10, 12, 12);
        assert_eq!(try_assign_deflate(&mut r_2), None);
        assert_eq!(r_2, RectI64::of(10, 10, 12, 12));
    }

    #[test]
    fn try_deflate_odd() {
        assert_eq!(try_deflate(&RectI64::of(-5, -5, 5, 5)), Some(RectI64::of(-4, -4, 4, 4)));
        assert_eq!(try_deflate(&RectI64::of(-4, -4, 4, 4)), Some(RectI64::of(-3, -3, 3, 3)));
        assert_eq!(try_deflate(&RectI64::of(-3, -3, 3, 3)), Some(RectI64::of(-2, -2, 2, 2)));
        assert_eq!(try_deflate(&RectI64::of(-2, -2, 2, 2)), Some(RectI64::of(-1, -1, 1, 1)));
    }

    #[test]
    fn try_deflate_even() {
        assert_eq!(try_deflate(&RectI64::of(-5, -5, 6, 6)), Some(RectI64::of(-4, -4, 5, 5)));
        assert_eq!(try_deflate(&RectI64::of(-4, -4, 5, 5)), Some(RectI64::of(-3, -3, 4, 4)));
        assert_eq!(try_deflate(&RectI64::of(-3, -3, 4, 4)), Some(RectI64::of(-2, -2, 3, 3)));
        assert_eq!(try_deflate(&RectI64::of(-2, -2, 3, 3)), Some(RectI64::of(-1, -1, 2, 2)));
    }

    #[test]
    fn try_deflate_small_size() {
        assert_eq!(try_deflate(&RectI64::of(10, 10, 10, 10)), None);
        assert_eq!(try_deflate(&RectI64::of(10, 10, 11, 11)), None);
        assert_eq!(try_deflate(&RectI64::of(10, 10, 12, 12)), None);
    }

    #[test]
    fn assign_deflate_odd() {
        let mut r = RectI64::of(-5, -5, 5, 5);
        assign_deflate(&mut r);
        assert_eq!(r, RectI64::of(-4, -4, 4, 4));
        assign_deflate(&mut r);
        assert_eq!(r, RectI64::of(-3, -3, 3, 3));
        assign_deflate(&mut r);
        assert_eq!(r, RectI64::of(-2, -2, 2, 2));
        assign_deflate(&mut r);
        assert_eq!(r, RectI64::of(-1, -1, 1, 1));
    }

    #[test]
    fn assign_deflate_even() {
        let mut r = RectI64::of(-5, -5, 6, 6);
        assign_deflate(&mut r);
        assert_eq!(r, RectI64::of(-4, -4, 5, 5));
        assign_deflate(&mut r);
        assert_eq!(r, RectI64::of(-3, -3, 4, 4));
        assign_deflate(&mut r);
        assert_eq!(r, RectI64::of(-2, -2, 3, 3));
        assign_deflate(&mut r);
        assert_eq!(r, RectI64::of(-1, -1, 2, 2));
    }

    #[test]
    fn deflate_odd() {
        assert_eq!(deflate(&RectI64::of(-5, -5, 5, 5)), RectI64::of(-4, -4, 4, 4));
        assert_eq!(deflate(&RectI64::of(-4, -4, 4, 4)), RectI64::of(-3, -3, 3, 3));
        assert_eq!(deflate(&RectI64::of(-3, -3, 3, 3)), RectI64::of(-2, -2, 2, 2));
        assert_eq!(deflate(&RectI64::of(-2, -2, 2, 2)), RectI64::of(-1, -1, 1, 1));
    }

    #[test]
    fn deflate_even() {
        assert_eq!(deflate(&RectI64::of(-5, -5, 6, 6)), RectI64::of(-4, -4, 5, 5));
        assert_eq!(deflate(&RectI64::of(-4, -4, 5, 5)), RectI64::of(-3, -3, 4, 4));
        assert_eq!(deflate(&RectI64::of(-3, -3, 4, 4)), RectI64::of(-2, -2, 3, 3));
        assert_eq!(deflate(&RectI64::of(-2, -2, 3, 3)), RectI64::of(-1, -1, 2, 2));
    }
}

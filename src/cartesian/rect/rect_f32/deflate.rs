use crate::cartesian::{
    point::point_f32::PointF32,
    rect::rect_f32::{RectF32, delta_x, delta_y},
};

pub fn try_assign_deflate(r: &mut RectF32) -> Option<()> {
    if delta_x(r) < 3.0 || delta_y(r) < 3.0 {
        return None;
    }
    r.min.x += 1.0;
    r.min.y += 1.0;
    r.max.x -= 1.0;
    r.max.y -= 1.0;
    Some(())
}

pub fn try_deflate(r: &RectF32) -> Option<RectF32> {
    if delta_x(r) < 3.0 || delta_y(r) < 3.0 {
        return None;
    }
    let min_x = r.min.x + 1.0;
    let min_y = r.min.y + 1.0;
    let max_x = r.max.x - 1.0;
    let max_y = r.max.y - 1.0;
    Some(RectF32 { min: PointF32 { x: min_x, y: min_y }, max: PointF32 { x: max_x, y: max_y } })
}

pub fn assign_deflate(r: &mut RectF32) {
    try_assign_deflate(r).unwrap()
}

pub fn deflate(r: &RectF32) -> RectF32 {
    try_deflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_f32::RectF32;

    use super::{assign_deflate, deflate, try_assign_deflate, try_deflate};

    #[test]
    fn try_assign_deflate_odd() {
        let mut r = RectF32::of(-5.0, -5.0, 5.0, 5.0);
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectF32::of(-4.0, -4.0, 4.0, 4.0));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectF32::of(-3.0, -3.0, 3.0, 3.0));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectF32::of(-2.0, -2.0, 2.0, 2.0));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectF32::of(-1.0, -1.0, 1.0, 1.0));
    }

    #[test]
    fn try_assign_deflate_even() {
        let mut r = RectF32::of(-5.0, -5.0, 6.0, 6.0);
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectF32::of(-4.0, -4.0, 5.0, 5.0));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectF32::of(-3.0, -3.0, 4.0, 4.0));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectF32::of(-2.0, -2.0, 3.0, 3.0));
        assert_eq!(try_assign_deflate(&mut r), Some(()));
        assert_eq!(r, RectF32::of(-1.0, -1.0, 2.0, 2.0));
    }

    #[test]
    fn try_assign_deflate_small_size() {
        let mut r_0 = RectF32::of(10.0, 10.0, 10.0, 10.0);
        assert_eq!(try_assign_deflate(&mut r_0), None);
        assert_eq!(r_0, RectF32::of(10.0, 10.0, 10.0, 10.0));

        let mut r_1 = RectF32::of(10.0, 10.0, 11.0, 11.0);
        assert_eq!(try_assign_deflate(&mut r_1), None);
        assert_eq!(r_1, RectF32::of(10.0, 10.0, 11.0, 11.0));

        let mut r_2 = RectF32::of(10.0, 10.0, 12.0, 12.0);
        assert_eq!(try_assign_deflate(&mut r_2), None);
        assert_eq!(r_2, RectF32::of(10.0, 10.0, 12.0, 12.0));
    }

    #[test]
    fn try_deflate_odd() {
        assert_eq!(try_deflate(&RectF32::of(-5.0, -5.0, 5.0, 5.0)), Some(RectF32::of(-4.0, -4.0, 4.0, 4.0)));
        assert_eq!(try_deflate(&RectF32::of(-4.0, -4.0, 4.0, 4.0)), Some(RectF32::of(-3.0, -3.0, 3.0, 3.0)));
        assert_eq!(try_deflate(&RectF32::of(-3.0, -3.0, 3.0, 3.0)), Some(RectF32::of(-2.0, -2.0, 2.0, 2.0)));
        assert_eq!(try_deflate(&RectF32::of(-2.0, -2.0, 2.0, 2.0)), Some(RectF32::of(-1.0, -1.0, 1.0, 1.0)));
    }

    #[test]
    fn try_deflate_even() {
        assert_eq!(try_deflate(&RectF32::of(-5.0, -5.0, 6.0, 6.0)), Some(RectF32::of(-4.0, -4.0, 5.0, 5.0)));
        assert_eq!(try_deflate(&RectF32::of(-4.0, -4.0, 5.0, 5.0)), Some(RectF32::of(-3.0, -3.0, 4.0, 4.0)));
        assert_eq!(try_deflate(&RectF32::of(-3.0, -3.0, 4.0, 4.0)), Some(RectF32::of(-2.0, -2.0, 3.0, 3.0)));
        assert_eq!(try_deflate(&RectF32::of(-2.0, -2.0, 3.0, 3.0)), Some(RectF32::of(-1.0, -1.0, 2.0, 2.0)));
    }

    #[test]
    fn try_deflate_small_size() {
        assert_eq!(try_deflate(&RectF32::of(10.0, 10.0, 10.0, 10.0)), None);
        assert_eq!(try_deflate(&RectF32::of(10.0, 10.0, 11.0, 11.0)), None);
        assert_eq!(try_deflate(&RectF32::of(10.0, 10.0, 12.0, 12.0)), None);
    }

    #[test]
    fn assign_deflate_odd() {
        let mut r = RectF32::of(-5.0, -5.0, 5.0, 5.0);
        assign_deflate(&mut r);
        assert_eq!(r, RectF32::of(-4.0, -4.0, 4.0, 4.0));
        assign_deflate(&mut r);
        assert_eq!(r, RectF32::of(-3.0, -3.0, 3.0, 3.0));
        assign_deflate(&mut r);
        assert_eq!(r, RectF32::of(-2.0, -2.0, 2.0, 2.0));
        assign_deflate(&mut r);
        assert_eq!(r, RectF32::of(-1.0, -1.0, 1.0, 1.0));
    }

    #[test]
    fn assign_deflate_even() {
        let mut r = RectF32::of(-5.0, -5.0, 6.0, 6.0);
        assign_deflate(&mut r);
        assert_eq!(r, RectF32::of(-4.0, -4.0, 5.0, 5.0));
        assign_deflate(&mut r);
        assert_eq!(r, RectF32::of(-3.0, -3.0, 4.0, 4.0));
        assign_deflate(&mut r);
        assert_eq!(r, RectF32::of(-2.0, -2.0, 3.0, 3.0));
        assign_deflate(&mut r);
        assert_eq!(r, RectF32::of(-1.0, -1.0, 2.0, 2.0));
    }

    #[test]
    fn deflate_odd() {
        assert_eq!(deflate(&RectF32::of(-5.0, -5.0, 5.0, 5.0)), RectF32::of(-4.0, -4.0, 4.0, 4.0));
        assert_eq!(deflate(&RectF32::of(-4.0, -4.0, 4.0, 4.0)), RectF32::of(-3.0, -3.0, 3.0, 3.0));
        assert_eq!(deflate(&RectF32::of(-3.0, -3.0, 3.0, 3.0)), RectF32::of(-2.0, -2.0, 2.0, 2.0));
        assert_eq!(deflate(&RectF32::of(-2.0, -2.0, 2.0, 2.0)), RectF32::of(-1.0, -1.0, 1.0, 1.0));
    }

    #[test]
    fn deflate_even() {
        assert_eq!(deflate(&RectF32::of(-5.0, -5.0, 6.0, 6.0)), RectF32::of(-4.0, -4.0, 5.0, 5.0));
        assert_eq!(deflate(&RectF32::of(-4.0, -4.0, 5.0, 5.0)), RectF32::of(-3.0, -3.0, 4.0, 4.0));
        assert_eq!(deflate(&RectF32::of(-3.0, -3.0, 4.0, 4.0)), RectF32::of(-2.0, -2.0, 3.0, 3.0));
        assert_eq!(deflate(&RectF32::of(-2.0, -2.0, 3.0, 3.0)), RectF32::of(-1.0, -1.0, 2.0, 2.0));
    }
}

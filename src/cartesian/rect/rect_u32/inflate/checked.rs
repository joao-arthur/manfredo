use crate::cartesian::{point::point_u32::PointU32, rect::rect_u32::RectU32};

pub fn try_assign_inflate(r: &mut RectU32) -> Option<()> {
    let min_x = r.min.x.checked_sub(1)?;
    let min_y = r.min.y.checked_sub(1)?;
    let max_x = r.max.x.checked_add(1)?;
    let max_y = r.max.y.checked_add(1)?;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

pub fn try_inflate(r: &RectU32) -> Option<RectU32> {
    let min_x = r.min.x.checked_sub(1)?;
    let min_y = r.min.y.checked_sub(1)?;
    let max_x = r.max.x.checked_add(1)?;
    let max_y = r.max.y.checked_add(1)?;
    Some(RectU32 { min: PointU32 { x: min_x, y: min_y }, max: PointU32 { x: max_x, y: max_y } })
}

pub fn assign_inflate(r: &mut RectU32) {
    try_assign_inflate(r).unwrap()
}

pub fn inflate(r: &RectU32) -> RectU32 {
    try_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{RectU32, assign_inflate, inflate, try_assign_inflate, try_inflate};

    #[test]
    fn try_assign_inflate_min_bounds() {
        let mut r = RectU32::of(7, 3, 4, 13);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(6, 2, 5, 14));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(5, 1, 6, 15));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(4, 0, 7, 16));
    }

    #[test]
    fn try_assign_inflate_max_bounds() {
        let mut r = RectU32::of(200, 230, u32::MAX - 5, u32::MAX - 3);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(199, 229, u32::MAX - 4, u32::MAX - 2));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(198, 228, u32::MAX - 3, u32::MAX - 1));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(197, 227, u32::MAX - 2, u32::MAX));
    }

    #[test]
    fn try_assign_inflate_to_bounds() {
        let mut r = RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::largest());
    }

    #[test]
    fn try_assign_inflate_width_to_bounds() {
        let mut r_min = RectU32::of(1, 10, 100, 100);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectU32::of(0, 9, 101, 101));

        let mut r_max = RectU32::of(10, 10, u32::MAX - 1, 100);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectU32::of(9, 9, u32::MAX, 101));
    }

    #[test]
    fn try_assign_inflate_height_to_bounds() {
        let mut r_min = RectU32::of(10, 1, 100, 100);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectU32::of(9, 0, 101, 101));

        let mut r_max = RectU32::of(10, 10, 100, u32::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectU32::of(9, 9, 101, u32::MAX));
    }

    #[test]
    fn try_assign_inflate_out_of_bounds() {
        let mut r_min_x = RectU32::of(0, 10, u32::MAX, 100);
        assert_eq!(try_assign_inflate(&mut r_min_x), None);
        assert_eq!(r_min_x, RectU32::of(0, 10, u32::MAX, 100));

        let mut r_max_x = RectU32::of(10, 10, u32::MAX, 100);
        assert_eq!(try_assign_inflate(&mut r_max_x), None);
        assert_eq!(r_max_x, RectU32::of(10, 10, u32::MAX, 100));

        let mut r_min_y = RectU32::of(10, 0, 100, 100);
        assert_eq!(try_assign_inflate(&mut r_min_y), None);
        assert_eq!(r_min_y, RectU32::of(10, 0, 100, 100));

        let mut r_max_y = RectU32::of(10, 10, 100, u32::MAX);
        assert_eq!(try_assign_inflate(&mut r_max_y), None);
        assert_eq!(r_max_y, RectU32::of(10, 10, 100, u32::MAX));

        let mut r_min = RectU32::of(0, 0, 10, 10);
        assert_eq!(try_assign_inflate(&mut r_min), None);
        assert_eq!(r_min, RectU32::of(0, 0, 10, 10));

        let mut r_max = RectU32::of(10, 10, u32::MAX, u32::MAX);
        assert_eq!(try_assign_inflate(&mut r_max), None);
        assert_eq!(r_max, RectU32::of(10, 10, u32::MAX, u32::MAX));
    }

    #[test]
    fn try_assign_inflate_limits_out_of_bounds() {
        let mut r = RectU32::largest();
        assert_eq!(try_assign_inflate(&mut r), None);
        assert_eq!(r, RectU32::largest());
    }

    #[test]
    fn try_inflate_min_bounds() {
        assert_eq!(try_inflate(&RectU32::of(7, 3, 4, 13)), Some(RectU32::of(6, 2, 5, 14)));
        assert_eq!(try_inflate(&RectU32::of(6, 2, 5, 14)), Some(RectU32::of(5, 1, 6, 15)));
        assert_eq!(try_inflate(&RectU32::of(5, 1, 6, 15)), Some(RectU32::of(4, 0, 7, 16)));
    }

    #[test]
    fn try_inflate_max_bounds() {
        assert_eq!(try_inflate(&RectU32::of(200, 230, u32::MAX - 5, u32::MAX - 3)), Some(RectU32::of(199, 229, u32::MAX - 4, u32::MAX - 2)));
        assert_eq!(try_inflate(&RectU32::of(199, 229, u32::MAX - 4, u32::MAX - 2)), Some(RectU32::of(198, 228, u32::MAX - 3, u32::MAX - 1)));
        assert_eq!(try_inflate(&RectU32::of(198, 228, u32::MAX - 3, u32::MAX - 1)), Some(RectU32::of(197, 227, u32::MAX - 2, u32::MAX)));
    }

    #[test]
    fn try_inflate_to_bounds() {
        assert_eq!(try_inflate(&RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1)), Some(RectU32::largest()));
    }

    #[test]
    fn try_inflate_width_to_bounds() {
        assert_eq!(try_inflate(&RectU32::of(1, 10, 100, 100)), Some(RectU32::of(0, 9, 101, 101)));
        assert_eq!(try_inflate(&RectU32::of(10, 10, u32::MAX - 1, 100)), Some(RectU32::of(9, 9, u32::MAX, 101)));
    }

    #[test]
    fn try_inflate_height_to_bounds() {
        assert_eq!(try_inflate(&RectU32::of(10, 1, 100, 100)), Some(RectU32::of(9, 0, 101, 101)));
        assert_eq!(try_inflate(&RectU32::of(10, 10, 100, u32::MAX - 1)), Some(RectU32::of(9, 9, 101, u32::MAX)));
    }

    #[test]
    fn try_inflate_out_of_bounds() {
        assert_eq!(try_inflate(&RectU32::of(0, 10, u32::MAX, 100)), None);
        assert_eq!(try_inflate(&RectU32::of(10, 10, u32::MAX, 100)), None);
        assert_eq!(try_inflate(&RectU32::of(10, 0, 100, 100)), None);
        assert_eq!(try_inflate(&RectU32::of(10, 10, 100, u32::MAX)), None);
        assert_eq!(try_inflate(&RectU32::of(0, 0, 10, 10)), None);
        assert_eq!(try_inflate(&RectU32::of(10, 10, u32::MAX, u32::MAX)), None);
    }

    #[test]
    fn try_inflate_limits_out_of_bounds() {
        assert_eq!(try_inflate(&RectU32::largest()), None);
    }

    #[test]
    fn assign_inflate_min_bounds() {
        let mut r = RectU32::of(7, 3, 4, 13);
        assign_inflate(&mut r);
        assert_eq!(r, RectU32::of(6, 2, 5, 14));
        assign_inflate(&mut r);
        assert_eq!(r, RectU32::of(5, 1, 6, 15));
        assign_inflate(&mut r);
        assert_eq!(r, RectU32::of(4, 0, 7, 16));
    }

    #[test]
    fn assign_inflate_max_bounds() {
        let mut r = RectU32::of(200, 230, u32::MAX - 5, u32::MAX - 3);
        assign_inflate(&mut r);
        assert_eq!(r, RectU32::of(199, 229, u32::MAX - 4, u32::MAX - 2));
        assign_inflate(&mut r);
        assert_eq!(r, RectU32::of(198, 228, u32::MAX - 3, u32::MAX - 1));
        assign_inflate(&mut r);
        assert_eq!(r, RectU32::of(197, 227, u32::MAX - 2, u32::MAX));
    }

    #[test]
    fn inflate_min_bounds() {
        assert_eq!(inflate(&RectU32::of(7, 3, 4, 13)), RectU32::of(6, 2, 5, 14));
        assert_eq!(inflate(&RectU32::of(6, 2, 5, 14)), RectU32::of(5, 1, 6, 15));
        assert_eq!(inflate(&RectU32::of(5, 1, 6, 15)), RectU32::of(4, 0, 7, 16));
    }

    #[test]
    fn inflate_max_bounds() {
        assert_eq!(inflate(&RectU32::of(200, 230, u32::MAX - 5, u32::MAX - 3)), RectU32::of(199, 229, u32::MAX - 4, u32::MAX - 2));
        assert_eq!(inflate(&RectU32::of(199, 229, u32::MAX - 4, u32::MAX - 2)), RectU32::of(198, 228, u32::MAX - 3, u32::MAX - 1));
        assert_eq!(inflate(&RectU32::of(198, 228, u32::MAX - 3, u32::MAX - 1)), RectU32::of(197, 227, u32::MAX - 2, u32::MAX));
    }
}

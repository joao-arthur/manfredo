use crate::cartesian::{point::point_u16::PointU16, rect::rect_u16::RectU16};

pub fn try_assign_inflate(r: &mut RectU16) -> Option<()> {
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

pub fn try_inflate(r: &RectU16) -> Option<RectU16> {
    let min_x = r.min.x.checked_sub(1)?;
    let min_y = r.min.y.checked_sub(1)?;
    let max_x = r.max.x.checked_add(1)?;
    let max_y = r.max.y.checked_add(1)?;
    Some(RectU16 { min: PointU16 { x: min_x, y: min_y }, max: PointU16 { x: max_x, y: max_y } })
}

pub fn assign_inflate(r: &mut RectU16) {
    try_assign_inflate(r).unwrap()
}

pub fn inflate(r: &RectU16) -> RectU16 {
    try_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{RectU16, assign_inflate, inflate, try_assign_inflate, try_inflate};

    #[test]
    fn try_assign_inflate_min_bounds() {
        let mut r = RectU16::of(7, 3, 4, 13);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(6, 2, 5, 14));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(5, 1, 6, 15));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(4, 0, 7, 16));
    }

    #[test]
    fn try_assign_inflate_max_bounds() {
        let mut r = RectU16::of(200, 230, u16::MAX - 5, u16::MAX - 3);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(199, 229, u16::MAX - 4, u16::MAX - 2));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(198, 228, u16::MAX - 3, u16::MAX - 1));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(197, 227, u16::MAX - 2, u16::MAX));
    }

    #[test]
    fn try_assign_inflate_to_bounds() {
        let mut r = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::largest());
    }

    #[test]
    fn try_assign_inflate_width_to_bounds() {
        let mut r_min = RectU16::of(1, 10, 100, 100);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectU16::of(0, 9, 101, 101));

        let mut r_max = RectU16::of(10, 10, u16::MAX - 1, 100);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectU16::of(9, 9, u16::MAX, 101));
    }

    #[test]
    fn try_assign_inflate_height_to_bounds() {
        let mut r_min = RectU16::of(10, 1, 100, 100);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectU16::of(9, 0, 101, 101));

        let mut r_max = RectU16::of(10, 10, 100, u16::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectU16::of(9, 9, 101, u16::MAX));
    }

    #[test]
    fn try_assign_inflate_out_of_bounds() {
        let mut r_min_x = RectU16::of(0, 10, u16::MAX, 100);
        assert_eq!(try_assign_inflate(&mut r_min_x), None);
        assert_eq!(r_min_x, RectU16::of(0, 10, u16::MAX, 100));

        let mut r_max_x = RectU16::of(10, 10, u16::MAX, 100);
        assert_eq!(try_assign_inflate(&mut r_max_x), None);
        assert_eq!(r_max_x, RectU16::of(10, 10, u16::MAX, 100));

        let mut r_min_y = RectU16::of(10, 0, 100, 100);
        assert_eq!(try_assign_inflate(&mut r_min_y), None);
        assert_eq!(r_min_y, RectU16::of(10, 0, 100, 100));

        let mut r_max_y = RectU16::of(10, 10, 100, u16::MAX);
        assert_eq!(try_assign_inflate(&mut r_max_y), None);
        assert_eq!(r_max_y, RectU16::of(10, 10, 100, u16::MAX));

        let mut r_min = RectU16::of(0, 0, 10, 10);
        assert_eq!(try_assign_inflate(&mut r_min), None);
        assert_eq!(r_min, RectU16::of(0, 0, 10, 10));

        let mut r_max = RectU16::of(10, 10, u16::MAX, u16::MAX);
        assert_eq!(try_assign_inflate(&mut r_max), None);
        assert_eq!(r_max, RectU16::of(10, 10, u16::MAX, u16::MAX));
    }

    #[test]
    fn try_assign_inflate_limits_out_of_bounds() {
        let mut r = RectU16::largest();
        assert_eq!(try_assign_inflate(&mut r), None);
        assert_eq!(r, RectU16::largest());
    }

    #[test]
    fn try_inflate_min_bounds() {
        assert_eq!(try_inflate(&RectU16::of(7, 3, 4, 13)), Some(RectU16::of(6, 2, 5, 14)));
        assert_eq!(try_inflate(&RectU16::of(6, 2, 5, 14)), Some(RectU16::of(5, 1, 6, 15)));
        assert_eq!(try_inflate(&RectU16::of(5, 1, 6, 15)), Some(RectU16::of(4, 0, 7, 16)));
    }

    #[test]
    fn try_inflate_max_bounds() {
        assert_eq!(try_inflate(&RectU16::of(200, 230, u16::MAX - 5, u16::MAX - 3)), Some(RectU16::of(199, 229, u16::MAX - 4, u16::MAX - 2)));
        assert_eq!(try_inflate(&RectU16::of(199, 229, u16::MAX - 4, u16::MAX - 2)), Some(RectU16::of(198, 228, u16::MAX - 3, u16::MAX - 1)));
        assert_eq!(try_inflate(&RectU16::of(198, 228, u16::MAX - 3, u16::MAX - 1)), Some(RectU16::of(197, 227, u16::MAX - 2, u16::MAX)));
    }

    #[test]
    fn try_inflate_to_bounds() {
        assert_eq!(try_inflate(&RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1)), Some(RectU16::largest()));
    }

    #[test]
    fn try_inflate_width_to_bounds() {
        assert_eq!(try_inflate(&RectU16::of(1, 10, 100, 100)), Some(RectU16::of(0, 9, 101, 101)));
        assert_eq!(try_inflate(&RectU16::of(10, 10, u16::MAX - 1, 100)), Some(RectU16::of(9, 9, u16::MAX, 101)));
    }

    #[test]
    fn try_inflate_height_to_bounds() {
        assert_eq!(try_inflate(&RectU16::of(10, 1, 100, 100)), Some(RectU16::of(9, 0, 101, 101)));
        assert_eq!(try_inflate(&RectU16::of(10, 10, 100, u16::MAX - 1)), Some(RectU16::of(9, 9, 101, u16::MAX)));
    }

    #[test]
    fn try_inflate_out_of_bounds() {
        assert_eq!(try_inflate(&RectU16::of(0, 10, u16::MAX, 100)), None);
        assert_eq!(try_inflate(&RectU16::of(10, 10, u16::MAX, 100)), None);
        assert_eq!(try_inflate(&RectU16::of(10, 0, 100, 100)), None);
        assert_eq!(try_inflate(&RectU16::of(10, 10, 100, u16::MAX)), None);
        assert_eq!(try_inflate(&RectU16::of(0, 0, 10, 10)), None);
        assert_eq!(try_inflate(&RectU16::of(10, 10, u16::MAX, u16::MAX)), None);
    }

    #[test]
    fn try_inflate_limits_out_of_bounds() {
        assert_eq!(try_inflate(&RectU16::largest()), None);
    }

    #[test]
    fn assign_inflate_min_bounds() {
        let mut r = RectU16::of(7, 3, 4, 13);
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(6, 2, 5, 14));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(5, 1, 6, 15));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(4, 0, 7, 16));
    }

    #[test]
    fn assign_inflate_max_bounds() {
        let mut r = RectU16::of(200, 230, u16::MAX - 5, u16::MAX - 3);
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(199, 229, u16::MAX - 4, u16::MAX - 2));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(198, 228, u16::MAX - 3, u16::MAX - 1));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(197, 227, u16::MAX - 2, u16::MAX));
    }

    #[test]
    fn inflate_min_bounds() {
        assert_eq!(inflate(&RectU16::of(7, 3, 4, 13)), RectU16::of(6, 2, 5, 14));
        assert_eq!(inflate(&RectU16::of(6, 2, 5, 14)), RectU16::of(5, 1, 6, 15));
        assert_eq!(inflate(&RectU16::of(5, 1, 6, 15)), RectU16::of(4, 0, 7, 16));
    }

    #[test]
    fn inflate_max_bounds() {
        assert_eq!(inflate(&RectU16::of(200, 230, u16::MAX - 5, u16::MAX - 3)), RectU16::of(199, 229, u16::MAX - 4, u16::MAX - 2));
        assert_eq!(inflate(&RectU16::of(199, 229, u16::MAX - 4, u16::MAX - 2)), RectU16::of(198, 228, u16::MAX - 3, u16::MAX - 1));
        assert_eq!(inflate(&RectU16::of(198, 228, u16::MAX - 3, u16::MAX - 1)), RectU16::of(197, 227, u16::MAX - 2, u16::MAX));
    }
}

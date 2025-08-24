use crate::cartesian::{point::point_i8::PointI8, rect::rect_i8::RectI8};

pub fn try_assign_inflate(r: &mut RectI8) -> Option<()> {
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

pub fn try_inflate(r: &RectI8) -> Option<RectI8> {
    let min_x = r.min.x.checked_sub(1)?;
    let min_y = r.min.y.checked_sub(1)?;
    let max_x = r.max.x.checked_add(1)?;
    let max_y = r.max.y.checked_add(1)?;
    Some(RectI8 { min: PointI8 { x: min_x, y: min_y }, max: PointI8 { x: max_x, y: max_y } })
}

pub fn assign_inflate(r: &mut RectI8) {
    try_assign_inflate(r).unwrap()
}

pub fn inflate(r: &RectI8) -> RectI8 {
    try_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{assign_inflate, inflate, try_assign_inflate, try_inflate};
    use crate::cartesian::rect::rect_i8::RectI8;

    #[test]
    fn try_assign_inflate_min_bounds() {
        let mut r = RectI8::of(i8::MIN + 7, i8::MIN + 3, i8::MIN + 9, i8::MIN + 13);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN + 6, i8::MIN + 2, i8::MIN + 10, i8::MIN + 14));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN + 5, i8::MIN + 1, i8::MIN + 11, i8::MIN + 15));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 12, i8::MIN + 16));
    }

    #[test]
    fn try_assign_inflate_max_bounds() {
        let mut r = RectI8::of(i8::MAX - 33, i8::MAX - 17, i8::MAX - 5, i8::MAX - 3);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX));
    }

    #[test]
    fn try_assign_inflate_to_bounds() {
        let mut r = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::largest());
    }

    #[test]
    fn try_assign_inflate_width_to_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 1, i8::MIN + 10, i8::MIN + 20, i8::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectI8::of(i8::MIN, i8::MIN + 9, i8::MIN + 21, i8::MIN + 21));

        let mut r_max = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 1, i8::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectI8::of(i8::MIN + 9, i8::MIN + 9, i8::MAX, i8::MIN + 21));
    }

    #[test]
    fn try_assign_inflate_height_to_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 10, i8::MIN + 1, i8::MIN + 20, i8::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectI8::of(i8::MIN + 9, i8::MIN, i8::MIN + 21, i8::MIN + 21));

        let mut r_max = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MIN + 20, i8::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectI8::of(i8::MIN + 9, i8::MIN + 9, i8::MIN + 21, i8::MAX));
    }

    #[test]
    fn try_assign_inflate_out_of_bounds() {
        let mut r_min_x = RectI8::of(i8::MIN, i8::MIN + 10, i8::MAX, i8::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_min_x), None);
        assert_eq!(r_min_x, RectI8::of(i8::MIN, i8::MIN + 10, i8::MAX, i8::MIN + 20));

        let mut r_max_x = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX, i8::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_max_x), None);
        assert_eq!(r_max_x, RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX, i8::MIN + 20));

        let mut r_min_y = RectI8::of(i8::MIN + 10, i8::MIN, i8::MIN + 20, i8::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_min_y), None);
        assert_eq!(r_min_y, RectI8::of(i8::MIN + 10, i8::MIN, i8::MIN + 20, i8::MIN + 20));

        let mut r_max_y = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MIN + 20, i8::MAX);
        assert_eq!(try_assign_inflate(&mut r_max_y), None);
        assert_eq!(r_max_y, RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MIN + 20, i8::MAX));

        let mut r_min = RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 10);
        assert_eq!(try_assign_inflate(&mut r_min), None);
        assert_eq!(r_min, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 10));

        let mut r_max = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX, i8::MAX);
        assert_eq!(try_assign_inflate(&mut r_max), None);
        assert_eq!(r_max, RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX, i8::MAX));
    }

    #[test]
    fn try_assign_inflate_limits_out_of_bounds() {
        let mut r = RectI8::largest();
        assert_eq!(try_assign_inflate(&mut r), None);
        assert_eq!(r, RectI8::largest());
    }

    #[test]
    fn try_inflate_min_bounds() {
        assert_eq!(
            try_inflate(&RectI8::of(i8::MIN + 7, i8::MIN + 3, i8::MIN + 9, i8::MIN + 13)),
            Some(RectI8::of(i8::MIN + 6, i8::MIN + 2, i8::MIN + 10, i8::MIN + 14))
        );
        assert_eq!(
            try_inflate(&RectI8::of(i8::MIN + 6, i8::MIN + 2, i8::MIN + 10, i8::MIN + 14)),
            Some(RectI8::of(i8::MIN + 5, i8::MIN + 1, i8::MIN + 11, i8::MIN + 15))
        );
        assert_eq!(
            try_inflate(&RectI8::of(i8::MIN + 5, i8::MIN + 1, i8::MIN + 11, i8::MIN + 15)),
            Some(RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 12, i8::MIN + 16))
        );
    }

    #[test]
    fn try_inflate_max_bounds() {
        assert_eq!(
            try_inflate(&RectI8::of(i8::MAX - 33, i8::MAX - 17, i8::MAX - 5, i8::MAX - 3)),
            Some(RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2))
        );
        assert_eq!(
            try_inflate(&RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2)),
            Some(RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1))
        );
        assert_eq!(
            try_inflate(&RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1)),
            Some(RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX))
        );
    }

    #[test]
    fn try_inflate_to_bounds() {
        assert_eq!(try_inflate(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1)), Some(RectI8::largest()));
    }

    #[test]
    fn try_inflate_width_to_bounds() {
        assert_eq!(
            try_inflate(&RectI8::of(i8::MIN + 1, i8::MIN + 10, i8::MIN + 20, i8::MIN + 20)),
            Some(RectI8::of(i8::MIN, i8::MIN + 9, i8::MIN + 21, i8::MIN + 21))
        );
        assert_eq!(
            try_inflate(&RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 1, i8::MIN + 20)),
            Some(RectI8::of(i8::MIN + 9, i8::MIN + 9, i8::MAX, i8::MIN + 21))
        );
    }

    #[test]
    fn try_inflate_height_to_bounds() {
        assert_eq!(
            try_inflate(&RectI8::of(i8::MIN + 10, i8::MIN + 1, i8::MIN + 20, i8::MIN + 20)),
            Some(RectI8::of(i8::MIN + 9, i8::MIN, i8::MIN + 21, i8::MIN + 21))
        );
        assert_eq!(
            try_inflate(&RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MIN + 20, i8::MAX - 1)),
            Some(RectI8::of(i8::MIN + 9, i8::MIN + 9, i8::MIN + 21, i8::MAX))
        );
    }

    #[test]
    fn try_inflate_out_of_bounds() {
        assert_eq!(try_inflate(&RectI8::of(i8::MIN, i8::MIN + 10, i8::MAX, i8::MIN + 20)), None);
        assert_eq!(try_inflate(&RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX, i8::MIN + 20)), None);
        assert_eq!(try_inflate(&RectI8::of(i8::MIN + 10, i8::MIN, i8::MIN + 20, i8::MIN + 20)), None);
        assert_eq!(try_inflate(&RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MIN + 20, i8::MAX)), None);
        assert_eq!(try_inflate(&RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 10)), None);
        assert_eq!(try_inflate(&RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX, i8::MAX)), None);
    }

    #[test]
    fn try_inflate_limits_out_of_bounds() {
        assert_eq!(try_inflate(&RectI8::largest()), None);
    }

    #[test]
    fn assign_inflate_min_bounds() {
        let mut r = RectI8::of(i8::MIN + 7, i8::MIN + 3, i8::MIN + 9, i8::MIN + 13);
        assign_inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 6, i8::MIN + 2, i8::MIN + 10, i8::MIN + 14));
        assign_inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 5, i8::MIN + 1, i8::MIN + 11, i8::MIN + 15));
        assign_inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 12, i8::MIN + 16));
    }

    #[test]
    fn assign_inflate_max_bounds() {
        let mut r = RectI8::of(i8::MAX - 33, i8::MAX - 17, i8::MAX - 5, i8::MAX - 3);
        assign_inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2));
        assign_inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1));
        assign_inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX));
    }

    #[test]
    fn inflate_min_bounds() {
        assert_eq!(
            inflate(&RectI8::of(i8::MIN + 7, i8::MIN + 3, i8::MIN + 9, i8::MIN + 13)),
            RectI8::of(i8::MIN + 6, i8::MIN + 2, i8::MIN + 10, i8::MIN + 14)
        );
        assert_eq!(
            inflate(&RectI8::of(i8::MIN + 6, i8::MIN + 2, i8::MIN + 10, i8::MIN + 14)),
            RectI8::of(i8::MIN + 5, i8::MIN + 1, i8::MIN + 11, i8::MIN + 15)
        );
        assert_eq!(
            inflate(&RectI8::of(i8::MIN + 5, i8::MIN + 1, i8::MIN + 11, i8::MIN + 15)),
            RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 12, i8::MIN + 16)
        );
    }

    #[test]
    fn inflate_max_bounds() {
        assert_eq!(
            inflate(&RectI8::of(i8::MAX - 33, i8::MAX - 17, i8::MAX - 5, i8::MAX - 3)),
            RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2)
        );
        assert_eq!(
            inflate(&RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2)),
            RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1)
        );
        assert_eq!(
            inflate(&RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1)),
            RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX)
        );
    }
}

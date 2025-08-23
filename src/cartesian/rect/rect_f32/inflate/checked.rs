use crate::cartesian::{
    point::point_f32::{MAX, MIN, PointF32},
    rect::rect_f32::RectF32,
};

pub fn try_assign_inflate(r: &mut RectF32) -> Option<()> {
    let is_min_x = r.min.x == MIN;
    let is_min_y = r.min.y == MIN;
    let is_max_x = r.max.x == MAX;
    let is_max_y = r.max.y == MAX;
    if is_min_x || is_max_x || is_min_y || is_max_y {
        return None;
    }
    r.min.x -= 1.0;
    r.min.y -= 1.0;
    r.max.x += 1.0;
    r.max.y += 1.0;
    Some(())
}

pub fn try_inflate(r: &RectF32) -> Option<RectF32> {
    let is_min_x = r.min.x == MIN;
    let is_min_y = r.min.y == MIN;
    let is_max_x = r.max.x == MAX;
    let is_max_y = r.max.y == MAX;
    if is_min_x || is_max_x || is_min_y || is_max_y {
        return None;
    }
    let min_x = r.min.x - 1.0;
    let min_y = r.min.y - 1.0;
    let max_x = r.max.x + 1.0;
    let max_y = r.max.y + 1.0;
    Some(RectF32 { min: PointF32 { x: min_x, y: min_y }, max: PointF32 { x: max_x, y: max_y } })
}

pub fn assign_inflate(r: &mut RectF32) {
    try_assign_inflate(r).unwrap()
}

pub fn inflate(r: &RectF32) -> RectF32 {
    try_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{
        point::point_f32::{MAX, MIN},
        rect::rect_f32::RectF32,
    };

    use super::{assign_inflate, inflate, try_assign_inflate, try_inflate};

    #[test]
    fn try_assign_inflate_min_bounds() {
        let mut r = RectF32::of(MIN + 7.0, MIN + 3.0, MIN + 9.0, MIN + 13.0);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectF32::of(MIN + 6.0, MIN + 2.0, MIN + 10.0, MIN + 14.0));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectF32::of(MIN + 5.0, MIN + 1.0, MIN + 11.0, MIN + 15.0));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectF32::of(MIN + 4.0, MIN, MIN + 12.0, MIN + 16.0));
    }

    #[test]
    fn try_assign_inflate_max_bounds() {
        let mut r = RectF32::of(MAX - 33.0, MAX - 17.0, MAX - 5.0, MAX - 3.0);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectF32::of(MAX - 34.0, MAX - 18.0, MAX - 4.0, MAX - 2.0));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectF32::of(MAX - 35.0, MAX - 19.0, MAX - 3.0, MAX - 1.0));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectF32::of(MAX - 36.0, MAX - 20.0, MAX - 2.0, MAX));
    }

    #[test]
    fn try_assign_inflate_to_bounds() {
        let mut r = RectF32::of(MIN + 1.0, MIN + 1.0, MAX - 1.0, MAX - 1.0);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectF32::largest());
    }

    #[test]
    fn try_assign_inflate_width_to_bounds() {
        let mut r_min = RectF32::of(MIN + 1.0, MIN + 10.0, MIN + 20.0, MIN + 20.0);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectF32::of(MIN, MIN + 9.0, MIN + 21.0, MIN + 21.0));

        let mut r_max = RectF32::of(MIN + 10.0, MIN + 10.0, MAX - 1.0, MIN + 20.0);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectF32::of(MIN + 9.0, MIN + 9.0, MAX, MIN + 21.0));
    }

    #[test]
    fn try_assign_inflate_height_to_bounds() {
        let mut r_min = RectF32::of(MIN + 10.0, MIN + 1.0, MIN + 20.0, MIN + 20.0);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectF32::of(MIN + 9.0, MIN, MIN + 21.0, MIN + 21.0));

        let mut r_max = RectF32::of(MIN + 10.0, MIN + 10.0, MIN + 20.0, MAX - 1.0);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectF32::of(MIN + 9.0, MIN + 9.0, MIN + 21.0, MAX));
    }

    #[test]
    fn try_assign_inflate_out_of_bounds() {
        let mut r_min_x = RectF32::of(MIN, MIN + 10.0, MAX, MIN + 20.0);
        assert_eq!(try_assign_inflate(&mut r_min_x), None);
        assert_eq!(r_min_x, RectF32::of(MIN, MIN + 10.0, MAX, MIN + 20.0));

        let mut r_max_x = RectF32::of(MIN + 10.0, MIN + 10.0, MAX, MIN + 20.0);
        assert_eq!(try_assign_inflate(&mut r_max_x), None);
        assert_eq!(r_max_x, RectF32::of(MIN + 10.0, MIN + 10.0, MAX, MIN + 20.0));

        let mut r_min_y = RectF32::of(MIN + 10.0, MIN, MIN + 20.0, MIN + 20.0);
        assert_eq!(try_assign_inflate(&mut r_min_y), None);
        assert_eq!(r_min_y, RectF32::of(MIN + 10.0, MIN, MIN + 20.0, MIN + 20.0));

        let mut r_max_y = RectF32::of(MIN + 10.0, MIN + 10.0, MIN + 20.0, MAX);
        assert_eq!(try_assign_inflate(&mut r_max_y), None);
        assert_eq!(r_max_y, RectF32::of(MIN + 10.0, MIN + 10.0, MIN + 20.0, MAX));

        let mut r_min = RectF32::of(MIN, MIN, MIN + 10.0, MIN + 10.0);
        assert_eq!(try_assign_inflate(&mut r_min), None);
        assert_eq!(r_min, RectF32::of(MIN, MIN, MIN + 10.0, MIN + 10.0));

        let mut r_max = RectF32::of(MIN + 10.0, MIN + 10.0, MAX, MAX);
        assert_eq!(try_assign_inflate(&mut r_max), None);
        assert_eq!(r_max, RectF32::of(MIN + 10.0, MIN + 10.0, MAX, MAX));
    }

    #[test]
    fn try_assign_inflate_limits_out_of_bounds() {
        let mut r = RectF32::largest();
        assert_eq!(try_assign_inflate(&mut r), None);
        assert_eq!(r, RectF32::largest());
    }

    #[test]
    fn try_inflate_min_bounds() {
        assert_eq!(
            try_inflate(&RectF32::of(MIN + 7.0, MIN + 3.0, MIN + 9.0, MIN + 13.0)),
            Some(RectF32::of(MIN + 6.0, MIN + 2.0, MIN + 10.0, MIN + 14.0))
        );
        assert_eq!(
            try_inflate(&RectF32::of(MIN + 6.0, MIN + 2.0, MIN + 10.0, MIN + 14.0)),
            Some(RectF32::of(MIN + 5.0, MIN + 1.0, MIN + 11.0, MIN + 15.0))
        );
        assert_eq!(
            try_inflate(&RectF32::of(MIN + 5.0, MIN + 1.0, MIN + 11.0, MIN + 15.0)),
            Some(RectF32::of(MIN + 4.0, MIN, MIN + 12.0, MIN + 16.0))
        );
    }

    #[test]
    fn try_inflate_max_bounds() {
        assert_eq!(
            try_inflate(&RectF32::of(MAX - 33.0, MAX - 17.0, MAX - 5.0, MAX - 3.0)),
            Some(RectF32::of(MAX - 34.0, MAX - 18.0, MAX - 4.0, MAX - 2.0))
        );
        assert_eq!(
            try_inflate(&RectF32::of(MAX - 34.0, MAX - 18.0, MAX - 4.0, MAX - 2.0)),
            Some(RectF32::of(MAX - 35.0, MAX - 19.0, MAX - 3.0, MAX - 1.0))
        );
        assert_eq!(
            try_inflate(&RectF32::of(MAX - 35.0, MAX - 19.0, MAX - 3.0, MAX - 1.0)),
            Some(RectF32::of(MAX - 36.0, MAX - 20.0, MAX - 2.0, MAX))
        );
    }

    #[test]
    fn try_inflate_to_bounds() {
        assert_eq!(try_inflate(&RectF32::of(MIN + 1.0, MIN + 1.0, MAX - 1.0, MAX - 1.0)), Some(RectF32::largest()));
    }

    #[test]
    fn try_inflate_width_to_bounds() {
        assert_eq!(
            try_inflate(&RectF32::of(MIN + 1.0, MIN + 10.0, MIN + 20.0, MIN + 20.0)),
            Some(RectF32::of(MIN, MIN + 9.0, MIN + 21.0, MIN + 21.0))
        );
        assert_eq!(
            try_inflate(&RectF32::of(MIN + 10.0, MIN + 10.0, MAX - 1.0, MIN + 20.0)),
            Some(RectF32::of(MIN + 9.0, MIN + 9.0, MAX, MIN + 21.0))
        );
    }

    #[test]
    fn try_inflate_height_to_bounds() {
        assert_eq!(
            try_inflate(&RectF32::of(MIN + 10.0, MIN + 1.0, MIN + 20.0, MIN + 20.0)),
            Some(RectF32::of(MIN + 9.0, MIN, MIN + 21.0, MIN + 21.0))
        );
        assert_eq!(
            try_inflate(&RectF32::of(MIN + 10.0, MIN + 10.0, MIN + 20.0, MAX - 1.0)),
            Some(RectF32::of(MIN + 9.0, MIN + 9.0, MIN + 21.0, MAX))
        );
    }

    #[test]
    fn try_inflate_out_of_bounds() {
        assert_eq!(try_inflate(&RectF32::of(MIN, MIN + 10.0, MAX, MIN + 20.0)), None);
        assert_eq!(try_inflate(&RectF32::of(MIN + 10.0, MIN + 10.0, MAX, MIN + 20.0)), None);
        assert_eq!(try_inflate(&RectF32::of(MIN + 10.0, MIN, MIN + 20.0, MIN + 20.0)), None);
        assert_eq!(try_inflate(&RectF32::of(MIN + 10.0, MIN + 10.0, MIN + 20.0, MAX)), None);
        assert_eq!(try_inflate(&RectF32::of(MIN, MIN, MIN + 10.0, MIN + 10.0)), None);
        assert_eq!(try_inflate(&RectF32::of(MIN + 10.0, MIN + 10.0, MAX, MAX)), None);
    }

    #[test]
    fn try_inflate_limits_out_of_bounds() {
        assert_eq!(try_inflate(&RectF32::largest()), None);
    }

    #[test]
    fn assign_inflate_min_bounds() {
        let mut r = RectF32::of(MIN + 7.0, MIN + 3.0, MIN + 9.0, MIN + 13.0);
        assign_inflate(&mut r);
        assert_eq!(r, RectF32::of(MIN + 6.0, MIN + 2.0, MIN + 10.0, MIN + 14.0));
        assign_inflate(&mut r);
        assert_eq!(r, RectF32::of(MIN + 5.0, MIN + 1.0, MIN + 11.0, MIN + 15.0));
        assign_inflate(&mut r);
        assert_eq!(r, RectF32::of(MIN + 4.0, MIN, MIN + 12.0, MIN + 16.0));
    }

    #[test]
    fn assign_inflate_max_bounds() {
        let mut r = RectF32::of(MAX - 33.0, MAX - 17.0, MAX - 5.0, MAX - 3.0);
        assign_inflate(&mut r);
        assert_eq!(r, RectF32::of(MAX - 34.0, MAX - 18.0, MAX - 4.0, MAX - 2.0));
        assign_inflate(&mut r);
        assert_eq!(r, RectF32::of(MAX - 35.0, MAX - 19.0, MAX - 3.0, MAX - 1.0));
        assign_inflate(&mut r);
        assert_eq!(r, RectF32::of(MAX - 36.0, MAX - 20.0, MAX - 2.0, MAX));
    }

    #[test]
    fn inflate_min_bounds() {
        assert_eq!(inflate(&RectF32::of(MIN + 7.0, MIN + 3.0, MIN + 9.0, MIN + 13.0)), RectF32::of(MIN + 6.0, MIN + 2.0, MIN + 10.0, MIN + 14.0));
        assert_eq!(inflate(&RectF32::of(MIN + 6.0, MIN + 2.0, MIN + 10.0, MIN + 14.0)), RectF32::of(MIN + 5.0, MIN + 1.0, MIN + 11.0, MIN + 15.0));
        assert_eq!(inflate(&RectF32::of(MIN + 5.0, MIN + 1.0, MIN + 11.0, MIN + 15.0)), RectF32::of(MIN + 4.0, MIN, MIN + 12.0, MIN + 16.0));
    }

    #[test]
    fn inflate_max_bounds() {
        assert_eq!(inflate(&RectF32::of(MAX - 33.0, MAX - 17.0, MAX - 5.0, MAX - 3.0)), RectF32::of(MAX - 34.0, MAX - 18.0, MAX - 4.0, MAX - 2.0));
        assert_eq!(inflate(&RectF32::of(MAX - 34.0, MAX - 18.0, MAX - 4.0, MAX - 2.0)), RectF32::of(MAX - 35.0, MAX - 19.0, MAX - 3.0, MAX - 1.0));
        assert_eq!(inflate(&RectF32::of(MAX - 35.0, MAX - 19.0, MAX - 3.0, MAX - 1.0)), RectF32::of(MAX - 36.0, MAX - 20.0, MAX - 2.0, MAX));
    }
}

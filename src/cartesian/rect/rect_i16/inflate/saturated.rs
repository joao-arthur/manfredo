use crate::cartesian::{point::point_i16::PointI16, rect::rect_i16::RectI16};

pub fn try_saturating_inflate_assign(r: &mut RectI16) -> Option<()> {
    let is_min_x = r.min.x == i16::MIN;
    let is_min_y = r.min.y == i16::MIN;
    let is_max_x = r.max.x == i16::MAX;
    let is_max_y = r.max.y == i16::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - i16::from(is_min_x) + i16::from(is_max_x);
    let min_y_modifier = 1 - i16::from(is_min_y) + i16::from(is_max_y);
    let max_x_modifier = 1 + i16::from(is_min_x) - i16::from(is_max_x);
    let max_y_modifier = 1 + i16::from(is_min_y) - i16::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
    Some(())
}

pub fn try_saturating_inflate(r: &RectI16) -> Option<RectI16> {
    let is_min_x = r.min.x == i16::MIN;
    let is_min_y = r.min.y == i16::MIN;
    let is_max_x = r.max.x == i16::MAX;
    let is_max_y = r.max.y == i16::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - i16::from(is_min_x) + i16::from(is_max_x);
    let min_y_modifier = 1 - i16::from(is_min_y) + i16::from(is_max_y);
    let max_x_modifier = 1 + i16::from(is_min_x) - i16::from(is_max_x);
    let max_y_modifier = 1 + i16::from(is_min_y) - i16::from(is_max_y);
    let min_x = r.min.x.saturating_sub(min_x_modifier);
    let min_y = r.min.y.saturating_sub(min_y_modifier);
    let max_x = r.max.x.saturating_add(max_x_modifier);
    let max_y = r.max.y.saturating_add(max_y_modifier);
    Some(RectI16 { min: PointI16 { x: min_x, y: min_y }, max: PointI16 { x: max_x, y: max_y } })
}

pub fn saturating_inflate_assign(r: &mut RectI16) {
    try_saturating_inflate_assign(r).unwrap()
}

pub fn saturating_inflate(r: &RectI16) -> RectI16 {
    try_saturating_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{saturating_inflate_assign, saturating_inflate, try_saturating_inflate_assign, try_saturating_inflate};
    use crate::cartesian::rect::rect_i16::RectI16;

    #[test]
    fn try_saturating_inflate_assign_min_bounds() {
        let mut r = RectI16::of(i16::MIN + 7, i16::MIN + 2, i16::MIN + 17, i16::MIN + 13);
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN + 6, i16::MIN + 1, i16::MIN + 18, i16::MIN + 14));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN + 5, i16::MIN, i16::MIN + 19, i16::MIN + 15));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 20, i16::MIN + 17));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN + 3, i16::MIN, i16::MIN + 21, i16::MIN + 19));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 22, i16::MIN + 21));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN + 1, i16::MIN, i16::MIN + 23, i16::MIN + 23));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 24, i16::MIN + 25));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 26, i16::MIN + 27));
    }

    #[test]
    fn try_saturating_inflate_assign_max_bounds() {
        let mut r = RectI16::of(i16::MAX - 33, i16::MAX - 17, i16::MAX - 5, i16::MAX - 3);
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 37, i16::MAX - 22, i16::MAX - 1, i16::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 38, i16::MAX - 24, i16::MAX, i16::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 40, i16::MAX - 26, i16::MAX, i16::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 42, i16::MAX - 28, i16::MAX, i16::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 44, i16::MAX - 30, i16::MAX, i16::MAX));
    }

    #[test]
    fn try_saturating_inflate_assign_to_bounds() {
        let mut r_odd = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_odd), Some(()));
        assert_eq!(r_odd, RectI16::largest());

        let mut r_even_1 = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_even_1), Some(()));
        assert_eq!(r_even_1, RectI16::largest());

        let mut r_even_2 = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX);
        assert_eq!(try_saturating_inflate_assign(&mut r_even_2), Some(()));
        assert_eq!(r_even_2, RectI16::largest());
    }

    #[test]
    fn try_saturating_inflate_assign_width_to_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 1, i16::MIN + 10, i16::MIN + 20, i16::MIN + 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectI16::of(i16::MIN, i16::MIN + 9, i16::MIN + 21, i16::MIN + 21));

        let mut r_max = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 1, i16::MIN + 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectI16::of(i16::MIN + 9, i16::MIN + 9, i16::MAX, i16::MIN + 21));
    }

    #[test]
    fn try_saturating_inflate_assign_height_to_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 1, i16::MIN + 20, i16::MIN + 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectI16::of(i16::MIN + 9, i16::MIN, i16::MIN + 21, i16::MIN + 21));

        let mut r_max = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MIN + 20, i16::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectI16::of(i16::MIN + 9, i16::MIN + 9, i16::MIN + 21, i16::MAX));
    }

    #[test]
    fn try_saturating_inflate_assign_out_of_bounds() {
        let mut r = RectI16::largest();
        assert_eq!(try_saturating_inflate_assign(&mut r), None);
        assert_eq!(r, RectI16::largest());

        let mut r_x = RectI16::of(i16::MIN, i16::MIN + 10, i16::MAX, i16::MIN + 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_x), None);
        assert_eq!(r_x, RectI16::of(i16::MIN, i16::MIN + 10, i16::MAX, i16::MIN + 20));

        let mut r_y = RectI16::of(i16::MIN + 10, i16::MIN, i16::MIN + 20, i16::MAX);
        assert_eq!(try_saturating_inflate_assign(&mut r_y), None);
        assert_eq!(r_y, RectI16::of(i16::MIN + 10, i16::MIN, i16::MIN + 20, i16::MAX));
    }

    #[test]
    fn try_saturating_inflate_min_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN + 7, i16::MIN + 2, i16::MIN + 17, i16::MIN + 13)),
            Some(RectI16::of(i16::MIN + 6, i16::MIN + 1, i16::MIN + 18, i16::MIN + 14))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN + 6, i16::MIN + 1, i16::MIN + 18, i16::MIN + 14)),
            Some(RectI16::of(i16::MIN + 5, i16::MIN, i16::MIN + 19, i16::MIN + 15))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN + 5, i16::MIN, i16::MIN + 19, i16::MIN + 15)),
            Some(RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 20, i16::MIN + 17))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 20, i16::MIN + 17)),
            Some(RectI16::of(i16::MIN + 3, i16::MIN, i16::MIN + 21, i16::MIN + 19))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN + 3, i16::MIN, i16::MIN + 21, i16::MIN + 19)),
            Some(RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 22, i16::MIN + 21))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 22, i16::MIN + 21)),
            Some(RectI16::of(i16::MIN + 1, i16::MIN, i16::MIN + 23, i16::MIN + 23))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN + 1, i16::MIN, i16::MIN + 23, i16::MIN + 23)),
            Some(RectI16::of(i16::MIN, i16::MIN, i16::MIN + 24, i16::MIN + 25))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN, i16::MIN, i16::MIN + 24, i16::MIN + 25)),
            Some(RectI16::of(i16::MIN, i16::MIN, i16::MIN + 26, i16::MIN + 27))
        );
    }

    #[test]
    fn try_saturating_inflate_max_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MAX - 33, i16::MAX - 17, i16::MAX - 5, i16::MAX - 3)),
            Some(RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2)),
            Some(RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1)),
            Some(RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX)),
            Some(RectI16::of(i16::MAX - 37, i16::MAX - 22, i16::MAX - 1, i16::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MAX - 37, i16::MAX - 22, i16::MAX - 1, i16::MAX)),
            Some(RectI16::of(i16::MAX - 38, i16::MAX - 24, i16::MAX, i16::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MAX - 38, i16::MAX - 24, i16::MAX, i16::MAX)),
            Some(RectI16::of(i16::MAX - 40, i16::MAX - 26, i16::MAX, i16::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MAX - 40, i16::MAX - 26, i16::MAX, i16::MAX)),
            Some(RectI16::of(i16::MAX - 42, i16::MAX - 28, i16::MAX, i16::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MAX - 42, i16::MAX - 28, i16::MAX, i16::MAX)),
            Some(RectI16::of(i16::MAX - 44, i16::MAX - 30, i16::MAX, i16::MAX))
        );
    }

    #[test]
    fn try_saturating_inflate_to_bounds() {
        assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1)), Some(RectI16::largest()));
        assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1)), Some(RectI16::largest()));
        assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX)), Some(RectI16::largest()));
    }

    #[test]
    fn try_saturating_inflate_width_to_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN + 1, i16::MIN + 10, i16::MIN + 20, i16::MIN + 20)),
            Some(RectI16::of(i16::MIN, i16::MIN + 9, i16::MIN + 21, i16::MIN + 21))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 1, i16::MIN + 20)),
            Some(RectI16::of(i16::MIN + 9, i16::MIN + 9, i16::MAX, i16::MIN + 21))
        );
    }

    #[test]
    fn try_saturating_inflate_height_to_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN + 10, i16::MIN + 1, i16::MIN + 20, i16::MIN + 20)),
            Some(RectI16::of(i16::MIN + 9, i16::MIN, i16::MIN + 21, i16::MIN + 21))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MIN + 20, i16::MAX - 1)),
            Some(RectI16::of(i16::MIN + 9, i16::MIN + 9, i16::MIN + 21, i16::MAX))
        );
    }

    #[test]
    fn try_saturating_inflate_out_of_bounds() {
        assert_eq!(try_saturating_inflate(&RectI16::largest()), None);
        assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN, i16::MIN + 10, i16::MAX, i16::MIN + 20)), None);
        assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN + 10, i16::MIN, i16::MIN + 20, i16::MAX)), None);
    }

    #[test]
    fn saturating_inflate_assign_min_bounds() {
        let mut r = RectI16::of(i16::MIN + 7, i16::MIN + 2, i16::MIN + 17, i16::MIN + 13);
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 6, i16::MIN + 1, i16::MIN + 18, i16::MIN + 14));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 5, i16::MIN, i16::MIN + 19, i16::MIN + 15));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 20, i16::MIN + 17));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 3, i16::MIN, i16::MIN + 21, i16::MIN + 19));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 22, i16::MIN + 21));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 1, i16::MIN, i16::MIN + 23, i16::MIN + 23));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 24, i16::MIN + 25));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 26, i16::MIN + 27));
    }

    #[test]
    fn saturating_inflate_assign_max_bounds() {
        let mut r = RectI16::of(i16::MAX - 33, i16::MAX - 17, i16::MAX - 5, i16::MAX - 3);
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 37, i16::MAX - 22, i16::MAX - 1, i16::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 38, i16::MAX - 24, i16::MAX, i16::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 40, i16::MAX - 26, i16::MAX, i16::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 42, i16::MAX - 28, i16::MAX, i16::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 44, i16::MAX - 30, i16::MAX, i16::MAX));
    }

    #[test]
    fn saturating_inflate_min_bounds() {
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MIN + 7, i16::MIN + 2, i16::MIN + 17, i16::MIN + 13)),
            RectI16::of(i16::MIN + 6, i16::MIN + 1, i16::MIN + 18, i16::MIN + 14)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MIN + 6, i16::MIN + 1, i16::MIN + 18, i16::MIN + 14)),
            RectI16::of(i16::MIN + 5, i16::MIN, i16::MIN + 19, i16::MIN + 15)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MIN + 5, i16::MIN, i16::MIN + 19, i16::MIN + 15)),
            RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 20, i16::MIN + 17)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 20, i16::MIN + 17)),
            RectI16::of(i16::MIN + 3, i16::MIN, i16::MIN + 21, i16::MIN + 19)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MIN + 3, i16::MIN, i16::MIN + 21, i16::MIN + 19)),
            RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 22, i16::MIN + 21)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 22, i16::MIN + 21)),
            RectI16::of(i16::MIN + 1, i16::MIN, i16::MIN + 23, i16::MIN + 23)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MIN + 1, i16::MIN, i16::MIN + 23, i16::MIN + 23)),
            RectI16::of(i16::MIN, i16::MIN, i16::MIN + 24, i16::MIN + 25)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MIN, i16::MIN, i16::MIN + 24, i16::MIN + 25)),
            RectI16::of(i16::MIN, i16::MIN, i16::MIN + 26, i16::MIN + 27)
        );
    }

    #[test]
    fn saturating_inflate_max_bounds() {
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MAX - 33, i16::MAX - 17, i16::MAX - 5, i16::MAX - 3)),
            RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2)),
            RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1)),
            RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX)),
            RectI16::of(i16::MAX - 37, i16::MAX - 22, i16::MAX - 1, i16::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MAX - 37, i16::MAX - 22, i16::MAX - 1, i16::MAX)),
            RectI16::of(i16::MAX - 38, i16::MAX - 24, i16::MAX, i16::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MAX - 38, i16::MAX - 24, i16::MAX, i16::MAX)),
            RectI16::of(i16::MAX - 40, i16::MAX - 26, i16::MAX, i16::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MAX - 40, i16::MAX - 26, i16::MAX, i16::MAX)),
            RectI16::of(i16::MAX - 42, i16::MAX - 28, i16::MAX, i16::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MAX - 42, i16::MAX - 28, i16::MAX, i16::MAX)),
            RectI16::of(i16::MAX - 44, i16::MAX - 30, i16::MAX, i16::MAX)
        );
    }
}

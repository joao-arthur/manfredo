use crate::cartesian::{
    point::point_u8,
    rect::rect_u8::{RectU8, delta_x, delta_y},
};

pub fn assign_inflate(r: &mut RectU8) -> Option<()> {
    let is_min_x = r.min.x == 0;
    let is_min_y = r.min.y == 0;
    let is_max_x = r.max.x == u8::MAX;
    let is_max_y = r.max.y == u8::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - u8::from(is_min_x) + u8::from(is_max_x);
    let min_y_modifier = 1 - u8::from(is_min_y) + u8::from(is_max_y);
    let max_x_modifier = 1 + u8::from(is_min_x) - u8::from(is_max_x);
    let max_y_modifier = 1 + u8::from(is_min_y) - u8::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
    Some(())
}

// inflate_to_bounds (min, max)
// inflate_to_bounds_height (min, max)
// inflate_to_bounds_width (min, max)
// inflate_out_of_bounds (min, max)
// inflate_out_of_bounds_height (min, max)
// inflate_out_of_bounds_width (min, max)

pub fn inflate(r: &RectU8) -> Option<RectU8> {
    let is_min_x = r.min.x == 0;
    let is_min_y = r.min.y == 0;
    let is_max_x = r.max.x == u8::MAX;
    let is_max_y = r.max.y == u8::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - u8::from(is_min_x) + u8::from(is_max_x);
    let min_y_modifier = 1 - u8::from(is_min_y) + u8::from(is_max_y);
    let max_x_modifier = 1 + u8::from(is_min_x) - u8::from(is_max_x);
    let max_y_modifier = 1 + u8::from(is_min_y) - u8::from(is_max_y);
    Some(RectU8 {
        min: point_u8::PointU8 { x: r.min.x.saturating_sub(min_x_modifier), y: r.min.y.saturating_sub(min_y_modifier) },
        max: point_u8::PointU8 { x: r.max.x.saturating_add(max_x_modifier), y: r.max.y.saturating_add(max_y_modifier) },
    })
}

pub fn assign_resize(r: &mut RectU8, size: u8) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_x = i16::from(delta_x(r)) + 1 - i16::from(size);
    let diff_y = i16::from(delta_y(r)) + 1 - i16::from(size);
    let temp_min_x = i16::from(r.min.x) + diff_x / 2;
    let temp_min_y = i16::from(r.min.y) + diff_y / 2;
    let min_x = temp_min_x.clamp(0, i16::from(u8::MAX) - i16::from(size) + 1);
    let min_y = temp_min_y.clamp(0, i16::from(u8::MAX) - i16::from(size) + 1);
    r.min.x = min_x as u8;
    r.min.y = min_y as u8;
    r.max.x = (min_x + i16::from(size) - 1) as u8;
    r.max.y = (min_y + i16::from(size) - 1) as u8;
    Some(())
}

// resize_to_bounds (min, max, even, odd)
// resize_out_of_bounds (min, max, even, odd)
// resize_out_of_bounds_width (min, max, even, odd)
// resize_out_of_bounds_height (min, max, even, odd)
// resize_limits_out_of_bounds (min, max, even, odd)

pub fn resize(r: &mut RectU8, size: u8) -> Option<RectU8> {
    if size < 3 {
        return None;
    }
    let diff_x = i16::from(delta_x(r)) + 1 - i16::from(size);
    let diff_y = i16::from(delta_y(r)) + 1 - i16::from(size);
    let temp_min_x = i16::from(r.min.x) + diff_x / 2;
    let temp_min_y = i16::from(r.min.y) + diff_y / 2;
    let clamped_min_x = temp_min_x.clamp(0, i16::from(u8::MAX) - i16::from(size) + 1);
    let clamped_min_y = temp_min_y.clamp(0, i16::from(u8::MAX) - i16::from(size) + 1);
    let min_x = clamped_min_x as u8;
    let min_y = clamped_min_y as u8;
    let max_x = (clamped_min_x + i16::from(size) - 1) as u8;
    let max_y = (clamped_min_y + i16::from(size) - 1) as u8;
    Some(RectU8 { min: point_u8::PointU8 { x: min_x, y: min_y }, max: point_u8::PointU8 { x: max_x, y: max_y } })
}

#[cfg(test)]
mod tests {
    use super::{RectU8, assign_inflate, assign_resize, inflate, resize};

    #[test]
    fn assign_inflate_min_bounds() {
        let mut r = RectU8::of(7, 2, 4, 13);
        assert_eq!(assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(6, 1, 5, 14));
        assert_eq!(assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(5, 0, 6, 15));
        assert_eq!(assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(4, 0, 7, 17));
        assert_eq!(assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(3, 0, 8, 19));
        assert_eq!(assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(2, 0, 9, 21));
        assert_eq!(assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(1, 0, 10, 23));
        assert_eq!(assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(0, 0, 11, 25));
        assert_eq!(assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(0, 0, 13, 27));
    }

    #[test]
    fn assign_inflate_max_bounds() {
        let mut r = RectU8::of(200, 230, u8::MAX - 5, u8::MAX - 3);
        assert_eq!(assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(199, 229, u8::MAX - 4, u8::MAX - 2));
        assert_eq!(assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(198, 228, u8::MAX - 3, u8::MAX - 1));
        assert_eq!(assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(197, 227, u8::MAX - 2, u8::MAX));
        assert_eq!(assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(196, 225, u8::MAX - 1, u8::MAX));
        assert_eq!(assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(195, 223, u8::MAX, u8::MAX));
        assert_eq!(assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(193, 221, u8::MAX, u8::MAX));
        assert_eq!(assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(191, 219, u8::MAX, u8::MAX));
        assert_eq!(assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(189, 217, u8::MAX, u8::MAX));
    }

    #[test]
    fn assign_inflate_to_bounds() {
        let mut r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert_eq!(assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU8::of(0, 0, u8::MAX, u8::MAX));
    }

    #[test]
    fn assign_inflate_width_to_bounds() {
        let mut r_min = RectU8::of(1, 10, 100, 100);
        assert_eq!(assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectU8::of(0, 9, 101, 101));

        let mut r_max = RectU8::of(10, 10, u8::MAX - 1, 100);
        assert_eq!(assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectU8::of(9, 9, u8::MAX, 101));
    }

    #[test]
    fn assign_inflate_height_to_bounds() {
        let mut r_min = RectU8::of(10, 1, 100, 100);
        assert_eq!(assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectU8::of(9, 0, 101, 101));

        let mut r_max = RectU8::of(10, 10, 100, u8::MAX - 1);
        assert_eq!(assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectU8::of(9, 9, 101, u8::MAX));
    }

    #[test]
    fn assign_inflate_height_out_of_bounds() {
        let mut r_width = RectU8::of(1, 10, u8::MAX - 1, 100);
        assert_eq!(assign_inflate(&mut r_width), Some(()));
        assert_eq!(r_width, RectU8::of(9, 9, u8::MAX, 101));

        let mut r_height = RectU8::of(10, 1, 100, 100);
        assert_eq!(assign_inflate(&mut r_height), Some(()));
        assert_eq!(r_height, RectU8::of(9, 0, 101, 101));
    }

























    #[test]
    fn resize_odd() {
        let mut r = RectU8::of(5, 5, 15, 15);
        assign_resize(&mut r, 11);
        assert_eq!(r, RectU8::of(5, 5, 15, 15));
        assign_resize(&mut r, 9);
        assert_eq!(r, RectU8::of(6, 6, 14, 14));
        assign_resize(&mut r, 7);
        assert_eq!(r, RectU8::of(7, 7, 13, 13));
        assign_resize(&mut r, 5);
        assert_eq!(r, RectU8::of(8, 8, 12, 12));
        assign_resize(&mut r, 3);
        assert_eq!(r, RectU8::of(9, 9, 11, 11));
        assign_resize(&mut r, 1);
        assert_eq!(r, RectU8::of(9, 9, 11, 11));
        assign_resize(&mut r, 3);
        assert_eq!(r, RectU8::of(9, 9, 11, 11));
        assign_resize(&mut r, 9);
        assert_eq!(r, RectU8::of(6, 6, 14, 14));
    }

    #[test]
    fn resize_even() {
        let mut r = RectU8::of(5, 5, 14, 14);
        assign_resize(&mut r, 10);
        assert_eq!(r, RectU8::of(5, 5, 14, 14));
        assign_resize(&mut r, 8);
        assert_eq!(r, RectU8::of(6, 6, 13, 13));
        assign_resize(&mut r, 6);
        assert_eq!(r, RectU8::of(7, 7, 12, 12));
        assign_resize(&mut r, 4);
        assert_eq!(r, RectU8::of(8, 8, 11, 11));
        assign_resize(&mut r, 2);
        assert_eq!(r, RectU8::of(8, 8, 11, 11));
        assign_resize(&mut r, 4);
        assert_eq!(r, RectU8::of(8, 8, 11, 11));
        assign_resize(&mut r, 8);
        assert_eq!(r, RectU8::of(6, 6, 13, 13));
    }

    #[test]
    fn resize_odd_rect_limits_out_of_bounds() {
        let mut r_min = RectU8::of(0, 0, 2, 2);
        assign_resize(&mut r_min, u8::MAX);
        assert_eq!(r_min, RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1));

        let mut r_max = RectU8::of(u8::MAX - 2, u8::MAX - 2, u8::MAX, u8::MAX);
        assign_resize(&mut r_max, u8::MAX);
        assert_eq!(r_max, RectU8::of(1, 1, u8::MAX, u8::MAX));
    }

    #[test]
    fn resize_even_small_rect_limits_out_of_bounds() {
        let mut r = RectU8::of(0, 0, 3, 3);
        assign_resize(&mut r, u8::MAX - 1);
        assert_eq!(r, RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 2));

        let mut r = RectU8::of(u8::MAX - 3, u8::MAX - 3, u8::MAX, u8::MAX);
        assign_resize(&mut r, u8::MAX - 1);
        assert_eq!(r, RectU8::of(2, 2, u8::MAX, u8::MAX));
    }
}

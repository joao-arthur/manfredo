use crate::cartesian::{point::point_u8::PointU8, rect::rect_u8::{delta_x, delta_y, RectU8}};

pub fn try_assign_deflate(r: &mut RectU8) -> Option<()> {
    if delta_x(r) < 3 || delta_y(r) < 3 {
        return None;
    }
    r.min.x += 1;
    r.min.y += 1;
    r.max.x -= 1;
    r.max.y -= 1;
    Some(())
}

pub fn assign_deflate(r: &mut RectU8) {
    try_assign_deflate(r).unwrap()
}

pub fn try_deflate(r: &mut RectU8) -> Option<RectU8> {
    if delta_x(r) < 3 || delta_y(r) < 3 {
        return None;
    }
    let min_x = r.min.x + 1;
    let min_y = r.min.y + 1;
    let max_x = r.max.x - 1;
    let max_y = r.max.y - 1;
    Some(RectU8 { min: PointU8 { x: min_x, y: min_y }, max: PointU8 { x: max_x, y: max_y } })
}

pub fn deflate(r: &mut RectU8) -> RectU8 {
    try_deflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_u8::RectU8;

    use super::try_assign_deflate;

    #[test]
    fn try_assign_deflate_odd() {
        let mut r = RectU8::of(0, 0, 9, 9);
        try_assign_deflate(&mut r);
        assert_eq!(r, RectU8::of(1, 1, 8, 8));
        try_assign_deflate(&mut r);
        assert_eq!(r, RectU8::of(2, 2, 7, 7));
        try_assign_deflate(&mut r);
        assert_eq!(r, RectU8::of(3, 3, 6, 6));
        try_assign_deflate(&mut r);
        assert_eq!(r, RectU8::of(4, 4, 5, 5));
        try_assign_deflate(&mut r);
        assert_eq!(r, RectU8::of(4, 4, 5, 5));
    }

    #[test]
    fn try_assign_deflate_even() {
        let mut r = RectU8::of(0, 0, 10, 10);
        try_assign_deflate(&mut r);
        assert_eq!(r, RectU8::of(1, 1, 9, 9));
        try_assign_deflate(&mut r);
        assert_eq!(r, RectU8::of(2, 2, 8, 8));
        try_assign_deflate(&mut r);
        assert_eq!(r, RectU8::of(3, 3, 7, 7));
        try_assign_deflate(&mut r);
        assert_eq!(r, RectU8::of(4, 4, 6, 6));
        try_assign_deflate(&mut r);
        assert_eq!(r, RectU8::of(4, 4, 6, 6));
    }

    #[test]
    fn try_assign_deflate_small_size() {
        let mut r = RectU8::of(10, 10, 100, 100);
        assert_eq!(try_assign_deflate(&mut r), None);
        assert_eq!(r, RectU8::of(10, 10, 100, 100));
    }
}

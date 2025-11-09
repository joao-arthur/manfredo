use super::checked_inflate_assign;
use crate::cartesian::{
    d1::point::point_f32::{MAX, MIN},
    d2::rect::rect_f32::Rect,
};

#[test]
fn min_bounds() {
    let mut r = Rect::new((MIN + 7.0, MIN + 3.0), (MIN + 9.0, MIN + 13.0));
    checked_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MIN + 6.0, MIN + 2.0), (MIN + 10.0, MIN + 14.0)));
    checked_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MIN + 5.0, MIN + 1.0), (MIN + 11.0, MIN + 15.0)));
    checked_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MIN + 4.0, MIN), (MIN + 12.0, MIN + 16.0)));
}

#[test]
fn max_bounds() {
    let mut r = Rect::new((MAX - 33.0, MAX - 17.0), (MAX - 5.0, MAX - 3.0));
    checked_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MAX - 34.0, MAX - 18.0), (MAX - 4.0, MAX - 2.0)));
    checked_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MAX - 35.0, MAX - 19.0), (MAX - 3.0, MAX - 1.0)));
    checked_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MAX - 36.0, MAX - 20.0), (MAX - 2.0, MAX)));
}

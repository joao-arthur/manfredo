use super::saturating_inflate_assign;
use crate::cartesian::{
    d1::point::point_f32::{MAX, MIN},
    d2::rect::rect_f32::Rect,
};

#[test]
fn min_bounds() {
    let mut r = Rect::new((MIN + 7.0, MIN + 2.0), (MIN + 17.0, MIN + 13.0));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MIN + 6.0, MIN + 1.0), (MIN + 18.0, MIN + 14.0)));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MIN + 5.0, MIN), (MIN + 19.0, MIN + 15.0)));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MIN + 4.0, MIN), (MIN + 20.0, MIN + 17.0)));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MIN + 3.0, MIN), (MIN + 21.0, MIN + 19.0)));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MIN + 2.0, MIN), (MIN + 22.0, MIN + 21.0)));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MIN + 1.0, MIN), (MIN + 23.0, MIN + 23.0)));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MIN, MIN), (MIN + 24.0, MIN + 25.0)));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MIN, MIN), (MIN + 26.0, MIN + 27.0)));
}

#[test]
fn max_bounds() {
    let mut r = Rect::new((MAX - 33.0, MAX - 17.0), (MAX - 5.0, MAX - 3.0));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MAX - 34.0, MAX - 18.0), (MAX - 4.0, MAX - 2.0)));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MAX - 35.0, MAX - 19.0), (MAX - 3.0, MAX - 1.0)));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MAX - 36.0, MAX - 20.0), (MAX - 2.0, MAX)));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MAX - 37.0, MAX - 22.0), (MAX - 1.0, MAX)));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MAX - 38.0, MAX - 24.0), (MAX, MAX)));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MAX - 40.0, MAX - 26.0), (MAX, MAX)));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MAX - 42.0, MAX - 28.0), (MAX, MAX)));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::new((MAX - 44.0, MAX - 30.0), (MAX, MAX)));
}

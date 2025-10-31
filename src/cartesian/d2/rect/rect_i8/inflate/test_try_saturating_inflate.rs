use super::try_saturating_inflate;
use crate::cartesian::{
    d1::point::point_i8::{MAX, MIN},
    d2::rect::rect_i8::Rect,
};

#[test]
fn min_bounds() {
    assert_eq!(try_saturating_inflate(&Rect::of((MIN + 7, MIN + 2), (MIN + 17, MIN + 13))), Some(Rect::of((MIN + 6, MIN + 1), (MIN + 18, MIN + 14))));
    assert_eq!(try_saturating_inflate(&Rect::of((MIN + 6, MIN + 1), (MIN + 18, MIN + 14))), Some(Rect::of((MIN + 5, MIN), (MIN + 19, MIN + 15))));
    assert_eq!(try_saturating_inflate(&Rect::of((MIN + 5, MIN), (MIN + 19, MIN + 15))), Some(Rect::of((MIN + 4, MIN), (MIN + 20, MIN + 17))));
    assert_eq!(try_saturating_inflate(&Rect::of((MIN + 4, MIN), (MIN + 20, MIN + 17))), Some(Rect::of((MIN + 3, MIN), (MIN + 21, MIN + 19))));
    assert_eq!(try_saturating_inflate(&Rect::of((MIN + 3, MIN), (MIN + 21, MIN + 19))), Some(Rect::of((MIN + 2, MIN), (MIN + 22, MIN + 21))));
    assert_eq!(try_saturating_inflate(&Rect::of((MIN + 2, MIN), (MIN + 22, MIN + 21))), Some(Rect::of((MIN + 1, MIN), (MIN + 23, MIN + 23))));
    assert_eq!(try_saturating_inflate(&Rect::of((MIN + 1, MIN), (MIN + 23, MIN + 23))), Some(Rect::of((MIN, MIN), (MIN + 24, MIN + 25))));
    assert_eq!(try_saturating_inflate(&Rect::of((MIN, MIN), (MIN + 24, MIN + 25))), Some(Rect::of((MIN, MIN), (MIN + 26, MIN + 27))));
}

#[test]
fn max_bounds() {
    assert_eq!(try_saturating_inflate(&Rect::of((MAX - 33, MAX - 17), (MAX - 5, MAX - 3))), Some(Rect::of((MAX - 34, MAX - 18), (MAX - 4, MAX - 2))));
    assert_eq!(try_saturating_inflate(&Rect::of((MAX - 34, MAX - 18), (MAX - 4, MAX - 2))), Some(Rect::of((MAX - 35, MAX - 19), (MAX - 3, MAX - 1))));
    assert_eq!(try_saturating_inflate(&Rect::of((MAX - 35, MAX - 19), (MAX - 3, MAX - 1))), Some(Rect::of((MAX - 36, MAX - 20), (MAX - 2, MAX))));
    assert_eq!(try_saturating_inflate(&Rect::of((MAX - 36, MAX - 20), (MAX - 2, MAX))), Some(Rect::of((MAX - 37, MAX - 22), (MAX - 1, MAX))));
    assert_eq!(try_saturating_inflate(&Rect::of((MAX - 37, MAX - 22), (MAX - 1, MAX))), Some(Rect::of((MAX - 38, MAX - 24), (MAX, MAX))));
    assert_eq!(try_saturating_inflate(&Rect::of((MAX - 38, MAX - 24), (MAX, MAX))), Some(Rect::of((MAX - 40, MAX - 26), (MAX, MAX))));
    assert_eq!(try_saturating_inflate(&Rect::of((MAX - 40, MAX - 26), (MAX, MAX))), Some(Rect::of((MAX - 42, MAX - 28), (MAX, MAX))));
    assert_eq!(try_saturating_inflate(&Rect::of((MAX - 42, MAX - 28), (MAX, MAX))), Some(Rect::of((MAX - 44, MAX - 30), (MAX, MAX))));
}

#[test]
fn to_bounds() {
    assert_eq!(try_saturating_inflate(&Rect::of((MIN + 1, MIN + 1), (MAX - 1, MAX - 1))), Some(Rect::largest()));
    assert_eq!(try_saturating_inflate(&Rect::of((MIN, MIN), (MAX - 1, MAX - 1))), Some(Rect::largest()));
    assert_eq!(try_saturating_inflate(&Rect::of((MIN + 1, MIN + 1), (MAX, MAX))), Some(Rect::largest()));
    assert_eq!(try_saturating_inflate(&Rect::of((MIN + 1, MIN + 10), (MAX - 10, MAX - 10))), Some(Rect::of((MIN, MIN + 9), (MAX - 9, MAX - 9))));
    assert_eq!(try_saturating_inflate(&Rect::of((MIN + 10, MIN + 1), (MAX - 10, MAX - 10))), Some(Rect::of((MIN + 9, MIN), (MAX - 9, MAX - 9))));
    assert_eq!(try_saturating_inflate(&Rect::of((MIN + 10, MIN + 10), (MAX - 1, MAX - 10))), Some(Rect::of((MIN + 9, MIN + 9), (MAX, MAX - 9))));
    assert_eq!(try_saturating_inflate(&Rect::of((MIN + 10, MIN + 10), (MAX - 10, MAX - 1))), Some(Rect::of((MIN + 9, MIN + 9), (MAX - 9, MAX))));
}

#[test]
fn out_of_bounds() {
    assert_eq!(try_saturating_inflate(&Rect::largest()), None);
    assert_eq!(try_saturating_inflate(&Rect::of((MIN, MIN + 10), (MAX, MAX - 10))), None);
    assert_eq!(try_saturating_inflate(&Rect::of((MIN + 10, MIN), (MAX - 10, MAX))), None);
}

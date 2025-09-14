use super::try_checked_add;
use crate::cartesian::d2::point::point_i16::Point;

const MIN: i16 = i16::MIN;
const MAX: i16 = i16::MAX;

#[test]
fn test() {
    assert_eq!(try_checked_add(&Point::of(0, 0), &Point::of(10, 13)), Some(Point::of(10, 13)));
    assert_eq!(try_checked_add(&Point::of(10, 10), &Point::of(-5, -3)), Some(Point::of(5, 7)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&Point::of(MIN + 2, MIN + 5), &Point::of(-2, -5)), Some(Point::min()));
    assert_eq!(try_checked_add(&Point::of(MAX - 2, MAX - 5), &Point::of(2, 5)), Some(Point::max()));
}

#[test]
fn out_of_bounds() {
    let p_min = Point::of(MIN + 2, MIN + 5);
    assert_eq!(try_checked_add(&p_min, &Point::of(-10, 0)), None);
    assert_eq!(try_checked_add(&p_min, &Point::of(0, -10)), None);
    assert_eq!(try_checked_add(&p_min, &Point::of(-10, -10)), None);

    let m_max = Point::of(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add(&m_max, &Point::of(10, 0)), None);
    assert_eq!(try_checked_add(&m_max, &Point::of(0, 10)), None);
    assert_eq!(try_checked_add(&m_max, &Point::of(10, 10)), None);
}

#[test]
fn limits_out_of_bounds() {
    let p_min = Point::of(MIN + 1, MIN + 1);
    assert_eq!(try_checked_add(&p_min, &Point::of(MIN, 0)), None);
    assert_eq!(try_checked_add(&p_min, &Point::of(0, MIN)), None);
    assert_eq!(try_checked_add(&p_min, &Point::min()), None);

    let p_max = Point::of(MAX - 1, MAX - 1);
    assert_eq!(try_checked_add(&p_max, &Point::of(MAX, 0)), None);
    assert_eq!(try_checked_add(&p_max, &Point::of(0, MAX)), None);
    assert_eq!(try_checked_add(&p_max, &Point::max()), None);
}

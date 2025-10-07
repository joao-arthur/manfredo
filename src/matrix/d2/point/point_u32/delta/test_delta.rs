use super::delta;
use crate::matrix::d2::point::point_u32::Point;

const MAX: u32 = u32::MAX;

#[test]
fn test_delta() {
    assert_eq!(delta(&Point::min(), &Point::min()), Point::min());
    assert_eq!(delta(&Point::min(), &Point::max()), Point::max());
}

#[test]
fn delta_min() {
    let p = Point::min();
    assert_eq!(delta(&p, &Point::min()), Point::min());
    assert_eq!(delta(&p, &Point::of(0, 1)), Point::of(0, 1));
    assert_eq!(delta(&p, &Point::of(0, 2)), Point::of(0, 2));

    assert_eq!(delta(&p, &Point::of(1, 0)), Point::of(1, 0));
    assert_eq!(delta(&p, &Point::of(1, 1)), Point::of(1, 1));
    assert_eq!(delta(&p, &Point::of(1, 2)), Point::of(1, 2));

    assert_eq!(delta(&p, &Point::of(2, 0)), Point::of(2, 0));
    assert_eq!(delta(&p, &Point::of(2, 1)), Point::of(2, 1));
    assert_eq!(delta(&p, &Point::of(2, 2)), Point::of(2, 2));
}

#[test]
fn delta_max() {
    let p = Point::of(MAX - 2, MAX - 2);
    assert_eq!(delta(&p, &Point::of(MAX - 2, MAX - 2)), Point::min());
    assert_eq!(delta(&p, &Point::of(MAX - 2, MAX - 1)), Point::of(0, 1));
    assert_eq!(delta(&p, &Point::of(MAX - 2, MAX)), Point::of(0, 2));

    assert_eq!(delta(&p, &Point::of(MAX - 1, MAX - 2)), Point::of(1, 0));
    assert_eq!(delta(&p, &Point::of(MAX - 1, MAX - 1)), Point::of(1, 1));
    assert_eq!(delta(&p, &Point::of(MAX - 1, MAX)), Point::of(1, 2));

    assert_eq!(delta(&p, &Point::of(MAX, MAX - 2)), Point::of(2, 0));
    assert_eq!(delta(&p, &Point::of(MAX, MAX - 1)), Point::of(2, 1));
    assert_eq!(delta(&p, &Point::max()), Point::of(2, 2));
}

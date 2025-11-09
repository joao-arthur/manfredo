use super::delta;
use crate::cartesian::d2::point::point_f64::Point;

#[test]
fn test_delta() {
    assert_eq!(delta(&Point::zero(), &Point::zero()), Point::zero());
    assert_eq!(delta(&Point::new(-4_503_599_627_370_496.0, -4_503_599_627_370_496.0), &Point::new(4_503_599_627_370_495.0, 4_503_599_627_370_495.0)), Point::max());
}

#[test]
fn delta_min() {
    let p = Point::new(-4_503_599_627_370_496.0, -4_503_599_627_370_496.0);
    assert_eq!(delta(&p, &Point::new(-4_503_599_627_370_496.0, -4_503_599_627_370_496.0)), Point::zero());
    assert_eq!(delta(&p, &Point::new(-4_503_599_627_370_496.0, -4_503_599_627_370_495.0)), Point::new(0.0, 1.0));
    assert_eq!(delta(&p, &Point::new(-4_503_599_627_370_496.0, -4_503_599_627_370_494.0)), Point::new(0.0, 2.0));

    assert_eq!(delta(&p, &Point::new(-4_503_599_627_370_495.0, -4_503_599_627_370_496.0)), Point::new(1.0, 0.0));
    assert_eq!(delta(&p, &Point::new(-4_503_599_627_370_495.0, -4_503_599_627_370_495.0)), Point::new(1.0, 1.0));
    assert_eq!(delta(&p, &Point::new(-4_503_599_627_370_495.0, -4_503_599_627_370_494.0)), Point::new(1.0, 2.0));

    assert_eq!(delta(&p, &Point::new(-4_503_599_627_370_494.0, -4_503_599_627_370_496.0)), Point::new(2.0, 0.0));
    assert_eq!(delta(&p, &Point::new(-4_503_599_627_370_494.0, -4_503_599_627_370_495.0)), Point::new(2.0, 1.0));
    assert_eq!(delta(&p, &Point::new(-4_503_599_627_370_494.0, -4_503_599_627_370_494.0)), Point::new(2.0, 2.0));
}

#[test]
fn delta_max() {
    let p = Point::new(4_503_599_627_370_493.0, 4_503_599_627_370_493.0);
    assert_eq!(delta(&p, &Point::new(4_503_599_627_370_493.0, 4_503_599_627_370_493.0)), Point::zero());
    assert_eq!(delta(&p, &Point::new(4_503_599_627_370_493.0, 4_503_599_627_370_494.0)), Point::new(0.0, 1.0));
    assert_eq!(delta(&p, &Point::new(4_503_599_627_370_493.0, 4_503_599_627_370_495.0)), Point::new(0.0, 2.0));

    assert_eq!(delta(&p, &Point::new(4_503_599_627_370_494.0, 4_503_599_627_370_493.0)), Point::new(1.0, 0.0));
    assert_eq!(delta(&p, &Point::new(4_503_599_627_370_494.0, 4_503_599_627_370_494.0)), Point::new(1.0, 1.0));
    assert_eq!(delta(&p, &Point::new(4_503_599_627_370_494.0, 4_503_599_627_370_495.0)), Point::new(1.0, 2.0));

    assert_eq!(delta(&p, &Point::new(4_503_599_627_370_495.0, 4_503_599_627_370_493.0)), Point::new(2.0, 0.0));
    assert_eq!(delta(&p, &Point::new(4_503_599_627_370_495.0, 4_503_599_627_370_494.0)), Point::new(2.0, 1.0));
    assert_eq!(delta(&p, &Point::new(4_503_599_627_370_495.0, 4_503_599_627_370_495.0)), Point::new(2.0, 2.0));
}

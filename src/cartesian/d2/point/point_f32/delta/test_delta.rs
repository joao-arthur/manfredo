use super::delta;
use crate::cartesian::d2::point::point_f32::Point;

#[test]
fn test_delta() {
    assert_eq!(delta(&Point::of(0.0, 0.0), &Point::of(0.0, 0.0)), Point::of(0.0, 0.0));
    assert_eq!(delta(&Point::of(-8_388_608.0, -8_388_608.0), &Point::of(8_388_607.0, 8_388_607.0)), Point::max());
}

#[test]
fn delta_min() {
    let p = Point::of(-8_388_608.0, -8_388_608.0);
    assert_eq!(delta(&p, &Point::of(-8_388_608.0, -8_388_608.0)), Point::of(0.0, 0.0));
    assert_eq!(delta(&p, &Point::of(-8_388_608.0, -8_388_607.0)), Point::of(0.0, 1.0));
    assert_eq!(delta(&p, &Point::of(-8_388_608.0, -8_388_606.0)), Point::of(0.0, 2.0));

    assert_eq!(delta(&p, &Point::of(-8_388_607.0, -8_388_608.0)), Point::of(1.0, 0.0));
    assert_eq!(delta(&p, &Point::of(-8_388_607.0, -8_388_607.0)), Point::of(1.0, 1.0));
    assert_eq!(delta(&p, &Point::of(-8_388_607.0, -8_388_606.0)), Point::of(1.0, 2.0));

    assert_eq!(delta(&p, &Point::of(-8_388_606.0, -8_388_608.0)), Point::of(2.0, 0.0));
    assert_eq!(delta(&p, &Point::of(-8_388_606.0, -8_388_607.0)), Point::of(2.0, 1.0));
    assert_eq!(delta(&p, &Point::of(-8_388_606.0, -8_388_606.0)), Point::of(2.0, 2.0));
}

#[test]
fn delta_max() {
    let p = Point::of(8_388_605.0, 8_388_605.0);
    assert_eq!(delta(&p, &Point::of(8_388_605.0, 8_388_605.0)), Point::of(0.0, 0.0));
    assert_eq!(delta(&p, &Point::of(8_388_605.0, 8_388_606.0)), Point::of(0.0, 1.0));
    assert_eq!(delta(&p, &Point::of(8_388_605.0, 8_388_607.0)), Point::of(0.0, 2.0));

    assert_eq!(delta(&p, &Point::of(8_388_606.0, 8_388_605.0)), Point::of(1.0, 0.0));
    assert_eq!(delta(&p, &Point::of(8_388_606.0, 8_388_606.0)), Point::of(1.0, 1.0));
    assert_eq!(delta(&p, &Point::of(8_388_606.0, 8_388_607.0)), Point::of(1.0, 2.0));

    assert_eq!(delta(&p, &Point::of(8_388_607.0, 8_388_605.0)), Point::of(2.0, 0.0));
    assert_eq!(delta(&p, &Point::of(8_388_607.0, 8_388_606.0)), Point::of(2.0, 1.0));
    assert_eq!(delta(&p, &Point::of(8_388_607.0, 8_388_607.0)), Point::of(2.0, 2.0));
}

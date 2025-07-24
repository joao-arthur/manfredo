use crate::cartesian::point::point_u16::PointU16;

#[derive(PartialEq, Debug, Clone)]
pub struct PointI16 {
    pub x: i16,
    pub y: i16,
}

impl PointI16 {
    pub fn of(x: i16, y: i16) -> Self {
        PointI16 { x, y }
    }
}

impl std::fmt::Display for PointI16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointI16, p2: &PointI16) -> u16 {
    (i32::from(p2.x) - i32::from(p1.x)).unsigned_abs() as u16
}

pub fn delta_y(p1: &PointI16, p2: &PointI16) -> u16 {
    (i32::from(p2.y) - i32::from(p1.y)).unsigned_abs() as u16
}

pub fn delta(p1: &PointI16, p2: &PointI16) -> PointU16 {
    PointU16 { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_u16::PointU16;

    use super::{PointI16, delta, delta_x, delta_y};

    #[test]
    fn point_i16() {
        assert_eq!(PointI16::of(i16::MIN, i16::MAX), PointI16 { x: i16::MIN, y: i16::MAX });
        assert_eq!(PointI16::of(i16::MIN, i16::MAX).to_string(), "(-32768, 32767)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointI16::of(0, i16::MIN), &PointI16::of(0, i16::MAX)), 0);
        assert_eq!(delta_x(&PointI16::of(i16::MIN, 0), &PointI16::of(i16::MAX, 0)), u16::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointI16::of(i16::MIN, 0), &PointI16::of(i16::MAX, 0)), 0);
        assert_eq!(delta_y(&PointI16::of(0, i16::MIN), &PointI16::of(0, i16::MAX)), u16::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointI16::of(0, 0), &PointI16::of(0, 0)), PointU16::of(0, 0));
        assert_eq!(delta(&PointI16::of(i16::MIN, i16::MIN), &PointI16::of(i16::MAX, i16::MAX)), PointU16::of(u16::MAX, u16::MAX));
    }

    #[test]
    fn delta_min() {
        let p1 = PointI16::of(i16::MIN, i16::MIN);
        assert_eq!(delta(&p1, &PointI16::of(i16::MIN, i16::MIN)), PointU16::of(0, 0));
        assert_eq!(delta(&p1, &PointI16::of(i16::MIN, i16::MIN + 1)), PointU16::of(0, 1));
        assert_eq!(delta(&p1, &PointI16::of(i16::MIN, i16::MIN + 2)), PointU16::of(0, 2));

        assert_eq!(delta(&p1, &PointI16::of(i16::MIN + 1, i16::MIN)), PointU16::of(1, 0));
        assert_eq!(delta(&p1, &PointI16::of(i16::MIN + 1, i16::MIN + 1)), PointU16::of(1, 1));
        assert_eq!(delta(&p1, &PointI16::of(i16::MIN + 1, i16::MIN + 2)), PointU16::of(1, 2));

        assert_eq!(delta(&p1, &PointI16::of(i16::MIN + 2, i16::MIN)), PointU16::of(2, 0));
        assert_eq!(delta(&p1, &PointI16::of(i16::MIN + 2, i16::MIN + 1)), PointU16::of(2, 1));
        assert_eq!(delta(&p1, &PointI16::of(i16::MIN + 2, i16::MIN + 2)), PointU16::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p1 = PointI16::of(i16::MAX - 2, i16::MAX - 2);
        assert_eq!(delta(&p1, &PointI16::of(i16::MAX - 2, i16::MAX - 2)), PointU16::of(0, 0));
        assert_eq!(delta(&p1, &PointI16::of(i16::MAX - 2, i16::MAX - 1)), PointU16::of(0, 1));
        assert_eq!(delta(&p1, &PointI16::of(i16::MAX - 2, i16::MAX)), PointU16::of(0, 2));

        assert_eq!(delta(&p1, &PointI16::of(i16::MAX - 1, i16::MAX - 2)), PointU16::of(1, 0));
        assert_eq!(delta(&p1, &PointI16::of(i16::MAX - 1, i16::MAX - 1)), PointU16::of(1, 1));
        assert_eq!(delta(&p1, &PointI16::of(i16::MAX - 1, i16::MAX)), PointU16::of(1, 2));

        assert_eq!(delta(&p1, &PointI16::of(i16::MAX, i16::MAX - 2)), PointU16::of(2, 0));
        assert_eq!(delta(&p1, &PointI16::of(i16::MAX, i16::MAX - 1)), PointU16::of(2, 1));
        assert_eq!(delta(&p1, &PointI16::of(i16::MAX, i16::MAX)), PointU16::of(2, 2));
    }
}

use crate::cartesian::point::point_u32::PointU32;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointI32 {
    pub x: i32,
    pub y: i32,
}

impl PointI32 {
    pub fn of(x: i32, y: i32) -> Self {
        PointI32 { x, y }
    }

    pub fn min() -> Self {
        PointI32 { x: i32::MIN, y: i32::MIN }
    }

    pub fn max() -> Self {
        PointI32 { x: i32::MAX, y: i32::MAX }
    }
}

impl std::fmt::Display for PointI32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointI32, p2: &PointI32) -> u32 {
    (i64::from(p2.x) - i64::from(p1.x)).unsigned_abs() as u32
}

pub fn delta_y(p1: &PointI32, p2: &PointI32) -> u32 {
    (i64::from(p2.y) - i64::from(p1.y)).unsigned_abs() as u32
}

pub fn delta(p1: &PointI32, p2: &PointI32) -> PointU32 {
    PointU32 { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_u32::PointU32;

    use super::{PointI32, delta, delta_x, delta_y};

    #[test]
    fn point_i32() {
        assert_eq!(PointI32::of(i32::MIN, i32::MAX), PointI32 { x: i32::MIN, y: i32::MAX });
        assert_eq!(PointI32::min(), PointI32 { x: i32::MIN, y: i32::MIN });
        assert_eq!(PointI32::max(), PointI32 { x: i32::MAX, y: i32::MAX });
        assert_eq!(PointI32::of(i32::MIN, i32::MAX).to_string(), "(-2147483648, 2147483647)");
        assert_eq!(PointI32::min().to_string(), "(-2147483648, -2147483648)");
        assert_eq!(PointI32::max().to_string(), "(2147483647, 2147483647)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointI32::of(0, i32::MIN), &PointI32::of(0, i32::MAX)), 0);
        assert_eq!(delta_x(&PointI32::of(i32::MIN, 0), &PointI32::of(i32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointI32::of(i32::MIN, 0), &PointI32::of(i32::MAX, 0)), 0);
        assert_eq!(delta_y(&PointI32::of(0, i32::MIN), &PointI32::of(0, i32::MAX)), u32::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointI32::of(0, 0), &PointI32::of(0, 0)), PointU32::of(0, 0));
        assert_eq!(delta(&PointI32::of(i32::MIN, i32::MIN), &PointI32::of(i32::MAX, i32::MAX)), PointU32::of(u32::MAX, u32::MAX));
    }

    #[test]
    fn delta_min() {
        let p1 = PointI32::of(i32::MIN, i32::MIN);
        assert_eq!(delta(&p1, &PointI32::of(i32::MIN, i32::MIN)), PointU32::of(0, 0));
        assert_eq!(delta(&p1, &PointI32::of(i32::MIN, i32::MIN + 1)), PointU32::of(0, 1));
        assert_eq!(delta(&p1, &PointI32::of(i32::MIN, i32::MIN + 2)), PointU32::of(0, 2));

        assert_eq!(delta(&p1, &PointI32::of(i32::MIN + 1, i32::MIN)), PointU32::of(1, 0));
        assert_eq!(delta(&p1, &PointI32::of(i32::MIN + 1, i32::MIN + 1)), PointU32::of(1, 1));
        assert_eq!(delta(&p1, &PointI32::of(i32::MIN + 1, i32::MIN + 2)), PointU32::of(1, 2));

        assert_eq!(delta(&p1, &PointI32::of(i32::MIN + 2, i32::MIN)), PointU32::of(2, 0));
        assert_eq!(delta(&p1, &PointI32::of(i32::MIN + 2, i32::MIN + 1)), PointU32::of(2, 1));
        assert_eq!(delta(&p1, &PointI32::of(i32::MIN + 2, i32::MIN + 2)), PointU32::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p1 = PointI32::of(i32::MAX - 2, i32::MAX - 2);
        assert_eq!(delta(&p1, &PointI32::of(i32::MAX - 2, i32::MAX - 2)), PointU32::of(0, 0));
        assert_eq!(delta(&p1, &PointI32::of(i32::MAX - 2, i32::MAX - 1)), PointU32::of(0, 1));
        assert_eq!(delta(&p1, &PointI32::of(i32::MAX - 2, i32::MAX)), PointU32::of(0, 2));

        assert_eq!(delta(&p1, &PointI32::of(i32::MAX - 1, i32::MAX - 2)), PointU32::of(1, 0));
        assert_eq!(delta(&p1, &PointI32::of(i32::MAX - 1, i32::MAX - 1)), PointU32::of(1, 1));
        assert_eq!(delta(&p1, &PointI32::of(i32::MAX - 1, i32::MAX)), PointU32::of(1, 2));

        assert_eq!(delta(&p1, &PointI32::of(i32::MAX, i32::MAX - 2)), PointU32::of(2, 0));
        assert_eq!(delta(&p1, &PointI32::of(i32::MAX, i32::MAX - 1)), PointU32::of(2, 1));
        assert_eq!(delta(&p1, &PointI32::of(i32::MAX, i32::MAX)), PointU32::of(2, 2));
    }
}

use crate::cartesian::point::point_u64::PointU64;

#[derive(PartialEq, Debug, Clone)]
pub struct PointI64 {
    pub x: i64,
    pub y: i64,
}

impl PointI64 {
    pub fn of(x: i64, y: i64) -> Self {
        PointI64 { x, y }
    }
}

impl std::fmt::Display for PointI64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointI64, p2: &PointI64) -> u64 {
    (i128::from(p2.x) - i128::from(p1.x)).unsigned_abs() as u64
}

pub fn delta_y(p1: &PointI64, p2: &PointI64) -> u64 {
    (i128::from(p2.y) - i128::from(p1.y)).unsigned_abs() as u64
}

pub fn delta(p1: &PointI64, p2: &PointI64) -> PointU64 {
    PointU64 { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_u64::PointU64;

    use super::{PointI64, delta, delta_x, delta_y};

    #[test]
    fn point_i64() {
        assert_eq!(PointI64::of(i64::MIN, i64::MAX), PointI64 { x: i64::MIN, y: i64::MAX });
        assert_eq!(PointI64::of(i64::MIN, i64::MAX).to_string(), "(-9223372036854775808, 9223372036854775807)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointI64::of(0, i64::MIN), &PointI64::of(0, i64::MAX)), 0);
        assert_eq!(delta_x(&PointI64::of(i64::MIN, 0), &PointI64::of(i64::MAX, 0)), u64::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointI64::of(i64::MIN, 0), &PointI64::of(i64::MAX, 0)), 0);
        assert_eq!(delta_y(&PointI64::of(0, i64::MIN), &PointI64::of(0, i64::MAX)), u64::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointI64::of(0, 0), &PointI64::of(0, 0)), PointU64::of(0, 0));
        assert_eq!(delta(&PointI64::of(i64::MIN, i64::MIN), &PointI64::of(i64::MAX, i64::MAX)), PointU64::of(u64::MAX, u64::MAX));
    }

    #[test]
    fn delta_min() {
        let p1 = PointI64::of(i64::MIN, i64::MIN);
        assert_eq!(delta(&p1, &PointI64::of(i64::MIN, i64::MIN)), PointU64::of(0, 0));
        assert_eq!(delta(&p1, &PointI64::of(i64::MIN, i64::MIN + 1)), PointU64::of(0, 1));
        assert_eq!(delta(&p1, &PointI64::of(i64::MIN, i64::MIN + 2)), PointU64::of(0, 2));

        assert_eq!(delta(&p1, &PointI64::of(i64::MIN + 1, i64::MIN)), PointU64::of(1, 0));
        assert_eq!(delta(&p1, &PointI64::of(i64::MIN + 1, i64::MIN + 1)), PointU64::of(1, 1));
        assert_eq!(delta(&p1, &PointI64::of(i64::MIN + 1, i64::MIN + 2)), PointU64::of(1, 2));

        assert_eq!(delta(&p1, &PointI64::of(i64::MIN + 2, i64::MIN)), PointU64::of(2, 0));
        assert_eq!(delta(&p1, &PointI64::of(i64::MIN + 2, i64::MIN + 1)), PointU64::of(2, 1));
        assert_eq!(delta(&p1, &PointI64::of(i64::MIN + 2, i64::MIN + 2)), PointU64::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p1 = PointI64::of(i64::MAX - 2, i64::MAX - 2);
        assert_eq!(delta(&p1, &PointI64::of(i64::MAX - 2, i64::MAX - 2)), PointU64::of(0, 0));
        assert_eq!(delta(&p1, &PointI64::of(i64::MAX - 2, i64::MAX - 1)), PointU64::of(0, 1));
        assert_eq!(delta(&p1, &PointI64::of(i64::MAX - 2, i64::MAX)), PointU64::of(0, 2));

        assert_eq!(delta(&p1, &PointI64::of(i64::MAX - 1, i64::MAX - 2)), PointU64::of(1, 0));
        assert_eq!(delta(&p1, &PointI64::of(i64::MAX - 1, i64::MAX - 1)), PointU64::of(1, 1));
        assert_eq!(delta(&p1, &PointI64::of(i64::MAX - 1, i64::MAX)), PointU64::of(1, 2));

        assert_eq!(delta(&p1, &PointI64::of(i64::MAX, i64::MAX - 2)), PointU64::of(2, 0));
        assert_eq!(delta(&p1, &PointI64::of(i64::MAX, i64::MAX - 1)), PointU64::of(2, 1));
        assert_eq!(delta(&p1, &PointI64::of(i64::MAX, i64::MAX)), PointU64::of(2, 2));
    }
}

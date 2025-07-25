#[derive(PartialEq, Debug, Clone)]
pub struct PointU16 {
    pub x: u16,
    pub y: u16,
}

impl PointU16 {
    pub fn of(x: u16, y: u16) -> Self {
        PointU16 { x, y }
    }
}

impl std::fmt::Display for PointU16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointU16, p2: &PointU16) -> u16 {
    p2.x - p1.x
}

pub fn delta_y(p1: &PointU16, p2: &PointU16) -> u16 {
    p2.y - p1.y
}

pub fn delta(p1: &PointU16, p2: &PointU16) -> PointU16 {
    PointU16 { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{PointU16, delta, delta_x, delta_y};

    #[test]
    fn point_u16() {
        assert_eq!(PointU16::of(0, u16::MAX), PointU16 { x: 0, y: u16::MAX });
        assert_eq!(PointU16::of(0, u16::MAX).to_string(), "(0, 65535)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointU16::of(0, 0), &PointU16::of(0, u16::MAX)), 0);
        assert_eq!(delta_x(&PointU16::of(0, 0), &PointU16::of(u16::MAX, 0)), u16::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointU16::of(0, 0), &PointU16::of(u16::MAX, 0)), 0);
        assert_eq!(delta_y(&PointU16::of(0, 0), &PointU16::of(0, u16::MAX)), u16::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointU16::of(0, 0), &PointU16::of(0, 0)), PointU16::of(0, 0));
        assert_eq!(delta(&PointU16::of(0, 0), &PointU16::of(u16::MAX, u16::MAX)), PointU16::of(u16::MAX, u16::MAX));
    }

    #[test]
    fn delta_min() {
        let p1 = PointU16::of(0, 0);
        assert_eq!(delta(&p1, &PointU16::of(0, 0)), PointU16::of(0, 0));
        assert_eq!(delta(&p1, &PointU16::of(0, 1)), PointU16::of(0, 1));
        assert_eq!(delta(&p1, &PointU16::of(0, 2)), PointU16::of(0, 2));

        assert_eq!(delta(&p1, &PointU16::of(1, 0)), PointU16::of(1, 0));
        assert_eq!(delta(&p1, &PointU16::of(1, 1)), PointU16::of(1, 1));
        assert_eq!(delta(&p1, &PointU16::of(1, 2)), PointU16::of(1, 2));

        assert_eq!(delta(&p1, &PointU16::of(2, 0)), PointU16::of(2, 0));
        assert_eq!(delta(&p1, &PointU16::of(2, 1)), PointU16::of(2, 1));
        assert_eq!(delta(&p1, &PointU16::of(2, 2)), PointU16::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p1 = PointU16::of(u16::MAX - 2, u16::MAX - 2);
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 2, u16::MAX - 2)), PointU16::of(0, 0));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 2, u16::MAX - 1)), PointU16::of(0, 1));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 2, u16::MAX)), PointU16::of(0, 2));

        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 1, u16::MAX - 2)), PointU16::of(1, 0));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 1, u16::MAX - 1)), PointU16::of(1, 1));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 1, u16::MAX)), PointU16::of(1, 2));

        assert_eq!(delta(&p1, &PointU16::of(u16::MAX, u16::MAX - 2)), PointU16::of(2, 0));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX, u16::MAX - 1)), PointU16::of(2, 1));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX, u16::MAX)), PointU16::of(2, 2));
    }
}

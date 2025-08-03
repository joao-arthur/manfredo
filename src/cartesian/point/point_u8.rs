#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointU8 {
    pub x: u8,
    pub y: u8,
}

impl PointU8 {
    pub fn of(x: u8, y: u8) -> Self {
        PointU8 { x, y }
    }

    pub fn min() -> Self {
        PointU8 { x: 0, y: 0 }
    }

    pub fn max() -> Self {
        PointU8 { x: u8::MAX, y: u8::MAX }
    }
}

impl std::fmt::Display for PointU8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointU8, p2: &PointU8) -> u8 {
    p2.x - p1.x
}

pub fn delta_y(p1: &PointU8, p2: &PointU8) -> u8 {
    p2.y - p1.y
}

pub fn delta(p1: &PointU8, p2: &PointU8) -> PointU8 {
    PointU8 { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{PointU8, delta, delta_x, delta_y};

    #[test]
    fn point_u8() {
        assert_eq!(PointU8::of(0, u8::MAX), PointU8 { x: 0, y: u8::MAX });
        assert_eq!(PointU8::min(), PointU8 { x: 0, y: 0 });
        assert_eq!(PointU8::max(), PointU8 { x: u8::MAX, y: u8::MAX });
        assert_eq!(PointU8::of(0, u8::MAX).to_string(), "(0, 255)");
        assert_eq!(PointU8::min().to_string(), "(0, 0)");
        assert_eq!(PointU8::max().to_string(), "(255, 255)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointU8::min(), &PointU8::of(0, u8::MAX)), 0);
        assert_eq!(delta_x(&PointU8::min(), &PointU8::of(u8::MAX, 0)), u8::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointU8::min(), &PointU8::of(u8::MAX, 0)), 0);
        assert_eq!(delta_y(&PointU8::min(), &PointU8::of(0, u8::MAX)), u8::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointU8::min(), &PointU8::min()), PointU8::min());
        assert_eq!(delta(&PointU8::min(), &PointU8::max()), PointU8::max());
    }

    #[test]
    fn delta_min() {
        let p1 = PointU8::min();
        assert_eq!(delta(&p1, &PointU8::of(0, 0)), PointU8::of(0, 0));
        assert_eq!(delta(&p1, &PointU8::of(0, 1)), PointU8::of(0, 1));
        assert_eq!(delta(&p1, &PointU8::of(0, 2)), PointU8::of(0, 2));

        assert_eq!(delta(&p1, &PointU8::of(1, 0)), PointU8::of(1, 0));
        assert_eq!(delta(&p1, &PointU8::of(1, 1)), PointU8::of(1, 1));
        assert_eq!(delta(&p1, &PointU8::of(1, 2)), PointU8::of(1, 2));

        assert_eq!(delta(&p1, &PointU8::of(2, 0)), PointU8::of(2, 0));
        assert_eq!(delta(&p1, &PointU8::of(2, 1)), PointU8::of(2, 1));
        assert_eq!(delta(&p1, &PointU8::of(2, 2)), PointU8::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p1 = PointU8::of(u8::MAX - 2, u8::MAX - 2);
        assert_eq!(delta(&p1, &PointU8::of(u8::MAX - 2, u8::MAX - 2)), PointU8::of(0, 0));
        assert_eq!(delta(&p1, &PointU8::of(u8::MAX - 2, u8::MAX - 1)), PointU8::of(0, 1));
        assert_eq!(delta(&p1, &PointU8::of(u8::MAX - 2, u8::MAX)), PointU8::of(0, 2));

        assert_eq!(delta(&p1, &PointU8::of(u8::MAX - 1, u8::MAX - 2)), PointU8::of(1, 0));
        assert_eq!(delta(&p1, &PointU8::of(u8::MAX - 1, u8::MAX - 1)), PointU8::of(1, 1));
        assert_eq!(delta(&p1, &PointU8::of(u8::MAX - 1, u8::MAX)), PointU8::of(1, 2));

        assert_eq!(delta(&p1, &PointU8::of(u8::MAX, u8::MAX - 2)), PointU8::of(2, 0));
        assert_eq!(delta(&p1, &PointU8::of(u8::MAX, u8::MAX - 1)), PointU8::of(2, 1));
        assert_eq!(delta(&p1, &PointU8::of(u8::MAX, u8::MAX)), PointU8::of(2, 2));
    }
}

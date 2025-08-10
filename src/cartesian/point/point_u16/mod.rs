use super::point_u8::PointU8;

pub mod checked;
pub mod saturating;
pub mod wrapping;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointU16 {
    pub x: u16,
    pub y: u16,
}

impl PointU16 {
    pub fn of(x: u16, y: u16) -> Self {
        PointU16 { x, y }
    }

    pub fn min() -> Self {
        PointU16 { x: 0, y: 0 }
    }

    pub fn max() -> Self {
        PointU16 { x: u16::MAX, y: u16::MAX }
    }
}

impl From<PointU8> for PointU16 {
    fn from(p: PointU8) -> Self {
        PointU16 { x: p.x.into(), y: p.y.into() }
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
    use crate::cartesian::point::point_u8::PointU8;

    use super::{PointU16, delta, delta_x, delta_y};

    #[test]
    fn point_u16() {
        assert_eq!(PointU16::of(0, u16::MAX), PointU16 { x: 0, y: u16::MAX });
        assert_eq!(PointU16::min(), PointU16 { x: 0, y: 0 });
        assert_eq!(PointU16::max(), PointU16 { x: u16::MAX, y: u16::MAX });
    }

    #[test]
    fn from() {
        assert_eq!(PointU16::from(PointU8::min()), PointU16 { x: u8::MIN.into(), y: u8::MIN.into() });
        assert_eq!(PointU16::from(PointU8::max()), PointU16 { x: u8::MAX.into(), y: u8::MAX.into() });
    }

    #[test]
    fn to_string() {
        assert_eq!(PointU16::of(0, u16::MAX).to_string(), "(0, 65535)");
        assert_eq!(PointU16::min().to_string(), "(0, 0)");
        assert_eq!(PointU16::max().to_string(), "(65535, 65535)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointU16::min(), &PointU16::of(0, u16::MAX)), 0);
        assert_eq!(delta_x(&PointU16::min(), &PointU16::of(u16::MAX, 0)), u16::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointU16::min(), &PointU16::of(u16::MAX, 0)), 0);
        assert_eq!(delta_y(&PointU16::min(), &PointU16::of(0, u16::MAX)), u16::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointU16::min(), &PointU16::min()), PointU16::min());
        assert_eq!(delta(&PointU16::min(), &PointU16::max()), PointU16::max());
    }

    #[test]
    fn delta_min() {
        let p = PointU16::min();
        assert_eq!(delta(&p, &PointU16::of(0, 0)), PointU16::of(0, 0));
        assert_eq!(delta(&p, &PointU16::of(0, 1)), PointU16::of(0, 1));
        assert_eq!(delta(&p, &PointU16::of(0, 2)), PointU16::of(0, 2));

        assert_eq!(delta(&p, &PointU16::of(1, 0)), PointU16::of(1, 0));
        assert_eq!(delta(&p, &PointU16::of(1, 1)), PointU16::of(1, 1));
        assert_eq!(delta(&p, &PointU16::of(1, 2)), PointU16::of(1, 2));

        assert_eq!(delta(&p, &PointU16::of(2, 0)), PointU16::of(2, 0));
        assert_eq!(delta(&p, &PointU16::of(2, 1)), PointU16::of(2, 1));
        assert_eq!(delta(&p, &PointU16::of(2, 2)), PointU16::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p = PointU16::of(u16::MAX - 2, u16::MAX - 2);
        assert_eq!(delta(&p, &PointU16::of(u16::MAX - 2, u16::MAX - 2)), PointU16::of(0, 0));
        assert_eq!(delta(&p, &PointU16::of(u16::MAX - 2, u16::MAX - 1)), PointU16::of(0, 1));
        assert_eq!(delta(&p, &PointU16::of(u16::MAX - 2, u16::MAX)), PointU16::of(0, 2));

        assert_eq!(delta(&p, &PointU16::of(u16::MAX - 1, u16::MAX - 2)), PointU16::of(1, 0));
        assert_eq!(delta(&p, &PointU16::of(u16::MAX - 1, u16::MAX - 1)), PointU16::of(1, 1));
        assert_eq!(delta(&p, &PointU16::of(u16::MAX - 1, u16::MAX)), PointU16::of(1, 2));

        assert_eq!(delta(&p, &PointU16::of(u16::MAX, u16::MAX - 2)), PointU16::of(2, 0));
        assert_eq!(delta(&p, &PointU16::of(u16::MAX, u16::MAX - 1)), PointU16::of(2, 1));
        assert_eq!(delta(&p, &PointU16::max()), PointU16::of(2, 2));
    }
}

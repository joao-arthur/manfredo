use super::{point_u8::PointU8, point_u16::PointU16, point_u32::PointU32};

pub mod add;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointU64 {
    pub x: u64,
    pub y: u64,
}

impl PointU64 {
    pub fn of(x: u64, y: u64) -> Self {
        PointU64 { x, y }
    }

    pub fn min() -> Self {
        PointU64 { x: 0, y: 0 }
    }

    pub fn max() -> Self {
        PointU64 { x: u64::MAX, y: u64::MAX }
    }
}

impl From<PointU8> for PointU64 {
    fn from(p: PointU8) -> Self {
        PointU64 { x: p.x.into(), y: p.y.into() }
    }
}

impl From<PointU16> for PointU64 {
    fn from(p: PointU16) -> Self {
        PointU64 { x: p.x.into(), y: p.y.into() }
    }
}

impl From<PointU32> for PointU64 {
    fn from(p: PointU32) -> Self {
        PointU64 { x: p.x.into(), y: p.y.into() }
    }
}

impl std::fmt::Display for PointU64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointU64, p2: &PointU64) -> u64 {
    p2.x - p1.x
}

pub fn delta_y(p1: &PointU64, p2: &PointU64) -> u64 {
    p2.y - p1.y
}

pub fn delta(p1: &PointU64, p2: &PointU64) -> PointU64 {
    PointU64 { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::{point_u8::PointU8, point_u16::PointU16, point_u32::PointU32};

    use super::{PointU64, delta, delta_x, delta_y};

    #[test]
    fn point_u64() {
        assert_eq!(PointU64::of(0, u64::MAX), PointU64 { x: 0, y: u64::MAX });
        assert_eq!(PointU64::min(), PointU64 { x: 0, y: 0 });
        assert_eq!(PointU64::max(), PointU64 { x: u64::MAX, y: u64::MAX });
    }

    #[test]
    fn from() {
        assert_eq!(PointU64::from(PointU8::min()), PointU64 { x: u8::MIN.into(), y: u8::MIN.into() });
        assert_eq!(PointU64::from(PointU8::max()), PointU64 { x: u8::MAX.into(), y: u8::MAX.into() });
        assert_eq!(PointU64::from(PointU16::min()), PointU64 { x: u16::MIN.into(), y: u16::MIN.into() });
        assert_eq!(PointU64::from(PointU16::max()), PointU64 { x: u16::MAX.into(), y: u16::MAX.into() });
        assert_eq!(PointU64::from(PointU32::min()), PointU64 { x: u32::MIN.into(), y: u32::MIN.into() });
        assert_eq!(PointU64::from(PointU32::max()), PointU64 { x: u32::MAX.into(), y: u32::MAX.into() });
    }

    #[test]
    fn to_string() {
        assert_eq!(PointU64::of(0, u64::MAX).to_string(), "(0, 18446744073709551615)");
        assert_eq!(PointU64::min().to_string(), "(0, 0)");
        assert_eq!(PointU64::max().to_string(), "(18446744073709551615, 18446744073709551615)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointU64::min(), &PointU64::of(0, u64::MAX)), 0);
        assert_eq!(delta_x(&PointU64::min(), &PointU64::of(u64::MAX, 0)), u64::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointU64::min(), &PointU64::of(u64::MAX, 0)), 0);
        assert_eq!(delta_y(&PointU64::min(), &PointU64::of(0, u64::MAX)), u64::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointU64::min(), &PointU64::min()), PointU64::min());
        assert_eq!(delta(&PointU64::min(), &PointU64::max()), PointU64::max());
    }

    #[test]
    fn delta_min() {
        let p = PointU64::min();
        assert_eq!(delta(&p, &PointU64::of(0, 0)), PointU64::of(0, 0));
        assert_eq!(delta(&p, &PointU64::of(0, 1)), PointU64::of(0, 1));
        assert_eq!(delta(&p, &PointU64::of(0, 2)), PointU64::of(0, 2));

        assert_eq!(delta(&p, &PointU64::of(1, 0)), PointU64::of(1, 0));
        assert_eq!(delta(&p, &PointU64::of(1, 1)), PointU64::of(1, 1));
        assert_eq!(delta(&p, &PointU64::of(1, 2)), PointU64::of(1, 2));

        assert_eq!(delta(&p, &PointU64::of(2, 0)), PointU64::of(2, 0));
        assert_eq!(delta(&p, &PointU64::of(2, 1)), PointU64::of(2, 1));
        assert_eq!(delta(&p, &PointU64::of(2, 2)), PointU64::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p = PointU64::of(u64::MAX - 2, u64::MAX - 2);
        assert_eq!(delta(&p, &PointU64::of(u64::MAX - 2, u64::MAX - 2)), PointU64::of(0, 0));
        assert_eq!(delta(&p, &PointU64::of(u64::MAX - 2, u64::MAX - 1)), PointU64::of(0, 1));
        assert_eq!(delta(&p, &PointU64::of(u64::MAX - 2, u64::MAX)), PointU64::of(0, 2));

        assert_eq!(delta(&p, &PointU64::of(u64::MAX - 1, u64::MAX - 2)), PointU64::of(1, 0));
        assert_eq!(delta(&p, &PointU64::of(u64::MAX - 1, u64::MAX - 1)), PointU64::of(1, 1));
        assert_eq!(delta(&p, &PointU64::of(u64::MAX - 1, u64::MAX)), PointU64::of(1, 2));

        assert_eq!(delta(&p, &PointU64::of(u64::MAX, u64::MAX - 2)), PointU64::of(2, 0));
        assert_eq!(delta(&p, &PointU64::of(u64::MAX, u64::MAX - 1)), PointU64::of(2, 1));
        assert_eq!(delta(&p, &PointU64::max()), PointU64::of(2, 2));
    }
}

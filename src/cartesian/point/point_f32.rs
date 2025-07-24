#[derive(PartialEq, Debug, Clone)]
pub struct PointF32 {
    pub x: f32,
    pub y: f32,
}

impl PointF32 {
    pub fn of(x: f32, y: f32) -> Self {
        PointF32 { x, y }
    }
}

impl std::fmt::Display for PointF32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub const MIN: f32 = -16_777_216.0;
pub const MAX: f32 = 16_777_215.0;

pub fn delta_x(p1: &PointF32, p2: &PointF32) -> f32 {
    p2.x - p1.x
}

pub fn delta_y(p1: &PointF32, p2: &PointF32) -> f32 {
    p2.y - p1.y
}

pub fn delta(p1: &PointF32, p2: &PointF32) -> PointF32 {
    PointF32 { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{MAX, MIN, PointF32, delta, delta_x, delta_y};

    #[test]
    fn point_f32() {
        assert_eq!(PointF32::of(MIN, MAX), PointF32 { x: MIN, y: MAX });
        assert_eq!(PointF32::of(MIN, MAX).to_string(), "(-16777216, 16777215)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointF32::of(0.0, 0.0), &PointF32::of(0.0, MAX)), 0.0);
        assert_eq!(delta_x(&PointF32::of(0.0, -8_388_608.0), &PointF32::of(0.0, 8_388_607.0)), 0.0);
        assert_eq!(delta_x(&PointF32::of(0.0, 0.0), &PointF32::of(MAX, 0.0)), MAX);
        assert_eq!(delta_x(&PointF32::of(-8_388_608.0, 0.0), &PointF32::of(8_388_607.0, 0.0)), MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointF32::of(0.0, 0.0), &PointF32::of(MAX, 0.0)), 0.0);
        assert_eq!(delta_y(&PointF32::of(-8_388_608.0, 0.0), &PointF32::of(8_388_607.0, 0.0)), 0.0);
        assert_eq!(delta_y(&PointF32::of(0.0, 0.0), &PointF32::of(0.0, MAX)), MAX);
        assert_eq!(delta_y(&PointF32::of(0.0, -8_388_608.0), &PointF32::of(0.0, 8_388_607.0)), MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointF32::of(0.0, 0.0), &PointF32::of(0.0, 0.0)), PointF32::of(0.0, 0.0));
        assert_eq!(delta(&PointF32::of(-8_388_608.0, -8_388_608.0), &PointF32::of(8_388_607.0, 8_388_607.0)), PointF32::of(MAX, MAX));
    }

    #[test]
    fn delta_min() {
        let p1 = PointF32::of(-8_388_608.0, -8_388_608.0);
        assert_eq!(delta(&p1, &PointF32::of(-8_388_608.0, -8_388_608.0)), PointF32::of(0.0, 0.0));
        assert_eq!(delta(&p1, &PointF32::of(-8_388_608.0, -8_388_607.0)), PointF32::of(0.0, 1.0));
        assert_eq!(delta(&p1, &PointF32::of(-8_388_608.0, -8_388_606.0)), PointF32::of(0.0, 2.0));

        assert_eq!(delta(&p1, &PointF32::of(-8_388_607.0, -8_388_608.0)), PointF32::of(1.0, 0.0));
        assert_eq!(delta(&p1, &PointF32::of(-8_388_607.0, -8_388_607.0)), PointF32::of(1.0, 1.0));
        assert_eq!(delta(&p1, &PointF32::of(-8_388_607.0, -8_388_606.0)), PointF32::of(1.0, 2.0));

        assert_eq!(delta(&p1, &PointF32::of(-8_388_606.0, -8_388_608.0)), PointF32::of(2.0, 0.0));
        assert_eq!(delta(&p1, &PointF32::of(-8_388_606.0, -8_388_607.0)), PointF32::of(2.0, 1.0));
        assert_eq!(delta(&p1, &PointF32::of(-8_388_606.0, -8_388_606.0)), PointF32::of(2.0, 2.0));
    }

    #[test]
    fn delta_max() {
        let p1 = PointF32::of(8_388_605.0, 8_388_605.0);
        assert_eq!(delta(&p1, &PointF32::of(8_388_605.0, 8_388_605.0)), PointF32::of(0.0, 0.0));
        assert_eq!(delta(&p1, &PointF32::of(8_388_605.0, 8_388_606.0)), PointF32::of(0.0, 1.0));
        assert_eq!(delta(&p1, &PointF32::of(8_388_605.0, 8_388_607.0)), PointF32::of(0.0, 2.0));

        assert_eq!(delta(&p1, &PointF32::of(8_388_606.0, 8_388_605.0)), PointF32::of(1.0, 0.0));
        assert_eq!(delta(&p1, &PointF32::of(8_388_606.0, 8_388_606.0)), PointF32::of(1.0, 1.0));
        assert_eq!(delta(&p1, &PointF32::of(8_388_606.0, 8_388_607.0)), PointF32::of(1.0, 2.0));

        assert_eq!(delta(&p1, &PointF32::of(8_388_607.0, 8_388_605.0)), PointF32::of(2.0, 0.0));
        assert_eq!(delta(&p1, &PointF32::of(8_388_607.0, 8_388_606.0)), PointF32::of(2.0, 1.0));
        assert_eq!(delta(&p1, &PointF32::of(8_388_607.0, 8_388_607.0)), PointF32::of(2.0, 2.0));
    }
}

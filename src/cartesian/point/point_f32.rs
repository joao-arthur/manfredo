pub const MIN: f32 = -16_777_216.0;
pub const MAX: f32 = 16_777_215.0;

#[derive(PartialEq, Debug, Clone)]
pub struct PointF32 {
    pub x: f32,
    pub y: f32,
}

impl PointF32 {
    pub fn of(x: f32, y: f32) -> Self {
        PointF32 { x, y }
    }

    pub fn min() -> Self {
        PointF32 { x: MIN, y: MIN }
    }

    pub fn max() -> Self {
        PointF32 { x: MAX, y: MAX }
    }
}

impl std::fmt::Display for PointF32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointF32, p2: &PointF32) -> f32 {
    p2.x - p1.x
}

pub fn delta_y(p1: &PointF32, p2: &PointF32) -> f32 {
    p2.y - p1.y
}

pub fn delta(p1: &PointF32, p2: &PointF32) -> PointF32 {
    PointF32 { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

pub fn saturating_translate(p: &mut PointF32, delta: &PointF32) {
    let temp_x = p.x + delta.x;
    let temp_y = p.y + delta.y;
    p.x = temp_x.clamp(MIN, MAX);
    p.y = temp_y.clamp(MIN, MAX);
}

pub fn checked_translate(p: &mut PointF32, delta: &PointF32) -> Result<(), ()> {
    let x = p.x + delta.x;
    let y = p.y + delta.y;
    if x < MIN || x > MAX || y < MIN || y > MAX {
        return Err(());
    }
    p.x = x;
    p.y = y;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{MAX, MIN, PointF32, checked_translate, delta, delta_x, delta_y, saturating_translate};

    #[test]
    fn point_f32() {
        assert_eq!(PointF32::of(MIN, MAX), PointF32 { x: MIN, y: MAX });
        assert_eq!(PointF32::min(), PointF32 { x: MIN, y: MIN });
        assert_eq!(PointF32::max(), PointF32 { x: MAX, y: MAX });
        assert_eq!(PointF32::of(MIN, MAX).to_string(), "(-16777216, 16777215)");
        assert_eq!(PointF32::min().to_string(), "(-16777216, -16777216)");
        assert_eq!(PointF32::max().to_string(), "(16777215, 16777215)");
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

    #[test]
    fn test_saturating_translate() {
        let mut r = PointF32::of(0.0, 0.0);
        saturating_translate(&mut r, &PointF32::of(10.0, 10.0));
        assert_eq!(r, PointF32::of(10.0, 10.0));
        saturating_translate(&mut r, &PointF32::of(-15.0, -15.0));
        assert_eq!(r, PointF32::of(-5.0, -5.0));
        saturating_translate(&mut r, &PointF32::of(2.0, 2.0));
        assert_eq!(r, PointF32::of(-3.0, -3.0));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut r = PointF32::of(MIN + 2.0, MIN + 5.0);
        saturating_translate(&mut r, &PointF32::of(-10.0, -10.0));
        assert_eq!(r, PointF32::of(MIN, MIN));
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut r = PointF32::of(MAX - 2.0, MAX - 5.0);
        saturating_translate(&mut r, &PointF32::of(10.0, 10.0));
        assert_eq!(r, PointF32::of(MAX, MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_min_delta() {
        let mut r = PointF32::of(MIN + 1.0, MIN + 1.0);
        saturating_translate(&mut r, &PointF32::min());
        assert_eq!(r, PointF32::of(MIN, MIN));
    }

    #[test]
    fn saturating_translate_max_bounds_max_delta() {
        let mut r = PointF32::of(MAX - 1.0, MAX - 1.0);
        saturating_translate(&mut r, &PointF32::max());
        assert_eq!(r, PointF32::of(MAX, MAX));
    }

    #[test]
    fn test_checked_translate() {
        let mut r = PointF32::of(0.0, 0.0);
        assert_eq!(checked_translate(&mut r, &PointF32::of(10.0, 10.0)), Ok(()));
        assert_eq!(r, PointF32::of(10.0, 10.0));
        assert_eq!(checked_translate(&mut r, &PointF32::of(-15.0, -15.0)), Ok(()));
        assert_eq!(r, PointF32::of(-5.0, -5.0));
        assert_eq!(checked_translate(&mut r, &PointF32::of(2.0, 2.0)), Ok(()));
        assert_eq!(r, PointF32::of(-3.0, -3.0));
    }

    #[test]
    fn checked_translate_min_bounds_err() {
        let mut r = PointF32::of(MIN + 2.0, MIN + 5.0);
        assert_eq!(checked_translate(&mut r, &PointF32::of(-10.0, -10.0)), Err(()));
        assert_eq!(r, PointF32::of(MIN + 2.0, MIN + 5.0));
    }

    #[test]
    fn checked_translate_max_bounds_err() {
        let mut r = PointF32::of(MAX - 2.0, MAX - 5.0);
        assert_eq!(checked_translate(&mut r, &PointF32::of(10.0, 10.0)), Err(()));
        assert_eq!(r, PointF32::of(MAX - 2.0, MAX - 5.0));
    }

    #[test]
    fn checked_translate_min_bounds_ok() {
        let mut r = PointF32::of(MIN + 2.0, MIN + 5.0);
        assert_eq!(checked_translate(&mut r, &PointF32::of(-2.0, -5.0)), Ok(()));
        assert_eq!(r, PointF32::of(MIN, MIN));
    }

    #[test]
    fn checked_translate_max_bounds_ok() {
        let mut r = PointF32::of(MAX - 2.0, MAX - 5.0);
        assert_eq!(checked_translate(&mut r, &PointF32::of(2.0, 5.0)), Ok(()));
        assert_eq!(r, PointF32::of(MAX, MAX));
    }

    #[test]
    fn checked_translate_min_bounds_min_delta() {
        let mut r = PointF32::of(MIN + 1.0, MIN + 1.0);
        assert_eq!(checked_translate(&mut r, &PointF32::min()), Err(()));
        assert_eq!(checked_translate(&mut r, &PointF32::of(MIN, 0.0)), Err(()));
        assert_eq!(checked_translate(&mut r, &PointF32::of(0.0, MIN)), Err(()));
        assert_eq!(r, PointF32::of(MIN + 1.0, MIN + 1.0));
    }

    #[test]
    fn checked_translate_max_bounds_max_delta() {
        let mut r = PointF32::of(MAX - 1.0, MAX - 1.0);
        assert_eq!(checked_translate(&mut r, &PointF32::max()), Err(()));
        assert_eq!(checked_translate(&mut r, &PointF32::of(MAX, 0.0)), Err(()));
        assert_eq!(checked_translate(&mut r, &PointF32::of(0.0, MAX)), Err(()));
        assert_eq!(r, PointF32::of(MAX - 1.0, MAX - 1.0));
    }
}

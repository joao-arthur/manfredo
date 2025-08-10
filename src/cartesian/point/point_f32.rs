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

pub fn assign_saturating_add(p: &mut PointF32, delta: &PointF32) {
    let temp_x = p.x + delta.x;
    let temp_y = p.y + delta.y;
    p.x = temp_x.clamp(MIN, MAX);
    p.y = temp_y.clamp(MIN, MAX);
}

pub fn try_assign_checked_add(p: &mut PointF32, delta: &PointF32) -> Result<(), ()> {
    let x = p.x + delta.x;
    let y = p.y + delta.y;
    if x < MIN || x > MAX || y < MIN || y > MAX {
        return Err(());
    }
    p.x = x;
    p.y = y;
    Ok(())
}

pub fn assign_checked_add(p: &mut PointF32, delta: &PointF32) {
    try_assign_checked_add(p, delta).unwrap()
}

pub fn saturating_add(p: &PointF32, delta: &PointF32) -> PointF32 {
    let temp_x = p.x + delta.x;
    let temp_y = p.y + delta.y;
    PointF32::of(temp_x.clamp(MIN, MAX), temp_y.clamp(MIN, MAX))
}

pub fn try_checked_add(p: &PointF32, delta: &PointF32) -> Option<PointF32> {
    let x = p.x + delta.x;
    let y = p.y + delta.y;
    if x < MIN || x > MAX || y < MIN || y > MAX {
        return None;
    }
    Some(PointF32 { x, y })
}

pub fn checked_add(p: &PointF32, delta: &PointF32) -> PointF32 {
    try_checked_add(p, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{
        MAX, MIN, PointF32, assign_checked_add, checked_add, delta, delta_x, delta_y, assign_saturating_add, saturating_add,
        try_assign_checked_add, try_checked_add,
    };

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
        let p = PointF32::of(-8_388_608.0, -8_388_608.0);
        assert_eq!(delta(&p, &PointF32::of(-8_388_608.0, -8_388_608.0)), PointF32::of(0.0, 0.0));
        assert_eq!(delta(&p, &PointF32::of(-8_388_608.0, -8_388_607.0)), PointF32::of(0.0, 1.0));
        assert_eq!(delta(&p, &PointF32::of(-8_388_608.0, -8_388_606.0)), PointF32::of(0.0, 2.0));

        assert_eq!(delta(&p, &PointF32::of(-8_388_607.0, -8_388_608.0)), PointF32::of(1.0, 0.0));
        assert_eq!(delta(&p, &PointF32::of(-8_388_607.0, -8_388_607.0)), PointF32::of(1.0, 1.0));
        assert_eq!(delta(&p, &PointF32::of(-8_388_607.0, -8_388_606.0)), PointF32::of(1.0, 2.0));

        assert_eq!(delta(&p, &PointF32::of(-8_388_606.0, -8_388_608.0)), PointF32::of(2.0, 0.0));
        assert_eq!(delta(&p, &PointF32::of(-8_388_606.0, -8_388_607.0)), PointF32::of(2.0, 1.0));
        assert_eq!(delta(&p, &PointF32::of(-8_388_606.0, -8_388_606.0)), PointF32::of(2.0, 2.0));
    }

    #[test]
    fn delta_max() {
        let p = PointF32::of(8_388_605.0, 8_388_605.0);
        assert_eq!(delta(&p, &PointF32::of(8_388_605.0, 8_388_605.0)), PointF32::of(0.0, 0.0));
        assert_eq!(delta(&p, &PointF32::of(8_388_605.0, 8_388_606.0)), PointF32::of(0.0, 1.0));
        assert_eq!(delta(&p, &PointF32::of(8_388_605.0, 8_388_607.0)), PointF32::of(0.0, 2.0));

        assert_eq!(delta(&p, &PointF32::of(8_388_606.0, 8_388_605.0)), PointF32::of(1.0, 0.0));
        assert_eq!(delta(&p, &PointF32::of(8_388_606.0, 8_388_606.0)), PointF32::of(1.0, 1.0));
        assert_eq!(delta(&p, &PointF32::of(8_388_606.0, 8_388_607.0)), PointF32::of(1.0, 2.0));

        assert_eq!(delta(&p, &PointF32::of(8_388_607.0, 8_388_605.0)), PointF32::of(2.0, 0.0));
        assert_eq!(delta(&p, &PointF32::of(8_388_607.0, 8_388_606.0)), PointF32::of(2.0, 1.0));
        assert_eq!(delta(&p, &PointF32::of(8_388_607.0, 8_388_607.0)), PointF32::of(2.0, 2.0));
    }

    #[test]
    fn test_assign_saturating_add() {
        let mut p = PointF32::of(0.0, 0.0);
        assign_saturating_add(&mut p, &PointF32::of(10.0, 15.0));
        assert_eq!(p, PointF32::of(10.0, 15.0));
        assign_saturating_add(&mut p, &PointF32::of(-15.0, -25.0));
        assert_eq!(p, PointF32::of(-5.0, -10.0));
        assign_saturating_add(&mut p, &PointF32::of(2.0, 3.0));
        assert_eq!(p, PointF32::of(-3.0, -7.0));
    }

    #[test]
    fn assign_saturating_add_min_bounds() {
        let mut p = PointF32::of(MIN + 2.0, MIN + 5.0);
        assign_saturating_add(&mut p, &PointF32::of(-10.0, -10.0));
        assert_eq!(p, PointF32::of(MIN, MIN));
    }

    #[test]
    fn assign_saturating_add_max_bounds() {
        let mut p = PointF32::of(MAX - 2.0, MAX - 5.0);
        assign_saturating_add(&mut p, &PointF32::of(10.0, 10.0));
        assert_eq!(p, PointF32::of(MAX, MAX));
    }

    #[test]
    fn assign_saturating_add_min_bounds_min_delta() {
        let mut p = PointF32::of(MIN + 1.0, MIN + 1.0);
        assign_saturating_add(&mut p, &PointF32::min());
        assert_eq!(p, PointF32::of(MIN, MIN));
    }

    #[test]
    fn assign_saturating_add_max_bounds_max_delta() {
        let mut p = PointF32::of(MAX - 1.0, MAX - 1.0);
        assign_saturating_add(&mut p, &PointF32::max());
        assert_eq!(p, PointF32::of(MAX, MAX));
    }

    #[test]
    fn test_try_assign_checked_add() {
        let mut p = PointF32::of(0.0, 0.0);
        assert_eq!(try_assign_checked_add(&mut p, &PointF32::of(10.0, 15.0)), Ok(()));
        assert_eq!(p, PointF32::of(10.0, 15.0));
        assert_eq!(try_assign_checked_add(&mut p, &PointF32::of(-15.0, -25.0)), Ok(()));
        assert_eq!(p, PointF32::of(-5.0, -10.0));
        assert_eq!(try_assign_checked_add(&mut p, &PointF32::of(2.0, 3.0)), Ok(()));
        assert_eq!(p, PointF32::of(-3.0, -7.0));
    }

    #[test]
    fn try_assign_checked_add_min_bounds_err() {
        let mut p = PointF32::of(MIN + 2.0, MIN + 5.0);
        assert_eq!(try_assign_checked_add(&mut p, &PointF32::of(-10.0, -10.0)), Err(()));
        assert_eq!(p, PointF32::of(MIN + 2.0, MIN + 5.0));
    }

    #[test]
    fn try_assign_checked_add_max_bounds_err() {
        let mut p = PointF32::of(MAX - 2.0, MAX - 5.0);
        assert_eq!(try_assign_checked_add(&mut p, &PointF32::of(10.0, 10.0)), Err(()));
        assert_eq!(p, PointF32::of(MAX - 2.0, MAX - 5.0));
    }

    #[test]
    fn try_assign_checked_add_min_bounds_ok() {
        let mut p = PointF32::of(MIN + 2.0, MIN + 5.0);
        assert_eq!(try_assign_checked_add(&mut p, &PointF32::of(-2.0, -5.0)), Ok(()));
        assert_eq!(p, PointF32::of(MIN, MIN));
    }

    #[test]
    fn try_assign_checked_add_max_bounds_ok() {
        let mut p = PointF32::of(MAX - 2.0, MAX - 5.0);
        assert_eq!(try_assign_checked_add(&mut p, &PointF32::of(2.0, 5.0)), Ok(()));
        assert_eq!(p, PointF32::of(MAX, MAX));
    }

    #[test]
    fn try_assign_checked_add_min_bounds_min_delta() {
        let mut p = PointF32::of(MIN + 1.0, MIN + 1.0);
        assert_eq!(try_assign_checked_add(&mut p, &PointF32::min()), Err(()));
        assert_eq!(try_assign_checked_add(&mut p, &PointF32::of(MIN, 0.0)), Err(()));
        assert_eq!(try_assign_checked_add(&mut p, &PointF32::of(0.0, MIN)), Err(()));
        assert_eq!(p, PointF32::of(MIN + 1.0, MIN + 1.0));
    }

    #[test]
    fn try_assign_checked_add_max_bounds_max_delta() {
        let mut p = PointF32::of(MAX - 1.0, MAX - 1.0);
        assert_eq!(try_assign_checked_add(&mut p, &PointF32::max()), Err(()));
        assert_eq!(try_assign_checked_add(&mut p, &PointF32::of(MAX, 0.0)), Err(()));
        assert_eq!(try_assign_checked_add(&mut p, &PointF32::of(0.0, MAX)), Err(()));
        assert_eq!(p, PointF32::of(MAX - 1.0, MAX - 1.0));
    }

    #[test]
    fn test_assign_checked_add() {
        let mut p = PointF32::of(0.0, 0.0);
        assign_checked_add(&mut p, &PointF32::of(10.0, 15.0));
        assert_eq!(p, PointF32::of(10.0, 15.0));
        assign_checked_add(&mut p, &PointF32::of(-15.0, -25.0));
        assert_eq!(p, PointF32::of(-5.0, -10.0));
        assign_checked_add(&mut p, &PointF32::of(2.0, 3.0));
        assert_eq!(p, PointF32::of(-3.0, -7.0));
    }

    #[test]
    fn test_saturating_add() {
        assert_eq!(saturating_add(&PointF32::of(0.0, 0.0), &PointF32::of(10.0, 15.0)), PointF32::of(10.0, 15.0));
        assert_eq!(saturating_add(&PointF32::of(0.0, 0.0), &PointF32::of(-15.0, -25.0)), PointF32::of(-15.0, -25.0));
    }

    #[test]
    fn saturating_add_to_bounds() {
        assert_eq!(saturating_add(&PointF32::of(MIN + 2.0, MIN + 5.0), &PointF32::of(-2.0, -5.0)), PointF32::min());
        assert_eq!(saturating_add(&PointF32::of(MAX - 2.0, MAX - 5.0), &PointF32::of(2.0, 5.0)), PointF32::max());
    }

    #[test]
    fn saturating_add_beyond_bounds() {
        assert_eq!(saturating_add(&PointF32::of(MIN + 2.0, MIN + 5.0), &PointF32::of(-10.0, -10.0)), PointF32::min());
        assert_eq!(saturating_add(&PointF32::of(MAX - 2.0, MAX - 5.0), &PointF32::of(10.0, 10.0)), PointF32::max());
    }

    #[test]
    fn saturating_add_limits() {
        assert_eq!(saturating_add(&PointF32::of(MIN + 1.0, MIN + 1.0), &PointF32::min()), PointF32::min());
        assert_eq!(saturating_add(&PointF32::of(MAX - 1.0, MAX - 1.0), &PointF32::max()), PointF32::max());
    }

    #[test]
    fn try_checked_add_min_bounds() {
        let p = PointF32::of(MIN + 2.0, MIN + 5.0);
        assert_eq!(try_checked_add(&p, &PointF32::of(-2.0, 0.0)), Some(PointF32::of(MIN, MIN + 5.0)));
        assert_eq!(try_checked_add(&p, &PointF32::of(0.0, -5.0)), Some(PointF32::of(MIN + 2.0, MIN)));
        assert_eq!(try_checked_add(&p, &PointF32::of(-2.0, -5.0)), Some(PointF32::min()));
        assert_eq!(try_checked_add(&p, &PointF32::of(-10.0, -10.0)), None);
        assert_eq!(try_checked_add(&p, &PointF32::of(MIN, 0.0)), None);
        assert_eq!(try_checked_add(&p, &PointF32::of(0.0, MIN)), None);
        assert_eq!(try_checked_add(&p, &PointF32::min()), None);
    }

    #[test]
    fn try_checked_add_max_bounds() {
        let p = PointF32::of(MAX - 2.0, MAX - 5.0);
        assert_eq!(try_checked_add(&p, &PointF32::of(2.0, 0.0)), Some(PointF32::of(MAX, MAX - 5.0)));
        assert_eq!(try_checked_add(&p, &PointF32::of(0.0, 5.0)), Some(PointF32::of(MAX - 2.0, MAX)));
        assert_eq!(try_checked_add(&p, &PointF32::of(2.0, 5.0)), Some(PointF32::max()));
        assert_eq!(try_checked_add(&p, &PointF32::of(10.0, 10.0)), None);
        assert_eq!(try_checked_add(&p, &PointF32::of(MAX, 0.0)), None);
        assert_eq!(try_checked_add(&p, &PointF32::of(0.0, MAX)), None);
        assert_eq!(try_checked_add(&p, &PointF32::max()), None);
    }

    #[test]
    fn test_checked_add() {
        assert_eq!(checked_add(&PointF32::of(0.0, 0.0), &PointF32::of(10.0, 15.0)), PointF32::of(10.0, 15.0));
        assert_eq!(checked_add(&PointF32::of(10.0, 15.0), &PointF32::of(-15.0, -25.0)), PointF32::of(-5.0, -10.0));
        assert_eq!(checked_add(&PointF32::of(-5.0, -10.0), &PointF32::of(2.0, 3.0)), PointF32::of(-3.0, -7.0));
    }
}

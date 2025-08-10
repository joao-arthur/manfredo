use crate::matrix::point::{point_i16::PointI16, point_u8::PointU8};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointU16 {
    pub row: u16,
    pub col: u16,
}

impl PointU16 {
    pub fn of(row: u16, col: u16) -> Self {
        PointU16 { row, col }
    }

    pub fn min() -> Self {
        PointU16 { row: 0, col: 0 }
    }

    pub fn max() -> Self {
        PointU16 { row: u16::MAX, col: u16::MAX }
    }
}

impl From<PointU8> for PointU16 {
    fn from(p: PointU8) -> Self {
        PointU16 { row: p.row.into(), col: p.col.into() }
    }
}

impl std::fmt::Display for PointU16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

pub fn delta_row(p1: &PointU16, p2: &PointU16) -> u16 {
    p2.row - p1.row
}

pub fn delta_col(p1: &PointU16, p2: &PointU16) -> u16 {
    p2.col - p1.col
}

pub fn delta(p1: &PointU16, p2: &PointU16) -> PointU16 {
    PointU16 { row: delta_row(p1, p2), col: delta_col(p1, p2) }
}

pub fn assign_saturating_add(p: &mut PointU16, delta: &PointI16) {
    let temp_row = i32::from(p.row) + i32::from(delta.row);
    let temp_col = i32::from(p.col) + i32::from(delta.col);
    let clamped_row = temp_row.clamp(0, i32::from(u16::MAX));
    let clamped_col = temp_col.clamp(0, i32::from(u16::MAX));
    p.row = clamped_row as u16;
    p.col = clamped_col as u16;
}

pub fn try_assign_checked_add(p: &mut PointU16, delta: &PointI16) -> Result<(), ()> {
    let row = u16::try_from(i32::from(p.row) + i32::from(delta.row)).map_err(|_| ())?;
    let col = u16::try_from(i32::from(p.col) + i32::from(delta.col)).map_err(|_| ())?;
    p.row = row;
    p.col = col;
    Ok(())
}

pub fn assign_checked_add(p: &mut PointU16, delta: &PointI16) {
    try_assign_checked_add(p, delta).unwrap()
}

pub fn saturating_add(p: &PointU16, delta: &PointI16) -> PointU16 {
    let temp_row = i32::from(p.row) + i32::from(delta.row);
    let temp_col = i32::from(p.col) + i32::from(delta.col);
    let clamped_row = temp_row.clamp(0, i32::from(u16::MAX));
    let clamped_col = temp_col.clamp(0, i32::from(u16::MAX));
    PointU16::of(clamped_row as u16, clamped_col as u16)
}

pub fn try_checked_add(p: &PointU16, delta: &PointI16) -> Option<PointU16> {
    let row = u16::try_from(i32::from(p.row) + i32::from(delta.row)).ok()?;
    let col = u16::try_from(i32::from(p.col) + i32::from(delta.col)).ok()?;
    Some(PointU16 { row, col })
}

pub fn checked_add(p: &PointU16, delta: &PointI16) -> PointU16 {
    try_checked_add(p, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::matrix::point::{point_i16::PointI16, point_u8::PointU8};

    use super::{
        PointU16, assign_checked_add, checked_add, delta, delta_col, delta_row, assign_saturating_add, saturating_add,
        try_assign_checked_add, try_checked_add,
    };

    #[test]
    fn point_u16() {
        assert_eq!(PointU16::of(0, u16::MAX), PointU16 { row: 0, col: u16::MAX });
        assert_eq!(PointU16::min(), PointU16 { row: 0, col: 0 });
        assert_eq!(PointU16::max(), PointU16 { row: u16::MAX, col: u16::MAX });
    }

    #[test]
    fn from() {
        assert_eq!(PointU16::from(PointU8::min()), PointU16 { row: u8::MIN.into(), col: u8::MIN.into() });
        assert_eq!(PointU16::from(PointU8::max()), PointU16 { row: u8::MAX.into(), col: u8::MAX.into() });
    }

    #[test]
    fn to_string() {
        assert_eq!(PointU16::of(0, u16::MAX).to_string(), "(0, 65535)");
        assert_eq!(PointU16::min().to_string(), "(0, 0)");
        assert_eq!(PointU16::max().to_string(), "(65535, 65535)");
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&PointU16::min(), &PointU16::of(0, u16::MAX)), 0);
        assert_eq!(delta_row(&PointU16::min(), &PointU16::of(u16::MAX, 0)), u16::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&PointU16::min(), &PointU16::of(u16::MAX, 0)), 0);
        assert_eq!(delta_col(&PointU16::min(), &PointU16::of(0, u16::MAX)), u16::MAX);
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

    #[test]
    fn test_assign_saturating_add() {
        let mut p = PointU16::of(0, 0);
        assign_saturating_add(&mut p, &PointI16::of(10, 13));
        assert_eq!(p, PointU16::of(10, 13));
        assign_saturating_add(&mut p, &PointI16::of(-5, -3));
        assert_eq!(p, PointU16::of(5, 10));
        assign_saturating_add(&mut p, &PointI16::of(2, -4));
        assert_eq!(p, PointU16::of(7, 6));
    }

    #[test]
    fn assign_saturating_add_min_bounds() {
        let mut p = PointU16::of(2, 5);
        assign_saturating_add(&mut p, &PointI16::of(-10, -10));
        assert_eq!(p, PointU16::of(0, 0));
    }

    #[test]
    fn assign_saturating_add_max_bounds() {
        let mut p = PointU16::of(u16::MAX - 2, u16::MAX - 5);
        assign_saturating_add(&mut p, &PointI16::of(10, 10));
        assert_eq!(p, PointU16::max());
    }

    #[test]
    fn assign_saturating_add_min_bounds_min_delta() {
        let mut p = PointU16::of(1, 1);
        assign_saturating_add(&mut p, &PointI16::min());
        assert_eq!(p, PointU16::of(0, 0));
    }

    #[test]
    fn assign_saturating_add_max_bounds_max_delta() {
        let mut p = PointU16::of(u16::MAX - 1, u16::MAX - 1);
        assign_saturating_add(&mut p, &PointI16::max());
        assert_eq!(p, PointU16::max());
    }

    #[test]
    fn test_try_assign_checked_add() {
        let mut p = PointU16::of(0, 0);
        assert_eq!(try_assign_checked_add(&mut p, &PointI16::of(10, 13)), Ok(()));
        assert_eq!(p, PointU16::of(10, 13));
        assert_eq!(try_assign_checked_add(&mut p, &PointI16::of(-5, -3)), Ok(()));
        assert_eq!(p, PointU16::of(5, 10));
        assert_eq!(try_assign_checked_add(&mut p, &PointI16::of(2, -4)), Ok(()));
        assert_eq!(p, PointU16::of(7, 6));
    }

    #[test]
    fn try_assign_checked_add_min_bounds_err() {
        let mut p = PointU16::of(2, 5);
        assert_eq!(try_assign_checked_add(&mut p, &PointI16::of(-10, -10)), Err(()));
        assert_eq!(p, PointU16::of(2, 5));
    }

    #[test]
    fn try_assign_checked_add_max_bounds_err() {
        let mut p = PointU16::of(u16::MAX - 2, u16::MAX - 5);
        assert_eq!(try_assign_checked_add(&mut p, &PointI16::of(10, 10)), Err(()));
        assert_eq!(p, PointU16::of(u16::MAX - 2, u16::MAX - 5));
    }

    #[test]
    fn try_assign_checked_add_min_bounds_ok() {
        let mut p = PointU16::of(2, 5);
        assert_eq!(try_assign_checked_add(&mut p, &PointI16::of(-2, -5)), Ok(()));
        assert_eq!(p, PointU16::of(0, 0));
    }

    #[test]
    fn try_assign_checked_add_max_bounds_ok() {
        let mut p = PointU16::of(u16::MAX - 2, u16::MAX - 5);
        assert_eq!(try_assign_checked_add(&mut p, &PointI16::of(2, 5)), Ok(()));
        assert_eq!(p, PointU16::max());
    }

    #[test]
    fn try_assign_checked_add_min_bounds_min_delta() {
        let mut p = PointU16::of(1, 1);
        assert_eq!(try_assign_checked_add(&mut p, &PointI16::min()), Err(()));
        assert_eq!(try_assign_checked_add(&mut p, &PointI16::of(i16::MIN, 0)), Err(()));
        assert_eq!(try_assign_checked_add(&mut p, &PointI16::of(0, i16::MIN)), Err(()));
        assert_eq!(p, PointU16::of(1, 1));
    }

    #[test]
    fn try_assign_checked_add_max_bounds_max_delta() {
        let mut p = PointU16::of(u16::MAX - 1, u16::MAX - 1);
        assert_eq!(try_assign_checked_add(&mut p, &PointI16::max()), Err(()));
        assert_eq!(try_assign_checked_add(&mut p, &PointI16::of(i16::MAX, 0)), Err(()));
        assert_eq!(try_assign_checked_add(&mut p, &PointI16::of(0, i16::MAX)), Err(()));
        assert_eq!(p, PointU16::of(u16::MAX - 1, u16::MAX - 1));
    }

    #[test]
    fn test_assign_checked_add() {
        let mut p = PointU16::of(0, 0);
        assign_checked_add(&mut p, &PointI16::of(10, 13));
        assert_eq!(p, PointU16::of(10, 13));
        assign_checked_add(&mut p, &PointI16::of(-5, -3));
        assert_eq!(p, PointU16::of(5, 10));
        assign_checked_add(&mut p, &PointI16::of(2, -4));
        assert_eq!(p, PointU16::of(7, 6));
    }

    #[test]
    fn test_saturating_add() {
        assert_eq!(saturating_add(&PointU16::of(0, 0), &PointI16::of(10, 13)), PointU16::of(10, 13));
        assert_eq!(saturating_add(&PointU16::of(10, 10), &PointI16::of(-5, -3)), PointU16::of(5, 7));
    }

    #[test]
    fn saturating_add_to_bounds() {
        assert_eq!(saturating_add(&PointU16::of(2, 5), &PointI16::of(-2, -5)), PointU16::min());
        assert_eq!(saturating_add(&PointU16::of(u16::MAX - 2, u16::MAX - 5), &PointI16::of(2, 5)), PointU16::max());
    }

    #[test]
    fn saturating_add_beyond_bounds() {
        assert_eq!(saturating_add(&PointU16::of(2, 5), &PointI16::of(-10, -10)), PointU16::min());
        assert_eq!(saturating_add(&PointU16::of(u16::MAX - 2, u16::MAX - 5), &PointI16::of(10, 10)), PointU16::max());
    }

    #[test]
    fn saturating_add_limits() {
        assert_eq!(saturating_add(&PointU16::of(1, 1), &PointI16::min()), PointU16::min());
        assert_eq!(saturating_add(&PointU16::of(u16::MAX - 1, u16::MAX - 1), &PointI16::max()), PointU16::max());
    }

    #[test]
    fn try_checked_add_min_bounds() {
        let p = PointU16::of(2, 5);
        assert_eq!(try_checked_add(&p, &PointI16::of(-2, 0)), Some(PointU16::of(0, 5)));
        assert_eq!(try_checked_add(&p, &PointI16::of(0, -5)), Some(PointU16::of(2, 0)));
        assert_eq!(try_checked_add(&p, &PointI16::of(-2, -5)), Some(PointU16::min()));
        assert_eq!(try_checked_add(&p, &PointI16::of(-10, -10)), None);
        assert_eq!(try_checked_add(&p, &PointI16::of(i16::MIN, 0)), None);
        assert_eq!(try_checked_add(&p, &PointI16::of(0, i16::MIN)), None);
        assert_eq!(try_checked_add(&p, &PointI16::min()), None);
    }

    #[test]
    fn try_checked_add_max_bounds() {
        let p = PointU16::of(u16::MAX - 2, u16::MAX - 5);
        assert_eq!(try_checked_add(&p, &PointI16::of(2, 0)), Some(PointU16::of(u16::MAX, u16::MAX - 5)));
        assert_eq!(try_checked_add(&p, &PointI16::of(0, 5)), Some(PointU16::of(u16::MAX - 2, u16::MAX)));
        assert_eq!(try_checked_add(&p, &PointI16::of(2, 5)), Some(PointU16::max()));
        assert_eq!(try_checked_add(&p, &PointI16::of(10, 10)), None);
        assert_eq!(try_checked_add(&p, &PointI16::of(i16::MAX, 0)), None);
        assert_eq!(try_checked_add(&p, &PointI16::of(0, i16::MAX)), None);
        assert_eq!(try_checked_add(&p, &PointI16::max()), None);
    }

    #[test]
    fn test_checked_add() {
        assert_eq!(checked_add(&PointU16::of(0, 0), &PointI16::of(10, 13)), PointU16::of(10, 13));
        assert_eq!(checked_add(&PointU16::of(10, 13), &PointI16::of(-5, -3)), PointU16::of(5, 10));
        assert_eq!(checked_add(&PointU16::of(5, 10), &PointI16::of(2, -4)), PointU16::of(7, 6));
    }
}

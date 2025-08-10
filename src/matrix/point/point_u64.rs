use crate::matrix::point::{point_i64::PointI64, point_u8::PointU8, point_u16::PointU16, point_u32::PointU32};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointU64 {
    pub row: u64,
    pub col: u64,
}

impl PointU64 {
    pub fn of(row: u64, col: u64) -> Self {
        PointU64 { row, col }
    }

    pub fn min() -> Self {
        PointU64 { row: 0, col: 0 }
    }

    pub fn max() -> Self {
        PointU64 { row: u64::MAX, col: u64::MAX }
    }
}

impl From<PointU8> for PointU64 {
    fn from(p: PointU8) -> Self {
        PointU64 { row: p.row.into(), col: p.col.into() }
    }
}

impl From<PointU16> for PointU64 {
    fn from(p: PointU16) -> Self {
        PointU64 { row: p.row.into(), col: p.col.into() }
    }
}

impl From<PointU32> for PointU64 {
    fn from(p: PointU32) -> Self {
        PointU64 { row: p.row.into(), col: p.col.into() }
    }
}

impl std::fmt::Display for PointU64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

pub fn delta_row(p1: &PointU64, p2: &PointU64) -> u64 {
    p2.row - p1.row
}

pub fn delta_col(p1: &PointU64, p2: &PointU64) -> u64 {
    p2.col - p1.col
}

pub fn delta(p1: &PointU64, p2: &PointU64) -> PointU64 {
    PointU64 { row: delta_row(p1, p2), col: delta_col(p1, p2) }
}

pub fn assign_saturating_add(p: &mut PointU64, delta: &PointI64) {
    let temp_row = i128::from(p.row) + i128::from(delta.row);
    let temp_col = i128::from(p.col) + i128::from(delta.col);
    let clamped_row = temp_row.clamp(0, i128::from(u64::MAX));
    let clamped_col = temp_col.clamp(0, i128::from(u64::MAX));
    p.row = clamped_row as u64;
    p.col = clamped_col as u64;
}

pub fn try_assign_checked_add(p: &mut PointU64, delta: &PointI64) -> Result<(), ()> {
    let row = u64::try_from(i128::from(p.row) + i128::from(delta.row)).map_err(|_| ())?;
    let col = u64::try_from(i128::from(p.col) + i128::from(delta.col)).map_err(|_| ())?;
    p.row = row;
    p.col = col;
    Ok(())
}

pub fn assign_checked_add(p: &mut PointU64, delta: &PointI64) {
    try_assign_checked_add(p, delta).unwrap()
}

pub fn saturating_add(p: &PointU64, delta: &PointI64) -> PointU64 {
    let temp_row = i128::from(p.row) + i128::from(delta.row);
    let temp_col = i128::from(p.col) + i128::from(delta.col);
    let clamped_row = temp_row.clamp(0, i128::from(u64::MAX));
    let clamped_col = temp_col.clamp(0, i128::from(u64::MAX));
    PointU64::of(clamped_row as u64, clamped_col as u64)
}

pub fn try_checked_add(p: &PointU64, delta: &PointI64) -> Option<PointU64> {
    let row = u64::try_from(i128::from(p.row) + i128::from(delta.row)).ok()?;
    let col = u64::try_from(i128::from(p.col) + i128::from(delta.col)).ok()?;
    Some(PointU64 { row, col })
}

pub fn checked_add(p: &PointU64, delta: &PointI64) -> PointU64 {
    try_checked_add(p, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::matrix::point::{point_i64::PointI64, point_u8::PointU8, point_u16::PointU16, point_u32::PointU32};

    use super::{
        PointU64, assign_checked_add, checked_add, delta, delta_col, delta_row, assign_saturating_add, saturating_add,
        try_assign_checked_add, try_checked_add,
    };

    #[test]
    fn point_u64() {
        assert_eq!(PointU64::of(0, u64::MAX), PointU64 { row: 0, col: u64::MAX });
        assert_eq!(PointU64::min(), PointU64 { row: 0, col: 0 });
        assert_eq!(PointU64::max(), PointU64 { row: u64::MAX, col: u64::MAX });
    }

    #[test]
    fn from() {
        assert_eq!(PointU64::from(PointU8::min()), PointU64 { row: u8::MIN.into(), col: u8::MIN.into() });
        assert_eq!(PointU64::from(PointU8::max()), PointU64 { row: u8::MAX.into(), col: u8::MAX.into() });
        assert_eq!(PointU64::from(PointU16::min()), PointU64 { row: u16::MIN.into(), col: u16::MIN.into() });
        assert_eq!(PointU64::from(PointU16::max()), PointU64 { row: u16::MAX.into(), col: u16::MAX.into() });
        assert_eq!(PointU64::from(PointU32::min()), PointU64 { row: u32::MIN.into(), col: u32::MIN.into() });
        assert_eq!(PointU64::from(PointU32::max()), PointU64 { row: u32::MAX.into(), col: u32::MAX.into() });
    }

    #[test]
    fn to_string() {
        assert_eq!(PointU64::of(0, u64::MAX).to_string(), "(0, 18446744073709551615)");
        assert_eq!(PointU64::min().to_string(), "(0, 0)");
        assert_eq!(PointU64::max().to_string(), "(18446744073709551615, 18446744073709551615)");
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&PointU64::min(), &PointU64::of(0, u64::MAX)), 0);
        assert_eq!(delta_row(&PointU64::min(), &PointU64::of(u64::MAX, 0)), u64::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&PointU64::min(), &PointU64::of(u64::MAX, 0)), 0);
        assert_eq!(delta_col(&PointU64::min(), &PointU64::of(0, u64::MAX)), u64::MAX);
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

    #[test]
    fn test_assign_saturating_add() {
        let mut p = PointU64::of(0, 0);
        assign_saturating_add(&mut p, &PointI64::of(10, 13));
        assert_eq!(p, PointU64::of(10, 13));
        assign_saturating_add(&mut p, &PointI64::of(-5, -3));
        assert_eq!(p, PointU64::of(5, 10));
        assign_saturating_add(&mut p, &PointI64::of(2, -4));
        assert_eq!(p, PointU64::of(7, 6));
    }

    #[test]
    fn assign_saturating_add_min_bounds() {
        let mut p = PointU64::of(2, 5);
        assign_saturating_add(&mut p, &PointI64::of(-10, -10));
        assert_eq!(p, PointU64::of(0, 0));
    }

    #[test]
    fn assign_saturating_add_max_bounds() {
        let mut p = PointU64::of(u64::MAX - 2, u64::MAX - 5);
        assign_saturating_add(&mut p, &PointI64::of(10, 10));
        assert_eq!(p, PointU64::max());
    }

    #[test]
    fn assign_saturating_add_min_bounds_min_delta() {
        let mut p = PointU64::of(1, 1);
        assign_saturating_add(&mut p, &PointI64::min());
        assert_eq!(p, PointU64::of(0, 0));
    }

    #[test]
    fn assign_saturating_add_max_bounds_max_delta() {
        let mut p = PointU64::of(u64::MAX - 1, u64::MAX - 1);
        assign_saturating_add(&mut p, &PointI64::max());
        assert_eq!(p, PointU64::max());
    }

    #[test]
    fn test_try_assign_checked_add() {
        let mut p = PointU64::of(0, 0);
        assert_eq!(try_assign_checked_add(&mut p, &PointI64::of(10, 10)), Ok(()));
        assert_eq!(p, PointU64::of(10, 10));
        assert_eq!(try_assign_checked_add(&mut p, &PointI64::of(-5, -5)), Ok(()));
        assert_eq!(p, PointU64::of(5, 5));
        assert_eq!(try_assign_checked_add(&mut p, &PointI64::of(2, 2)), Ok(()));
        assert_eq!(p, PointU64::of(7, 7));
    }

    #[test]
    fn try_assign_checked_add_min_bounds_err() {
        let mut p = PointU64::of(2, 5);
        assert_eq!(try_assign_checked_add(&mut p, &PointI64::of(-10, -10)), Err(()));
        assert_eq!(p, PointU64::of(2, 5));
    }

    #[test]
    fn try_assign_checked_add_max_bounds_err() {
        let mut p = PointU64::of(u64::MAX - 2, u64::MAX - 5);
        assert_eq!(try_assign_checked_add(&mut p, &PointI64::of(10, 10)), Err(()));
        assert_eq!(p, PointU64::of(u64::MAX - 2, u64::MAX - 5));
    }

    #[test]
    fn try_assign_checked_add_min_bounds_ok() {
        let mut p = PointU64::of(2, 5);
        assert_eq!(try_assign_checked_add(&mut p, &PointI64::of(-2, -5)), Ok(()));
        assert_eq!(p, PointU64::of(0, 0));
    }

    #[test]
    fn try_assign_checked_add_max_bounds_ok() {
        let mut p = PointU64::of(u64::MAX - 2, u64::MAX - 5);
        assert_eq!(try_assign_checked_add(&mut p, &PointI64::of(2, 5)), Ok(()));
        assert_eq!(p, PointU64::max());
    }

    #[test]
    fn try_assign_checked_add_min_bounds_min_delta() {
        let mut p = PointU64::of(1, 1);
        assert_eq!(try_assign_checked_add(&mut p, &PointI64::min()), Err(()));
        assert_eq!(try_assign_checked_add(&mut p, &PointI64::of(i64::MIN, 0)), Err(()));
        assert_eq!(try_assign_checked_add(&mut p, &PointI64::of(0, i64::MIN)), Err(()));
        assert_eq!(p, PointU64::of(1, 1));
    }

    #[test]
    fn try_assign_checked_add_max_bounds_max_delta() {
        let mut p = PointU64::of(u64::MAX - 1, u64::MAX - 1);
        assert_eq!(try_assign_checked_add(&mut p, &PointI64::max()), Err(()));
        assert_eq!(try_assign_checked_add(&mut p, &PointI64::of(i64::MAX, 0)), Err(()));
        assert_eq!(try_assign_checked_add(&mut p, &PointI64::of(0, i64::MAX)), Err(()));
        assert_eq!(p, PointU64::of(u64::MAX - 1, u64::MAX - 1));
    }

    #[test]
    fn test_assign_checked_add() {
        let mut p = PointU64::of(0, 0);
        assign_checked_add(&mut p, &PointI64::of(10, 13));
        assert_eq!(p, PointU64::of(10, 13));
        assign_checked_add(&mut p, &PointI64::of(-5, -3));
        assert_eq!(p, PointU64::of(5, 10));
        assign_checked_add(&mut p, &PointI64::of(2, -4));
        assert_eq!(p, PointU64::of(7, 6));
    }

    #[test]
    fn test_saturating_add() {
        assert_eq!(saturating_add(&PointU64::of(0, 0), &PointI64::of(10, 13)), PointU64::of(10, 13));
        assert_eq!(saturating_add(&PointU64::of(10, 10), &PointI64::of(-5, -3)), PointU64::of(5, 7));
    }

    #[test]
    fn saturating_add_to_bounds() {
        assert_eq!(saturating_add(&PointU64::of(2, 5), &PointI64::of(-2, -5)), PointU64::min());
        assert_eq!(saturating_add(&PointU64::of(u64::MAX - 2, u64::MAX - 5), &PointI64::of(2, 5)), PointU64::max());
    }

    #[test]
    fn saturating_add_beyond_bounds() {
        assert_eq!(saturating_add(&PointU64::of(2, 5), &PointI64::of(-10, -10)), PointU64::min());
        assert_eq!(saturating_add(&PointU64::of(u64::MAX - 2, u64::MAX - 5), &PointI64::of(10, 10)), PointU64::max());
    }

    #[test]
    fn saturating_add_limits() {
        assert_eq!(saturating_add(&PointU64::of(1, 1), &PointI64::min()), PointU64::min());
        assert_eq!(saturating_add(&PointU64::of(u64::MAX - 1, u64::MAX - 1), &PointI64::max()), PointU64::max());
    }

    #[test]
    fn try_checked_add_min_bounds() {
        let p = PointU64::of(2, 5);
        assert_eq!(try_checked_add(&p, &PointI64::of(-2, 0)), Some(PointU64::of(0, 5)));
        assert_eq!(try_checked_add(&p, &PointI64::of(0, -5)), Some(PointU64::of(2, 0)));
        assert_eq!(try_checked_add(&p, &PointI64::of(-2, -5)), Some(PointU64::min()));
        assert_eq!(try_checked_add(&p, &PointI64::of(-10, -10)), None);
        assert_eq!(try_checked_add(&p, &PointI64::of(i64::MIN, 0)), None);
        assert_eq!(try_checked_add(&p, &PointI64::of(0, i64::MIN)), None);
        assert_eq!(try_checked_add(&p, &PointI64::min()), None);
    }

    #[test]
    fn try_checked_add_max_bounds() {
        let p = PointU64::of(u64::MAX - 2, u64::MAX - 5);
        assert_eq!(try_checked_add(&p, &PointI64::of(2, 0)), Some(PointU64::of(u64::MAX, u64::MAX - 5)));
        assert_eq!(try_checked_add(&p, &PointI64::of(0, 5)), Some(PointU64::of(u64::MAX - 2, u64::MAX)));
        assert_eq!(try_checked_add(&p, &PointI64::of(2, 5)), Some(PointU64::max()));
        assert_eq!(try_checked_add(&p, &PointI64::of(10, 10)), None);
        assert_eq!(try_checked_add(&p, &PointI64::of(i64::MAX, 0)), None);
        assert_eq!(try_checked_add(&p, &PointI64::of(0, i64::MAX)), None);
        assert_eq!(try_checked_add(&p, &PointI64::max()), None);
    }

    #[test]
    fn test_checked_add() {
        assert_eq!(checked_add(&PointU64::of(0, 0), &PointI64::of(10, 13)), PointU64::of(10, 13));
        assert_eq!(checked_add(&PointU64::of(10, 13), &PointI64::of(-5, -3)), PointU64::of(5, 10));
        assert_eq!(checked_add(&PointU64::of(5, 10), &PointI64::of(2, -4)), PointU64::of(7, 6));
    }
}

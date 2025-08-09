use crate::matrix::point::point_i32::PointI32;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointU32 {
    pub row: u32,
    pub col: u32,
}

impl PointU32 {
    pub fn of(row: u32, col: u32) -> Self {
        PointU32 { row, col }
    }

    pub fn min() -> Self {
        PointU32 { row: 0, col: 0 }
    }

    pub fn max() -> Self {
        PointU32 { row: u32::MAX, col: u32::MAX }
    }
}

impl std::fmt::Display for PointU32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

pub fn delta_row(p1: &PointU32, p2: &PointU32) -> u32 {
    p2.row - p1.row
}

pub fn delta_col(p1: &PointU32, p2: &PointU32) -> u32 {
    p2.col - p1.col
}

pub fn delta(p1: &PointU32, p2: &PointU32) -> PointU32 {
    PointU32 { row: delta_row(p1, p2), col: delta_col(p1, p2) }
}

pub fn saturating_translate(p: &mut PointU32, delta: &PointI32) {
    let temp_row = i64::from(p.row) + i64::from(delta.row);
    let temp_col = i64::from(p.col) + i64::from(delta.col);
    let clamped_row = temp_row.clamp(0, i64::from(u32::MAX));
    let clamped_col = temp_col.clamp(0, i64::from(u32::MAX));
    p.row = clamped_row as u32;
    p.col = clamped_col as u32;
}

pub fn checked_translate(p: &mut PointU32, delta: &PointI32) -> Result<(), ()> {
    let row = u32::try_from(i64::from(p.row) + i64::from(delta.row)).map_err(|_| ())?;
    let col = u32::try_from(i64::from(p.col) + i64::from(delta.col)).map_err(|_| ())?;
    p.row = row;
    p.col = col;
    Ok(())
}

pub fn saturating_translated(p: &PointU32, delta: &PointI32) -> PointU32 {
    let temp_row = i64::from(p.row) + i64::from(delta.row);
    let temp_col = i64::from(p.col) + i64::from(delta.col);
    let clamped_row = temp_row.clamp(0, i64::from(u32::MAX));
    let clamped_col = temp_col.clamp(0, i64::from(u32::MAX));
    PointU32::of(clamped_row as u32, clamped_col as u32)
}

pub fn checked_translated(p: &PointU32, delta: &PointI32) -> Option<PointU32> {
    let row = u32::try_from(i64::from(p.row) + i64::from(delta.row)).ok()?;
    let col = u32::try_from(i64::from(p.col) + i64::from(delta.col)).ok()?;
    Some(PointU32 { row, col })
}

#[cfg(test)]
mod tests {
    use crate::matrix::point::point_i32::PointI32;

    use super::{PointU32, checked_translate, checked_translated, delta, delta_col, delta_row, saturating_translate, saturating_translated};

    #[test]
    fn point_u32() {
        assert_eq!(PointU32::of(0, u32::MAX), PointU32 { row: 0, col: u32::MAX });
        assert_eq!(PointU32::min(), PointU32 { row: 0, col: 0 });
        assert_eq!(PointU32::max(), PointU32 { row: u32::MAX, col: u32::MAX });
        assert_eq!(PointU32::of(0, u32::MAX).to_string(), "(0, 4294967295)");
        assert_eq!(PointU32::min().to_string(), "(0, 0)");
        assert_eq!(PointU32::max().to_string(), "(4294967295, 4294967295)");
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&PointU32::min(), &PointU32::of(0, u32::MAX)), 0);
        assert_eq!(delta_row(&PointU32::min(), &PointU32::of(u32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&PointU32::min(), &PointU32::of(u32::MAX, 0)), 0);
        assert_eq!(delta_col(&PointU32::min(), &PointU32::of(0, u32::MAX)), u32::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointU32::min(), &PointU32::min()), PointU32::min());
        assert_eq!(delta(&PointU32::min(), &PointU32::max()), PointU32::max());
    }

    #[test]
    fn delta_min() {
        let p = PointU32::min();
        assert_eq!(delta(&p, &PointU32::of(0, 0)), PointU32::of(0, 0));
        assert_eq!(delta(&p, &PointU32::of(0, 1)), PointU32::of(0, 1));
        assert_eq!(delta(&p, &PointU32::of(0, 2)), PointU32::of(0, 2));

        assert_eq!(delta(&p, &PointU32::of(1, 0)), PointU32::of(1, 0));
        assert_eq!(delta(&p, &PointU32::of(1, 1)), PointU32::of(1, 1));
        assert_eq!(delta(&p, &PointU32::of(1, 2)), PointU32::of(1, 2));

        assert_eq!(delta(&p, &PointU32::of(2, 0)), PointU32::of(2, 0));
        assert_eq!(delta(&p, &PointU32::of(2, 1)), PointU32::of(2, 1));
        assert_eq!(delta(&p, &PointU32::of(2, 2)), PointU32::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p = PointU32::of(u32::MAX - 2, u32::MAX - 2);
        assert_eq!(delta(&p, &PointU32::of(u32::MAX - 2, u32::MAX - 2)), PointU32::of(0, 0));
        assert_eq!(delta(&p, &PointU32::of(u32::MAX - 2, u32::MAX - 1)), PointU32::of(0, 1));
        assert_eq!(delta(&p, &PointU32::of(u32::MAX - 2, u32::MAX)), PointU32::of(0, 2));

        assert_eq!(delta(&p, &PointU32::of(u32::MAX - 1, u32::MAX - 2)), PointU32::of(1, 0));
        assert_eq!(delta(&p, &PointU32::of(u32::MAX - 1, u32::MAX - 1)), PointU32::of(1, 1));
        assert_eq!(delta(&p, &PointU32::of(u32::MAX - 1, u32::MAX)), PointU32::of(1, 2));

        assert_eq!(delta(&p, &PointU32::of(u32::MAX, u32::MAX - 2)), PointU32::of(2, 0));
        assert_eq!(delta(&p, &PointU32::of(u32::MAX, u32::MAX - 1)), PointU32::of(2, 1));
        assert_eq!(delta(&p, &PointU32::max()), PointU32::of(2, 2));
    }

    #[test]
    fn test_saturating_translate() {
        let mut p = PointU32::of(0, 0);
        saturating_translate(&mut p, &PointI32::of(10, 13));
        assert_eq!(p, PointU32::of(10, 13));
        saturating_translate(&mut p, &PointI32::of(-5, -3));
        assert_eq!(p, PointU32::of(5, 10));
        saturating_translate(&mut p, &PointI32::of(2, -4));
        assert_eq!(p, PointU32::of(7, 6));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut p = PointU32::of(2, 5);
        saturating_translate(&mut p, &PointI32::of(-10, -10));
        assert_eq!(p, PointU32::of(0, 0));
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut p = PointU32::of(u32::MAX - 2, u32::MAX - 5);
        saturating_translate(&mut p, &PointI32::of(10, 10));
        assert_eq!(p, PointU32::max());
    }

    #[test]
    fn saturating_translate_min_bounds_min_delta() {
        let mut p = PointU32::of(1, 1);
        saturating_translate(&mut p, &PointI32::min());
        assert_eq!(p, PointU32::of(0, 0));
    }

    #[test]
    fn saturating_translate_max_bounds_max_delta() {
        let mut p = PointU32::of(u32::MAX - 1, u32::MAX - 1);
        saturating_translate(&mut p, &PointI32::max());
        assert_eq!(p, PointU32::max());
    }

    #[test]
    fn test_checked_translate() {
        let mut p = PointU32::of(0, 0);
        assert_eq!(checked_translate(&mut p, &PointI32::of(10, 13)), Ok(()));
        assert_eq!(p, PointU32::of(10, 13));
        assert_eq!(checked_translate(&mut p, &PointI32::of(-5, -3)), Ok(()));
        assert_eq!(p, PointU32::of(5, 10));
        assert_eq!(checked_translate(&mut p, &PointI32::of(2, -4)), Ok(()));
        assert_eq!(p, PointU32::of(7, 6));
    }

    #[test]
    fn checked_translate_min_bounds_err() {
        let mut p = PointU32::of(2, 5);
        assert_eq!(checked_translate(&mut p, &PointI32::of(-10, -10)), Err(()));
        assert_eq!(p, PointU32::of(2, 5));
    }

    #[test]
    fn checked_translate_max_bounds_err() {
        let mut p = PointU32::of(u32::MAX - 2, u32::MAX - 5);
        assert_eq!(checked_translate(&mut p, &PointI32::of(10, 10)), Err(()));
        assert_eq!(p, PointU32::of(u32::MAX - 2, u32::MAX - 5));
    }

    #[test]
    fn checked_translate_min_bounds_ok() {
        let mut p = PointU32::of(2, 5);
        assert_eq!(checked_translate(&mut p, &PointI32::of(-2, -5)), Ok(()));
        assert_eq!(p, PointU32::of(0, 0));
    }

    #[test]
    fn checked_translate_max_bounds_ok() {
        let mut p = PointU32::of(u32::MAX - 2, u32::MAX - 5);
        assert_eq!(checked_translate(&mut p, &PointI32::of(2, 5)), Ok(()));
        assert_eq!(p, PointU32::max());
    }

    #[test]
    fn checked_translate_min_bounds_min_delta() {
        let mut p = PointU32::of(1, 1);
        assert_eq!(checked_translate(&mut p, &PointI32::min()), Err(()));
        assert_eq!(checked_translate(&mut p, &PointI32::of(i32::MIN, 0)), Err(()));
        assert_eq!(checked_translate(&mut p, &PointI32::of(0, i32::MIN)), Err(()));
        assert_eq!(p, PointU32::of(1, 1));
    }

    #[test]
    fn checked_translate_max_bounds_max_delta() {
        let mut p = PointU32::of(u32::MAX - 1, u32::MAX - 1);
        assert_eq!(checked_translate(&mut p, &PointI32::max()), Err(()));
        assert_eq!(checked_translate(&mut p, &PointI32::of(i32::MAX, 0)), Err(()));
        assert_eq!(checked_translate(&mut p, &PointI32::of(0, i32::MAX)), Err(()));
        assert_eq!(p, PointU32::of(u32::MAX - 1, u32::MAX - 1));
    }

    #[test]
    fn test_saturating_translated() {
        assert_eq!(saturating_translated(&PointU32::of(0, 0), &PointI32::of(10, 13)), PointU32::of(10, 13));
        assert_eq!(saturating_translated(&PointU32::of(10, 10), &PointI32::of(-5, -3)), PointU32::of(5, 7));
    }

    #[test]
    fn saturating_translated_to_bounds() {
        assert_eq!(saturating_translated(&PointU32::of(2, 5), &PointI32::of(-2, -5)), PointU32::min());
        assert_eq!(saturating_translated(&PointU32::of(u32::MAX - 2, u32::MAX - 5), &PointI32::of(2, 5)), PointU32::max());
    }

    #[test]
    fn saturating_translated_beyond_bounds() {
        assert_eq!(saturating_translated(&PointU32::of(2, 5), &PointI32::of(-10, -10)), PointU32::min());
        assert_eq!(saturating_translated(&PointU32::of(u32::MAX - 2, u32::MAX - 5), &PointI32::of(10, 10)), PointU32::max());
    }

    #[test]
    fn saturating_translated_limits() {
        assert_eq!(saturating_translated(&PointU32::of(1, 1), &PointI32::min()), PointU32::min());
        assert_eq!(saturating_translated(&PointU32::of(u32::MAX - 1, u32::MAX - 1), &PointI32::max()), PointU32::max());
    }

    #[test]
    fn checked_translated_min_bounds() {
        let p = PointU32::of(2, 5);
        assert_eq!(checked_translated(&p, &PointI32::of(-2, 0)), Some(PointU32::of(0, 5)));
        assert_eq!(checked_translated(&p, &PointI32::of(0, -5)), Some(PointU32::of(2, 0)));
        assert_eq!(checked_translated(&p, &PointI32::of(-2, -5)), Some(PointU32::min()));
        assert_eq!(checked_translated(&p, &PointI32::of(-10, -10)), None);
        assert_eq!(checked_translated(&p, &PointI32::of(i32::MIN, 0)), None);
        assert_eq!(checked_translated(&p, &PointI32::of(0, i32::MIN)), None);
        assert_eq!(checked_translated(&p, &PointI32::min()), None);
    }

    #[test]
    fn checked_translated_max_bounds() {
        let p = PointU32::of(u32::MAX - 2, u32::MAX - 5);
        assert_eq!(checked_translated(&p, &PointI32::of(2, 0)), Some(PointU32::of(u32::MAX, u32::MAX - 5)));
        assert_eq!(checked_translated(&p, &PointI32::of(0, 5)), Some(PointU32::of(u32::MAX - 2, u32::MAX)));
        assert_eq!(checked_translated(&p, &PointI32::of(2, 5)), Some(PointU32::max()));
        assert_eq!(checked_translated(&p, &PointI32::of(10, 10)), None);
        assert_eq!(checked_translated(&p, &PointI32::of(i32::MAX, 0)), None);
        assert_eq!(checked_translated(&p, &PointI32::of(0, i32::MAX)), None);
        assert_eq!(checked_translated(&p, &PointI32::max()), None);
    }
}

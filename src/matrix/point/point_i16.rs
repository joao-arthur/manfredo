use crate::matrix::point::{point_i8::PointI8, point_u16::PointU16};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointI16 {
    pub row: i16,
    pub col: i16,
}

impl PointI16 {
    pub fn of(row: i16, col: i16) -> Self {
        PointI16 { row, col }
    }

    pub fn min() -> Self {
        PointI16 { row: i16::MIN, col: i16::MIN }
    }

    pub fn max() -> Self {
        PointI16 { row: i16::MAX, col: i16::MAX }
    }
}

impl From<PointI8> for PointI16 {
    fn from(p: PointI8) -> Self {
        PointI16 { row: p.row.into(), col: p.col.into() }
    }
}

impl std::fmt::Display for PointI16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

pub fn delta_row(p1: &PointI16, p2: &PointI16) -> u16 {
    (i32::from(p2.row) - i32::from(p1.row)).unsigned_abs() as u16
}

pub fn delta_col(p1: &PointI16, p2: &PointI16) -> u16 {
    (i32::from(p2.col) - i32::from(p1.col)).unsigned_abs() as u16
}

pub fn delta(p1: &PointI16, p2: &PointI16) -> PointU16 {
    PointU16 { row: delta_row(p1, p2), col: delta_col(p1, p2) }
}

pub fn saturating_translate(p: &mut PointI16, delta: &PointI16) {
    p.row = p.row.saturating_add(delta.row);
    p.col = p.col.saturating_add(delta.col);
}

pub fn checked_translate(p: &mut PointI16, delta: &PointI16) -> Result<(), ()> {
    let row = p.row.checked_add(delta.row).ok_or(())?;
    let col = p.col.checked_add(delta.col).ok_or(())?;
    p.row = row;
    p.col = col;
    Ok(())
}

pub fn saturating_translated(p: &PointI16, delta: &PointI16) -> PointI16 {
    PointI16::of(p.row.saturating_add(delta.row), p.col.saturating_add(delta.col))
}

pub fn checked_translated(p: &PointI16, delta: &PointI16) -> Option<PointI16> {
    let row = p.row.checked_add(delta.row)?;
    let col = p.col.checked_add(delta.col)?;
    Some(PointI16 { row, col })
}

#[cfg(test)]
mod tests {
    use crate::matrix::point::{point_i8::PointI8, point_u16::PointU16};

    use super::{PointI16, checked_translate, checked_translated, delta, delta_col, delta_row, saturating_translate, saturating_translated};

    #[test]
    fn point_i16() {
        assert_eq!(PointI16::of(i16::MIN, i16::MAX), PointI16 { row: i16::MIN, col: i16::MAX });
        assert_eq!(PointI16::min(), PointI16 { row: i16::MIN, col: i16::MIN });
        assert_eq!(PointI16::max(), PointI16 { row: i16::MAX, col: i16::MAX });
    }

    #[test]
    fn from() {
        assert_eq!(PointI16::from(PointI8::min()), PointI16 { row: i8::MIN.into(), col: i8::MIN.into() });
        assert_eq!(PointI16::from(PointI8::max()), PointI16 { row: i8::MAX.into(), col: i8::MAX.into() });
    }

    #[test]
    fn to_string() {
        assert_eq!(PointI16::of(i16::MIN, i16::MAX).to_string(), "(-32768, 32767)");
        assert_eq!(PointI16::min().to_string(), "(-32768, -32768)");
        assert_eq!(PointI16::max().to_string(), "(32767, 32767)");
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&PointI16::of(0, i16::MIN), &PointI16::of(0, i16::MAX)), 0);
        assert_eq!(delta_row(&PointI16::of(i16::MIN, 0), &PointI16::of(i16::MAX, 0)), u16::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&PointI16::of(i16::MIN, 0), &PointI16::of(i16::MAX, 0)), 0);
        assert_eq!(delta_col(&PointI16::of(0, i16::MIN), &PointI16::of(0, i16::MAX)), u16::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointI16::of(0, 0), &PointI16::of(0, 0)), PointU16::of(0, 0));
        assert_eq!(delta(&PointI16::min(), &PointI16::max()), PointU16::max());
    }

    #[test]
    fn delta_min() {
        let p = PointI16::min();
        assert_eq!(delta(&p, &PointI16::min()), PointU16::of(0, 0));
        assert_eq!(delta(&p, &PointI16::of(i16::MIN, i16::MIN + 1)), PointU16::of(0, 1));
        assert_eq!(delta(&p, &PointI16::of(i16::MIN, i16::MIN + 2)), PointU16::of(0, 2));

        assert_eq!(delta(&p, &PointI16::of(i16::MIN + 1, i16::MIN)), PointU16::of(1, 0));
        assert_eq!(delta(&p, &PointI16::of(i16::MIN + 1, i16::MIN + 1)), PointU16::of(1, 1));
        assert_eq!(delta(&p, &PointI16::of(i16::MIN + 1, i16::MIN + 2)), PointU16::of(1, 2));

        assert_eq!(delta(&p, &PointI16::of(i16::MIN + 2, i16::MIN)), PointU16::of(2, 0));
        assert_eq!(delta(&p, &PointI16::of(i16::MIN + 2, i16::MIN + 1)), PointU16::of(2, 1));
        assert_eq!(delta(&p, &PointI16::of(i16::MIN + 2, i16::MIN + 2)), PointU16::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p = PointI16::of(i16::MAX - 2, i16::MAX - 2);
        assert_eq!(delta(&p, &PointI16::of(i16::MAX - 2, i16::MAX - 2)), PointU16::of(0, 0));
        assert_eq!(delta(&p, &PointI16::of(i16::MAX - 2, i16::MAX - 1)), PointU16::of(0, 1));
        assert_eq!(delta(&p, &PointI16::of(i16::MAX - 2, i16::MAX)), PointU16::of(0, 2));

        assert_eq!(delta(&p, &PointI16::of(i16::MAX - 1, i16::MAX - 2)), PointU16::of(1, 0));
        assert_eq!(delta(&p, &PointI16::of(i16::MAX - 1, i16::MAX - 1)), PointU16::of(1, 1));
        assert_eq!(delta(&p, &PointI16::of(i16::MAX - 1, i16::MAX)), PointU16::of(1, 2));

        assert_eq!(delta(&p, &PointI16::of(i16::MAX, i16::MAX - 2)), PointU16::of(2, 0));
        assert_eq!(delta(&p, &PointI16::of(i16::MAX, i16::MAX - 1)), PointU16::of(2, 1));
        assert_eq!(delta(&p, &PointI16::max()), PointU16::of(2, 2));
    }

    #[test]
    fn test_saturating_translate() {
        let mut p = PointI16::of(0, 0);
        saturating_translate(&mut p, &PointI16::of(10, 15));
        assert_eq!(p, PointI16::of(10, 15));
        saturating_translate(&mut p, &PointI16::of(-15, -25));
        assert_eq!(p, PointI16::of(-5, -10));
        saturating_translate(&mut p, &PointI16::of(2, 3));
        assert_eq!(p, PointI16::of(-3, -7));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut p = PointI16::of(i16::MIN + 2, i16::MIN + 5);
        saturating_translate(&mut p, &PointI16::of(-10, -10));
        assert_eq!(p, PointI16::min());
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut p = PointI16::of(i16::MAX - 2, i16::MAX - 5);
        saturating_translate(&mut p, &PointI16::of(10, 10));
        assert_eq!(p, PointI16::max());
    }

    #[test]
    fn saturating_translate_min_bounds_min_delta() {
        let mut p = PointI16::of(i16::MIN + 1, i16::MIN + 1);
        saturating_translate(&mut p, &PointI16::min());
        assert_eq!(p, PointI16::min());
    }

    #[test]
    fn saturating_translate_max_bounds_max_delta() {
        let mut p = PointI16::of(i16::MAX - 1, i16::MAX - 1);
        saturating_translate(&mut p, &PointI16::max());
        assert_eq!(p, PointI16::max());
    }

    #[test]
    fn test_checked_translate() {
        let mut p = PointI16::of(0, 0);
        assert_eq!(checked_translate(&mut p, &PointI16::of(10, 15)), Ok(()));
        assert_eq!(p, PointI16::of(10, 15));
        assert_eq!(checked_translate(&mut p, &PointI16::of(-15, -25)), Ok(()));
        assert_eq!(p, PointI16::of(-5, -10));
        assert_eq!(checked_translate(&mut p, &PointI16::of(2, 3)), Ok(()));
        assert_eq!(p, PointI16::of(-3, -7));
    }

    #[test]
    fn checked_translate_min_bounds_err() {
        let mut p = PointI16::of(i16::MIN + 2, i16::MIN + 5);
        assert_eq!(checked_translate(&mut p, &PointI16::of(-10, -10)), Err(()));
        assert_eq!(p, PointI16::of(i16::MIN + 2, i16::MIN + 5));
    }

    #[test]
    fn checked_translate_max_bounds_err() {
        let mut p = PointI16::of(i16::MAX - 2, i16::MAX - 5);
        assert_eq!(checked_translate(&mut p, &PointI16::of(10, 10)), Err(()));
        assert_eq!(p, PointI16::of(i16::MAX - 2, i16::MAX - 5));
    }

    #[test]
    fn checked_translate_min_bounds_ok() {
        let mut p = PointI16::of(i16::MIN + 2, i16::MIN + 5);
        assert_eq!(checked_translate(&mut p, &PointI16::of(-2, -5)), Ok(()));
        assert_eq!(p, PointI16::min());
    }

    #[test]
    fn checked_translate_max_bounds_ok() {
        let mut p = PointI16::of(i16::MAX - 2, i16::MAX - 5);
        assert_eq!(checked_translate(&mut p, &PointI16::of(2, 5)), Ok(()));
        assert_eq!(p, PointI16::max());
    }

    #[test]
    fn checked_translate_min_bounds_min_delta() {
        let mut p = PointI16::of(i16::MIN + 1, i16::MIN + 1);
        assert_eq!(checked_translate(&mut p, &PointI16::min()), Err(()));
        assert_eq!(checked_translate(&mut p, &PointI16::of(i16::MIN, 0)), Err(()));
        assert_eq!(checked_translate(&mut p, &PointI16::of(0, i16::MIN)), Err(()));
        assert_eq!(p, PointI16::of(i16::MIN + 1, i16::MIN + 1));
    }

    #[test]
    fn checked_translate_max_bounds_max_delta() {
        let mut p = PointI16::of(i16::MAX - 1, i16::MAX - 1);
        assert_eq!(checked_translate(&mut p, &PointI16::max()), Err(()));
        assert_eq!(checked_translate(&mut p, &PointI16::of(i16::MAX, 0)), Err(()));
        assert_eq!(checked_translate(&mut p, &PointI16::of(0, i16::MAX)), Err(()));
        assert_eq!(p, PointI16::of(i16::MAX - 1, i16::MAX - 1));
    }

    #[test]
    fn test_saturating_translated() {
        assert_eq!(saturating_translated(&PointI16::of(0, 0), &PointI16::of(10, 15)), PointI16::of(10, 15));
        assert_eq!(saturating_translated(&PointI16::of(0, 0), &PointI16::of(-15, -25)), PointI16::of(-15, -25));
    }

    #[test]
    fn saturating_translated_to_bounds() {
        assert_eq!(saturating_translated(&PointI16::of(i16::MIN + 2, i16::MIN + 5), &PointI16::of(-2, -5)), PointI16::min());
        assert_eq!(saturating_translated(&PointI16::of(i16::MAX - 2, i16::MAX - 5), &PointI16::of(2, 5)), PointI16::max());
    }

    #[test]
    fn saturating_translated_beyond_bounds() {
        assert_eq!(saturating_translated(&PointI16::of(i16::MIN + 2, i16::MIN + 5), &PointI16::of(-10, -10)), PointI16::min());
        assert_eq!(saturating_translated(&PointI16::of(i16::MAX - 2, i16::MAX - 5), &PointI16::of(10, 10)), PointI16::max());
    }

    #[test]
    fn saturating_translated_limits() {
        assert_eq!(saturating_translated(&PointI16::of(i16::MIN + 1, i16::MIN + 1), &PointI16::min()), PointI16::min());
        assert_eq!(saturating_translated(&PointI16::of(i16::MAX - 1, i16::MAX - 1), &PointI16::max()), PointI16::max());
    }

    #[test]
    fn checked_translated_min_bounds() {
        let p = PointI16::of(i16::MIN + 2, i16::MIN + 5);
        assert_eq!(checked_translated(&p, &PointI16::of(-2, 0)), Some(PointI16::of(i16::MIN, i16::MIN + 5)));
        assert_eq!(checked_translated(&p, &PointI16::of(0, -5)), Some(PointI16::of(i16::MIN + 2, i16::MIN)));
        assert_eq!(checked_translated(&p, &PointI16::of(-2, -5)), Some(PointI16::min()));
        assert_eq!(checked_translated(&p, &PointI16::of(-10, -10)), None);
        assert_eq!(checked_translated(&p, &PointI16::of(i16::MIN, 0)), None);
        assert_eq!(checked_translated(&p, &PointI16::of(0, i16::MIN)), None);
        assert_eq!(checked_translated(&p, &PointI16::min()), None);
    }

    #[test]
    fn checked_translated_max_bounds() {
        let p = PointI16::of(i16::MAX - 2, i16::MAX - 5);
        assert_eq!(checked_translated(&p, &PointI16::of(2, 0)), Some(PointI16::of(i16::MAX, i16::MAX - 5)));
        assert_eq!(checked_translated(&p, &PointI16::of(0, 5)), Some(PointI16::of(i16::MAX - 2, i16::MAX)));
        assert_eq!(checked_translated(&p, &PointI16::of(2, 5)), Some(PointI16::max()));
        assert_eq!(checked_translated(&p, &PointI16::of(10, 10)), None);
        assert_eq!(checked_translated(&p, &PointI16::of(i16::MAX, 0)), None);
        assert_eq!(checked_translated(&p, &PointI16::of(0, i16::MAX)), None);
        assert_eq!(checked_translated(&p, &PointI16::max()), None);
    }
}

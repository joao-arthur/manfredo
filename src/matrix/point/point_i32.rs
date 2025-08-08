use crate::matrix::point::point_u32::PointU32;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointI32 {
    pub row: i32,
    pub col: i32,
}

impl PointI32 {
    pub fn of(row: i32, col: i32) -> Self {
        PointI32 { row, col }
    }

    pub fn min() -> Self {
        PointI32 { row: i32::MIN, col: i32::MIN }
    }

    pub fn max() -> Self {
        PointI32 { row: i32::MAX, col: i32::MAX }
    }
}

impl std::fmt::Display for PointI32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

pub fn delta_row(p1: &PointI32, p2: &PointI32) -> u32 {
    (i64::from(p2.row) - i64::from(p1.row)).unsigned_abs() as u32
}

pub fn delta_col(p1: &PointI32, p2: &PointI32) -> u32 {
    (i64::from(p2.col) - i64::from(p1.col)).unsigned_abs() as u32
}

pub fn delta(p1: &PointI32, p2: &PointI32) -> PointU32 {
    PointU32 { row: delta_row(p1, p2), col: delta_col(p1, p2) }
}

pub fn saturating_translate(p: &mut PointI32, delta: &PointI32) {
    p.row = p.row.saturating_add(delta.row);
    p.col = p.col.saturating_add(delta.col);
}

pub fn checked_translate(p: &mut PointI32, delta: &PointI32) -> Result<(), ()> {
    let row = p.row.checked_add(delta.row).ok_or(())?;
    let col = p.col.checked_add(delta.col).ok_or(())?;
    p.row = row;
    p.col = col;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::matrix::point::point_u32::PointU32;

    use super::{PointI32, checked_translate, delta, delta_col, delta_row, saturating_translate};

    #[test]
    fn point_i32() {
        assert_eq!(PointI32::of(i32::MIN, i32::MAX), PointI32 { row: i32::MIN, col: i32::MAX });
        assert_eq!(PointI32::min(), PointI32 { row: i32::MIN, col: i32::MIN });
        assert_eq!(PointI32::max(), PointI32 { row: i32::MAX, col: i32::MAX });
        assert_eq!(PointI32::of(i32::MIN, i32::MAX).to_string(), "(-2147483648, 2147483647)");
        assert_eq!(PointI32::min().to_string(), "(-2147483648, -2147483648)");
        assert_eq!(PointI32::max().to_string(), "(2147483647, 2147483647)");
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&PointI32::of(0, i32::MIN), &PointI32::of(0, i32::MAX)), 0);
        assert_eq!(delta_row(&PointI32::of(i32::MIN, 0), &PointI32::of(i32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&PointI32::of(i32::MIN, 0), &PointI32::of(i32::MAX, 0)), 0);
        assert_eq!(delta_col(&PointI32::of(0, i32::MIN), &PointI32::of(0, i32::MAX)), u32::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointI32::of(0, 0), &PointI32::of(0, 0)), PointU32::of(0, 0));
        assert_eq!(delta(&PointI32::min(), &PointI32::of(i32::MAX, i32::MAX)), PointU32::of(u32::MAX, u32::MAX));
    }

    #[test]
    fn delta_min() {
        let p1 = PointI32::of(i32::MIN, i32::MIN);
        assert_eq!(delta(&p1, &PointI32::of(i32::MIN, i32::MIN)), PointU32::of(0, 0));
        assert_eq!(delta(&p1, &PointI32::of(i32::MIN, i32::MIN + 1)), PointU32::of(0, 1));
        assert_eq!(delta(&p1, &PointI32::of(i32::MIN, i32::MIN + 2)), PointU32::of(0, 2));

        assert_eq!(delta(&p1, &PointI32::of(i32::MIN + 1, i32::MIN)), PointU32::of(1, 0));
        assert_eq!(delta(&p1, &PointI32::of(i32::MIN + 1, i32::MIN + 1)), PointU32::of(1, 1));
        assert_eq!(delta(&p1, &PointI32::of(i32::MIN + 1, i32::MIN + 2)), PointU32::of(1, 2));

        assert_eq!(delta(&p1, &PointI32::of(i32::MIN + 2, i32::MIN)), PointU32::of(2, 0));
        assert_eq!(delta(&p1, &PointI32::of(i32::MIN + 2, i32::MIN + 1)), PointU32::of(2, 1));
        assert_eq!(delta(&p1, &PointI32::of(i32::MIN + 2, i32::MIN + 2)), PointU32::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p1 = PointI32::of(i32::MAX - 2, i32::MAX - 2);
        assert_eq!(delta(&p1, &PointI32::of(i32::MAX - 2, i32::MAX - 2)), PointU32::of(0, 0));
        assert_eq!(delta(&p1, &PointI32::of(i32::MAX - 2, i32::MAX - 1)), PointU32::of(0, 1));
        assert_eq!(delta(&p1, &PointI32::of(i32::MAX - 2, i32::MAX)), PointU32::of(0, 2));

        assert_eq!(delta(&p1, &PointI32::of(i32::MAX - 1, i32::MAX - 2)), PointU32::of(1, 0));
        assert_eq!(delta(&p1, &PointI32::of(i32::MAX - 1, i32::MAX - 1)), PointU32::of(1, 1));
        assert_eq!(delta(&p1, &PointI32::of(i32::MAX - 1, i32::MAX)), PointU32::of(1, 2));

        assert_eq!(delta(&p1, &PointI32::of(i32::MAX, i32::MAX - 2)), PointU32::of(2, 0));
        assert_eq!(delta(&p1, &PointI32::of(i32::MAX, i32::MAX - 1)), PointU32::of(2, 1));
        assert_eq!(delta(&p1, &PointI32::of(i32::MAX, i32::MAX)), PointU32::of(2, 2));
    }

    #[test]
    fn test_saturating_translate() {
        let mut r = PointI32::of(0, 0);
        saturating_translate(&mut r, &PointI32::of(10, 15));
        assert_eq!(r, PointI32::of(10, 15));
        saturating_translate(&mut r, &PointI32::of(-15, -25));
        assert_eq!(r, PointI32::of(-5, -10));
        saturating_translate(&mut r, &PointI32::of(2, 3));
        assert_eq!(r, PointI32::of(-3, -7));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut r = PointI32::of(i32::MIN + 2, i32::MIN + 5);
        saturating_translate(&mut r, &PointI32::of(-10, -10));
        assert_eq!(r, PointI32::of(i32::MIN, i32::MIN));
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut r = PointI32::of(i32::MAX - 2, i32::MAX - 5);
        saturating_translate(&mut r, &PointI32::of(10, 10));
        assert_eq!(r, PointI32::of(i32::MAX, i32::MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_min_delta() {
        let mut r = PointI32::of(i32::MIN + 1, i32::MIN + 1);
        saturating_translate(&mut r, &PointI32::min());
        assert_eq!(r, PointI32::of(i32::MIN, i32::MIN));
    }

    #[test]
    fn saturating_translate_max_bounds_max_delta() {
        let mut r = PointI32::of(i32::MAX - 1, i32::MAX - 1);
        saturating_translate(&mut r, &PointI32::max());
        assert_eq!(r, PointI32::of(i32::MAX, i32::MAX));
    }

    #[test]
    fn test_checked_translate() {
        let mut r = PointI32::of(0, 0);
        assert_eq!(checked_translate(&mut r, &PointI32::of(10, 15)), Ok(()));
        assert_eq!(r, PointI32::of(10, 15));
        assert_eq!(checked_translate(&mut r, &PointI32::of(-15, -25)), Ok(()));
        assert_eq!(r, PointI32::of(-5, -10));
        assert_eq!(checked_translate(&mut r, &PointI32::of(2, 3)), Ok(()));
        assert_eq!(r, PointI32::of(-3, -7));
    }

    #[test]
    fn checked_translate_min_bounds_err() {
        let mut r = PointI32::of(i32::MIN + 2, i32::MIN + 5);
        assert_eq!(checked_translate(&mut r, &PointI32::of(-10, -10)), Err(()));
        assert_eq!(r, PointI32::of(i32::MIN + 2, i32::MIN + 5));
    }

    #[test]
    fn checked_translate_max_bounds_err() {
        let mut r = PointI32::of(i32::MAX - 2, i32::MAX - 5);
        assert_eq!(checked_translate(&mut r, &PointI32::of(10, 10)), Err(()));
        assert_eq!(r, PointI32::of(i32::MAX - 2, i32::MAX - 5));
    }

    #[test]
    fn checked_translate_min_bounds_ok() {
        let mut r = PointI32::of(i32::MIN + 2, i32::MIN + 5);
        assert_eq!(checked_translate(&mut r, &PointI32::of(-2, -5)), Ok(()));
        assert_eq!(r, PointI32::of(i32::MIN, i32::MIN));
    }

    #[test]
    fn checked_translate_max_bounds_ok() {
        let mut r = PointI32::of(i32::MAX - 2, i32::MAX - 5);
        assert_eq!(checked_translate(&mut r, &PointI32::of(2, 5)), Ok(()));
        assert_eq!(r, PointI32::of(i32::MAX, i32::MAX));
    }

    #[test]
    fn checked_translate_min_bounds_min_delta() {
        let mut r = PointI32::of(i32::MIN + 1, i32::MIN + 1);
        assert_eq!(checked_translate(&mut r, &PointI32::min()), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI32::of(i32::MIN, 0)), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI32::of(0, i32::MIN)), Err(()));
        assert_eq!(r, PointI32::of(i32::MIN + 1, i32::MIN + 1));
    }

    #[test]
    fn checked_translate_max_bounds_max_delta() {
        let mut r = PointI32::of(i32::MAX - 1, i32::MAX - 1);
        assert_eq!(checked_translate(&mut r, &PointI32::max()), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI32::of(i32::MAX, 0)), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI32::of(0, i32::MAX)), Err(()));
        assert_eq!(r, PointI32::of(i32::MAX - 1, i32::MAX - 1));
    }
}

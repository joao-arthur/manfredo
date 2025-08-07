use crate::matrix::point::point_i64::PointI64;

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

pub fn saturating_translate(p: &mut PointU64, delta: &PointI64) {
    let temp_row = i128::from(p.row) + i128::from(delta.row);
    let temp_col = i128::from(p.col) + i128::from(delta.col);
    let clamped_row = temp_row.clamp(0, i128::from(u64::MAX));
    let clamped_col = temp_col.clamp(0, i128::from(u64::MAX));
    p.row = clamped_row as u64;
    p.col = clamped_col as u64;
}

pub fn checked_translate(p: &mut PointU64, delta: &PointI64) -> Result<(), ()> {
    let row = u64::try_from(i128::from(p.row) + i128::from(delta.row)).map_err(|_| ())?;
    let col = u64::try_from(i128::from(p.col) + i128::from(delta.col)).map_err(|_| ())?;
    p.row = row as u64;
    p.col = col as u64;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::matrix::point::point_i64::PointI64;

    use super::{PointU64, checked_translate, delta, delta_col, delta_row, saturating_translate};

    #[test]
    fn point_u64() {
        assert_eq!(PointU64::of(0, u64::MAX), PointU64 { row: 0, col: u64::MAX });
        assert_eq!(PointU64::min(), PointU64 { row: 0, col: 0 });
        assert_eq!(PointU64::max(), PointU64 { row: u64::MAX, col: u64::MAX });
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
        let p1 = PointU64::min();
        assert_eq!(delta(&p1, &PointU64::of(0, 0)), PointU64::of(0, 0));
        assert_eq!(delta(&p1, &PointU64::of(0, 1)), PointU64::of(0, 1));
        assert_eq!(delta(&p1, &PointU64::of(0, 2)), PointU64::of(0, 2));

        assert_eq!(delta(&p1, &PointU64::of(1, 0)), PointU64::of(1, 0));
        assert_eq!(delta(&p1, &PointU64::of(1, 1)), PointU64::of(1, 1));
        assert_eq!(delta(&p1, &PointU64::of(1, 2)), PointU64::of(1, 2));

        assert_eq!(delta(&p1, &PointU64::of(2, 0)), PointU64::of(2, 0));
        assert_eq!(delta(&p1, &PointU64::of(2, 1)), PointU64::of(2, 1));
        assert_eq!(delta(&p1, &PointU64::of(2, 2)), PointU64::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p1 = PointU64::of(u64::MAX - 2, u64::MAX - 2);
        assert_eq!(delta(&p1, &PointU64::of(u64::MAX - 2, u64::MAX - 2)), PointU64::of(0, 0));
        assert_eq!(delta(&p1, &PointU64::of(u64::MAX - 2, u64::MAX - 1)), PointU64::of(0, 1));
        assert_eq!(delta(&p1, &PointU64::of(u64::MAX - 2, u64::MAX)), PointU64::of(0, 2));

        assert_eq!(delta(&p1, &PointU64::of(u64::MAX - 1, u64::MAX - 2)), PointU64::of(1, 0));
        assert_eq!(delta(&p1, &PointU64::of(u64::MAX - 1, u64::MAX - 1)), PointU64::of(1, 1));
        assert_eq!(delta(&p1, &PointU64::of(u64::MAX - 1, u64::MAX)), PointU64::of(1, 2));

        assert_eq!(delta(&p1, &PointU64::of(u64::MAX, u64::MAX - 2)), PointU64::of(2, 0));
        assert_eq!(delta(&p1, &PointU64::of(u64::MAX, u64::MAX - 1)), PointU64::of(2, 1));
        assert_eq!(delta(&p1, &PointU64::of(u64::MAX, u64::MAX)), PointU64::of(2, 2));
    }

    #[test]
    fn test_saturating_translate() {
        let mut r = PointU64::of(0, 0);
        saturating_translate(&mut r, &PointI64::of(10, 10));
        assert_eq!(r, PointU64::of(10, 10));
        saturating_translate(&mut r, &PointI64::of(-5, -5));
        assert_eq!(r, PointU64::of(5, 5));
        saturating_translate(&mut r, &PointI64::of(2, 2));
        assert_eq!(r, PointU64::of(7, 7));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut r = PointU64::of(2, 5);
        saturating_translate(&mut r, &PointI64::of(-10, -10));
        assert_eq!(r, PointU64::of(0, 0));
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut r = PointU64::of(u64::MAX - 2, u64::MAX - 5);
        saturating_translate(&mut r, &PointI64::of(10, 10));
        assert_eq!(r, PointU64::of(u64::MAX, u64::MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_min_delta() {
        let mut r = PointU64::of(1, 1);
        saturating_translate(&mut r, &PointI64::min());
        assert_eq!(r, PointU64::of(0, 0));
    }

    #[test]
    fn saturating_translate_max_bounds_max_delta() {
        let mut r = PointU64::of(u64::MAX - 1, u64::MAX - 1);
        saturating_translate(&mut r, &PointI64::max());
        assert_eq!(r, PointU64::of(u64::MAX, u64::MAX));
    }

    #[test]
    fn test_checked_translate() {
        let mut r = PointU64::of(0, 0);
        assert_eq!(checked_translate(&mut r, &PointI64::of(10, 10)), Ok(()));
        assert_eq!(r, PointU64::of(10, 10));
        assert_eq!(checked_translate(&mut r, &PointI64::of(-5, -5)), Ok(()));
        assert_eq!(r, PointU64::of(5, 5));
        assert_eq!(checked_translate(&mut r, &PointI64::of(2, 2)), Ok(()));
        assert_eq!(r, PointU64::of(7, 7));
    }

    #[test]
    fn checked_translate_min_bounds_err() {
        let mut r = PointU64::of(2, 5);
        assert_eq!(checked_translate(&mut r, &PointI64::of(-10, -10)), Err(()));
        assert_eq!(r, PointU64::of(2, 5));
    }

    #[test]
    fn checked_translate_max_bounds_err() {
        let mut r = PointU64::of(u64::MAX - 2, u64::MAX - 5);
        assert_eq!(checked_translate(&mut r, &PointI64::of(10, 10)), Err(()));
        assert_eq!(r, PointU64::of(u64::MAX - 2, u64::MAX - 5));
    }

    #[test]
    fn checked_translate_min_bounds_ok() {
        let mut r = PointU64::of(2, 5);
        assert_eq!(checked_translate(&mut r, &PointI64::of(-2, -5)), Ok(()));
        assert_eq!(r, PointU64::of(0, 0));
    }

    #[test]
    fn checked_translate_max_bounds_ok() {
        let mut r = PointU64::of(u64::MAX - 2, u64::MAX - 5);
        assert_eq!(checked_translate(&mut r, &PointI64::of(2, 5)), Ok(()));
        assert_eq!(r, PointU64::of(u64::MAX, u64::MAX));
    }

    #[test]
    fn checked_translate_min_bounds_min_delta() {
        let mut r = PointU64::of(1, 1);
        assert_eq!(checked_translate(&mut r, &PointI64::min()), Err(()));
        assert_eq!(r, PointU64::of(1, 1));
    }

    #[test]
    fn checked_translate_max_bounds_max_delta() {
        let mut r = PointU64::of(u64::MAX - 1, u64::MAX - 1);
        assert_eq!(checked_translate(&mut r, &PointI64::max()), Err(()));
        assert_eq!(r, PointU64::of(u64::MAX - 1, u64::MAX - 1));
    }
}

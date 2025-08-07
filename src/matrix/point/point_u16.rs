use crate::matrix::point::point_i16::PointI16;

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

pub fn saturating_translate(p: &mut PointU16, delta: &PointI16) {
    let temp_row = i32::from(p.row) + i32::from(delta.row);
    let temp_col = i32::from(p.col) + i32::from(delta.col);
    let clamped_row = temp_row.clamp(0, i32::from(u16::MAX));
    let clamped_col = temp_col.clamp(0, i32::from(u16::MAX));
    p.row = clamped_row as u16;
    p.col = clamped_col as u16;
}

pub fn checked_translate(p: &mut PointU16, delta: &PointI16) -> Result<(), ()> {
    let row = u16::try_from(i32::from(p.row) + i32::from(delta.row)).map_err(|_| ())?;
    let col = u16::try_from(i32::from(p.col) + i32::from(delta.col)).map_err(|_| ())?;
    p.row = row as u16;
    p.col = col as u16;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::matrix::point::point_i16::PointI16;

    use super::{PointU16, delta, delta_col, delta_row, saturating_translate, checked_translate};

    #[test]
    fn point_u16() {
        assert_eq!(PointU16::of(0, u16::MAX), PointU16 { row: 0, col: u16::MAX });
        assert_eq!(PointU16::min(), PointU16 { row: 0, col: 0 });
        assert_eq!(PointU16::max(), PointU16 { row: u16::MAX, col: u16::MAX });
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
        let p1 = PointU16::min();
        assert_eq!(delta(&p1, &PointU16::of(0, 0)), PointU16::of(0, 0));
        assert_eq!(delta(&p1, &PointU16::of(0, 1)), PointU16::of(0, 1));
        assert_eq!(delta(&p1, &PointU16::of(0, 2)), PointU16::of(0, 2));

        assert_eq!(delta(&p1, &PointU16::of(1, 0)), PointU16::of(1, 0));
        assert_eq!(delta(&p1, &PointU16::of(1, 1)), PointU16::of(1, 1));
        assert_eq!(delta(&p1, &PointU16::of(1, 2)), PointU16::of(1, 2));

        assert_eq!(delta(&p1, &PointU16::of(2, 0)), PointU16::of(2, 0));
        assert_eq!(delta(&p1, &PointU16::of(2, 1)), PointU16::of(2, 1));
        assert_eq!(delta(&p1, &PointU16::of(2, 2)), PointU16::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p1 = PointU16::of(u16::MAX - 2, u16::MAX - 2);
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 2, u16::MAX - 2)), PointU16::of(0, 0));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 2, u16::MAX - 1)), PointU16::of(0, 1));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 2, u16::MAX)), PointU16::of(0, 2));

        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 1, u16::MAX - 2)), PointU16::of(1, 0));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 1, u16::MAX - 1)), PointU16::of(1, 1));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX - 1, u16::MAX)), PointU16::of(1, 2));

        assert_eq!(delta(&p1, &PointU16::of(u16::MAX, u16::MAX - 2)), PointU16::of(2, 0));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX, u16::MAX - 1)), PointU16::of(2, 1));
        assert_eq!(delta(&p1, &PointU16::of(u16::MAX, u16::MAX)), PointU16::of(2, 2));
    }

    #[test]
    fn test_saturating_translate() {
        let mut r = PointU16::of(0, 0);
        saturating_translate(&mut r, &PointI16::of(10, 10));
        assert_eq!(r, PointU16::of(10, 10));
        saturating_translate(&mut r, &PointI16::of(-5, -5));
        assert_eq!(r, PointU16::of(5, 5));
        saturating_translate(&mut r, &PointI16::of(2, 2));
        assert_eq!(r, PointU16::of(7, 7));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut r = PointU16::of(2, 5);
        saturating_translate(&mut r, &PointI16::of(-10, -10));
        assert_eq!(r, PointU16::of(0, 0));
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut r = PointU16::of(u16::MAX - 2, u16::MAX - 5);
        saturating_translate(&mut r, &PointI16::of(10, 10));
        assert_eq!(r, PointU16::of(u16::MAX, u16::MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_min_delta() {
        let mut r = PointU16::of(1, 1);
        saturating_translate(&mut r, &PointI16::min());
        assert_eq!(r, PointU16::of(0, 0));
    }

    #[test]
    fn saturating_translate_max_bounds_max_delta() {
        let mut r = PointU16::of(u16::MAX - 1, u16::MAX - 1);
        saturating_translate(&mut r, &PointI16::max());
        assert_eq!(r, PointU16::of(u16::MAX, u16::MAX));
    }

    #[test]
    fn test_checked_translate() {
        let mut r = PointU16::of(0, 0);
        assert_eq!(checked_translate(&mut r, &PointI16::of(10, 10)), Ok(()));
        assert_eq!(r, PointU16::of(10, 10));
        assert_eq!(checked_translate(&mut r, &PointI16::of(-5, -5)), Ok(()));
        assert_eq!(r, PointU16::of(5, 5));
        assert_eq!(checked_translate(&mut r, &PointI16::of(2, 2)), Ok(()));
        assert_eq!(r, PointU16::of(7, 7));
    }

    #[test]
    fn checked_translate_min_bounds_err() {
        let mut r = PointU16::of(2, 5);
        assert_eq!(checked_translate(&mut r, &PointI16::of(-10, -10)), Err(()));
        assert_eq!(r, PointU16::of(2, 5));
    }

    #[test]
    fn checked_translate_max_bounds_err() {
        let mut r = PointU16::of(u16::MAX - 2, u16::MAX - 5);
        assert_eq!(checked_translate(&mut r, &PointI16::of(10, 10)), Err(()));
        assert_eq!(r, PointU16::of(u16::MAX - 2, u16::MAX - 5));
    }

    #[test]
    fn checked_translate_min_bounds_ok() {
        let mut r = PointU16::of(2, 5);
        assert_eq!(checked_translate(&mut r, &PointI16::of(-2, -5)), Ok(()));
        assert_eq!(r, PointU16::of(0, 0));
    }

    #[test]
    fn checked_translate_max_bounds_ok() {
        let mut r = PointU16::of(u16::MAX - 2, u16::MAX - 5);
        assert_eq!(checked_translate(&mut r, &PointI16::of(2, 5)), Ok(()));
        assert_eq!(r, PointU16::of(u16::MAX, u16::MAX));
    }

    #[test]
    fn checked_translate_min_bounds_min_delta() {
        let mut r = PointU16::of(1, 1);
        assert_eq!(checked_translate(&mut r, &PointI16::min()), Err(()));
        assert_eq!(r, PointU16::of(1, 1));
    }

    #[test]
    fn checked_translate_max_bounds_max_delta() {
        let mut r = PointU16::of(u16::MAX - 1, u16::MAX - 1);
        assert_eq!(checked_translate(&mut r, &PointI16::max()), Err(()));
        assert_eq!(r, PointU16::of(u16::MAX - 1, u16::MAX - 1));
    }
}

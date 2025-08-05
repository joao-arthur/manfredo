use crate::matrix::point::point_u8::PointU8;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct PointI8 {
    pub row: i8,
    pub col: i8,
}

impl PointI8 {
    pub fn of(row: i8, col: i8) -> Self {
        PointI8 { row, col }
    }

    pub fn min() -> Self {
        PointI8 { row: i8::MIN, col: i8::MIN }
    }

    pub fn max() -> Self {
        PointI8 { row: i8::MAX, col: i8::MAX }
    }
}

impl std::fmt::Display for PointI8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

pub fn delta_row(p1: &PointI8, p2: &PointI8) -> u8 {
    (i16::from(p2.row) - i16::from(p1.row)).unsigned_abs() as u8
}

pub fn delta_col(p1: &PointI8, p2: &PointI8) -> u8 {
    (i16::from(p2.col) - i16::from(p1.col)).unsigned_abs() as u8
}

pub fn delta(p1: &PointI8, p2: &PointI8) -> PointU8 {
    PointU8 { row: delta_row(p1, p2), col: delta_col(p1, p2) }
}

pub fn saturating_translate(p: &mut PointI8, delta: &PointI8) {
    let temp_row = i16::from(p.row) + i16::from(delta.row);
    let temp_col = i16::from(p.col) + i16::from(delta.col);
    let clamped_row = temp_row.clamp(i16::from(i8::MIN), i16::from(i8::MAX));
    let clamped_col = temp_col.clamp(i16::from(i8::MIN), i16::from(i8::MAX));
    p.row = clamped_row as i8;
    p.col = clamped_col as i8;
}

#[cfg(test)]
mod tests {
    use crate::matrix::point::point_u8::PointU8;

    use super::{PointI8, delta, delta_col, delta_row, saturating_translate};

    #[test]
    fn point_i8() {
        assert_eq!(PointI8::of(i8::MIN, i8::MAX), PointI8 { row: i8::MIN, col: i8::MAX });
        assert_eq!(PointI8::min(), PointI8 { row: i8::MIN, col: i8::MIN });
        assert_eq!(PointI8::max(), PointI8 { row: i8::MAX, col: i8::MAX });
        assert_eq!(PointI8::of(i8::MIN, i8::MAX).to_string(), "(-128, 127)");
        assert_eq!(PointI8::min().to_string(), "(-128, -128)");
        assert_eq!(PointI8::max().to_string(), "(127, 127)");
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&PointI8::of(0, i8::MIN), &PointI8::of(0, i8::MAX)), 0);
        assert_eq!(delta_row(&PointI8::of(i8::MIN, 0), &PointI8::of(i8::MAX, 0)), u8::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&PointI8::of(i8::MIN, 0), &PointI8::of(i8::MAX, 0)), 0);
        assert_eq!(delta_col(&PointI8::of(0, i8::MIN), &PointI8::of(0, i8::MAX)), u8::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointI8::of(0, 0), &PointI8::of(0, 0)), PointU8::of(0, 0));
        assert_eq!(delta(&PointI8::min(), &PointI8::of(i8::MAX, i8::MAX)), PointU8::of(u8::MAX, u8::MAX));
    }

    #[test]
    fn delta_min() {
        let p1 = PointI8::of(i8::MIN, i8::MIN);
        assert_eq!(delta(&p1, &PointI8::of(i8::MIN, i8::MIN)), PointU8::of(0, 0));
        assert_eq!(delta(&p1, &PointI8::of(i8::MIN, i8::MIN + 1)), PointU8::of(0, 1));
        assert_eq!(delta(&p1, &PointI8::of(i8::MIN, i8::MIN + 2)), PointU8::of(0, 2));

        assert_eq!(delta(&p1, &PointI8::of(i8::MIN + 1, i8::MIN)), PointU8::of(1, 0));
        assert_eq!(delta(&p1, &PointI8::of(i8::MIN + 1, i8::MIN + 1)), PointU8::of(1, 1));
        assert_eq!(delta(&p1, &PointI8::of(i8::MIN + 1, i8::MIN + 2)), PointU8::of(1, 2));

        assert_eq!(delta(&p1, &PointI8::of(i8::MIN + 2, i8::MIN)), PointU8::of(2, 0));
        assert_eq!(delta(&p1, &PointI8::of(i8::MIN + 2, i8::MIN + 1)), PointU8::of(2, 1));
        assert_eq!(delta(&p1, &PointI8::of(i8::MIN + 2, i8::MIN + 2)), PointU8::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p1 = PointI8::of(i8::MAX - 2, i8::MAX - 2);
        assert_eq!(delta(&p1, &PointI8::of(i8::MAX - 2, i8::MAX - 2)), PointU8::of(0, 0));
        assert_eq!(delta(&p1, &PointI8::of(i8::MAX - 2, i8::MAX - 1)), PointU8::of(0, 1));
        assert_eq!(delta(&p1, &PointI8::of(i8::MAX - 2, i8::MAX)), PointU8::of(0, 2));

        assert_eq!(delta(&p1, &PointI8::of(i8::MAX - 1, i8::MAX - 2)), PointU8::of(1, 0));
        assert_eq!(delta(&p1, &PointI8::of(i8::MAX - 1, i8::MAX - 1)), PointU8::of(1, 1));
        assert_eq!(delta(&p1, &PointI8::of(i8::MAX - 1, i8::MAX)), PointU8::of(1, 2));

        assert_eq!(delta(&p1, &PointI8::of(i8::MAX, i8::MAX - 2)), PointU8::of(2, 0));
        assert_eq!(delta(&p1, &PointI8::of(i8::MAX, i8::MAX - 1)), PointU8::of(2, 1));
        assert_eq!(delta(&p1, &PointI8::of(i8::MAX, i8::MAX)), PointU8::of(2, 2));
    }

    #[test]
    fn test_saturating_translate() {
        let mut r = PointI8::of(0, 0);
        saturating_translate(&mut r, &PointI8::of(10, 10));
        assert_eq!(r, PointI8::of(10, 10));
        saturating_translate(&mut r, &PointI8::of(-15, -15));
        assert_eq!(r, PointI8::of(-5, -5));
        saturating_translate(&mut r, &PointI8::of(2, 2));
        assert_eq!(r, PointI8::of(-3, -3));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut r = PointI8::of(i8::MIN + 2, i8::MIN + 5);
        saturating_translate(&mut r, &PointI8::of(-10, -10));
        assert_eq!(r, PointI8::of(i8::MIN, i8::MIN));
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut r = PointI8::of(i8::MAX - 2, i8::MAX - 5);
        saturating_translate(&mut r, &PointI8::of(10, 10));
        assert_eq!(r, PointI8::of(i8::MAX, i8::MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_min_delta() {
        let mut r = PointI8::of(i8::MIN + 1, i8::MIN + 1);
        saturating_translate(&mut r, &PointI8::min());
        assert_eq!(r, PointI8::of(i8::MIN, i8::MIN));
    }

    #[test]
    fn saturating_translate_max_bounds_max_delta() {
        let mut r = PointI8::of(i8::MAX - 1, i8::MAX - 1);
        saturating_translate(&mut r, &PointI8::max());
        assert_eq!(r, PointI8::of(i8::MAX, i8::MAX));
    }
}

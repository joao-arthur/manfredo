use std::ops::RangeInclusive;

use crate::matrix::{
    point::{point_i64::PointI64, point_u64},
    rect::{rect_u8::RectU8, rect_u16::RectU16, rect_u32::RectU32},
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct RectU64 {
    pub min: point_u64::PointU64,
    pub max: point_u64::PointU64,
}

impl RectU64 {
    pub fn largest() -> RectU64 {
        RectU64 { min: point_u64::PointU64::min(), max: point_u64::PointU64::max() }
    }

    pub fn of(row1: u64, col1: u64, row2: u64, col2: u64) -> Self {
        RectU64 { min: point_u64::PointU64::of(row1, col1), max: point_u64::PointU64::of(row2, col2) }
    }

    pub fn iter_row(&self) -> RangeInclusive<u64> {
        self.min.row..=self.max.row
    }

    pub fn iter_col(&self) -> RangeInclusive<u64> {
        self.min.col..=self.max.col
    }
}

impl From<RectU8> for RectU64 {
    fn from(r: RectU8) -> Self {
        RectU64 { min: point_u64::PointU64::of(r.min.row.into(), r.min.col.into()), max: point_u64::PointU64::of(r.max.row.into(), r.max.col.into()) }
    }
}

impl From<RectU16> for RectU64 {
    fn from(r: RectU16) -> Self {
        RectU64 { min: point_u64::PointU64::of(r.min.row.into(), r.min.col.into()), max: point_u64::PointU64::of(r.max.row.into(), r.max.col.into()) }
    }
}

impl From<RectU32> for RectU64 {
    fn from(r: RectU32) -> Self {
        RectU64 { min: point_u64::PointU64::of(r.min.row.into(), r.min.col.into()), max: point_u64::PointU64::of(r.max.row.into(), r.max.col.into()) }
    }
}

impl std::fmt::Display for RectU64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_row(r: &RectU64) -> u64 {
    point_u64::delta_row(&r.min, &r.max)
}

pub fn delta_col(r: &RectU64) -> u64 {
    point_u64::delta_col(&r.min, &r.max)
}

pub fn max_delta(r: &RectU64) -> u64 {
    std::cmp::max(delta_row(r), delta_col(r))
}

pub fn len_row(r: &RectU64) -> u64 {
    delta_row(r) + 1
}

pub fn len_col(r: &RectU64) -> u64 {
    delta_col(r) + 1
}

pub fn max_len(r: &RectU64) -> u64 {
    std::cmp::max(len_row(r), len_col(r))
}

pub fn inflate(r: &mut RectU64) {
    let is_min_row = r.min.row == 0;
    let is_min_col = r.min.col == 0;
    let is_max_row = r.max.row == u64::MAX;
    let is_max_col = r.max.col == u64::MAX;
    if (is_min_row && is_max_row) || (is_min_col && is_max_col) {
        return;
    }
    let min_row_modifier = 1 - u64::from(is_min_row) + u64::from(is_max_row);
    let min_col_modifier = 1 - u64::from(is_min_col) + u64::from(is_max_col);
    let max_row_modifier = 1 + u64::from(is_min_row) - u64::from(is_max_row);
    let max_col_modifier = 1 + u64::from(is_min_col) - u64::from(is_max_col);
    r.min.row = r.min.row.saturating_sub(min_row_modifier);
    r.min.col = r.min.col.saturating_sub(min_col_modifier);
    r.max.row = r.max.row.saturating_add(max_row_modifier);
    r.max.col = r.max.col.saturating_add(max_col_modifier);
}

pub fn deflate(r: &mut RectU64) {
    if delta_row(r) < 3 || delta_col(r) < 3 {
        return;
    }
    r.min.row += 1;
    r.min.col += 1;
    r.max.row -= 1;
    r.max.col -= 1;
}

pub fn resize(r: &mut RectU64, size: u64) {
    if size < 3 {
        return;
    }
    let diff_row = i128::from(delta_row(r)) + 1 - i128::from(size);
    let diff_col = i128::from(delta_col(r)) + 1 - i128::from(size);
    let temp_min_row = i128::from(r.min.row) + diff_row / 2;
    let temp_min_col = i128::from(r.min.col) + diff_col / 2;
    let min_row = temp_min_row.clamp(0, i128::from(u64::MAX) - i128::from(size) + 1);
    let min_col = temp_min_col.clamp(0, i128::from(u64::MAX) - i128::from(size) + 1);
    r.min.row = min_row as u64;
    r.min.col = min_col as u64;
    r.max.row = (min_row + i128::from(size) - 1) as u64;
    r.max.col = (min_col + i128::from(size) - 1) as u64;
}

pub fn assign_saturating_add(r: &mut RectU64, delta: &PointI64) {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let temp_min_row = i128::from(r.min.row) + i128::from(delta.row);
    let temp_min_col = i128::from(r.min.col) + i128::from(delta.col);
    let clamped_row = temp_min_row.clamp(0, i128::from(u64::MAX) - i128::from(d_row));
    let clamped_col = temp_min_col.clamp(0, i128::from(u64::MAX) - i128::from(d_col));
    let min_row = clamped_row as u64;
    let min_col = clamped_col as u64;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = min_row + d_row;
    r.max.col = min_col + d_col;
}

pub fn try_assign_checked_add(r: &mut RectU64, delta: &PointI64) -> Result<(), ()> {
    let min_row = u64::try_from(i128::from(r.min.row) + i128::from(delta.row)).map_err(|_| ())?;
    let min_col = u64::try_from(i128::from(r.min.col) + i128::from(delta.col)).map_err(|_| ())?;
    let max_row = u64::try_from(i128::from(r.max.row) + i128::from(delta.row)).map_err(|_| ())?;
    let max_col = u64::try_from(i128::from(r.max.col) + i128::from(delta.col)).map_err(|_| ())?;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
    Ok(())
}

pub fn assign_checked_add(r: &mut RectU64, delta: &PointI64) {
    try_assign_checked_add(r, delta).unwrap()
}

pub fn saturating_add(r: &RectU64, delta: &PointI64) -> RectU64 {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let temp_min_row = i128::from(r.min.row) + i128::from(delta.row);
    let temp_min_col = i128::from(r.min.col) + i128::from(delta.col);
    let clamped_row = temp_min_row.clamp(0, i128::from(u64::MAX) - i128::from(d_row));
    let clamped_col = temp_min_col.clamp(0, i128::from(u64::MAX) - i128::from(d_col));
    let min_row = clamped_row as u64;
    let min_col = clamped_col as u64;
    RectU64 { min: point_u64::PointU64 { row: min_row, col: min_col }, max: point_u64::PointU64 { row: min_row + d_row, col: min_col + d_col } }
}

pub fn contains(r: &RectU64, p: &point_u64::PointU64) -> bool {
    p.row >= r.min.row && p.row <= r.max.row && p.col >= r.min.col && p.col <= r.max.col
}

#[cfg(test)]
mod tests {
    use crate::matrix::{
        point::{point_i64::PointI64, point_u64::PointU64},
        rect::{rect_u8::RectU8, rect_u16::RectU16, rect_u32::RectU32},
    };

    use super::{
        RectU64, assign_checked_add, assign_saturating_add, contains, deflate, delta_col, delta_row, inflate, len_col, len_row, max_delta, max_len,
        resize, try_assign_checked_add,
    };

    #[test]
    fn rect_u64() {
        assert_eq!(RectU64::largest(), RectU64 { min: PointU64 { row: 0, col: 0 }, max: PointU64 { row: u64::MAX, col: u64::MAX } });
        assert_eq!(
            RectU64::of(4096, 8192, 16384, 32768),
            RectU64 { min: PointU64 { row: 4096, col: 8192 }, max: PointU64 { row: 16384, col: 32768 } }
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(RectU64::of(4096, 8192, 16384, 32768).to_string(), "((4096, 8192), (16384, 32768))");
        assert_eq!(RectU64::of(u64::MAX, 0, 0, u64::MAX).to_string(), "((18446744073709551615, 0), (0, 18446744073709551615))");
    }

    #[test]
    fn from() {
        assert_eq!(
            RectU64::from(RectU8::largest()),
            RectU64 { min: PointU64 { row: 0, col: 0 }, max: PointU64 { row: u8::MAX.into(), col: u8::MAX.into() } }
        );
        assert_eq!(
            RectU64::from(RectU16::largest()),
            RectU64 { min: PointU64 { row: 0, col: 0 }, max: PointU64 { row: u16::MAX.into(), col: u16::MAX.into() } }
        );
        assert_eq!(
            RectU64::from(RectU32::largest()),
            RectU64 { min: PointU64 { row: 0, col: 0 }, max: PointU64 { row: u32::MAX.into(), col: u32::MAX.into() } }
        );
    }

    #[test]
    fn iter_row() {
        assert_eq!(RectU64::of(3, 6, 2, 8).iter_row().collect::<Vec<u64>>(), []);
        assert_eq!(RectU64::of(3, 6, 3, 8).iter_row().collect::<Vec<u64>>(), [3]);
        assert_eq!(RectU64::of(3, 6, 4, 8).iter_row().collect::<Vec<u64>>(), [3, 4]);
        assert_eq!(RectU64::of(3, 6, 5, 8).iter_row().collect::<Vec<u64>>(), [3, 4, 5]);
        assert_eq!(RectU64::of(3, 6, 6, 8).iter_row().collect::<Vec<u64>>(), [3, 4, 5, 6]);
        assert_eq!(RectU64::of(3, 6, 6, 8).iter_row().rev().collect::<Vec<u64>>(), [6, 5, 4, 3]);
        assert_eq!(RectU64::of(3, 6, 5, 8).iter_row().rev().collect::<Vec<u64>>(), [5, 4, 3]);
        assert_eq!(RectU64::of(3, 6, 4, 8).iter_row().rev().collect::<Vec<u64>>(), [4, 3]);
        assert_eq!(RectU64::of(3, 6, 3, 8).iter_row().rev().collect::<Vec<u64>>(), [3]);
        assert_eq!(RectU64::of(3, 6, 2, 8).iter_row().rev().collect::<Vec<u64>>(), []);
    }

    #[test]
    fn iter_col() {
        assert_eq!(RectU64::of(3, 6, 4, 5).iter_col().collect::<Vec<u64>>(), []);
        assert_eq!(RectU64::of(3, 6, 4, 6).iter_col().collect::<Vec<u64>>(), [6]);
        assert_eq!(RectU64::of(3, 6, 4, 7).iter_col().collect::<Vec<u64>>(), [6, 7]);
        assert_eq!(RectU64::of(3, 6, 4, 8).iter_col().collect::<Vec<u64>>(), [6, 7, 8]);
        assert_eq!(RectU64::of(3, 6, 4, 9).iter_col().collect::<Vec<u64>>(), [6, 7, 8, 9]);
        assert_eq!(RectU64::of(3, 6, 4, 9).iter_col().rev().collect::<Vec<u64>>(), [9, 8, 7, 6]);
        assert_eq!(RectU64::of(3, 6, 4, 8).iter_col().rev().collect::<Vec<u64>>(), [8, 7, 6]);
        assert_eq!(RectU64::of(3, 6, 4, 7).iter_col().rev().collect::<Vec<u64>>(), [7, 6]);
        assert_eq!(RectU64::of(3, 6, 4, 6).iter_col().rev().collect::<Vec<u64>>(), [6]);
        assert_eq!(RectU64::of(3, 6, 4, 5).iter_col().rev().collect::<Vec<u64>>(), []);
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&RectU64::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_row(&RectU64::of(0, 0, u64::MAX, 0)), u64::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&RectU64::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_col(&RectU64::of(0, 0, 0, u64::MAX)), u64::MAX);
    }

    #[test]
    fn test_max_delta() {
        assert_eq!(max_delta(&RectU64::of(0, 5, 10, 10)), 10);
        assert_eq!(max_delta(&RectU64::of(5, 0, 9, 9)), 9);
    }

    #[test]
    fn max_delta_0() {
        assert_eq!(max_delta(&RectU64::of(0, 0, 0, 0)), 0);
        assert_eq!(max_delta(&RectU64::of(1, 1, 1, 1)), 0);
        assert_eq!(max_delta(&RectU64::of(5, 10, 5, 10)), 0);
    }

    #[test]
    fn max_delta_1() {
        assert_eq!(max_delta(&RectU64::of(0, 0, 1, 1)), 1);
        assert_eq!(max_delta(&RectU64::of(5, 5, 6, 6)), 1);
        assert_eq!(max_delta(&RectU64::of(0, 0, 0, 1)), 1);
        assert_eq!(max_delta(&RectU64::of(5, 9, 5, 10)), 1);
    }

    #[test]
    fn max_delta_bounds() {
        assert_eq!(max_delta(&RectU64::of(0, 0, u64::MAX, u64::MAX - 1)), u64::MAX);
        assert_eq!(max_delta(&RectU64::of(0, 0, u64::MAX - 1, u64::MAX)), u64::MAX);
    }

    #[test]
    fn test_len_row() {
        assert_eq!(len_row(&RectU64::of(0, 0, 0, 0)), 1);
        assert_eq!(len_row(&RectU64::of(0, 0, u64::MAX - 1, 0)), u64::MAX);
    }

    #[test]
    fn test_len_col() {
        assert_eq!(len_col(&RectU64::of(0, 0, 0, 0)), 1);
        assert_eq!(len_col(&RectU64::of(0, 0, 0, u64::MAX - 1)), u64::MAX);
    }

    #[test]
    fn test_max_len() {
        assert_eq!(max_len(&RectU64::of(0, 5, 10, 10)), 11);
        assert_eq!(max_len(&RectU64::of(5, 0, 9, 9)), 10);
    }

    #[test]
    fn max_len_1() {
        assert_eq!(max_len(&RectU64::of(0, 0, 0, 0)), 1);
        assert_eq!(max_len(&RectU64::of(1, 1, 1, 1)), 1);
        assert_eq!(max_len(&RectU64::of(5, 10, 5, 10)), 1);
    }

    #[test]
    fn max_len_2() {
        assert_eq!(max_len(&RectU64::of(0, 0, 1, 1)), 2);
        assert_eq!(max_len(&RectU64::of(5, 5, 6, 6)), 2);
        assert_eq!(max_len(&RectU64::of(0, 0, 0, 1)), 2);
        assert_eq!(max_len(&RectU64::of(5, 9, 5, 10)), 2);
    }

    #[test]
    fn max_len_bounds() {
        assert_eq!(max_len(&RectU64::of(1, 0, u64::MAX - 1, u64::MAX - 1)), u64::MAX);
        assert_eq!(max_len(&RectU64::of(0, 1, u64::MAX - 1, u64::MAX - 1)), u64::MAX);
        assert_eq!(max_len(&RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 1)), u64::MAX);
        assert_eq!(max_len(&RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 2)), u64::MAX);
    }

    #[test]
    fn inflate_min_bounds() {
        let mut r = RectU64::of(7, 2, 4, 13);
        inflate(&mut r);
        assert_eq!(r, RectU64::of(6, 1, 5, 14));
        inflate(&mut r);
        assert_eq!(r, RectU64::of(5, 0, 6, 15));
        inflate(&mut r);
        assert_eq!(r, RectU64::of(4, 0, 7, 17));
        inflate(&mut r);
        assert_eq!(r, RectU64::of(3, 0, 8, 19));
        inflate(&mut r);
        assert_eq!(r, RectU64::of(2, 0, 9, 21));
        inflate(&mut r);
        assert_eq!(r, RectU64::of(1, 0, 10, 23));
        inflate(&mut r);
        assert_eq!(r, RectU64::of(0, 0, 11, 25));
        inflate(&mut r);
        assert_eq!(r, RectU64::of(0, 0, 13, 27));
    }

    #[test]
    fn inflate_max_bounds() {
        let mut r = RectU64::of(200, 230, u64::MAX - 5, u64::MAX - 3);
        inflate(&mut r);
        assert_eq!(r, RectU64::of(199, 229, u64::MAX - 4, u64::MAX - 2));
        inflate(&mut r);
        assert_eq!(r, RectU64::of(198, 228, u64::MAX - 3, u64::MAX - 1));
        inflate(&mut r);
        assert_eq!(r, RectU64::of(197, 227, u64::MAX - 2, u64::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU64::of(196, 225, u64::MAX - 1, u64::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU64::of(195, 223, u64::MAX, u64::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU64::of(193, 221, u64::MAX, u64::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU64::of(191, 219, u64::MAX, u64::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU64::of(189, 217, u64::MAX, u64::MAX));
    }

    #[test]
    fn inflate_almost_min_bounds() {
        let mut r = RectU64::of(1, 1, u64::MAX, u64::MAX);
        inflate(&mut r);
        assert_eq!(r, RectU64::largest());
    }

    #[test]
    fn inflate_almost_max_bounds() {
        let mut r = RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1);
        inflate(&mut r);
        assert_eq!(r, RectU64::largest());
    }

    #[test]
    fn inflate_max_width() {
        let mut r = RectU64::of(0, 10, u64::MAX, 250);
        inflate(&mut r);
        assert_eq!(r, RectU64::of(0, 10, u64::MAX, 250));
    }

    #[test]
    fn inflate_max_height() {
        let mut r = RectU64::of(10, 0, 250, u64::MAX);
        inflate(&mut r);
        assert_eq!(r, RectU64::of(10, 0, 250, u64::MAX));
    }

    #[test]
    fn deflate_odd() {
        let mut r = RectU64::of(0, 0, 9, 9);
        deflate(&mut r);
        assert_eq!(r, RectU64::of(1, 1, 8, 8));
        deflate(&mut r);
        assert_eq!(r, RectU64::of(2, 2, 7, 7));
        deflate(&mut r);
        assert_eq!(r, RectU64::of(3, 3, 6, 6));
        deflate(&mut r);
        assert_eq!(r, RectU64::of(4, 4, 5, 5));
        deflate(&mut r);
        assert_eq!(r, RectU64::of(4, 4, 5, 5));
    }

    #[test]
    fn deflate_even() {
        let mut r = RectU64::of(0, 0, 10, 10);
        deflate(&mut r);
        assert_eq!(r, RectU64::of(1, 1, 9, 9));
        deflate(&mut r);
        assert_eq!(r, RectU64::of(2, 2, 8, 8));
        deflate(&mut r);
        assert_eq!(r, RectU64::of(3, 3, 7, 7));
        deflate(&mut r);
        assert_eq!(r, RectU64::of(4, 4, 6, 6));
        deflate(&mut r);
        assert_eq!(r, RectU64::of(4, 4, 6, 6));
    }

    #[test]
    fn resize_odd() {
        let mut r = RectU64::of(5, 5, 15, 15);
        resize(&mut r, 11);
        assert_eq!(r, RectU64::of(5, 5, 15, 15));
        resize(&mut r, 9);
        assert_eq!(r, RectU64::of(6, 6, 14, 14));
        resize(&mut r, 7);
        assert_eq!(r, RectU64::of(7, 7, 13, 13));
        resize(&mut r, 5);
        assert_eq!(r, RectU64::of(8, 8, 12, 12));
        resize(&mut r, 3);
        assert_eq!(r, RectU64::of(9, 9, 11, 11));
        resize(&mut r, 1);
        assert_eq!(r, RectU64::of(9, 9, 11, 11));
        resize(&mut r, 3);
        assert_eq!(r, RectU64::of(9, 9, 11, 11));
        resize(&mut r, 9);
        assert_eq!(r, RectU64::of(6, 6, 14, 14));
    }

    #[test]
    fn resize_even() {
        let mut r = RectU64::of(5, 5, 14, 14);
        resize(&mut r, 10);
        assert_eq!(r, RectU64::of(5, 5, 14, 14));
        resize(&mut r, 8);
        assert_eq!(r, RectU64::of(6, 6, 13, 13));
        resize(&mut r, 6);
        assert_eq!(r, RectU64::of(7, 7, 12, 12));
        resize(&mut r, 4);
        assert_eq!(r, RectU64::of(8, 8, 11, 11));
        resize(&mut r, 2);
        assert_eq!(r, RectU64::of(8, 8, 11, 11));
        resize(&mut r, 4);
        assert_eq!(r, RectU64::of(8, 8, 11, 11));
        resize(&mut r, 8);
        assert_eq!(r, RectU64::of(6, 6, 13, 13));
    }

    #[test]
    fn resize_odd_min_bounds_big_delta() {
        let mut r = RectU64::of(0, 0, 2, 2);
        resize(&mut r, u64::MAX);
        assert_eq!(r, RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));
    }

    #[test]
    fn resize_even_min_bounds_big_delta() {
        let mut r = RectU64::of(0, 0, 3, 3);
        resize(&mut r, u64::MAX - 1);
        assert_eq!(r, RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 2));
    }

    #[test]
    fn resize_odd_max_bounds_big_delta() {
        let mut r = RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX);
        resize(&mut r, u64::MAX);
        assert_eq!(r, RectU64::of(1, 1, u64::MAX, u64::MAX));
    }

    #[test]
    fn resize_even_max_bounds_big_delta() {
        let mut r = RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX);
        resize(&mut r, u64::MAX - 1);
        assert_eq!(r, RectU64::of(2, 2, u64::MAX, u64::MAX));
    }

    #[test]
    fn test_assign_saturating_add() {
        let mut r = RectU64::of(0, 0, 12, 15);
        assign_saturating_add(&mut r, &PointI64::of(5, 4));
        assert_eq!(r, RectU64::of(5, 4, 17, 19));
        assign_saturating_add(&mut r, &PointI64::of(-4, -2));
        assert_eq!(r, RectU64::of(1, 2, 13, 17));
        assign_saturating_add(&mut r, &PointI64::of(10, 20));
        assert_eq!(r, RectU64::of(11, 22, 23, 37));
    }

    #[test]
    fn assign_saturating_add_min_bounds() {
        let mut r = RectU64::of(2, 5, 12, 15);
        assign_saturating_add(&mut r, &PointI64::of(-10, -10));
        assert_eq!(r, RectU64::of(0, 0, 10, 10));
    }

    #[test]
    fn assign_saturating_add_max_bounds() {
        let mut r = RectU64::of(240, 235, u64::MAX - 5, u64::MAX - 10);
        assign_saturating_add(&mut r, &PointI64::of(20, 20));
        assert_eq!(r, RectU64::of(245, 245, u64::MAX, u64::MAX));
    }

    #[test]
    fn assign_saturating_add_min_bounds_big_delta() {
        let mut r = RectU64::of(0, 0, 10, 10);
        assign_saturating_add(&mut r, &PointI64::min());
        assert_eq!(r, RectU64::of(0, 0, 10, 10));
    }

    #[test]
    fn assign_saturating_add_max_bounds_big_delta() {
        let mut r = RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX);
        assign_saturating_add(&mut r, &PointI64::max());
        assert_eq!(r, RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX));
    }

    #[test]
    fn assign_saturating_add_min_bounds_big_rect_big_delta() {
        let mut r = RectU64::of(1, 1, u64::MAX, u64::MAX);
        assign_saturating_add(&mut r, &PointI64::min());
        assert_eq!(r, RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));
    }

    #[test]
    fn assign_saturating_add_max_bounds_big_rect_big_delta() {
        let mut r = RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1);
        assign_saturating_add(&mut r, &PointI64::max());
        assert_eq!(r, RectU64::of(1, 1, u64::MAX, u64::MAX));
    }

    #[test]
    fn test_try_assign_checked_add() {
        let mut r = RectU64::of(0, 0, 12, 15);
        assert_eq!(try_assign_checked_add(&mut r, &PointI64::of(5, 4)), Ok(()));
        assert_eq!(r, RectU64::of(5, 4, 17, 19));
        assert_eq!(try_assign_checked_add(&mut r, &PointI64::of(-4, -2)), Ok(()));
        assert_eq!(r, RectU64::of(1, 2, 13, 17));
        assert_eq!(try_assign_checked_add(&mut r, &PointI64::of(10, 20)), Ok(()));
        assert_eq!(r, RectU64::of(11, 22, 23, 37));
    }

    #[test]
    fn try_assign_checked_add_min_bounds() {
        let mut r = RectU64::of(2, 5, 12, 15);
        assert_eq!(try_assign_checked_add(&mut r, &PointI64::of(-10, -10)), Err(()));
        assert_eq!(r, RectU64::of(2, 5, 12, 15));
    }

    #[test]
    fn try_assign_checked_add_max_bounds() {
        let mut r = RectU64::of(240, 235, u64::MAX - 5, u64::MAX - 10);
        assert_eq!(try_assign_checked_add(&mut r, &PointI64::of(20, 20)), Err(()));
        assert_eq!(r, RectU64::of(240, 235, u64::MAX - 5, u64::MAX - 10));
    }

    #[test]
    fn try_assign_checked_add_min_bounds_big_rect_big_delta() {
        let mut r = RectU64::of(1, 1, u64::MAX, u64::MAX);
        assert_eq!(try_assign_checked_add(&mut r, &PointI64::min()), Err(()));
        assert_eq!(try_assign_checked_add(&mut r, &PointI64::of(i64::MIN, 0)), Err(()));
        assert_eq!(try_assign_checked_add(&mut r, &PointI64::of(0, i64::MIN)), Err(()));
        assert_eq!(r, RectU64::of(1, 1, u64::MAX, u64::MAX));
    }

    #[test]
    fn try_assign_checked_add_max_bounds_big_rect_big_delta() {
        let mut r = RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1);
        assert_eq!(try_assign_checked_add(&mut r, &PointI64::max()), Err(()));
        assert_eq!(try_assign_checked_add(&mut r, &PointI64::of(i64::MAX, 0)), Err(()));
        assert_eq!(try_assign_checked_add(&mut r, &PointI64::of(0, i64::MAX)), Err(()));
        assert_eq!(r, RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));
    }

    #[test]
    fn try_assign_checked_add_min_bounds_big_rect_small_delta() {
        let mut r = RectU64::of(1, 1, u64::MAX, u64::MAX);
        assert_eq!(try_assign_checked_add(&mut r, &PointI64::of(-1, -1)), Ok(()));
        assert_eq!(r, RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));
    }

    #[test]
    fn try_assign_checked_add_max_bounds_big_rect_small_delta() {
        let mut r = RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1);
        assert_eq!(try_assign_checked_add(&mut r, &PointI64::of(1, 1)), Ok(()));
        assert_eq!(r, RectU64::of(1, 1, u64::MAX, u64::MAX));
    }

    #[test]
    fn test_assign_checked_add() {
        let mut r = RectU64::of(0, 0, 12, 15);
        assign_checked_add(&mut r, &PointI64::of(5, 4));
        assert_eq!(r, RectU64::of(5, 4, 17, 19));
        assign_checked_add(&mut r, &PointI64::of(-4, -2));
        assert_eq!(r, RectU64::of(1, 2, 13, 17));
        assign_checked_add(&mut r, &PointI64::of(10, 20));
        assert_eq!(r, RectU64::of(11, 22, 23, 37));
    }

    #[test]
    fn contains_inside_borders() {
        let r = RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 1);
        assert!(contains(&r, &PointU64::of(1, 1)));
        assert!(contains(&r, &PointU64::of(1, u64::MAX - 1)));
        assert!(contains(&r, &PointU64::of(u64::MAX - 1, 1)));
        assert!(contains(&r, &PointU64::of(u64::MAX - 1, u64::MAX - 1)));
    }

    #[test]
    fn contains_outside_borders() {
        let r = RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 1);
        assert!(!contains(&r, &PointU64::of(0, 0)));
        assert!(!contains(&r, &PointU64::of(0, u64::MAX)));
        assert!(!contains(&r, &PointU64::of(u64::MAX, 0)));
        assert!(!contains(&r, &PointU64::max()));
        assert!(!contains(&r, &PointU64::of(1, 0)));
        assert!(!contains(&r, &PointU64::of(1, u64::MAX)));
        assert!(!contains(&r, &PointU64::of(u64::MAX - 1, 0)));
        assert!(!contains(&r, &PointU64::of(u64::MAX - 1, u64::MAX)));
        assert!(!contains(&r, &PointU64::of(0, 1)));
        assert!(!contains(&r, &PointU64::of(0, u64::MAX - 1)));
        assert!(!contains(&r, &PointU64::of(u64::MAX, 1)));
        assert!(!contains(&r, &PointU64::of(u64::MAX, u64::MAX - 1)));
    }

    #[test]
    fn contains_inside() {
        let r = RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 1);
        assert!(contains(&r, &PointU64::of(10, 10)));
        assert!(contains(&r, &PointU64::of(u64::MAX - 10, u64::MAX - 10)));
    }
}

use std::ops::RangeInclusive;

use crate::matrix::{
    point::{point_i32::PointI32, point_u32},
    rect::{rect_u8::RectU8, rect_u16::RectU16},
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct RectU32 {
    pub min: point_u32::PointU32,
    pub max: point_u32::PointU32,
}

impl RectU32 {
    pub fn of(row1: u32, col1: u32, row2: u32, col2: u32) -> Self {
        RectU32 { min: point_u32::PointU32::of(row1, col1), max: point_u32::PointU32::of(row2, col2) }
    }

    pub fn iter_row(&self) -> RangeInclusive<u32> {
        self.min.row..=self.max.row
    }

    pub fn iter_col(&self) -> RangeInclusive<u32> {
        self.min.col..=self.max.col
    }
}

impl From<RectU8> for RectU32 {
    fn from(r: RectU8) -> Self {
        RectU32 { min: point_u32::PointU32::of(r.min.row.into(), r.min.col.into()), max: point_u32::PointU32::of(r.max.row.into(), r.max.col.into()) }
    }
}

impl From<RectU16> for RectU32 {
    fn from(r: RectU16) -> Self {
        RectU32 { min: point_u32::PointU32::of(r.min.row.into(), r.min.col.into()), max: point_u32::PointU32::of(r.max.row.into(), r.max.col.into()) }
    }
}

impl std::fmt::Display for RectU32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_row(r: &RectU32) -> u32 {
    point_u32::delta_row(&r.min, &r.max)
}

pub fn delta_col(r: &RectU32) -> u32 {
    point_u32::delta_col(&r.min, &r.max)
}

pub fn max_delta(r: &RectU32) -> u32 {
    std::cmp::max(delta_row(r), delta_col(r))
}

pub fn len_row(r: &RectU32) -> u32 {
    delta_row(r) + 1
}

pub fn len_col(r: &RectU32) -> u32 {
    delta_col(r) + 1
}

pub fn max_len(r: &RectU32) -> u32 {
    std::cmp::max(len_row(r), len_col(r))
}

pub fn inflate(r: &mut RectU32) {
    let is_min_row = r.min.row == 0;
    let is_min_col = r.min.col == 0;
    let is_max_row = r.max.row == u32::MAX;
    let is_max_col = r.max.col == u32::MAX;
    if (is_min_row && is_max_row) || (is_min_col && is_max_col) {
        return;
    }
    let min_row_modifier = 1 - u32::from(is_min_row) + u32::from(is_max_row);
    let min_col_modifier = 1 - u32::from(is_min_col) + u32::from(is_max_col);
    let max_row_modifier = 1 + u32::from(is_min_row) - u32::from(is_max_row);
    let max_col_modifier = 1 + u32::from(is_min_col) - u32::from(is_max_col);
    r.min.row = r.min.row.saturating_sub(min_row_modifier);
    r.min.col = r.min.col.saturating_sub(min_col_modifier);
    r.max.row = r.max.row.saturating_add(max_row_modifier);
    r.max.col = r.max.col.saturating_add(max_col_modifier);
}

pub fn deflate(r: &mut RectU32) {
    if delta_row(r) < 3 || delta_col(r) < 3 {
        return;
    }
    r.min.row += 1;
    r.min.col += 1;
    r.max.row -= 1;
    r.max.col -= 1;
}

pub fn resize(r: &mut RectU32, size: u32) {
    if size < 3 {
        return;
    }
    let diff_row = i64::from(delta_row(r)) + 1 - i64::from(size);
    let diff_col = i64::from(delta_col(r)) + 1 - i64::from(size);
    let temp_min_row = i64::from(r.min.row) + diff_row / 2;
    let temp_min_col = i64::from(r.min.col) + diff_col / 2;
    let min_row = temp_min_row.clamp(0, i64::from(u32::MAX) - i64::from(size) + 1);
    let min_col = temp_min_col.clamp(0, i64::from(u32::MAX) - i64::from(size) + 1);
    r.min.row = min_row as u32;
    r.min.col = min_col as u32;
    r.max.row = (min_row + i64::from(size) - 1) as u32;
    r.max.col = (min_col + i64::from(size) - 1) as u32;
}

pub fn saturating_translate(r: &mut RectU32, delta: &PointI32) {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let temp_min_row = i64::from(r.min.row) + i64::from(delta.row);
    let temp_min_col = i64::from(r.min.col) + i64::from(delta.col);
    let clamped_row = temp_min_row.clamp(0, i64::from(u32::MAX) - i64::from(d_row));
    let clamped_col = temp_min_col.clamp(0, i64::from(u32::MAX) - i64::from(d_col));
    let min_row = clamped_row as u32;
    let min_col = clamped_col as u32;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = min_row + d_row;
    r.max.col = min_col + d_col;
}

pub fn checked_translate(r: &mut RectU32, delta: &PointI32) -> Result<(), ()> {
    let min_row = u32::try_from(i64::from(r.min.row) + i64::from(delta.row)).map_err(|_| ())?;
    let min_col = u32::try_from(i64::from(r.min.col) + i64::from(delta.col)).map_err(|_| ())?;
    let max_row = u32::try_from(i64::from(r.max.row) + i64::from(delta.row)).map_err(|_| ())?;
    let max_col = u32::try_from(i64::from(r.max.col) + i64::from(delta.col)).map_err(|_| ())?;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
    Ok(())
}

pub fn contains(r: &RectU32, p: &point_u32::PointU32) -> bool {
    p.row >= r.min.row && p.row <= r.max.row && p.col >= r.min.col && p.col <= r.max.col
}

#[cfg(test)]
mod tests {
    use crate::matrix::{
        point::{point_i32::PointI32, point_u32::PointU32},
        rect::{rect_u8::RectU8, rect_u16::RectU16},
    };

    use super::{
        RectU32, checked_translate, contains, deflate, delta_col, delta_row, inflate, len_col, len_row, max_delta, max_len, resize,
        saturating_translate,
    };

    #[test]
    fn rect_u32() {
        assert_eq!(RectU32::of(256, 512, 1024, 2048), RectU32 { min: PointU32 { row: 256, col: 512 }, max: PointU32 { row: 1024, col: 2048 } });
        assert_eq!(RectU32::of(u32::MAX, 0, 0, u32::MAX).to_string(), "((4294967295, 0), (0, 4294967295))");
    }

    #[test]
    fn from() {
        assert_eq!(
            RectU32::from(RectU8::of(0, 0, u8::MAX, u8::MAX)),
            RectU32 { min: PointU32 { row: 0, col: 0 }, max: PointU32 { row: u8::MAX.into(), col: u8::MAX.into() } }
        );
        assert_eq!(
            RectU32::from(RectU16::of(0, 0, u16::MAX, u16::MAX)),
            RectU32 { min: PointU32 { row: 0, col: 0 }, max: PointU32 { row: u16::MAX.into(), col: u16::MAX.into() } }
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(RectU32::of(u32::MAX, 0, 0, u32::MAX).to_string(), "((4294967295, 0), (0, 4294967295))");
    }

    #[test]
    fn iter_row() {
        assert_eq!(RectU32::of(3, 6, 2, 8).iter_row().collect::<Vec<u32>>(), []);
        assert_eq!(RectU32::of(3, 6, 3, 8).iter_row().collect::<Vec<u32>>(), [3]);
        assert_eq!(RectU32::of(3, 6, 4, 8).iter_row().collect::<Vec<u32>>(), [3, 4]);
        assert_eq!(RectU32::of(3, 6, 5, 8).iter_row().collect::<Vec<u32>>(), [3, 4, 5]);
        assert_eq!(RectU32::of(3, 6, 6, 8).iter_row().collect::<Vec<u32>>(), [3, 4, 5, 6]);
        assert_eq!(RectU32::of(3, 6, 6, 8).iter_row().rev().collect::<Vec<u32>>(), [6, 5, 4, 3]);
        assert_eq!(RectU32::of(3, 6, 5, 8).iter_row().rev().collect::<Vec<u32>>(), [5, 4, 3]);
        assert_eq!(RectU32::of(3, 6, 4, 8).iter_row().rev().collect::<Vec<u32>>(), [4, 3]);
        assert_eq!(RectU32::of(3, 6, 3, 8).iter_row().rev().collect::<Vec<u32>>(), [3]);
        assert_eq!(RectU32::of(3, 6, 2, 8).iter_row().rev().collect::<Vec<u32>>(), []);
    }

    #[test]
    fn iter_col() {
        assert_eq!(RectU32::of(3, 6, 4, 5).iter_col().collect::<Vec<u32>>(), []);
        assert_eq!(RectU32::of(3, 6, 4, 6).iter_col().collect::<Vec<u32>>(), [6]);
        assert_eq!(RectU32::of(3, 6, 4, 7).iter_col().collect::<Vec<u32>>(), [6, 7]);
        assert_eq!(RectU32::of(3, 6, 4, 8).iter_col().collect::<Vec<u32>>(), [6, 7, 8]);
        assert_eq!(RectU32::of(3, 6, 4, 9).iter_col().collect::<Vec<u32>>(), [6, 7, 8, 9]);
        assert_eq!(RectU32::of(3, 6, 4, 9).iter_col().rev().collect::<Vec<u32>>(), [9, 8, 7, 6]);
        assert_eq!(RectU32::of(3, 6, 4, 8).iter_col().rev().collect::<Vec<u32>>(), [8, 7, 6]);
        assert_eq!(RectU32::of(3, 6, 4, 7).iter_col().rev().collect::<Vec<u32>>(), [7, 6]);
        assert_eq!(RectU32::of(3, 6, 4, 6).iter_col().rev().collect::<Vec<u32>>(), [6]);
        assert_eq!(RectU32::of(3, 6, 4, 5).iter_col().rev().collect::<Vec<u32>>(), []);
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&RectU32::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_row(&RectU32::of(0, 0, u32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&RectU32::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_col(&RectU32::of(0, 0, 0, u32::MAX)), u32::MAX);
    }

    #[test]
    fn test_max_delta() {
        assert_eq!(max_delta(&RectU32::of(0, 5, 10, 10)), 10);
        assert_eq!(max_delta(&RectU32::of(5, 0, 9, 9)), 9);
    }

    #[test]
    fn max_delta_0() {
        assert_eq!(max_delta(&RectU32::of(0, 0, 0, 0)), 0);
        assert_eq!(max_delta(&RectU32::of(1, 1, 1, 1)), 0);
        assert_eq!(max_delta(&RectU32::of(5, 10, 5, 10)), 0);
    }

    #[test]
    fn max_delta_1() {
        assert_eq!(max_delta(&RectU32::of(0, 0, 1, 1)), 1);
        assert_eq!(max_delta(&RectU32::of(5, 5, 6, 6)), 1);
        assert_eq!(max_delta(&RectU32::of(0, 0, 0, 1)), 1);
        assert_eq!(max_delta(&RectU32::of(5, 9, 5, 10)), 1);
    }

    #[test]
    fn max_delta_bounds() {
        assert_eq!(max_delta(&RectU32::of(0, 0, u32::MAX, u32::MAX - 1)), u32::MAX);
        assert_eq!(max_delta(&RectU32::of(0, 0, u32::MAX - 1, u32::MAX)), u32::MAX);
    }

    #[test]
    fn test_len_row() {
        assert_eq!(len_row(&RectU32::of(0, 0, 0, 0)), 1);
        assert_eq!(len_row(&RectU32::of(0, 0, u32::MAX - 1, 0)), u32::MAX);
    }

    #[test]
    fn test_len_col() {
        assert_eq!(len_col(&RectU32::of(0, 0, 0, 0)), 1);
        assert_eq!(len_col(&RectU32::of(0, 0, 0, u32::MAX - 1)), u32::MAX);
    }

    #[test]
    fn test_max_len() {
        assert_eq!(max_len(&RectU32::of(0, 5, 10, 10)), 11);
        assert_eq!(max_len(&RectU32::of(5, 0, 9, 9)), 10);
    }

    #[test]
    fn max_len_1() {
        assert_eq!(max_len(&RectU32::of(0, 0, 0, 0)), 1);
        assert_eq!(max_len(&RectU32::of(1, 1, 1, 1)), 1);
        assert_eq!(max_len(&RectU32::of(5, 10, 5, 10)), 1);
    }

    #[test]
    fn max_len_2() {
        assert_eq!(max_len(&RectU32::of(0, 0, 1, 1)), 2);
        assert_eq!(max_len(&RectU32::of(5, 5, 6, 6)), 2);
        assert_eq!(max_len(&RectU32::of(0, 0, 0, 1)), 2);
        assert_eq!(max_len(&RectU32::of(5, 9, 5, 10)), 2);
    }

    #[test]
    fn max_len_bounds() {
        assert_eq!(max_len(&RectU32::of(1, 0, u32::MAX - 1, u32::MAX - 1)), u32::MAX);
        assert_eq!(max_len(&RectU32::of(0, 1, u32::MAX - 1, u32::MAX - 1)), u32::MAX);
        assert_eq!(max_len(&RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 1)), u32::MAX);
        assert_eq!(max_len(&RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 2)), u32::MAX);
    }

    #[test]
    fn inflate_min_bounds() {
        let mut r = RectU32::of(7, 2, 4, 13);
        inflate(&mut r);
        assert_eq!(r, RectU32::of(6, 1, 5, 14));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(5, 0, 6, 15));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(4, 0, 7, 17));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(3, 0, 8, 19));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(2, 0, 9, 21));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(1, 0, 10, 23));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(0, 0, 11, 25));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(0, 0, 13, 27));
    }

    #[test]
    fn inflate_max_bounds() {
        let mut r = RectU32::of(200, 230, u32::MAX - 5, u32::MAX - 3);
        inflate(&mut r);
        assert_eq!(r, RectU32::of(199, 229, u32::MAX - 4, u32::MAX - 2));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(198, 228, u32::MAX - 3, u32::MAX - 1));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(197, 227, u32::MAX - 2, u32::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(196, 225, u32::MAX - 1, u32::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(195, 223, u32::MAX, u32::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(193, 221, u32::MAX, u32::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(191, 219, u32::MAX, u32::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(189, 217, u32::MAX, u32::MAX));
    }

    #[test]
    fn inflate_almost_min_bounds() {
        let mut r = RectU32::of(1, 1, u32::MAX, u32::MAX);
        inflate(&mut r);
        assert_eq!(r, RectU32::of(0, 0, u32::MAX, u32::MAX));
    }

    #[test]
    fn inflate_almost_max_bounds() {
        let mut r = RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1);
        inflate(&mut r);
        assert_eq!(r, RectU32::of(0, 0, u32::MAX, u32::MAX));
    }

    #[test]
    fn inflate_max_width() {
        let mut r = RectU32::of(0, 10, u32::MAX, 250);
        inflate(&mut r);
        assert_eq!(r, RectU32::of(0, 10, u32::MAX, 250));
    }

    #[test]
    fn inflate_max_height() {
        let mut r = RectU32::of(10, 0, 250, u32::MAX);
        inflate(&mut r);
        assert_eq!(r, RectU32::of(10, 0, 250, u32::MAX));
    }

    #[test]
    fn deflate_odd() {
        let mut r = RectU32::of(0, 0, 9, 9);
        deflate(&mut r);
        assert_eq!(r, RectU32::of(1, 1, 8, 8));
        deflate(&mut r);
        assert_eq!(r, RectU32::of(2, 2, 7, 7));
        deflate(&mut r);
        assert_eq!(r, RectU32::of(3, 3, 6, 6));
        deflate(&mut r);
        assert_eq!(r, RectU32::of(4, 4, 5, 5));
        deflate(&mut r);
        assert_eq!(r, RectU32::of(4, 4, 5, 5));
    }

    #[test]
    fn deflate_even() {
        let mut r = RectU32::of(0, 0, 10, 10);
        deflate(&mut r);
        assert_eq!(r, RectU32::of(1, 1, 9, 9));
        deflate(&mut r);
        assert_eq!(r, RectU32::of(2, 2, 8, 8));
        deflate(&mut r);
        assert_eq!(r, RectU32::of(3, 3, 7, 7));
        deflate(&mut r);
        assert_eq!(r, RectU32::of(4, 4, 6, 6));
        deflate(&mut r);
        assert_eq!(r, RectU32::of(4, 4, 6, 6));
    }

    #[test]
    fn resize_odd() {
        let mut r = RectU32::of(5, 5, 15, 15);
        resize(&mut r, 11);
        assert_eq!(r, RectU32::of(5, 5, 15, 15));
        resize(&mut r, 9);
        assert_eq!(r, RectU32::of(6, 6, 14, 14));
        resize(&mut r, 7);
        assert_eq!(r, RectU32::of(7, 7, 13, 13));
        resize(&mut r, 5);
        assert_eq!(r, RectU32::of(8, 8, 12, 12));
        resize(&mut r, 3);
        assert_eq!(r, RectU32::of(9, 9, 11, 11));
        resize(&mut r, 1);
        assert_eq!(r, RectU32::of(9, 9, 11, 11));
        resize(&mut r, 3);
        assert_eq!(r, RectU32::of(9, 9, 11, 11));
        resize(&mut r, 9);
        assert_eq!(r, RectU32::of(6, 6, 14, 14));
    }

    #[test]
    fn resize_even() {
        let mut r = RectU32::of(5, 5, 14, 14);
        resize(&mut r, 10);
        assert_eq!(r, RectU32::of(5, 5, 14, 14));
        resize(&mut r, 8);
        assert_eq!(r, RectU32::of(6, 6, 13, 13));
        resize(&mut r, 6);
        assert_eq!(r, RectU32::of(7, 7, 12, 12));
        resize(&mut r, 4);
        assert_eq!(r, RectU32::of(8, 8, 11, 11));
        resize(&mut r, 2);
        assert_eq!(r, RectU32::of(8, 8, 11, 11));
        resize(&mut r, 4);
        assert_eq!(r, RectU32::of(8, 8, 11, 11));
        resize(&mut r, 8);
        assert_eq!(r, RectU32::of(6, 6, 13, 13));
    }

    #[test]
    fn resize_odd_min_bounds_big_delta() {
        let mut r = RectU32::of(0, 0, 2, 2);
        resize(&mut r, u32::MAX);
        assert_eq!(r, RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1));
    }

    #[test]
    fn resize_even_min_bounds_big_delta() {
        let mut r = RectU32::of(0, 0, 3, 3);
        resize(&mut r, u32::MAX - 1);
        assert_eq!(r, RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 2));
    }

    #[test]
    fn resize_odd_max_bounds_big_delta() {
        let mut r = RectU32::of(u32::MAX - 2, u32::MAX - 2, u32::MAX, u32::MAX);
        resize(&mut r, u32::MAX);
        assert_eq!(r, RectU32::of(1, 1, u32::MAX, u32::MAX));
    }

    #[test]
    fn resize_even_max_bounds_big_delta() {
        let mut r = RectU32::of(u32::MAX - 3, u32::MAX - 3, u32::MAX, u32::MAX);
        resize(&mut r, u32::MAX - 1);
        assert_eq!(r, RectU32::of(2, 2, u32::MAX, u32::MAX));
    }

    #[test]
    fn test_saturating_translate() {
        let mut r = RectU32::of(0, 0, 12, 15);
        saturating_translate(&mut r, &PointI32::of(5, 4));
        assert_eq!(r, RectU32::of(5, 4, 17, 19));
        saturating_translate(&mut r, &PointI32::of(-4, -2));
        assert_eq!(r, RectU32::of(1, 2, 13, 17));
        saturating_translate(&mut r, &PointI32::of(10, 20));
        assert_eq!(r, RectU32::of(11, 22, 23, 37));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut r = RectU32::of(2, 5, 12, 15);
        saturating_translate(&mut r, &PointI32::of(-10, -10));
        assert_eq!(r, RectU32::of(0, 0, 10, 10));
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut r = RectU32::of(240, 235, u32::MAX - 5, u32::MAX - 10);
        saturating_translate(&mut r, &PointI32::of(20, 20));
        assert_eq!(r, RectU32::of(245, 245, u32::MAX, u32::MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_big_delta() {
        let mut r = RectU32::of(0, 0, 10, 10);
        saturating_translate(&mut r, &PointI32::min());
        assert_eq!(r, RectU32::of(0, 0, 10, 10));
    }

    #[test]
    fn saturating_translate_max_bounds_big_delta() {
        let mut r = RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX, u32::MAX);
        saturating_translate(&mut r, &PointI32::max());
        assert_eq!(r, RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX, u32::MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_big_rect_big_delta() {
        let mut r = RectU32::of(1, 1, u32::MAX, u32::MAX);
        saturating_translate(&mut r, &PointI32::min());
        assert_eq!(r, RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1));
    }

    #[test]
    fn saturating_translate_max_bounds_big_rect_big_delta() {
        let mut r = RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1);
        saturating_translate(&mut r, &PointI32::max());
        assert_eq!(r, RectU32::of(1, 1, u32::MAX, u32::MAX));
    }

    #[test]
    fn test_checked_translate() {
        let mut r = RectU32::of(0, 0, 12, 15);
        assert_eq!(checked_translate(&mut r, &PointI32::of(5, 4)), Ok(()));
        assert_eq!(r, RectU32::of(5, 4, 17, 19));
        assert_eq!(checked_translate(&mut r, &PointI32::of(-4, -2)), Ok(()));
        assert_eq!(r, RectU32::of(1, 2, 13, 17));
        assert_eq!(checked_translate(&mut r, &PointI32::of(10, 20)), Ok(()));
        assert_eq!(r, RectU32::of(11, 22, 23, 37));
    }

    #[test]
    fn checked_translate_min_bounds() {
        let mut r = RectU32::of(2, 5, 12, 15);
        assert_eq!(checked_translate(&mut r, &PointI32::of(-10, -10)), Err(()));
        assert_eq!(r, RectU32::of(2, 5, 12, 15));
    }

    #[test]
    fn checked_translate_max_bounds() {
        let mut r = RectU32::of(240, 235, u32::MAX - 5, u32::MAX - 10);
        assert_eq!(checked_translate(&mut r, &PointI32::of(20, 20)), Err(()));
        assert_eq!(r, RectU32::of(240, 235, u32::MAX - 5, u32::MAX - 10));
    }

    #[test]
    fn checked_translate_min_bounds_big_rect_big_delta() {
        let mut r = RectU32::of(1, 1, u32::MAX, u32::MAX);
        assert_eq!(checked_translate(&mut r, &PointI32::min()), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI32::of(i32::MIN, 0)), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI32::of(0, i32::MIN)), Err(()));
        assert_eq!(r, RectU32::of(1, 1, u32::MAX, u32::MAX));
    }

    #[test]
    fn checked_translate_max_bounds_big_rect_big_delta() {
        let mut r = RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1);
        assert_eq!(checked_translate(&mut r, &PointI32::max()), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI32::of(i32::MAX, 0)), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI32::of(0, i32::MAX)), Err(()));
        assert_eq!(r, RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1));
    }

    #[test]
    fn checked_translate_min_bounds_big_rect_small_delta() {
        let mut r = RectU32::of(1, 1, u32::MAX, u32::MAX);
        assert_eq!(checked_translate(&mut r, &PointI32::of(-1, -1)), Ok(()));
        assert_eq!(r, RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1));
    }

    #[test]
    fn checked_translate_max_bounds_big_rect_small_delta() {
        let mut r = RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1);
        assert_eq!(checked_translate(&mut r, &PointI32::of(1, 1)), Ok(()));
        assert_eq!(r, RectU32::of(1, 1, u32::MAX, u32::MAX));
    }

    #[test]
    fn contains_inside_borders() {
        let r = RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1);
        assert!(contains(&r, &PointU32::of(1, 1)));
        assert!(contains(&r, &PointU32::of(1, u32::MAX - 1)));
        assert!(contains(&r, &PointU32::of(u32::MAX - 1, 1)));
        assert!(contains(&r, &PointU32::of(u32::MAX - 1, u32::MAX - 1)));
    }

    #[test]
    fn contains_outside_borders() {
        let r = RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1);
        assert!(!contains(&r, &PointU32::of(0, 0)));
        assert!(!contains(&r, &PointU32::of(0, u32::MAX)));
        assert!(!contains(&r, &PointU32::of(u32::MAX, 0)));
        assert!(!contains(&r, &PointU32::max()));
        assert!(!contains(&r, &PointU32::of(1, 0)));
        assert!(!contains(&r, &PointU32::of(1, u32::MAX)));
        assert!(!contains(&r, &PointU32::of(u32::MAX - 1, 0)));
        assert!(!contains(&r, &PointU32::of(u32::MAX - 1, u32::MAX)));
        assert!(!contains(&r, &PointU32::of(0, 1)));
        assert!(!contains(&r, &PointU32::of(0, u32::MAX - 1)));
        assert!(!contains(&r, &PointU32::of(u32::MAX, 1)));
        assert!(!contains(&r, &PointU32::of(u32::MAX, u32::MAX - 1)));
    }

    #[test]
    fn contains_inside() {
        let r = RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1);
        assert!(contains(&r, &PointU32::of(10, 10)));
        assert!(contains(&r, &PointU32::of(u32::MAX - 10, u32::MAX - 10)));
    }
}

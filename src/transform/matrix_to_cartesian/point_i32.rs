type Cartesian = crate::cartesian::point::point_i32::Point;
type Matrix = crate::matrix::point::point_u32::Point;

pub fn matrix_to_cartesian(point: &Matrix) -> Cartesian {
    let x = i64::from(i32::MIN) + i64::from(point.col);
    let y = i64::from(i32::MAX) - i64::from(point.row);
    Cartesian { x: x as i32, y: y as i32 }
}

#[cfg(test)]
mod tests {
    use super::{Cartesian, Matrix, matrix_to_cartesian};

    #[test]
    fn edges() {
        assert_eq!(matrix_to_cartesian(&Matrix::of(0, 0)), Cartesian::of(i32::MIN, i32::MAX));
        assert_eq!(matrix_to_cartesian(&Matrix::of(u32::MAX, 0)), Cartesian::min());
        assert_eq!(matrix_to_cartesian(&Matrix::of(0, u32::MAX)), Cartesian::max());
        assert_eq!(matrix_to_cartesian(&Matrix::max()), Cartesian::of(i32::MAX, i32::MIN));
    }

    #[test]
    fn sequence_min() {
        assert_eq!(matrix_to_cartesian(&Matrix::of(1, 1)), Cartesian::of(i32::MIN + 1, i32::MAX - 1));
        assert_eq!(matrix_to_cartesian(&Matrix::of(2, 2)), Cartesian::of(i32::MIN + 2, i32::MAX - 2));
        assert_eq!(matrix_to_cartesian(&Matrix::of(3, 3)), Cartesian::of(i32::MIN + 3, i32::MAX - 3));
    }

    #[test]
    fn sequence_max() {
        assert_eq!(matrix_to_cartesian(&Matrix::of(u32::MAX - 1, u32::MAX - 1)), Cartesian::of(i32::MAX - 1, i32::MIN + 1));
        assert_eq!(matrix_to_cartesian(&Matrix::of(u32::MAX - 2, u32::MAX - 2)), Cartesian::of(i32::MAX - 2, i32::MIN + 2));
        assert_eq!(matrix_to_cartesian(&Matrix::of(u32::MAX - 3, u32::MAX - 3)), Cartesian::of(i32::MAX - 3, i32::MIN + 3));
    }
}

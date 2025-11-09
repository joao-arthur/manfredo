type Cartesian = crate::cartesian::d2::point::point_i64::Point;
type Matrix = crate::matrix::d2::point::point_u64::Point;

pub fn matrix_to_cartesian(point: &Matrix) -> Cartesian {
    let x = i128::from(i64::MIN) + i128::from(point.col);
    let y = i128::from(i64::MAX) - i128::from(point.row);
    Cartesian { x: x as i64, y: y as i64 }
}

#[cfg(test)]
mod tests {
    use super::{Cartesian, Matrix, matrix_to_cartesian};

    #[test]
    fn edges() {
        assert_eq!(matrix_to_cartesian(&Matrix::min()), Cartesian::new(i64::MIN, i64::MAX));
        assert_eq!(matrix_to_cartesian(&Matrix::new(u64::MAX, 0)), Cartesian::min());
        assert_eq!(matrix_to_cartesian(&Matrix::new(0, u64::MAX)), Cartesian::max());
        assert_eq!(matrix_to_cartesian(&Matrix::max()), Cartesian::new(i64::MAX, i64::MIN));
    }

    #[test]
    fn sequence_min() {
        assert_eq!(matrix_to_cartesian(&Matrix::new(1, 1)), Cartesian::new(i64::MIN + 1, i64::MAX - 1));
        assert_eq!(matrix_to_cartesian(&Matrix::new(2, 2)), Cartesian::new(i64::MIN + 2, i64::MAX - 2));
        assert_eq!(matrix_to_cartesian(&Matrix::new(3, 3)), Cartesian::new(i64::MIN + 3, i64::MAX - 3));
    }

    #[test]
    fn sequence_max() {
        assert_eq!(matrix_to_cartesian(&Matrix::new(u64::MAX - 1, u64::MAX - 1)), Cartesian::new(i64::MAX - 1, i64::MIN + 1));
        assert_eq!(matrix_to_cartesian(&Matrix::new(u64::MAX - 2, u64::MAX - 2)), Cartesian::new(i64::MAX - 2, i64::MIN + 2));
        assert_eq!(matrix_to_cartesian(&Matrix::new(u64::MAX - 3, u64::MAX - 3)), Cartesian::new(i64::MAX - 3, i64::MIN + 3));
    }
}

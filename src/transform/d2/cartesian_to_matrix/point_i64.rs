type Cartesian = crate::cartesian::d2::point::point_i64::Point;
type Matrix = crate::matrix::d2::point::point_u64::Point;

pub fn cartesian_to_matrix(point: &Cartesian) -> Matrix {
    let row = i128::from(i64::MAX) - i128::from(point.y);
    let col = i128::from(point.x) - i128::from(i64::MIN);
    Matrix { row: row as u64, col: col as u64 }
}

#[cfg(test)]
mod tests {
    use super::{Cartesian, Matrix, cartesian_to_matrix};

    #[test]
    fn bounds() {
        assert_eq!(cartesian_to_matrix(&Cartesian::min()), Matrix::new(u64::MAX, 0));
        assert_eq!(cartesian_to_matrix(&Cartesian::new(i64::MAX, i64::MIN)), Matrix::max());
        assert_eq!(cartesian_to_matrix(&Cartesian::new(i64::MIN, i64::MAX)), Matrix::min());
        assert_eq!(cartesian_to_matrix(&Cartesian::max()), Matrix::new(0, u64::MAX));
    }

    #[test]
    fn sequence_min() {
        assert_eq!(cartesian_to_matrix(&Cartesian::new(i64::MIN + 1, i64::MIN + 1)), Matrix::new(u64::MAX - 1, 1));
        assert_eq!(cartesian_to_matrix(&Cartesian::new(i64::MIN + 2, i64::MIN + 2)), Matrix::new(u64::MAX - 2, 2));
        assert_eq!(cartesian_to_matrix(&Cartesian::new(i64::MIN + 3, i64::MIN + 3)), Matrix::new(u64::MAX - 3, 3));
    }

    #[test]
    fn sequence_max() {
        assert_eq!(cartesian_to_matrix(&Cartesian::new(i64::MAX - 1, i64::MAX - 1)), Matrix::new(1, u64::MAX - 1));
        assert_eq!(cartesian_to_matrix(&Cartesian::new(i64::MAX - 2, i64::MAX - 2)), Matrix::new(2, u64::MAX - 2));
        assert_eq!(cartesian_to_matrix(&Cartesian::new(i64::MAX - 3, i64::MAX - 3)), Matrix::new(3, u64::MAX - 3));
    }
}

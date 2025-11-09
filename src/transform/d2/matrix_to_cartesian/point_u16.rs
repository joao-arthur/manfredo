type Cartesian = crate::cartesian::d2::point::point_u16::Point;
type Matrix = crate::matrix::d2::point::point_u16::Point;

pub fn matrix_to_cartesian(point: &Matrix) -> Cartesian {
    Cartesian { x: point.col, y: u16::MAX - point.row }
}

#[cfg(test)]
mod tests {
    use super::{Cartesian, Matrix, matrix_to_cartesian};

    #[test]
    fn edges() {
        assert_eq!(matrix_to_cartesian(&Matrix::min()), Cartesian::new(0, u16::MAX));
        assert_eq!(matrix_to_cartesian(&Matrix::new(u16::MAX, 0)), Cartesian::min());
        assert_eq!(matrix_to_cartesian(&Matrix::new(0, u16::MAX)), Cartesian::max());
        assert_eq!(matrix_to_cartesian(&Matrix::max()), Cartesian::new(u16::MAX, 0));
    }

    #[test]
    fn sequence_min() {
        assert_eq!(matrix_to_cartesian(&Matrix::new(1, 1)), Cartesian::new(1, u16::MAX - 1));
        assert_eq!(matrix_to_cartesian(&Matrix::new(2, 2)), Cartesian::new(2, u16::MAX - 2));
        assert_eq!(matrix_to_cartesian(&Matrix::new(3, 3)), Cartesian::new(3, u16::MAX - 3));
    }

    #[test]
    fn sequence_max() {
        assert_eq!(matrix_to_cartesian(&Matrix::new(u16::MAX - 1, u16::MAX - 1)), Cartesian::new(u16::MAX - 1, 1));
        assert_eq!(matrix_to_cartesian(&Matrix::new(u16::MAX - 2, u16::MAX - 2)), Cartesian::new(u16::MAX - 2, 2));
        assert_eq!(matrix_to_cartesian(&Matrix::new(u16::MAX - 3, u16::MAX - 3)), Cartesian::new(u16::MAX - 3, 3));
    }
}

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
        assert_eq!(matrix_to_cartesian(&Matrix::of(0, 0)), Cartesian::of(0, u16::MAX));
        assert_eq!(matrix_to_cartesian(&Matrix::of(u16::MAX, 0)), Cartesian::min());
        assert_eq!(matrix_to_cartesian(&Matrix::of(0, u16::MAX)), Cartesian::max());
        assert_eq!(matrix_to_cartesian(&Matrix::max()), Cartesian::of(u16::MAX, 0));
    }

    #[test]
    fn sequence_min() {
        assert_eq!(matrix_to_cartesian(&Matrix::of(1, 1)), Cartesian::of(1, u16::MAX - 1));
        assert_eq!(matrix_to_cartesian(&Matrix::of(2, 2)), Cartesian::of(2, u16::MAX - 2));
        assert_eq!(matrix_to_cartesian(&Matrix::of(3, 3)), Cartesian::of(3, u16::MAX - 3));
    }

    #[test]
    fn sequence_max() {
        assert_eq!(matrix_to_cartesian(&Matrix::of(u16::MAX - 1, u16::MAX - 1)), Cartesian::of(u16::MAX - 1, 1));
        assert_eq!(matrix_to_cartesian(&Matrix::of(u16::MAX - 2, u16::MAX - 2)), Cartesian::of(u16::MAX - 2, 2));
        assert_eq!(matrix_to_cartesian(&Matrix::of(u16::MAX - 3, u16::MAX - 3)), Cartesian::of(u16::MAX - 3, 3));
    }
}

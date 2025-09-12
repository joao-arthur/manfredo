type Cartesian = crate::cartesian::point::point_u8::Point;
type Matrix = crate::matrix::point::point_u8::Point;

pub fn matrix_to_cartesian(point: &Matrix) -> Cartesian {
    Cartesian { x: point.col, y: u8::MAX - point.row }
}

#[cfg(test)]
mod tests {
    use super::{Cartesian, Matrix, matrix_to_cartesian};

    #[test]
    fn edges() {
        assert_eq!(matrix_to_cartesian(&Matrix::of(0, 0)), Cartesian::of(0, u8::MAX));
        assert_eq!(matrix_to_cartesian(&Matrix::of(u8::MAX, 0)), Cartesian::of(0, 0));
        assert_eq!(matrix_to_cartesian(&Matrix::of(0, u8::MAX)), Cartesian::max());
        assert_eq!(matrix_to_cartesian(&Matrix::max()), Cartesian::of(u8::MAX, 0));
    }

    #[test]
    fn sequence_min() {
        assert_eq!(matrix_to_cartesian(&Matrix::of(1, 1)), Cartesian::of(1, u8::MAX - 1));
        assert_eq!(matrix_to_cartesian(&Matrix::of(2, 2)), Cartesian::of(2, u8::MAX - 2));
        assert_eq!(matrix_to_cartesian(&Matrix::of(3, 3)), Cartesian::of(3, u8::MAX - 3));
    }

    #[test]
    fn sequence_max() {
        assert_eq!(matrix_to_cartesian(&Matrix::of(u8::MAX - 1, u8::MAX - 1)), Cartesian::of(u8::MAX - 1, 1));
        assert_eq!(matrix_to_cartesian(&Matrix::of(u8::MAX - 2, u8::MAX - 2)), Cartesian::of(u8::MAX - 2, 2));
        assert_eq!(matrix_to_cartesian(&Matrix::of(u8::MAX - 3, u8::MAX - 3)), Cartesian::of(u8::MAX - 3, 3));
    }
}

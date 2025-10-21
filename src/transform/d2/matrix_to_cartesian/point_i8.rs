type Cartesian = crate::cartesian::d2::point::point_i8::Point;
type Matrix = crate::matrix::d2::point::point_u8::Point;

pub fn matrix_to_cartesian(point: &Matrix) -> Cartesian {
    let x = i16::from(i8::MIN) + i16::from(point.col);
    let y = i16::from(i8::MAX) - i16::from(point.row);
    Cartesian { x: x as i8, y: y as i8 }
}

#[cfg(test)]
mod tests {
    use super::{Cartesian, Matrix, matrix_to_cartesian};

    #[test]
    fn edges() {
        assert_eq!(matrix_to_cartesian(&Matrix::min()), Cartesian::of(i8::MIN, i8::MAX));
        assert_eq!(matrix_to_cartesian(&Matrix::of(u8::MAX, 0)), Cartesian::min());
        assert_eq!(matrix_to_cartesian(&Matrix::of(0, u8::MAX)), Cartesian::max());
        assert_eq!(matrix_to_cartesian(&Matrix::max()), Cartesian::of(i8::MAX, i8::MIN));
    }

    #[test]
    fn sequence_min() {
        assert_eq!(matrix_to_cartesian(&Matrix::of(1, 1)), Cartesian::of(i8::MIN + 1, i8::MAX - 1));
        assert_eq!(matrix_to_cartesian(&Matrix::of(2, 2)), Cartesian::of(i8::MIN + 2, i8::MAX - 2));
        assert_eq!(matrix_to_cartesian(&Matrix::of(3, 3)), Cartesian::of(i8::MIN + 3, i8::MAX - 3));
    }

    #[test]
    fn sequence_max() {
        assert_eq!(matrix_to_cartesian(&Matrix::of(u8::MAX - 1, u8::MAX - 1)), Cartesian::of(i8::MAX - 1, i8::MIN + 1));
        assert_eq!(matrix_to_cartesian(&Matrix::of(u8::MAX - 2, u8::MAX - 2)), Cartesian::of(i8::MAX - 2, i8::MIN + 2));
        assert_eq!(matrix_to_cartesian(&Matrix::of(u8::MAX - 3, u8::MAX - 3)), Cartesian::of(i8::MAX - 3, i8::MIN + 3));
    }
}

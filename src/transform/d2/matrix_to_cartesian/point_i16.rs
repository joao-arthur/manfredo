type Cartesian = crate::cartesian::d2::point::point_i16::Point;
type Matrix = crate::matrix::d2::point::point_u16::Point;

pub fn matrix_to_cartesian(point: &Matrix) -> Cartesian {
    let x = i32::from(i16::MIN) + i32::from(point.col);
    let y = i32::from(i16::MAX) - i32::from(point.row);
    Cartesian { x: x as i16, y: y as i16 }
}

#[cfg(test)]
mod tests {
    use super::{Cartesian, Matrix, matrix_to_cartesian};

    #[test]
    fn edges() {
        assert_eq!(matrix_to_cartesian(&Matrix::of(0, 0)), Cartesian::of(i16::MIN, i16::MAX));
        assert_eq!(matrix_to_cartesian(&Matrix::of(u16::MAX, 0)), Cartesian::min());
        assert_eq!(matrix_to_cartesian(&Matrix::of(0, u16::MAX)), Cartesian::max());
        assert_eq!(matrix_to_cartesian(&Matrix::max()), Cartesian::of(i16::MAX, i16::MIN));
    }

    #[test]
    fn sequence_min() {
        assert_eq!(matrix_to_cartesian(&Matrix::of(1, 1)), Cartesian::of(i16::MIN + 1, i16::MAX - 1));
        assert_eq!(matrix_to_cartesian(&Matrix::of(2, 2)), Cartesian::of(i16::MIN + 2, i16::MAX - 2));
        assert_eq!(matrix_to_cartesian(&Matrix::of(3, 3)), Cartesian::of(i16::MIN + 3, i16::MAX - 3));
    }

    #[test]
    fn sequence_max() {
        assert_eq!(matrix_to_cartesian(&Matrix::of(u16::MAX - 1, u16::MAX - 1)), Cartesian::of(i16::MAX - 1, i16::MIN + 1));
        assert_eq!(matrix_to_cartesian(&Matrix::of(u16::MAX - 2, u16::MAX - 2)), Cartesian::of(i16::MAX - 2, i16::MIN + 2));
        assert_eq!(matrix_to_cartesian(&Matrix::of(u16::MAX - 3, u16::MAX - 3)), Cartesian::of(i16::MAX - 3, i16::MIN + 3));
    }
}

type Cartesian = crate::cartesian::d2::point::point_i8::Point;
type Matrix = crate::matrix::d2::point::point_u8::Point;

pub fn cartesian_to_matrix(point: &Cartesian) -> Matrix {
    let row = i16::from(i8::MAX) - i16::from(point.y);
    let col = i16::from(point.x) - i16::from(i8::MIN);
    Matrix { row: row as u8, col: col as u8 }
}

#[cfg(test)]
mod tests {
    use super::{Cartesian, Matrix, cartesian_to_matrix};

    #[test]
    fn bounds() {
        assert_eq!(cartesian_to_matrix(&Cartesian::min()), Matrix::new(u8::MAX, 0));
        assert_eq!(cartesian_to_matrix(&Cartesian::new(i8::MAX, i8::MIN)), Matrix::max());
        assert_eq!(cartesian_to_matrix(&Cartesian::new(i8::MIN, i8::MAX)), Matrix::min());
        assert_eq!(cartesian_to_matrix(&Cartesian::max()), Matrix::new(0, u8::MAX));
    }

    #[test]
    fn sequence_min() {
        assert_eq!(cartesian_to_matrix(&Cartesian::new(i8::MIN + 1, i8::MIN + 1)), Matrix::new(u8::MAX - 1, 1));
        assert_eq!(cartesian_to_matrix(&Cartesian::new(i8::MIN + 2, i8::MIN + 2)), Matrix::new(u8::MAX - 2, 2));
        assert_eq!(cartesian_to_matrix(&Cartesian::new(i8::MIN + 3, i8::MIN + 3)), Matrix::new(u8::MAX - 3, 3));
    }

    #[test]
    fn sequence_max() {
        assert_eq!(cartesian_to_matrix(&Cartesian::new(i8::MAX - 1, i8::MAX - 1)), Matrix::new(1, u8::MAX - 1));
        assert_eq!(cartesian_to_matrix(&Cartesian::new(i8::MAX - 2, i8::MAX - 2)), Matrix::new(2, u8::MAX - 2));
        assert_eq!(cartesian_to_matrix(&Cartesian::new(i8::MAX - 3, i8::MAX - 3)), Matrix::new(3, u8::MAX - 3));
    }
}

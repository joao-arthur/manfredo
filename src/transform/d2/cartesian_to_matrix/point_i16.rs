type Cartesian = crate::cartesian::d2::point::point_i16::Point;
type Matrix = crate::matrix::d2::point::point_u16::Point;

pub fn cartesian_to_matrix(point: &Cartesian) -> Matrix {
    let row = i32::from(i16::MAX) - i32::from(point.y);
    let col = i32::from(point.x) - i32::from(i16::MIN);
    Matrix { row: row as u16, col: col as u16 }
}

#[cfg(test)]
mod tests {
    use super::{Cartesian, Matrix, cartesian_to_matrix};

    #[test]
    fn bounds() {
        assert_eq!(cartesian_to_matrix(&Cartesian::min()), Matrix::of(u16::MAX, 0));
        assert_eq!(cartesian_to_matrix(&Cartesian::of(i16::MAX, i16::MIN)), Matrix::max());
        assert_eq!(cartesian_to_matrix(&Cartesian::of(i16::MIN, i16::MAX)), Matrix::min());
        assert_eq!(cartesian_to_matrix(&Cartesian::max()), Matrix::of(0, u16::MAX));
    }

    #[test]
    fn sequence_min() {
        assert_eq!(cartesian_to_matrix(&Cartesian::of(i16::MIN + 1, i16::MIN + 1)), Matrix::of(u16::MAX - 1, 1));
        assert_eq!(cartesian_to_matrix(&Cartesian::of(i16::MIN + 2, i16::MIN + 2)), Matrix::of(u16::MAX - 2, 2));
        assert_eq!(cartesian_to_matrix(&Cartesian::of(i16::MIN + 3, i16::MIN + 3)), Matrix::of(u16::MAX - 3, 3));
    }

    #[test]
    fn sequence_max() {
        assert_eq!(cartesian_to_matrix(&Cartesian::of(i16::MAX - 1, i16::MAX - 1)), Matrix::of(1, u16::MAX - 1));
        assert_eq!(cartesian_to_matrix(&Cartesian::of(i16::MAX - 2, i16::MAX - 2)), Matrix::of(2, u16::MAX - 2));
        assert_eq!(cartesian_to_matrix(&Cartesian::of(i16::MAX - 3, i16::MAX - 3)), Matrix::of(3, u16::MAX - 3));
    }
}

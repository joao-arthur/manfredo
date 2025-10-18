type Cartesian = crate::cartesian::d2::point::point_u32::Point;
type Matrix = crate::matrix::d2::point::point_u32::Point;

pub fn cartesian_to_matrix(point: &Cartesian) -> Matrix {
    Matrix { row: u32::MAX - point.y, col: point.x }
}

#[cfg(test)]
mod tests {
    use super::{Cartesian, Matrix, cartesian_to_matrix};

    #[test]
    fn bounds() {
        assert_eq!(cartesian_to_matrix(&Cartesian::min()), Matrix::of(u32::MAX, 0));
        assert_eq!(cartesian_to_matrix(&Cartesian::of(u32::MAX, 0)), Matrix::max());
        assert_eq!(cartesian_to_matrix(&Cartesian::of(0, u32::MAX)), Matrix::of(0, 0));
        assert_eq!(cartesian_to_matrix(&Cartesian::max()), Matrix::of(0, u32::MAX));
    }

    #[test]
    fn sequence_min() {
        assert_eq!(cartesian_to_matrix(&Cartesian::of(1, 1)), Matrix::of(u32::MAX - 1, 1));
        assert_eq!(cartesian_to_matrix(&Cartesian::of(2, 2)), Matrix::of(u32::MAX - 2, 2));
        assert_eq!(cartesian_to_matrix(&Cartesian::of(3, 3)), Matrix::of(u32::MAX - 3, 3));
    }

    #[test]
    fn sequence_max() {
        assert_eq!(cartesian_to_matrix(&Cartesian::of(u32::MAX - 1, u32::MAX - 1)), Matrix::of(1, u32::MAX - 1));
        assert_eq!(cartesian_to_matrix(&Cartesian::of(u32::MAX - 2, u32::MAX - 2)), Matrix::of(2, u32::MAX - 2));
        assert_eq!(cartesian_to_matrix(&Cartesian::of(u32::MAX - 3, u32::MAX - 3)), Matrix::of(3, u32::MAX - 3));
    }
}

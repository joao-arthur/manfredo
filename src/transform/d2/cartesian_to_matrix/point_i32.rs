type Cartesian = crate::cartesian::point::point_i32::Point;
type Matrix = crate::matrix::point::point_u32::Point;

pub fn cartesian_to_matrix(point: &Cartesian) -> Matrix {
    let row = i64::from(i32::MAX) - i64::from(point.y);
    let col = i64::from(point.x) - i64::from(i32::MIN);
    Matrix { row: row as u32, col: col as u32 }
}

#[cfg(test)]
mod tests {
    use super::{Cartesian, Matrix, cartesian_to_matrix};

    #[test]
    fn bounds() {
        assert_eq!(cartesian_to_matrix(&Cartesian::min()), Matrix::of(u32::MAX, 0));
        assert_eq!(cartesian_to_matrix(&Cartesian::of(i32::MAX, i32::MIN)), Matrix::max());
        assert_eq!(cartesian_to_matrix(&Cartesian::of(i32::MIN, i32::MAX)), Matrix::of(0, 0));
        assert_eq!(cartesian_to_matrix(&Cartesian::max()), Matrix::of(0, u32::MAX));
    }

    #[test]
    fn sequence_min() {
        assert_eq!(cartesian_to_matrix(&Cartesian::of(i32::MIN + 1, i32::MIN + 1)), Matrix::of(u32::MAX - 1, 1));
        assert_eq!(cartesian_to_matrix(&Cartesian::of(i32::MIN + 2, i32::MIN + 2)), Matrix::of(u32::MAX - 2, 2));
        assert_eq!(cartesian_to_matrix(&Cartesian::of(i32::MIN + 3, i32::MIN + 3)), Matrix::of(u32::MAX - 3, 3));
    }

    #[test]
    fn sequence_max() {
        assert_eq!(cartesian_to_matrix(&Cartesian::of(i32::MAX - 1, i32::MAX - 1)), Matrix::of(1, u32::MAX - 1));
        assert_eq!(cartesian_to_matrix(&Cartesian::of(i32::MAX - 2, i32::MAX - 2)), Matrix::of(2, u32::MAX - 2));
        assert_eq!(cartesian_to_matrix(&Cartesian::of(i32::MAX - 3, i32::MAX - 3)), Matrix::of(3, u32::MAX - 3));
    }
}

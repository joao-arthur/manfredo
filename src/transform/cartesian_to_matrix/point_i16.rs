type CartesianPoint = crate::cartesian::point::point_i16::PointI16;
type MatrixPoint = crate::matrix::point::point_u16::PointU16;

pub fn cartesian_to_matrix(point: &CartesianPoint) -> MatrixPoint {
    let row = i32::from(i16::MAX) - i32::from(point.y);
    let col = i32::from(point.x) - i32::from(i16::MIN);
    MatrixPoint { row: row as u16, col: col as u16 }
}

#[cfg(test)]
mod tests {
    use super::{CartesianPoint, MatrixPoint, cartesian_to_matrix};

    #[test]
    fn cartesian_to_matrix_bounds() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i16::MIN, i16::MIN)), MatrixPoint::of(u16::MAX, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i16::MAX, i16::MIN)), MatrixPoint::of(u16::MAX, u16::MAX));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i16::MIN, i16::MAX)), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i16::MAX, i16::MAX)), MatrixPoint::of(0, u16::MAX));
    }

    #[test]
    fn cartesian_to_matrix_sequence_min() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i16::MIN + 1, i16::MIN + 1)), MatrixPoint::of(u16::MAX - 1, 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i16::MIN + 2, i16::MIN + 2)), MatrixPoint::of(u16::MAX - 2, 2));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i16::MIN + 3, i16::MIN + 3)), MatrixPoint::of(u16::MAX - 3, 3));
    }

    #[test]
    fn cartesian_to_matrix_sequence_max() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i16::MAX - 1, i16::MAX - 1)), MatrixPoint::of(1, u16::MAX - 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i16::MAX - 2, i16::MAX - 2)), MatrixPoint::of(2, u16::MAX - 2));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i16::MAX - 3, i16::MAX - 3)), MatrixPoint::of(3, u16::MAX - 3));
    }
}

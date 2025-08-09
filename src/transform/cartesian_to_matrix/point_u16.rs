type CartesianPoint = crate::cartesian::point::point_u16::PointU16;
type MatrixPoint = crate::matrix::point::point_u16::PointU16;

pub fn cartesian_to_matrix(point: &CartesianPoint) -> MatrixPoint {
    MatrixPoint { row: u16::MAX - point.y, col: point.x }
}

#[cfg(test)]
mod tests {
    use super::{CartesianPoint, MatrixPoint, cartesian_to_matrix};

    #[test]
    fn cartesian_to_matrix_bounds() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(0, 0)), MatrixPoint::of(u16::MAX, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(u16::MAX, 0)), MatrixPoint::max());
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(0, u16::MAX)), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::max()), MatrixPoint::of(0, u16::MAX));
    }

    #[test]
    fn cartesian_to_matrix_sequence_min() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(1, 1)), MatrixPoint::of(u16::MAX - 1, 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(2, 2)), MatrixPoint::of(u16::MAX - 2, 2));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(3, 3)), MatrixPoint::of(u16::MAX - 3, 3));
    }

    #[test]
    fn cartesian_to_matrix_sequence_max() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(u16::MAX - 1, u16::MAX - 1)), MatrixPoint::of(1, u16::MAX - 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(u16::MAX - 2, u16::MAX - 2)), MatrixPoint::of(2, u16::MAX - 2));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(u16::MAX - 3, u16::MAX - 3)), MatrixPoint::of(3, u16::MAX - 3));
    }
}

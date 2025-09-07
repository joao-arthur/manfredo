type CartesianPoint = crate::cartesian::point::point_u64::PointU64;
type MatrixPoint = crate::matrix::point::point_u64::PointU64;

pub fn cartesian_to_matrix(point: &CartesianPoint) -> MatrixPoint {
    MatrixPoint { row: u64::MAX - point.y, col: point.x }
}

#[cfg(test)]
mod tests {
    use super::{CartesianPoint, MatrixPoint, cartesian_to_matrix};

    #[test]
    fn bounds() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(0, 0)), MatrixPoint::of(u64::MAX, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(u64::MAX, 0)), MatrixPoint::max());
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(0, u64::MAX)), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::max()), MatrixPoint::of(0, u64::MAX));
    }

    #[test]
    fn sequence_min() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(1, 1)), MatrixPoint::of(u64::MAX - 1, 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(2, 2)), MatrixPoint::of(u64::MAX - 2, 2));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(3, 3)), MatrixPoint::of(u64::MAX - 3, 3));
    }

    #[test]
    fn sequence_max() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(u64::MAX - 1, u64::MAX - 1)), MatrixPoint::of(1, u64::MAX - 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(u64::MAX - 2, u64::MAX - 2)), MatrixPoint::of(2, u64::MAX - 2));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(u64::MAX - 3, u64::MAX - 3)), MatrixPoint::of(3, u64::MAX - 3));
    }
}

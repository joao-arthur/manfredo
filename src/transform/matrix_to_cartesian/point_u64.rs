type CartesianPoint = crate::cartesian::point::point_u64::PointU64;
type MatrixPoint = crate::matrix::point::point_u64::PointU64;

pub fn matrix_to_cartesian(point: &MatrixPoint) -> CartesianPoint {
    CartesianPoint { x: point.col, y: u64::MAX - point.row }
}

#[cfg(test)]
mod tests {
    use super::{CartesianPoint, MatrixPoint, matrix_to_cartesian};

    #[test]
    fn test_matrix_to_cartesian() {
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 0)), CartesianPoint::of(0, u64::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u64::MAX, 0)), CartesianPoint::of(0, 0));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, u64::MAX)), CartesianPoint::of(u64::MAX, u64::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u64::MAX, u64::MAX)), CartesianPoint::of(u64::MAX, 0));
    }

    #[test]
    fn matrix_to_cartesian_edges() {
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(1, 0)), CartesianPoint::of(0, u64::MAX - 1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(2, 0)), CartesianPoint::of(0, u64::MAX - 2));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(3, 0)), CartesianPoint::of(0, u64::MAX - 3));

        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u64::MAX - 1, 0)), CartesianPoint::of(0, 1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u64::MAX - 2, 0)), CartesianPoint::of(0, 2));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u64::MAX - 3, 0)), CartesianPoint::of(0, 3));

        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 1)), CartesianPoint::of(1, u64::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 2)), CartesianPoint::of(2, u64::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 3)), CartesianPoint::of(3, u64::MAX));

        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, u64::MAX - 1)), CartesianPoint::of(u64::MAX - 1, u64::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, u64::MAX - 2)), CartesianPoint::of(u64::MAX - 2, u64::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, u64::MAX - 3)), CartesianPoint::of(u64::MAX - 3, u64::MAX));
    }
}

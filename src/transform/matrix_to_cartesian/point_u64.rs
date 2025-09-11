type CartesianPoint = crate::cartesian::point::point_u64::Point;
type MatrixPoint = crate::matrix::point::point_u64::Point;

pub fn matrix_to_cartesian(point: &MatrixPoint) -> CartesianPoint {
    CartesianPoint { x: point.col, y: u64::MAX - point.row }
}

#[cfg(test)]
mod tests {
    use super::{CartesianPoint, MatrixPoint, matrix_to_cartesian};

    #[test]
    fn edges() {
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 0)), CartesianPoint::of(0, u64::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u64::MAX, 0)), CartesianPoint::of(0, 0));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, u64::MAX)), CartesianPoint::max());
        assert_eq!(matrix_to_cartesian(&MatrixPoint::max()), CartesianPoint::of(u64::MAX, 0));
    }

    #[test]
    fn sequence_min() {
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(1, 1)), CartesianPoint::of(1, u64::MAX - 1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(2, 2)), CartesianPoint::of(2, u64::MAX - 2));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(3, 3)), CartesianPoint::of(3, u64::MAX - 3));
    }

    #[test]
    fn sequence_max() {
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u64::MAX - 1, u64::MAX - 1)), CartesianPoint::of(u64::MAX - 1, 1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u64::MAX - 2, u64::MAX - 2)), CartesianPoint::of(u64::MAX - 2, 2));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u64::MAX - 3, u64::MAX - 3)), CartesianPoint::of(u64::MAX - 3, 3));
    }
}

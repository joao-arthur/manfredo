type CartesianPoint = crate::cartesian::point::point_i64::PointI64;
type MatrixPoint = crate::matrix::point::point_u64::PointU64;

pub fn matrix_to_cartesian(point: &MatrixPoint) -> CartesianPoint {
    let x = i128::from(i64::MIN) + i128::from(point.col);
    let y = i128::from(i64::MAX) - i128::from(point.row);
    CartesianPoint { x: x as i64, y: y as i64 }
}

#[cfg(test)]
mod tests {
    use super::{CartesianPoint, MatrixPoint, matrix_to_cartesian};

    #[test]
    fn test_matrix_to_cartesian() {
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 0)), CartesianPoint::of(i64::MIN, i64::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u64::MAX, 0)), CartesianPoint::of(i64::MIN, i64::MIN));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, u64::MAX)), CartesianPoint::of(i64::MAX, i64::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u64::MAX, u64::MAX)), CartesianPoint::of(i64::MAX, i64::MIN));
    }

    #[test]
    fn matrix_to_cartesian_edges() {
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(1, 0)), CartesianPoint::of(i64::MIN, i64::MAX - 1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(2, 0)), CartesianPoint::of(i64::MIN, i64::MAX - 2));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(3, 0)), CartesianPoint::of(i64::MIN, i64::MAX - 3));

        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u64::MAX - 1, 0)), CartesianPoint::of(i64::MIN, i64::MIN + 1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u64::MAX - 2, 0)), CartesianPoint::of(i64::MIN, i64::MIN + 2));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u64::MAX - 3, 0)), CartesianPoint::of(i64::MIN, i64::MIN + 3));

        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 1)), CartesianPoint::of(i64::MIN + 1, i64::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 2)), CartesianPoint::of(i64::MIN + 2, i64::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 3)), CartesianPoint::of(i64::MIN + 3, i64::MAX));

        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, u64::MAX - 1)), CartesianPoint::of(i64::MAX - 1, i64::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, u64::MAX - 2)), CartesianPoint::of(i64::MAX - 2, i64::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, u64::MAX - 3)), CartesianPoint::of(i64::MAX - 3, i64::MAX));
    }
}

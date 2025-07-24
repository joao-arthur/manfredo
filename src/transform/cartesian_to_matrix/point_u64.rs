type CartesianPoint = crate::cartesian::point::point_u64::PointU64;
type MatrixPoint = crate::matrix::point::point_u64::PointU64;

pub fn cartesian_to_matrix(point: &CartesianPoint) -> MatrixPoint {
    MatrixPoint { row: u64::MAX - point.y, col: point.x }
}

#[cfg(test)]
mod tests {
    use super::{CartesianPoint, MatrixPoint, cartesian_to_matrix};

    #[test]
    fn test_cartesian_to_matrix() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(0, 0)), MatrixPoint::of(u64::MAX, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(u64::MAX, 0)), MatrixPoint::of(u64::MAX, u64::MAX));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(0, u64::MAX)), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(u64::MAX, u64::MAX)), MatrixPoint::of(0, u64::MAX));
    }
}

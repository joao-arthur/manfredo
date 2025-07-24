type CartesianPoint = crate::cartesian::point::point_u32::PointU32;
type MatrixPoint = crate::matrix::point::point_u32::PointU32;

pub fn cartesian_to_matrix(point: &CartesianPoint) -> MatrixPoint {
    MatrixPoint { row: u32::MAX - point.y, col: point.x }
}

#[cfg(test)]
mod tests {
    use super::{CartesianPoint, MatrixPoint, cartesian_to_matrix};

    #[test]
    fn test_cartesian_to_matrix() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(0, 0)), MatrixPoint::of(u32::MAX, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(0, u32::MAX)), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(u32::MAX, 0)), MatrixPoint::of(u32::MAX, u32::MAX));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(u32::MAX, u32::MAX)), MatrixPoint::of(0, u32::MAX));
    }
}

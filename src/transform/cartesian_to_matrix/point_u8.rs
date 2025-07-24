type CartesianPoint = crate::cartesian::point::point_u8::PointU8;
type MatrixPoint = crate::matrix::point::point_u8::PointU8;

pub fn cartesian_to_matrix(point: &CartesianPoint) -> MatrixPoint {
    MatrixPoint { row: u8::MAX - point.y, col: point.x }
}

#[cfg(test)]
mod tests {
    use super::{CartesianPoint, MatrixPoint, cartesian_to_matrix};

    #[test]
    fn test_cartesian_to_matrix() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(0, 0)), MatrixPoint::of(u8::MAX, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(u8::MAX, 0)), MatrixPoint::of(u8::MAX, u8::MAX));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(0, u8::MAX)), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(u8::MAX, u8::MAX)), MatrixPoint::of(0, u8::MAX));
    }

    #[test]
    fn test_cartesian_to_matrix_edges() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(0, 0)), MatrixPoint::of(u8::MAX, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(u8::MAX, 0)), MatrixPoint::of(u8::MAX, u8::MAX));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(0, u8::MAX)), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(u8::MAX, u8::MAX)), MatrixPoint::of(0, u8::MAX));
    }
}

type CartesianPoint = crate::cartesian::point::point_i32::PointI32;
type MatrixPoint = crate::matrix::point::point_u32::PointU32;

pub fn matrix_to_cartesian(point: &MatrixPoint) -> CartesianPoint {
    let x = i64::from(i32::MIN) + i64::from(point.col);
    let y = i64::from(i32::MAX) - i64::from(point.row);
    CartesianPoint { x: x as i32, y: y as i32 }
}

#[cfg(test)]
mod tests {
    use super::{CartesianPoint, MatrixPoint, matrix_to_cartesian};

    #[test]
    fn test_matrix_to_cartesian() {
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 0)), CartesianPoint::of(i32::MIN, i32::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u32::MAX, 0)), CartesianPoint::of(i32::MIN, i32::MIN));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, u32::MAX)), CartesianPoint::of(i32::MAX, i32::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u32::MAX, u32::MAX)), CartesianPoint::of(i32::MAX, i32::MIN));
    }

    #[test]
    fn matrix_to_cartesian_edges() {
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(1, 0)), CartesianPoint::of(i32::MIN, i32::MAX - 1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(2, 0)), CartesianPoint::of(i32::MIN, i32::MAX - 2));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(3, 0)), CartesianPoint::of(i32::MIN, i32::MAX - 3));

        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u32::MAX - 1, 0)), CartesianPoint::of(i32::MIN, i32::MIN + 1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u32::MAX - 2, 0)), CartesianPoint::of(i32::MIN, i32::MIN + 2));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u32::MAX - 3, 0)), CartesianPoint::of(i32::MIN, i32::MIN + 3));

        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 1)), CartesianPoint::of(i32::MIN + 1, i32::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 2)), CartesianPoint::of(i32::MIN + 2, i32::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 3)), CartesianPoint::of(i32::MIN + 3, i32::MAX));

        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, u32::MAX - 1)), CartesianPoint::of(i32::MAX - 1, i32::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, u32::MAX - 2)), CartesianPoint::of(i32::MAX - 2, i32::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, u32::MAX - 3)), CartesianPoint::of(i32::MAX - 3, i32::MAX));
    }
}

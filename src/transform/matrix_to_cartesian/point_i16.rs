type CartesianPoint = crate::cartesian::point::point_i16::PointI16;
type MatrixPoint = crate::matrix::point::point_u16::PointU16;

pub fn matrix_to_cartesian(point: &MatrixPoint) -> CartesianPoint {
    let x = i32::from(i16::MIN) + i32::from(point.col);
    let y = i32::from(i16::MAX) - i32::from(point.row);
    CartesianPoint { x: x as i16, y: y as i16 }
}

#[cfg(test)]
mod tests {
    use super::{CartesianPoint, MatrixPoint, matrix_to_cartesian};

    #[test]
    fn test_matrix_to_cartesian() {
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 0)), CartesianPoint::of(i16::MIN, i16::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u16::MAX, 0)), CartesianPoint::of(i16::MIN, i16::MIN));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, u16::MAX)), CartesianPoint::of(i16::MAX, i16::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u16::MAX, u16::MAX)), CartesianPoint::of(i16::MAX, i16::MIN));
    }

    #[test]
    fn matrix_to_cartesian_edges() {
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(1, 0)), CartesianPoint::of(i16::MIN, i16::MAX - 1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(2, 0)), CartesianPoint::of(i16::MIN, i16::MAX - 2));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(3, 0)), CartesianPoint::of(i16::MIN, i16::MAX - 3));

        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u16::MAX - 1, 0)), CartesianPoint::of(i16::MIN, i16::MIN + 1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u16::MAX - 2, 0)), CartesianPoint::of(i16::MIN, i16::MIN + 2));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u16::MAX - 3, 0)), CartesianPoint::of(i16::MIN, i16::MIN + 3));

        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 1)), CartesianPoint::of(i16::MIN + 1, i16::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 2)), CartesianPoint::of(i16::MIN + 2, i16::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 3)), CartesianPoint::of(i16::MIN + 3, i16::MAX));

        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, u16::MAX - 1)), CartesianPoint::of(i16::MAX - 1, i16::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, u16::MAX - 2)), CartesianPoint::of(i16::MAX - 2, i16::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, u16::MAX - 3)), CartesianPoint::of(i16::MAX - 3, i16::MAX));
    }
}

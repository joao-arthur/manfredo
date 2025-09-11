type CartesianPoint = crate::cartesian::point::point_i32::PointI32;
type MatrixPoint = crate::matrix::point::point_u32::Point;

pub fn matrix_to_cartesian(point: &MatrixPoint) -> CartesianPoint {
    let x = i64::from(i32::MIN) + i64::from(point.col);
    let y = i64::from(i32::MAX) - i64::from(point.row);
    CartesianPoint { x: x as i32, y: y as i32 }
}

#[cfg(test)]
mod tests {
    use super::{CartesianPoint, MatrixPoint, matrix_to_cartesian};

    #[test]
    fn edges() {
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 0)), CartesianPoint::of(i32::MIN, i32::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u32::MAX, 0)), CartesianPoint::min());
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, u32::MAX)), CartesianPoint::max());
        assert_eq!(matrix_to_cartesian(&MatrixPoint::max()), CartesianPoint::of(i32::MAX, i32::MIN));
    }

    #[test]
    fn sequence_min() {
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(1, 1)), CartesianPoint::of(i32::MIN + 1, i32::MAX - 1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(2, 2)), CartesianPoint::of(i32::MIN + 2, i32::MAX - 2));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(3, 3)), CartesianPoint::of(i32::MIN + 3, i32::MAX - 3));
    }

    #[test]
    fn sequence_max() {
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u32::MAX - 1, u32::MAX - 1)), CartesianPoint::of(i32::MAX - 1, i32::MIN + 1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u32::MAX - 2, u32::MAX - 2)), CartesianPoint::of(i32::MAX - 2, i32::MIN + 2));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u32::MAX - 3, u32::MAX - 3)), CartesianPoint::of(i32::MAX - 3, i32::MIN + 3));
    }
}

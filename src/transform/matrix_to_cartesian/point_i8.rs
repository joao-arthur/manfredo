type CartesianPoint = crate::cartesian::point::point_i8::PointI8;
type MatrixPoint = crate::matrix::point::point_u8::PointU8;

pub fn matrix_to_cartesian(point: &MatrixPoint) -> CartesianPoint {
    let x = i16::from(i8::MIN) + i16::from(point.col);
    let y = i16::from(i8::MAX) - i16::from(point.row);
    CartesianPoint { x: x as i8, y: y as i8 }
}

#[cfg(test)]
mod tests {
    use super::{CartesianPoint, MatrixPoint, matrix_to_cartesian};

    #[test]
    fn edges() {
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 0)), CartesianPoint::of(i8::MIN, i8::MAX));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u8::MAX, 0)), CartesianPoint::min());
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, u8::MAX)), CartesianPoint::max());
        assert_eq!(matrix_to_cartesian(&MatrixPoint::max()), CartesianPoint::of(i8::MAX, i8::MIN));
    }

    #[test]
    fn sequence_min() {
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(1, 1)), CartesianPoint::of(i8::MIN + 1, i8::MAX - 1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(2, 2)), CartesianPoint::of(i8::MIN + 2, i8::MAX - 2));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(3, 3)), CartesianPoint::of(i8::MIN + 3, i8::MAX - 3));
    }

    #[test]
    fn sequence_max() {
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u8::MAX - 1, u8::MAX - 1)), CartesianPoint::of(i8::MAX - 1, i8::MIN + 1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u8::MAX - 2, u8::MAX - 2)), CartesianPoint::of(i8::MAX - 2, i8::MIN + 2));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(u8::MAX - 3, u8::MAX - 3)), CartesianPoint::of(i8::MAX - 3, i8::MIN + 3));
    }
}

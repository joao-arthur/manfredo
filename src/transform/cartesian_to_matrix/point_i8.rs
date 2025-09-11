type CartesianPoint = crate::cartesian::point::point_i8::Point;
type MatrixPoint = crate::matrix::point::point_u8::Point;

pub fn cartesian_to_matrix(point: &CartesianPoint) -> MatrixPoint {
    let row = i16::from(i8::MAX) - i16::from(point.y);
    let col = i16::from(point.x) - i16::from(i8::MIN);
    MatrixPoint { row: row as u8, col: col as u8 }
}

#[cfg(test)]
mod tests {
    use super::{CartesianPoint, MatrixPoint, cartesian_to_matrix};

    #[test]
    fn bounds() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::min()), MatrixPoint::of(u8::MAX, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i8::MAX, i8::MIN)), MatrixPoint::max());
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i8::MIN, i8::MAX)), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::max()), MatrixPoint::of(0, u8::MAX));
    }

    #[test]
    fn sequence_min() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i8::MIN + 1, i8::MIN + 1)), MatrixPoint::of(u8::MAX - 1, 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i8::MIN + 2, i8::MIN + 2)), MatrixPoint::of(u8::MAX - 2, 2));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i8::MIN + 3, i8::MIN + 3)), MatrixPoint::of(u8::MAX - 3, 3));
    }

    #[test]
    fn sequence_max() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i8::MAX - 1, i8::MAX - 1)), MatrixPoint::of(1, u8::MAX - 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i8::MAX - 2, i8::MAX - 2)), MatrixPoint::of(2, u8::MAX - 2));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i8::MAX - 3, i8::MAX - 3)), MatrixPoint::of(3, u8::MAX - 3));
    }
}

type CartesianPoint = crate::cartesian::point::point_i64::Point;
type MatrixPoint = crate::matrix::point::point_u64::Point;

pub fn cartesian_to_matrix(point: &CartesianPoint) -> MatrixPoint {
    let row = i128::from(i64::MAX) - i128::from(point.y);
    let col = i128::from(point.x) - i128::from(i64::MIN);
    MatrixPoint { row: row as u64, col: col as u64 }
}

#[cfg(test)]
mod tests {
    use super::{CartesianPoint, MatrixPoint, cartesian_to_matrix};

    #[test]
    fn bounds() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::min()), MatrixPoint::of(u64::MAX, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i64::MAX, i64::MIN)), MatrixPoint::max());
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i64::MIN, i64::MAX)), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::max()), MatrixPoint::of(0, u64::MAX));
    }

    #[test]
    fn sequence_min() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i64::MIN + 1, i64::MIN + 1)), MatrixPoint::of(u64::MAX - 1, 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i64::MIN + 2, i64::MIN + 2)), MatrixPoint::of(u64::MAX - 2, 2));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i64::MIN + 3, i64::MIN + 3)), MatrixPoint::of(u64::MAX - 3, 3));
    }

    #[test]
    fn sequence_max() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i64::MAX - 1, i64::MAX - 1)), MatrixPoint::of(1, u64::MAX - 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i64::MAX - 2, i64::MAX - 2)), MatrixPoint::of(2, u64::MAX - 2));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i64::MAX - 3, i64::MAX - 3)), MatrixPoint::of(3, u64::MAX - 3));
    }
}

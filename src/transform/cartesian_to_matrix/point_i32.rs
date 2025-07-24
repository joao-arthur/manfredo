type CartesianPoint = crate::cartesian::point::point_i32::PointI32;
type MatrixPoint = crate::matrix::point::point_u32::PointU32;

pub fn cartesian_to_matrix(point: &CartesianPoint) -> MatrixPoint {
    let temp_row = i64::from(i32::MAX) - i64::from(point.y);
    let temp_col = i64::from(i32::MAX) + i64::from(point.x) + 1;
    let row = temp_row.clamp(0, i64::from(u32::MAX));
    let col = temp_col.clamp(0, i64::from(u32::MAX));
    MatrixPoint { row: row as u32, col: col as u32 }
}

#[cfg(test)]
mod tests {
    use super::{CartesianPoint, MatrixPoint, cartesian_to_matrix};

    #[test]
    fn cartesian_to_matrix_bounds() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i32::MIN, i32::MIN)), MatrixPoint::of(u32::MAX, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i32::MAX, i32::MIN)), MatrixPoint::of(u32::MAX, u32::MAX));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i32::MIN, i32::MAX)), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i32::MAX, i32::MAX)), MatrixPoint::of(0, u32::MAX));
    }

    #[test]
    fn cartesian_to_matrix_sequence_min() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i32::MIN + 1, i32::MIN + 1)), MatrixPoint::of(u32::MAX - 1, 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i32::MIN + 2, i32::MIN + 2)), MatrixPoint::of(u32::MAX - 2, 2));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i32::MIN + 3, i32::MIN + 3)), MatrixPoint::of(u32::MAX - 3, 3));
    }

    #[test]
    fn cartesian_to_matrix_sequence_max() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i32::MAX - 1, i32::MAX - 1)), MatrixPoint::of(1, u32::MAX - 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i32::MAX - 2, i32::MAX - 2)), MatrixPoint::of(2, u32::MAX - 2));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i32::MAX - 3, i32::MAX - 3)), MatrixPoint::of(3, u32::MAX - 3));
    }
}

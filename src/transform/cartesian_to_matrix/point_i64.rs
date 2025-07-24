type CartesianPoint = crate::cartesian::point::point_i64::PointI64;
type MatrixPoint = crate::matrix::point::point_u64::PointU64;

pub fn cartesian_to_matrix(point: &CartesianPoint) -> MatrixPoint {
    let temp_row = i128::from(i64::MAX) - i128::from(point.y);
    let temp_col = i128::from(i64::MAX) + i128::from(point.x) + 1;
    let row = temp_row.clamp(0, i128::from(u64::MAX));
    let col = temp_col.clamp(0, i128::from(u64::MAX));
    MatrixPoint { row: row as u64, col: col as u64 }
}

#[cfg(test)]
mod tests {
    use super::{CartesianPoint, MatrixPoint, cartesian_to_matrix};

    #[test]
    fn test_cartesian_to_matrix() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i64::MIN, i64::MIN)), MatrixPoint::of(u64::MAX, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i64::MIN, i64::MAX)), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i64::MAX, i64::MIN)), MatrixPoint::of(u64::MAX, u64::MAX));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i64::MAX, i64::MAX)), MatrixPoint::of(0, u64::MAX));
    }
}

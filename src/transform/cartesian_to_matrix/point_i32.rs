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
    fn test_cartesian_to_matrix() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i32::MIN, i32::MIN)), MatrixPoint::of(u32::MAX, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i32::MAX, i32::MIN)), MatrixPoint::of(u32::MAX, u32::MAX));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i32::MIN, i32::MAX)), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i32::MAX, i32::MAX)), MatrixPoint::of(0, u32::MAX));
    }
}

type CartesianPoint = crate::cartesian::point::point_i16::PointI16;
type MatrixPoint = crate::matrix::point::point_u16::PointU16;

pub fn cartesian_to_matrix(point: &CartesianPoint) -> MatrixPoint {
    let temp_row = i32::from(i16::MAX) - i32::from(point.y);
    let temp_col = i32::from(i16::MAX) + i32::from(point.x) + 1;
    let row = temp_row.clamp(0, i32::from(u16::MAX));
    let col = temp_col.clamp(0, i32::from(u16::MAX));
    MatrixPoint { row: row as u16, col: col as u16 }
}

#[cfg(test)]
mod tests {
    use super::{CartesianPoint, MatrixPoint, cartesian_to_matrix};

    #[test]
    fn test_cartesian_to_matrix() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i16::MIN, i16::MIN)), MatrixPoint::of(u16::MAX, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i16::MIN, i16::MAX)), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i16::MAX, i16::MIN)), MatrixPoint::of(u16::MAX, u16::MAX));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i16::MAX, i16::MAX)), MatrixPoint::of(0, u16::MAX));
    }
}

type CartesianPoint = crate::cartesian::point::point_i8::PointI8;
type MatrixPoint = crate::matrix::point::point_u8::PointU8;

pub fn cartesian_to_matrix(point: &CartesianPoint) -> MatrixPoint {
    let temp_row = i16::from(i8::MAX) - i16::from(point.y);
    let temp_col = i16::from(i8::MAX) + i16::from(point.x) + 1;
    let row = temp_row.clamp(0, i16::from(u8::MAX));
    let col = temp_col.clamp(0, i16::from(u8::MAX));
    MatrixPoint { row: row as u8, col: col as u8 }
}

#[cfg(test)]
mod tests {
    use super::{CartesianPoint, MatrixPoint, cartesian_to_matrix};

    #[test]
    fn test_cartesian_to_matrix() {
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i8::MIN, i8::MIN)), MatrixPoint::of(u8::MAX, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i8::MIN, i8::MAX)), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i8::MAX, i8::MIN)), MatrixPoint::of(u8::MAX, u8::MAX));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(i8::MAX, i8::MAX)), MatrixPoint::of(0, u8::MAX));
    }
}

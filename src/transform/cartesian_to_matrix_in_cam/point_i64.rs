type CartesianPoint = crate::cartesian::point::point_i64::PointI64;
type MatrixPoint = crate::matrix::point::point_u64::PointU64;
type Cam = crate::matrix::rect::rect_u64::RectU64;

pub fn cartesian_to_matrix_in_cam(point: &CartesianPoint, cam: &Cam) -> MatrixPoint {
    let row = i128::from(i64::MAX) - i128::from(point.y) + i128::from(cam.min.row);
    let col = i128::from(point.x) - i128::from(i64::MIN) + i128::from(cam.min.col);
    MatrixPoint { row: row as u64, col: col as u64 }
}

#[cfg(test)]
mod tests {
    use super::{Cam, CartesianPoint, MatrixPoint, cartesian_to_matrix_in_cam};

    #[test]
    fn test_3x3() {
        let cam = Cam::of(0, 0, 2, 2);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN, i64::MAX - 2), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN, i64::MAX - 1), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN, i64::MAX), &cam), MatrixPoint::of(0, 0));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN + 1, i64::MAX - 2), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN + 1, i64::MAX - 1), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN + 1, i64::MAX), &cam), MatrixPoint::of(0, 1));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN + 2, i64::MAX - 2), &cam), MatrixPoint::of(2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN + 2, i64::MAX - 1), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN + 2, i64::MAX), &cam), MatrixPoint::of(0, 2));
    }

    #[test]
    fn test_4x4() {
        let cam = Cam::of(10, 10, 13, 13);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN, i64::MAX - 3), &cam), MatrixPoint::of(13, 10));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN, i64::MAX - 2), &cam), MatrixPoint::of(12, 10));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN, i64::MAX - 1), &cam), MatrixPoint::of(11, 10));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN, i64::MAX), &cam), MatrixPoint::of(10, 10));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN + 1, i64::MAX - 3), &cam), MatrixPoint::of(13, 11));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN + 1, i64::MAX - 2), &cam), MatrixPoint::of(12, 11));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN + 1, i64::MAX - 1), &cam), MatrixPoint::of(11, 11));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN + 1, i64::MAX), &cam), MatrixPoint::of(10, 11));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN + 2, i64::MAX - 3), &cam), MatrixPoint::of(13, 12));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN + 2, i64::MAX - 2), &cam), MatrixPoint::of(12, 12));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN + 2, i64::MAX - 1), &cam), MatrixPoint::of(11, 12));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN + 2, i64::MAX), &cam), MatrixPoint::of(10, 12));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN + 3, i64::MAX - 3), &cam), MatrixPoint::of(13, 13));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN + 3, i64::MAX - 2), &cam), MatrixPoint::of(12, 13));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN + 3, i64::MAX - 1), &cam), MatrixPoint::of(11, 13));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN + 3, i64::MAX), &cam), MatrixPoint::of(10, 13));
    }

    #[test]
    fn bounds() {
        let cam = Cam::largest();
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::min(), &cam), MatrixPoint::of(u64::MAX, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MAX, i64::MIN), &cam), MatrixPoint::max());
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN, i64::MAX), &cam), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::max(), &cam), MatrixPoint::of(0, u64::MAX));
    }

    #[test]
    fn sequence_min() {
        let cam = Cam::largest();
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN + 1, i64::MIN + 1), &cam), MatrixPoint::of(u64::MAX - 1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN + 2, i64::MIN + 2), &cam), MatrixPoint::of(u64::MAX - 2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MIN + 3, i64::MIN + 3), &cam), MatrixPoint::of(u64::MAX - 3, 3));
    }

    #[test]
    fn sequence_max() {
        let cam = Cam::largest();
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MAX - 1, i64::MAX - 1), &cam), MatrixPoint::of(1, u64::MAX - 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MAX - 2, i64::MAX - 2), &cam), MatrixPoint::of(2, u64::MAX - 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i64::MAX - 3, i64::MAX - 3), &cam), MatrixPoint::of(3, u64::MAX - 3));
    }
}

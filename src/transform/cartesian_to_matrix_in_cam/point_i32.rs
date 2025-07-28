type CartesianPoint = crate::cartesian::point::point_i32::PointI32;
type MatrixPoint = crate::matrix::point::point_u32::PointU32;
type Cam = crate::matrix::rect::rect_u32::RectU32;

pub fn cartesian_to_matrix_in_cam(point: &CartesianPoint, cam: &Cam) -> MatrixPoint {
    let row = i64::from(i32::MAX) - i64::from(point.y) + i64::from(cam.min.row);
    let col = i64::from(point.x) - i64::from(i32::MIN) + i64::from(cam.min.col);
    MatrixPoint { row: row as u32, col: col as u32 }
}

#[cfg(test)]
mod tests {
    use super::{Cam, CartesianPoint, MatrixPoint, cartesian_to_matrix_in_cam};

    #[test]
    fn cartesian_to_matrix_in_cam_3x3() {
        let cam = Cam::of(0, 0, 2, 2);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN, i32::MAX - 2), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN, i32::MAX - 1), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN, i32::MAX), &cam), MatrixPoint::of(0, 0));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN + 1, i32::MAX - 2), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN + 1, i32::MAX - 1), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN + 1, i32::MAX), &cam), MatrixPoint::of(0, 1));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN + 2, i32::MAX - 2), &cam), MatrixPoint::of(2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN + 2, i32::MAX - 1), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN + 2, i32::MAX), &cam), MatrixPoint::of(0, 2));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_4x4() {
        let cam = Cam::of(10, 10, 13, 13);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN, i32::MAX - 3), &cam), MatrixPoint::of(13, 10));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN, i32::MAX - 2), &cam), MatrixPoint::of(12, 10));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN, i32::MAX - 1), &cam), MatrixPoint::of(11, 10));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN, i32::MAX), &cam), MatrixPoint::of(10, 10));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN + 1, i32::MAX - 3), &cam), MatrixPoint::of(13, 11));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN + 1, i32::MAX - 2), &cam), MatrixPoint::of(12, 11));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN + 1, i32::MAX - 1), &cam), MatrixPoint::of(11, 11));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN + 1, i32::MAX), &cam), MatrixPoint::of(10, 11));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN + 2, i32::MAX - 3), &cam), MatrixPoint::of(13, 12));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN + 2, i32::MAX - 2), &cam), MatrixPoint::of(12, 12));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN + 2, i32::MAX - 1), &cam), MatrixPoint::of(11, 12));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN + 2, i32::MAX), &cam), MatrixPoint::of(10, 12));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN + 3, i32::MAX - 3), &cam), MatrixPoint::of(13, 13));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN + 3, i32::MAX - 2), &cam), MatrixPoint::of(12, 13));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN + 3, i32::MAX - 1), &cam), MatrixPoint::of(11, 13));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN + 3, i32::MAX), &cam), MatrixPoint::of(10, 13));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_bounds() {
        let cam = Cam::of(0, 0, u32::MAX, u32::MAX);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN, i32::MIN), &cam), MatrixPoint::of(u32::MAX, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MAX, i32::MIN), &cam), MatrixPoint::of(u32::MAX, u32::MAX));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN, i32::MAX), &cam), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MAX, i32::MAX), &cam), MatrixPoint::of(0, u32::MAX));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_sequence_min() {
        let cam = Cam::of(0, 0, u32::MAX, u32::MAX);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN + 1, i32::MIN + 1), &cam), MatrixPoint::of(u32::MAX - 1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN + 2, i32::MIN + 2), &cam), MatrixPoint::of(u32::MAX - 2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MIN + 3, i32::MIN + 3), &cam), MatrixPoint::of(u32::MAX - 3, 3));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_sequence_max() {
        let cam = Cam::of(0, 0, u32::MAX, u32::MAX);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MAX - 1, i32::MAX - 1), &cam), MatrixPoint::of(1, u32::MAX - 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MAX - 2, i32::MAX - 2), &cam), MatrixPoint::of(2, u32::MAX - 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i32::MAX - 3, i32::MAX - 3), &cam), MatrixPoint::of(3, u32::MAX - 3));
    }
}

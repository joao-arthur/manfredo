type CartesianPoint = crate::cartesian::point::point_i64::PointI64;
type MatrixPoint = crate::matrix::point::point_u64::PointU64;
type Cam = crate::cartesian::rect::rect_i64::RectI64;

pub fn cartesian_in_cam_to_matrix(point: &CartesianPoint, cam: &Cam) -> MatrixPoint {
    let row = i128::from(cam.max.y) - i128::from(point.y);
    let col = i128::from(point.x) - i128::from(cam.min.x);
    MatrixPoint { row: row as u64, col: col as u64 }
}

#[cfg(test)]
mod tests {
    use super::{Cam, CartesianPoint, MatrixPoint, cartesian_in_cam_to_matrix};

    #[test]
    fn cartesian_to_matrix_in_cam_3x3() {
        let cam = Cam::of(-1, -1, 1, 1);
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(-1, -1), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(-1, 0), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(-1, 1), &cam), MatrixPoint::of(0, 0));

        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(0, -1), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(0, 0), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(0, 1), &cam), MatrixPoint::of(0, 1));

        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(1, -1), &cam), MatrixPoint::of(2, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(1, 0), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(1, 1), &cam), MatrixPoint::of(0, 2));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_4x4() {
        let cam = Cam::of(-2, -2, 1, 1);
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(-2, -2), &cam), MatrixPoint::of(3, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(-2, -1), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(-2, 0), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(-2, 1), &cam), MatrixPoint::of(0, 0));

        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(-1, -2), &cam), MatrixPoint::of(3, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(-1, -1), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(-1, 0), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(-1, 1), &cam), MatrixPoint::of(0, 1));

        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(0, -2), &cam), MatrixPoint::of(3, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(0, -1), &cam), MatrixPoint::of(2, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(0, 0), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(0, 1), &cam), MatrixPoint::of(0, 2));

        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(1, -2), &cam), MatrixPoint::of(3, 3));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(1, -1), &cam), MatrixPoint::of(2, 3));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(1, 0), &cam), MatrixPoint::of(1, 3));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(1, 1), &cam), MatrixPoint::of(0, 3));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_bounds() {
        let cam = Cam::of(i64::MIN, i64::MIN, i64::MAX, i64::MAX);
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i64::MIN, i64::MIN), &cam), MatrixPoint::of(u64::MAX, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i64::MAX, i64::MIN), &cam), MatrixPoint::of(u64::MAX, u64::MAX));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i64::MIN, i64::MAX), &cam), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i64::MAX, i64::MAX), &cam), MatrixPoint::of(0, u64::MAX));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_sequence_min() {
        let cam = Cam::of(i64::MIN, i64::MIN, i64::MAX, i64::MAX);
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i64::MIN + 1, i64::MIN + 1), &cam), MatrixPoint::of(u64::MAX - 1, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i64::MIN + 2, i64::MIN + 2), &cam), MatrixPoint::of(u64::MAX - 2, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i64::MIN + 3, i64::MIN + 3), &cam), MatrixPoint::of(u64::MAX - 3, 3));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_sequence_max() {
        let cam = Cam::of(i64::MIN, i64::MIN, i64::MAX, i64::MAX);
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i64::MAX - 1, i64::MAX - 1), &cam), MatrixPoint::of(1, u64::MAX - 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i64::MAX - 2, i64::MAX - 2), &cam), MatrixPoint::of(2, u64::MAX - 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i64::MAX - 3, i64::MAX - 3), &cam), MatrixPoint::of(3, u64::MAX - 3));
    }
}

use crate::cartesian::rect::rect_u64::RectU64;

type CartesianPoint = crate::cartesian::point::point_u64::PointU64;
type MatrixPoint = crate::matrix::point::point_u64::PointU64;

pub fn cartesian_to_matrix_in_cam(point: &CartesianPoint, cam: &RectU64) -> MatrixPoint {
    let row = i128::from(cam.max.y) - i128::from(point.y);
    let col = i128::from(point.x) - i128::from(cam.min.x);
    MatrixPoint { row: row as u64, col: col as u64 }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_u64::RectU64;

    use super::{CartesianPoint, MatrixPoint, cartesian_to_matrix_in_cam};

    #[test]
    fn cartesian_to_matrix_in_cam_3x3() {
        let cam = RectU64::of(0, 0, 2, 2);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 0), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 1), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 2), &cam), MatrixPoint::of(0, 0));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 0), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 1), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 2), &cam), MatrixPoint::of(0, 1));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, 0), &cam), MatrixPoint::of(2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, 1), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, 2), &cam), MatrixPoint::of(0, 2));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_4x4() {
        let cam = RectU64::of(0, 0, 3, 3);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 0), &cam), MatrixPoint::of(3, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 1), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 2), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 3), &cam), MatrixPoint::of(0, 0));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 0), &cam), MatrixPoint::of(3, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 1), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 2), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 3), &cam), MatrixPoint::of(0, 1));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, 0), &cam), MatrixPoint::of(3, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, 1), &cam), MatrixPoint::of(2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, 2), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, 3), &cam), MatrixPoint::of(0, 2));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(3, 0), &cam), MatrixPoint::of(3, 3));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(3, 1), &cam), MatrixPoint::of(2, 3));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(3, 2), &cam), MatrixPoint::of(1, 3));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(3, 3), &cam), MatrixPoint::of(0, 3));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_bounds() {
        let cam = RectU64::of(0, 0, u64::MAX, u64::MAX);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 0), &cam), MatrixPoint::of(u64::MAX, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u64::MAX, 0), &cam), MatrixPoint::of(u64::MAX, u64::MAX));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, u64::MAX), &cam), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u64::MAX, u64::MAX), &cam), MatrixPoint::of(0, u64::MAX));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_sequence_min() {
        let cam = RectU64::of(0, 0, u64::MAX, u64::MAX);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 1), &cam), MatrixPoint::of(u64::MAX - 1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, 2), &cam), MatrixPoint::of(u64::MAX - 2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(3, 3), &cam), MatrixPoint::of(u64::MAX - 3, 3));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_sequence_max() {
        let cam = RectU64::of(0, 0, u64::MAX, u64::MAX);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u64::MAX - 1, u64::MAX - 1), &cam), MatrixPoint::of(1, u64::MAX - 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u64::MAX - 2, u64::MAX - 2), &cam), MatrixPoint::of(2, u64::MAX - 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u64::MAX - 3, u64::MAX - 3), &cam), MatrixPoint::of(3, u64::MAX - 3));
    }
}

use crate::cartesian::rect::rect_u32::RectU32;

type CartesianPoint = crate::cartesian::point::point_u32::PointU32;
type MatrixPoint = crate::matrix::point::point_u32::PointU32;

pub fn cartesian_to_matrix_in_cam(point: &CartesianPoint, cam: &RectU32) -> MatrixPoint {
    let row = i64::from(cam.max.y) - i64::from(point.y);
    let col = i64::from(point.x) - i64::from(cam.min.x);
    MatrixPoint { row: row as u32, col: col as u32 }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_u32::RectU32;

    use super::{CartesianPoint, MatrixPoint, cartesian_to_matrix_in_cam};

    #[test]
    fn cartesian_to_matrix_in_cam_3x3() {
        let cam = RectU32::of(0, 0, 2, 2);
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
        let cam = RectU32::of(0, 0, 3, 3);
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
        let cam = RectU32::of(0, 0, u32::MAX, u32::MAX);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 0), &cam), MatrixPoint::of(u32::MAX, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u32::MAX, 0), &cam), MatrixPoint::of(u32::MAX, u32::MAX));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, u32::MAX), &cam), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u32::MAX, u32::MAX), &cam), MatrixPoint::of(0, u32::MAX));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_sequence_min() {
        let cam = RectU32::of(0, 0, u32::MAX, u32::MAX);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 1), &cam), MatrixPoint::of(u32::MAX - 1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, 2), &cam), MatrixPoint::of(u32::MAX - 2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(3, 3), &cam), MatrixPoint::of(u32::MAX - 3, 3));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_sequence_max() {
        let cam = RectU32::of(0, 0, u32::MAX, u32::MAX);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u32::MAX - 1, u32::MAX - 1), &cam), MatrixPoint::of(1, u32::MAX - 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u32::MAX - 2, u32::MAX - 2), &cam), MatrixPoint::of(2, u32::MAX - 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u32::MAX - 3, u32::MAX - 3), &cam), MatrixPoint::of(3, u32::MAX - 3));
    }
}

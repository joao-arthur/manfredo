type CartesianPoint = crate::cartesian::point::point_u32::PointU32;
type MatrixPoint = crate::matrix::point::point_u32::PointU32;
type Cam = crate::cartesian::rect::rect_u32::RectU32;

pub fn cartesian_in_cam_to_matrix(point: &CartesianPoint, cam: &Cam) -> MatrixPoint {
    MatrixPoint { row: cam.max.y - point.y, col: point.x - cam.min.x }
}

#[cfg(test)]
mod tests {
    use super::{Cam, CartesianPoint, MatrixPoint, cartesian_in_cam_to_matrix};

    #[test]
    fn test_3x3() {
        let cam = Cam::of(0, 0, 2, 2);
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(0, 0), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(0, 1), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(0, 2), &cam), MatrixPoint::of(0, 0));

        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(1, 0), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(1, 1), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(1, 2), &cam), MatrixPoint::of(0, 1));

        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(2, 0), &cam), MatrixPoint::of(2, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(2, 1), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(2, 2), &cam), MatrixPoint::of(0, 2));
    }

    #[test]
    fn test_4x4() {
        let cam = Cam::of(10, 10, 13, 13);
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(10, 10), &cam), MatrixPoint::of(3, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(10, 11), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(10, 12), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(10, 13), &cam), MatrixPoint::of(0, 0));

        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(11, 10), &cam), MatrixPoint::of(3, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(11, 11), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(11, 12), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(11, 13), &cam), MatrixPoint::of(0, 1));

        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(12, 10), &cam), MatrixPoint::of(3, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(12, 11), &cam), MatrixPoint::of(2, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(12, 12), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(12, 13), &cam), MatrixPoint::of(0, 2));

        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(13, 10), &cam), MatrixPoint::of(3, 3));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(13, 11), &cam), MatrixPoint::of(2, 3));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(13, 12), &cam), MatrixPoint::of(1, 3));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(13, 13), &cam), MatrixPoint::of(0, 3));
    }

    #[test]
    fn bounds() {
        let cam = Cam::largest();
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(0, 0), &cam), MatrixPoint::of(u32::MAX, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(u32::MAX, 0), &cam), MatrixPoint::max());
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(0, u32::MAX), &cam), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::max(), &cam), MatrixPoint::of(0, u32::MAX));
    }

    #[test]
    fn sequence_min() {
        let cam = Cam::largest();
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(1, 1), &cam), MatrixPoint::of(u32::MAX - 1, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(2, 2), &cam), MatrixPoint::of(u32::MAX - 2, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(3, 3), &cam), MatrixPoint::of(u32::MAX - 3, 3));
    }

    #[test]
    fn sequence_max() {
        let cam = Cam::largest();
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(u32::MAX - 1, u32::MAX - 1), &cam), MatrixPoint::of(1, u32::MAX - 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(u32::MAX - 2, u32::MAX - 2), &cam), MatrixPoint::of(2, u32::MAX - 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(u32::MAX - 3, u32::MAX - 3), &cam), MatrixPoint::of(3, u32::MAX - 3));
    }
}
